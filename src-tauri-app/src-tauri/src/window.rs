//! 浮层窗口系统模块 (M1)
//!
//! 负责 QuickInput 置顶浮层窗口的创建、系统级置顶/不抢焦点样式设置、
//! 以及窗口句柄访问。首期实现 Windows 平台。

use tauri::{AppHandle, Manager, WebviewWindow};

/// 浮层窗口的 label（与 tauri.conf.json 中的 label 一致）
pub const OVERLAY_WINDOW_LABEL: &str = "overlay";

/// 获取主屏工作区（物理像素，排除任务栏）
#[cfg(target_os = "windows")]
fn primary_work_area() -> (i32, i32, i32, i32) {
    use windows::Win32::Foundation::RECT;
    use windows::Win32::UI::WindowsAndMessaging::{
        SystemParametersInfoW, SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS, SPI_GETWORKAREA,
    };

    let mut rect = RECT::default();
    let ok = unsafe {
        SystemParametersInfoW(
            SPI_GETWORKAREA,
            0,
            Some(&mut rect as *mut RECT as *mut std::ffi::c_void),
            SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS(0),
        )
    };
    if ok.is_ok() {
        (rect.left, rect.top, rect.right, rect.bottom)
    } else {
        (0, 0, 1920, 1040)
    }
}

#[cfg(not(target_os = "windows"))]
fn primary_work_area() -> (i32, i32, i32, i32) {
    (0, 0, 1920, 1040)
}

/// 计算布局默认位置（逻辑坐标）
///
/// - 竖向：屏幕右上角，距顶部约一个标题栏+菜单栏（64 逻辑像素），
///   距右边留 24 逻辑像素内边距（已补偿窗口边框宽度，保证外框不贴边/不超出屏幕）
/// - 横向：屏幕底部工作区上方，水平居中
pub fn default_overlay_position(app: &AppHandle, layout: &str, w: f64, h: f64) -> (i32, i32) {
    let win = get_overlay_window(app);
    let scale = win
        .as_ref()
        .and_then(|win| win.scale_factor().ok())
        .unwrap_or(1.0);
    // 窗口边框宽度补偿（outer - inner，物理像素 → 逻辑像素）：
    // set_size 的宽 w 是客户区宽度，而定位用的是窗口外框，
    // 不补偿会导致外框右侧被边框吃掉边距、贴边甚至超出屏幕。
    let border_w = win
        .as_ref()
        .and_then(|win| {
            let o = win.outer_size().ok()?;
            let i = win.inner_size().ok()?;
            Some(o.width.saturating_sub(i.width) as f64 / scale)
        })
        .unwrap_or(0.0);
    let (wx, wy, wr, wb) = primary_work_area();
    // 物理坐标 → 逻辑坐标
    let (left, top, right, bottom) = (
        wx as f64 / scale,
        wy as f64 / scale,
        wr as f64 / scale,
        wb as f64 / scale,
    );
    if layout == "horizontal" {
        let x = left + (right - left - w - border_w) / 2.0;
        let y = bottom - h - 8.0;
        (x.round() as i32, y.round().max(0.0) as i32)
    } else {
        let x = right - w - border_w - 24.0;
        let y = top + 64.0;
        (x.round() as i32, y.round().max(0.0) as i32)
    }
}

/// 读取配置中的生效布局（配置未加载或无 overlay 节时为竖向默认）
pub fn current_layout(app: &AppHandle) -> String {
    app.try_state::<crate::AppState>()
        .and_then(|state| {
            state
                .config_manager
                .lock()
                .ok()
                .and_then(|mgr| mgr.config().overlay.clone())
        })
        .map(|ov| ov.effective_layout().to_string())
        .unwrap_or_else(|| "vertical".to_string())
}

/// 应用悬浮窗几何：按布局设置尺寸（优先记忆尺寸），并定位到记忆位置或默认位置
///
/// 横向布局的高度此处设为单行高度，前端加载后按按钮行数自适应修正。
pub fn apply_overlay_geometry(app: &AppHandle, layout: &str) {
    use tauri::LogicalPosition;

    let Some(win) = get_overlay_window(app) else {
        return;
    };

    // try_state：on_page_load 可能早于 setup 触发（此时配置未加载），
    // 取不到状态时回退默认值，绝不 panic。
    let overlay = app
        .try_state::<crate::AppState>()
        .and_then(|state| {
            state
                .config_manager
                .lock()
                .ok()
                .and_then(|mgr| mgr.config().overlay.clone())
        })
        .unwrap_or_default();
    let (w, h) = overlay.effective_size(layout);
    let _ = win.set_size(tauri::LogicalSize::new(w as f64, h as f64));

    let (x, y) = overlay
        .saved_position(layout)
        .unwrap_or_else(|| default_overlay_position(app, layout, w as f64, h as f64));
    let _ = win.set_position(LogicalPosition::new(x, y));
}

