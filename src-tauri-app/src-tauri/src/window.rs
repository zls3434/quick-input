//! 浮层窗口系统模块 (M1)
//!
//! 负责 QuickInput 置顶浮层窗口的创建、系统级置顶/不抢焦点样式设置、
//! 以及窗口句柄访问。首期实现 Windows 平台。

use tauri::{AppHandle, Manager, WebviewWindow};

/// 浮层窗口的 label（与 tauri.conf.json 中的 label 一致）
pub const OVERLAY_WINDOW_LABEL: &str = "overlay";

/// 各布局的默认窗口尺寸（逻辑像素）
pub const VERTICAL_SIZE: (f64, f64) = (300.0, 400.0);
pub const HORIZONTAL_SIZE: (f64, f64) = (720.0, 116.0);

/// 布局对应的窗口尺寸
pub fn layout_size(layout: &str) -> (f64, f64) {
    if layout == "horizontal" {
        HORIZONTAL_SIZE
    } else {
        VERTICAL_SIZE
    }
}

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
/// - 竖向：屏幕右上角，距顶部约一个标题栏+菜单栏（64 逻辑像素）
/// - 横向：屏幕底部工作区上方，水平居中
pub fn default_overlay_position(app: &AppHandle, layout: &str, w: f64, h: f64) -> (i32, i32) {
    let scale = get_overlay_window(app)
        .and_then(|win| win.scale_factor().ok())
        .unwrap_or(1.0);
    let (wx, wy, wr, wb) = primary_work_area();
    // 物理坐标 → 逻辑坐标
    let (left, top, right, bottom) = (
        wx as f64 / scale,
        wy as f64 / scale,
        wr as f64 / scale,
        wb as f64 / scale,
    );
    if layout == "horizontal" {
        let x = left + (right - left - w) / 2.0;
        let y = bottom - h - 8.0;
        (x.round() as i32, y.round().max(0.0) as i32)
    } else {
        let x = right - w - 8.0;
        let y = top + 64.0;
        (x.round() as i32, y.round().max(0.0) as i32)
    }
}

/// 应用悬浮窗几何：按布局设置尺寸，并定位到记忆位置或默认位置
pub fn apply_overlay_geometry(app: &AppHandle, layout: &str) {
    use tauri::LogicalPosition;

    let Some(win) = get_overlay_window(app) else {
        return;
    };
    let (w, h) = layout_size(layout);
    let _ = win.set_size(tauri::LogicalSize::new(w, h));

    let state = app.state::<crate::AppState>();
    let overlay = state
        .config_manager
        .lock()
        .ok()
        .and_then(|mgr| mgr.config().overlay.clone())
        .unwrap_or_default();
    let (x, y) = overlay
        .saved_position(layout)
        .unwrap_or_else(|| default_overlay_position(app, layout, w, h));
    let _ = win.set_position(LogicalPosition::new(x, y));
}

/// 获取浮层窗口句柄
pub fn get_overlay_window(app: &AppHandle) -> Option<WebviewWindow> {
    app.get_webview_window(OVERLAY_WINDOW_LABEL)
}

/// 应用置顶窗口系统级样式
///
/// 在 Tauri 窗口创建后，通过原生 API 设置窗口扩展样式以确保：
/// 1. 窗口始终置顶 (WS_EX_TOPMOST / HWND_TOPMOST)
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

        unsafe {
            // 设置扩展样式：不抢焦点 + 置顶 + 工具窗口（不在任务栏）
            let ex_style = GetWindowLongPtrW(hwnd, GWL_EXSTYLE);
            let new_style = ex_style
                | (WS_EX_NOACTIVATE.0 as isize)
                | (WS_EX_TOPMOST.0 as isize)
                | (WS_EX_TOOLWINDOW.0 as isize);
            SetWindowLongPtrW(hwnd, GWL_EXSTYLE, new_style);

            // 确保 Z-order 置顶（仅改样式可能不生效）
            let _ = SetWindowPos(
                hwnd,
                HWND_TOPMOST,
                0, 0, 0, 0,
                SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE,
            );

            // 显示但不激活窗口
            let _ = ShowWindow(hwnd, SW_SHOWNOACTIVATE);
        }
    }

    Ok(())
}