/// 获取浮层窗口句柄
pub fn get_overlay_window(app: &AppHandle) -> Option<WebviewWindow> {
    app.get_webview_window(OVERLAY_WINDOW_LABEL)
}

/// 应用置顶窗口系统级样式
///
/// 在 Tauri 窗口创建后，通过原生 API 设置窗口扩展样式以确保：
/// 1. 窗口置顶按配置生效（WS_EX_TOPMOST / HWND_TOPMOST，可取消）
/// 2. 点击不激活/不抢焦点 (WS_EX_NOACTIVATE)
/// 3. 不在任务栏显示 (WS_EX_TOOLWINDOW)
pub fn apply_overlay_styles(app: &AppHandle) -> Result<(), anyhow::Error> {
    #[cfg(target_os = "windows")]
    {
        apply_windows_overlay_styles(app)?;
    }

    #[cfg(target_os = "macos")]
    {
        // macOS 使用 NSPanel 实现，预留
        let _ = app;
    }

    #[cfg(target_os = "linux")]
    {
        // Linux 使用 override_redirect 实现，预留
        let _ = app;
    }

    Ok(())
}

/// Windows 平台：通过 HWND 设置置顶/不抢焦点扩展样式
#[cfg(target_os = "windows")]
fn apply_windows_overlay_styles(app: &AppHandle) -> Result<(), anyhow::Error> {
    use raw_window_handle::{HasWindowHandle, RawWindowHandle};
    use windows::Win32::Foundation::HWND;
    use windows::Win32::UI::WindowsAndMessaging::*;

    let window = get_overlay_window(app)
        .ok_or_else(|| anyhow::anyhow!("浮层窗口 (overlay) 未找到"))?;

    let handle = window.window_handle()?;
    let raw = handle.as_raw();

    #[cfg(target_os = "windows")]
    if let RawWindowHandle::Win32(win32) = raw {
        let hwnd = HWND(win32.hwnd.get() as *mut std::ffi::c_void);

        // 置顶状态按配置生效（配置未加载时回退默认置顶，见 current_layout 同款 try_state 模式）
        let always_on_top = app
            .try_state::<crate::AppState>()
            .and_then(|state| {
                state
                    .config_manager
                    .lock()
                    .ok()
                    .and_then(|mgr| mgr.config().overlay.clone())
            })
            .map(|ov| ov.effective_always_on_top())
            .unwrap_or(true);

        unsafe {
            // 窗口样式：移除最大化/最小化按钮位。
            // 悬浮窗不是常规窗口，不应响应 Windows 屏幕快捷布局
            // （Aero Snap / Snap Layouts：拖到屏幕边缘分屏、拖到顶部最大化、
            //  Win+Up/Win+Arrow 等），这些行为依赖 WS_MAXIMIZEBOX/WS_MINIMIZEBOX；
            // 保留 WS_THICKFRAME（resizable 位）以维持拖拽缩放。
            // 注：tauri.conf.json 已配置 maximizable/minimizable=false 从源头禁用，
            // 此处再兜底一次（tao 在窗口显示时会按 flags 重新应用样式位）。
            let style = GetWindowLongPtrW(hwnd, GWL_STYLE);
            SetWindowLongPtrW(
                hwnd,
                GWL_STYLE,
                style & !(WS_MAXIMIZEBOX.0 as isize) & !(WS_MINIMIZEBOX.0 as isize),
            );

            // 设置扩展样式：不抢焦点 + 工具窗口（不在任务栏）；置顶位按配置设置/清除
            let ex_style = GetWindowLongPtrW(hwnd, GWL_EXSTYLE);
            let mut new_style =
                ex_style | (WS_EX_NOACTIVATE.0 as isize) | (WS_EX_TOOLWINDOW.0 as isize);
            let zorder;
            if always_on_top {
                new_style |= WS_EX_TOPMOST.0 as isize;
                zorder = HWND_TOPMOST;
            } else {
                new_style &= !(WS_EX_TOPMOST.0 as isize);
                zorder = HWND_NOTOPMOST;
            }
            SetWindowLongPtrW(hwnd, GWL_EXSTYLE, new_style);

            // 确保 Z-order 生效（仅改样式可能不生效；不带显示标志，维持隐藏）
            let _ = SetWindowPos(
                hwnd,
                zorder,
                0, 0, 0, 0,
                SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE,
            );

            // 窗口显示由前端页面加载完成后调用 show() 触发：
            // 此处过早 ShowWindow 会在 WebView2 内容（含透明背景 CSS）未就绪时
            // 暴露默认白底，造成启动白屏闪烁。
        }
    }

    Ok(())
}