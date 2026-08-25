//! 浮层窗口系统模块 (M1)
//!
//! 负责 QuickInput 置顶浮层窗口的创建、系统级置顶/不抢焦点样式设置、
//! 以及窗口句柄访问。首期实现 Windows 平台。

use tauri::{AppHandle, Emitter, Manager, WebviewWindow};

/// 模板输入弹窗打开前的系统前台窗口（关闭弹窗时恢复）
#[cfg(target_os = "windows")]
static DIALOG_PREV_FOREGROUND: std::sync::Mutex<Option<isize>> = std::sync::Mutex::new(None);

/// 切换悬浮窗"可输入"模式（模板输入弹窗期间临时启用，其余时间禁用）
///
/// - enabled=true：记录当前系统前台窗口 → 递归移除窗口树 WS_EX_NOACTIVATE
///   → 激活悬浮窗（SetForegroundWindow + AttachThreadInput 绕过前台锁），
///   使模板输入框获得真实键盘输入焦点。
/// - enabled=false：递归恢复 WS_EX_NOACTIVATE（点击不抢焦点）；若当前前台
///   仍是悬浮窗，则把前台还给弹窗打开前记录的窗口。
#[cfg(target_os = "windows")]
pub fn set_overlay_focusable(app: &AppHandle, enabled: bool) -> Result<(), String> {
    use raw_window_handle::{HasWindowHandle, RawWindowHandle};
    use windows::Win32::Foundation::{BOOL, HWND, LPARAM};
    use windows::Win32::System::Threading::{AttachThreadInput, GetCurrentThreadId};
    use windows::Win32::UI::WindowsAndMessaging::*;

    let window = get_overlay_window(app).ok_or_else(|| "浮层窗口 (overlay) 未找到".to_string())?;
    let handle = window.window_handle().map_err(|e| e.to_string())?;
    let raw = handle.as_raw();

    let hwnd = match raw {
        RawWindowHandle::Win32(win32) => HWND(win32.hwnd.get() as *mut std::ffi::c_void),
        _ => return Ok(()), // 非 Windows 平台忽略
    };

    // 递归为窗口树设置/移除 WS_EX_NOACTIVATE
    unsafe extern "system" fn collect_child(h: HWND, l: LPARAM) -> BOOL {
        let v = unsafe { &mut *(l.0 as *mut Vec<HWND>) };
        v.push(h);
        BOOL(1)
    }
    fn apply_recursive(h: HWND, no_activate: bool) {
        unsafe {
            let ex = GetWindowLongPtrW(h, GWL_EXSTYLE);
            let new_ex = if no_activate {
                ex | (WS_EX_NOACTIVATE.0 as isize)
            } else {
                ex & !(WS_EX_NOACTIVATE.0 as isize)
            };
            SetWindowLongPtrW(h, GWL_EXSTYLE, new_ex);
            let mut children: Vec<HWND> = Vec::new();
            EnumChildWindows(
                h,
                Some(collect_child),
                LPARAM(&mut children as *mut Vec<HWND> as isize),
            );
            for ch in children {
                apply_recursive(ch, no_activate);
            }
        }
    }

    // AttachThreadInput + SetForegroundWindow：非前台线程激活窗口的标准技巧
    fn activate(hwnd: HWND) {
        unsafe {
            let fg = GetForegroundWindow();
            let fg_tid = GetWindowThreadProcessId(fg, None);
            let my_tid = GetCurrentThreadId();
            let attached = fg_tid != 0 && fg_tid != my_tid;
            if attached {
                let _ = AttachThreadInput(my_tid, fg_tid, true);
            }
            let _ = SetForegroundWindow(hwnd);
            if attached {
                let _ = AttachThreadInput(my_tid, fg_tid, false);
            }
        }
    }

    if enabled {
        // 记录弹窗打开前的系统前台窗口（供关闭后还原）
        let fg = unsafe { GetForegroundWindow() };
        if !fg.0.is_null() && fg != hwnd {
            if let Ok(mut slot) = DIALOG_PREV_FOREGROUND.lock() {
                *slot = Some(fg.0 as isize);
            }
        }
        apply_recursive(hwnd, false);
        unsafe {
            let _ = SetWindowPos(
                hwnd,
                HWND_TOPMOST,
                0,
                0,
                0,
                0,
                SWP_NOMOVE | SWP_NOSIZE | SWP_FRAMECHANGED,
            );
        }
        activate(hwnd);
    } else {
        apply_recursive(hwnd, true);
        unsafe {
            let _ = SetWindowPos(
                hwnd,
                HWND_TOPMOST,
                0,
                0,
                0,
                0,
                SWP_NOMOVE | SWP_NOSIZE | SWP_FRAMECHANGED,
            );
        }
        // 仅当用户未自行切走时，把前台还给弹窗打开前的窗口
        let fg = unsafe { GetForegroundWindow() };
        if fg == hwnd {
            let prev = DIALOG_PREV_FOREGROUND
                .lock()
                .ok()
                .and_then(|mut s| s.take());
            if let Some(v) = prev {
                activate(HWND(v as *mut std::ffi::c_void));
            }
        }
    }

    Ok(())
}

/// 非 Windows 平台占位实现
#[cfg(not(target_os = "windows"))]
pub fn set_overlay_focusable(_app: &AppHandle, _enabled: bool) -> Result<(), String> {
    Ok(())
}

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

/// 应用悬浮窗几何：按布局设置尺寸（优先记忆尺寸），并按
/// 吸附记忆位置 > 自由拖动记忆位置 > 默认位置 的优先级定位
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

    // 吸附记忆优先：按当前前台目标实时推导吸附位置（目标不可用时回退）
    let (x, y) = snap_position_now(app, layout, w as f64, h as f64)
        .or_else(|| overlay.saved_position(layout))
        .unwrap_or_else(|| default_overlay_position(app, layout, w as f64, h as f64));
    // 记忆位置钳制到主屏工作区内：避免配置残留（拖动测试/历史版本把窗口拖出屏幕）
    // 导致恢复超屏位置——超屏透明窗口在 Windows 上会引发 DWM 合成异常（拖动卡顿）
    // 且边框超出屏幕边缘。
    let (x, y) = clamp_to_work_area(app, x, y);
    let _ = win.set_position(LogicalPosition::new(x, y));
}

/// 将位置钳制到主屏工作区（逻辑坐标），保证窗口主体在屏内
fn clamp_to_work_area(app: &AppHandle, x: i32, y: i32) -> (i32, i32) {
    let scale = get_overlay_window(app)
        .and_then(|win| win.scale_factor().ok())
        .unwrap_or(1.0);
    let (wx, wy, wr, wb) = primary_work_area();
    let (left, top, right, bottom) = (
        wx as f64 / scale,
        wy as f64 / scale,
        wr as f64 / scale,
        wb as f64 / scale,
    );
    // 水平：窗口左缘至少在工作区内，右缘最多距右边界 40 逻辑像素（窗口比屏宽时保持左对齐）
    let x_min = left as i32;
    let x_max = ((right - 40.0) as i32).max(x_min);
    let cx = x.clamp(x_min, x_max);
    // 垂直：窗口顶部至少在工作区内，底部最多距底边界 40 逻辑像素
    let y_min = top as i32;
    let y_max = ((bottom - 40.0) as i32).max(y_min);
    let cy = y.clamp(y_min, y_max);
    (cx, cy)
}

/// 获取悬浮窗句柄
pub fn get_overlay_window(app: &AppHandle) -> Option<WebviewWindow> {
    app.get_webview_window(OVERLAY_WINDOW_LABEL)
}

// ============================================================
// 边缘吸附
// ============================================================

/// 用户拖动会话进行中标志：拖动开始时由前端置位；
/// 结束（左键松开）由跟随线程检测，处理完毕后复位（见 [run_snap_follow]）
static OVERLAY_DRAGGING: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// 设置拖动会话标志（前端拖动开始时调用）
pub fn set_overlay_dragging(dragging: bool) {
    OVERLAY_DRAGGING.store(dragging, std::sync::atomic::Ordering::SeqCst);
}

/// 左键是否按下（物理状态，跨进程有效）
#[cfg(target_os = "windows")]
fn left_button_down() -> bool {
    use windows::Win32::UI::Input::KeyboardAndMouse::{GetAsyncKeyState, VK_LBUTTON};
    (unsafe { GetAsyncKeyState(VK_LBUTTON.0 as i32) } as u16 & 0x8000) != 0
}

#[cfg(not(target_os = "windows"))]
fn left_button_down() -> bool {
    false
}

/// 拖动会话结束处理：在恢复跟随之前，同步评估松手位置并更新吸附记忆
///
/// 顺序必须"先评估后跟随"，不可颠倒：若先恢复跟随，残留的旧吸附记忆
/// 会立即把悬浮窗拉回吸附位；随后前端 600ms 防抖保存时读到的又是
/// "已弹回"的位置，判定恒为命中 → 吸附永远无法通过拖动取消
/// （即"拖离边缘后弹回、吸附无法解除"缺陷的根因）。
fn handle_drag_end(app: &AppHandle) {
    let layout = current_layout(app);
    match evaluate_drag_snap(app) {
        DragSnapOutcome::Snapped { process, edge, offset, pos } => {
            if write_snap_edge(app, &process, &layout, Some((edge, offset))) {
                move_overlay(app, pos.0, pos.1);
            }
        }
        DragSnapOutcome::Missed { process } => {
            // 拖离吸附区：清除记忆即解除吸附，悬浮窗停在松手位置
            let _ = write_snap_edge(app, &process, &layout, None);
        }
        DragSnapOutcome::NoTarget => {}
    }
}

/// 写入/清除吸附记忆（边 + 沿边偏移）并落盘（无状态/加锁失败/校验失败返回 false）
fn write_snap_edge(
    app: &AppHandle,
    process: &str,
    layout: &str,
    edge: Option<(&str, f64)>,
) -> bool {
    let Some(state) = app.try_state::<crate::AppState>() else {
        return false;
    };
    let Ok(mut mgr) = state.config_manager.lock() else {
        return false;
    };
    let config = mgr.config_mut();
    let overlay = config.overlay.get_or_insert_with(Default::default);
    overlay.set_snap_edge(process, layout, edge);
    let probe = config.clone();
    if probe.validate().is_err() {
        return false;
    }
    mgr.save().is_ok()
}

/// 按当前前台目标实时推导吸附位置（逻辑坐标）
///
/// 读取配置中该目标进程在本布局的吸附边记忆；无记忆或目标不可用时返回 None。
/// 全屏/最大化目标自动把 win-* 边映射为同朝向 screen-* 边。
fn snap_position_now(app: &AppHandle, layout: &str, fallback_w: f64, fallback_h: f64) -> Option<(i32, i32)> {
    let win = get_overlay_window(app)?;
    let info = crate::target_window::current_target_window()?;
    let scale = win.scale_factor().ok().unwrap_or(1.0);
    let (ow, oh) = win
        .outer_size()
        .map(|s| (s.width as f64 / scale, s.height as f64 / scale))
        .unwrap_or((fallback_w, fallback_h));
    let (x, y) = snap_position_for(app, &info, layout, ow, oh, scale)?;
    Some((x.round() as i32, y.round() as i32))
}

/// 由目标状态 + 悬浮窗尺寸计算吸附位置（逻辑坐标；纯计算 + 配置读取）
///
/// 与探测解耦：跟随线程传入缓存的目标状态与尺寸即可高频调用，
/// 不重复探测进程/几何。
fn snap_position_for(
    app: &AppHandle,
    info: &crate::target_window::TargetWindowInfo,
    layout: &str,
    ow: f64,
    oh: f64,
    scale: f64,
) -> Option<(f64, f64)> {
    let overlay = app
        .try_state::<crate::AppState>()
        .and_then(|state| {
            state
                .config_manager
                .lock()
                .ok()
                .and_then(|mgr| mgr.config().overlay.clone())
        })?;
    let (edge, offset) = overlay.snap_edge_offset(&info.process_name, layout)?;
    let edge = crate::target_window::map_edge_for_target(&edge, info);
    crate::target_window::snapped_position(edge, ow, oh, offset, info, scale)
}

/// 拖动结束后的吸附判定结果
pub enum DragSnapOutcome {
    /// 命中吸附：目标进程名、吸附边、沿边偏移比例、吸附后位置（逻辑坐标）
    Snapped {
        process: String,
        edge: &'static str,
        offset: f64,
        pos: (i32, i32),
    },
    /// 未命中任何吸附点（目标存在）：应清除该进程本布局的吸附记忆
    Missed { process: String },
    /// 无可用目标（前台是本应用/探测失败）：不动吸附记忆
    NoTarget,
}

/// 拖动结束后评估吸附：读取悬浮窗当前实时几何（外框，逻辑坐标）做判定
///
/// 由 `save_overlay_geometry(user_drag=true)` 调用。位置取窗口实时值
/// 而非前端回传值，保证与物理状态一致。判定本身与布局无关
/// （横竖排吸附几何一致），布局归属由调用方写入记忆时区分。
pub fn evaluate_drag_snap(app: &AppHandle) -> DragSnapOutcome {
    let Some(win) = get_overlay_window(app) else {
        return DragSnapOutcome::NoTarget;
    };
    let Some(info) = crate::target_window::current_target_window() else {
        return DragSnapOutcome::NoTarget;
    };
    let (Ok(pos), Ok(outer), Ok(scale)) = (win.outer_position(), win.outer_size(), win.scale_factor())
    else {
        return DragSnapOutcome::NoTarget;
    };
    let (ox, oy) = (pos.x as f64 / scale, pos.y as f64 / scale);
    let (ow, oh) = (outer.width as f64 / scale, outer.height as f64 / scale);

    match crate::target_window::detect_snap_edge(ox, oy, ow, oh, &info, scale) {
        Some(edge) => {
            let offset = crate::target_window::offset_from_position(
                edge, ox, oy, ow, oh, &info, scale,
            );
            match crate::target_window::snapped_position(edge, ow, oh, offset, &info, scale) {
                Some((x, y)) => DragSnapOutcome::Snapped {
                    process: info.process_name,
                    edge,
                    offset,
                    pos: (x.round() as i32, y.round() as i32),
                },
                // 边合法则位置计算必成功；防御性回退
                None => DragSnapOutcome::Missed {
                    process: info.process_name,
                },
            }
        }
        None => DragSnapOutcome::Missed {
            process: info.process_name,
        },
    }
}

/// 移动悬浮窗到指定逻辑坐标（保持 Z-order/样式，仅改位置）
pub fn move_overlay(app: &AppHandle, x: i32, y: i32) {
    use tauri::LogicalPosition;
    if let Some(win) = get_overlay_window(app) {
        let _ = win.set_position(LogicalPosition::new(x, y));
    }
}

// ---- 跟随轮询节奏（自适应）----
// 目标几何变化期间快轮询（接近显示刷新率，跟随近实时无跳变感）；
// 静止后回落慢轮询。快轮询期间每轮仅微秒级 Win32 调用（无进程探测、
// 无跨线程代理），CPU 增量可忽略；稳态较旧实现更低（进程探测移出热路径）。
/// 快轮询间隔（毫秒）：目标几何变化期间生效
const FOLLOW_ACTIVE_MS: u64 = 8;
/// 慢轮询间隔（毫秒）：目标静止后生效（与焦点轮询同频）
const FOLLOW_IDLE_MS: u64 = 150;
/// 目标静止多久后退回慢轮询
const FOLLOW_SETTLE_MS: u64 = 250;

/// 直接 SetWindowPos 移动悬浮窗到物理像素坐标
///
/// 绕过 Tauri 跨线程事件代理（代理每跳增加至多一个事件循环周期的延迟，
/// 高频跟随时表现为抖动）；Win32 窗口操作本身线程安全。
/// 返回是否成功。
#[cfg(target_os = "windows")]
fn move_overlay_native(hwnd: isize, x: i32, y: i32) -> bool {
    use windows::Win32::Foundation::HWND;
    use windows::Win32::UI::WindowsAndMessaging::{
        SetWindowPos, SWP_NOACTIVATE, SWP_NOSIZE, SWP_NOZORDER,
    };
    let h = HWND(hwnd as *mut std::ffi::c_void);
    unsafe {
        SetWindowPos(
            h,
            HWND(std::ptr::null_mut()),
            x,
            y,
            0,
            0,
            SWP_NOSIZE | SWP_NOZORDER | SWP_NOACTIVATE,
        )
    }
    .is_ok()
}

/// 锚定方向下高度变化后的新 Y（物理像素，纯函数）
///
/// - anchor="top"：顶边不动（向下扩展）；"bottom"：底边不动（向上扩展）；
///   "center"：居中对称扩展
/// - 无吸附锚定时按 fallback_keep_top 选择保顶/保底
fn anchored_new_y(anchor: Option<&str>, fallback_keep_top: bool, cur_y: i32, cur_h: i32, new_h: i32) -> i32 {
    match anchor {
        Some("top") => cur_y,
        Some("bottom") => cur_y + cur_h - new_h,
        Some("center") => cur_y + (cur_h - new_h) / 2,
        _ => {
            if fallback_keep_top {
                cur_y
            } else {
                cur_y + cur_h - new_h
            }
        }
    }
}

/// 解析当前吸附锚定方向（探测前台目标 + 读吸附记忆 → top/bottom/center）
fn current_snap_anchor(app: &AppHandle) -> Option<String> {
    let info = crate::target_window::current_target_window()?;
    let overlay = app
        .try_state::<crate::AppState>()
        .and_then(|state| {
            state
                .config_manager
                .lock()
                .ok()
                .and_then(|mgr| mgr.config().overlay.clone())
        })?;
    let edge = overlay.snap_edge(&info.process_name, &current_layout(app))?;
    let edge = crate::target_window::map_edge_for_target(&edge, &info);
    crate::target_window::height_anchor_for_edge(edge).map(str::to_string)
}

/// 横排高度自适应：单次原生原子调整（位置 + 尺寸一次 SetWindowPos 生效）
///
/// 由前端测量目标客户区高度后调用。相比前端分步 setSize/setPosition：
/// - 无中间可见态（先改尺寸后改位置的两步会各产生一次可见跳变）
/// - 锚定方向解析（吸附边 → 保顶/保底/居中）在后端一次完成
/// - 目标客户区高度为物理像素
pub fn apply_overlay_height_anchored(
    app: &AppHandle,
    target_inner_h: u32,
    fallback_keep_top: bool,
) -> Result<(), String> {
    let win = get_overlay_window(app).ok_or_else(|| "浮层窗口 (overlay) 未找到".to_string())?;
    let pos = win.outer_position().map_err(|e| e.to_string())?;
    let inner = win.inner_size().map_err(|e| e.to_string())?;
    let outer = win.outer_size().map_err(|e| e.to_string())?;
    // 边框高度（外框 - 客户区，物理像素）
    let chrome_h = outer.height.saturating_sub(inner.height) as i32;
    let new_outer_h = target_inner_h as i32 + chrome_h;
    let anchor = current_snap_anchor(app);
    let new_y = anchored_new_y(
        anchor.as_deref(),
        fallback_keep_top,
        pos.y,
        outer.height as i32,
        new_outer_h,
    );

    #[cfg(target_os = "windows")]
    {
        use windows::Win32::Foundation::HWND;
        use windows::Win32::UI::WindowsAndMessaging::{
            SetWindowPos, SWP_NOACTIVATE, SWP_NOZORDER,
        };
        let h = overlay_hwnd(app).ok_or_else(|| "浮层窗口句柄解析失败".to_string())?;
        let hwnd = HWND(h as *mut std::ffi::c_void);
        let ok = unsafe {
            SetWindowPos(
                hwnd,
                HWND(std::ptr::null_mut()),
                pos.x,
                new_y,
                outer.width as i32,
                new_outer_h,
                SWP_NOZORDER | SWP_NOACTIVATE,
            )
        }
        .is_ok();
        if ok {
            Ok(())
        } else {
            Err("SetWindowPos 原子调整失败".to_string())
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        // 非 Windows 回退：分步调整（无可观测跳变问题的平台可接受）
        let _ = win.set_size(tauri::PhysicalSize::new(outer.width, new_outer_h as u32));
        let _ = win.set_position(tauri::PhysicalPosition::new(pos.x, new_y));
        Ok(())
    }
}

/// 解析悬浮窗原生句柄（生命周期内稳定，跟随线程缓存一次即可）
#[cfg(target_os = "windows")]
fn overlay_hwnd(app: &AppHandle) -> Option<isize> {
    use raw_window_handle::{HasWindowHandle, RawWindowHandle};
    let win = get_overlay_window(app)?;
    let raw = win.window_handle().ok()?.as_raw();
    if let RawWindowHandle::Win32(w) = raw {
        let v = w.hwnd.get() as isize;
        if v != 0 {
            return Some(v);
        }
    }
    None
}

/// 吸附跟随线程：自适应轮询，按吸附记忆与目标窗口实时状态重定位悬浮窗
///
/// 状态机：
/// - 拖动会话进行中（标志置位 + 左键按住）：暂停跟随，避免与系统
///   模态移动循环抢位（DragEnter 循环不向前端派发 mouseup，
///   故以物理按键状态判定会话结束）
/// - 拖动会话结束（标志置位 + 左键松开）：先同步评估松手位置并更新
///   吸附记忆（[handle_drag_end]），再复位标志、恢复跟随
/// - 目标解析：前台句柄不变时仅增量刷新几何（[refresh_geometry]），
///   进程名探测仅在前台切换时执行 → 稳态 CPU 趋近于零
/// - 定位：吸附位置换算为物理像素后直接 SetWindowPos（[move_overlay_native]），
///   与实际位置一致时不产生任何窗口消息
/// - 间隔自适应：目标几何变化后 FOLLOW_SETTLE_MS 内快轮询
///   （FOLLOW_ACTIVE_MS），随后回落 FOLLOW_IDLE_MS
/// - 应用退出（watcher_stopped）：线程结束
pub fn run_snap_follow(app: AppHandle) {
    #[cfg(target_os = "windows")]
    {
        use std::time::{Duration, Instant};
        use crate::target_window as tw;

        // 目标缓存：前台句柄 + 上次几何状态
        let mut tracker: Option<(isize, tw::TargetWindowInfo)> = None;
        // 悬浮窗句柄与 DPI 缓存：句柄稳定；DPI 仅在目标换显示器时刷新
        let mut ov_hwnd: Option<isize> = None;
        let mut scale: f64 = 1.0;
        let mut scale_mon: (i32, i32, i32, i32) = (i32::MIN, 0, 0, 0);
        // 最近一次"需要快轮询"的事件（目标几何变化 / 本线程移动了悬浮窗）
        let mut last_change = Instant::now();

        loop {
            if app
                .try_state::<crate::AppState>()
                .map(|s| s.watcher_stopped.load(std::sync::atomic::Ordering::SeqCst))
                .unwrap_or(true)
            {
                break;
            }

            // ---- 拖动会话状态机（先评估后跟随，顺序不可颠倒）----
            if OVERLAY_DRAGGING.load(std::sync::atomic::Ordering::SeqCst) {
                if left_button_down() {
                    std::thread::sleep(Duration::from_millis(FOLLOW_IDLE_MS));
                    continue;
                }
                handle_drag_end(&app);
                OVERLAY_DRAGGING.store(false, std::sync::atomic::Ordering::SeqCst);
                // 会话改写了位置与记忆：丢弃缓存，下轮全量对账
                tracker = None;
                last_change = Instant::now();
            }

            // ---- 目标解析与几何增量刷新 ----
            if let Some(hwnd) = tw::foreground_hwnd() {
                if tw::is_self_window(hwnd) {
                    // 前台是本应用（模板输入弹窗等）：无跟随目标
                    tracker = None;
                } else {
                    match &mut tracker {
                        Some((t_hwnd, info)) if *t_hwnd == hwnd => {
                            if tw::refresh_geometry(hwnd, info) {
                                last_change = Instant::now();
                            }
                        }
                        _ => match tw::probe_target(hwnd) {
                            Some(info) => {
                                // 前台进程切换：立即发射配置切换（本线程空闲档 150ms
                                // 内响应，快于 500ms 焦点轮询；探测已得到进程名，
                                // 零额外系统调用。与焦点监听线程经 current_process
                                // 互斥去重，先到先得，不重复发射）
                                if let Some(state) = app.try_state::<crate::AppState>() {
                                    let should_emit = {
                                        let mut cur = state.current_process.lock().unwrap();
                                        if cur.eq_ignore_ascii_case(&info.process_name) {
                                            false
                                        } else {
                                            *cur = info.process_name.clone();
                                            true
                                        }
                                    };
                                    if should_emit {
                                        let _ = app.emit("ConfigSwitched", ());
                                    }
                                }
                                tracker = Some((hwnd, info));
                                last_change = Instant::now();
                            }
                            None => tracker = None,
                        },
                    }
                }
            } else {
                tracker = None;
            }

            // ---- 吸附定位（有缓存目标才计算；已到位则零开销跳过）----
            if let Some((_, info)) = &tracker {
                // 目标换显示器时刷新 DPI 缓存（混合 DPI 多屏）
                if scale_mon != info.monitor_rect {
                    if let Some(win) = get_overlay_window(&app) {
                        scale = win.scale_factor().ok().unwrap_or(1.0);
                    }
                    scale_mon = info.monitor_rect;
                }
                if ov_hwnd.is_none() {
                    ov_hwnd = overlay_hwnd(&app);
                }
                let mut moved = false;
                if let Some(h) = ov_hwnd {
                    let layout = current_layout(&app);
                    // 尺寸来自实际外框（物理 → 逻辑），免跨线程代理
                    if let Some((l, t, r, b)) = tw::phys_rect(h) {
                        let ow = (r - l) as f64 / scale;
                        let oh = (b - t) as f64 / scale;
                        if let Some((lx, ly)) = snap_position_for(&app, info, &layout, ow, oh, scale)
                        {
                            let (px, py) = ((lx * scale).round() as i32, (ly * scale).round() as i32);
                            if l != px || t != py {
                                if move_overlay_native(h, px, py) {
                                    moved = true;
                                }
                            }
                        }
                    }
                }
                if moved {
                    last_change = Instant::now();
                }
            }

            // ---- 健康自检：非意图隐藏 / 样式丢失自愈（原生查询，微秒级）----
            if ensure_overlay_sane(&app) {
                last_change = Instant::now();
            }

            // ---- 自适应间隔：活跃窗口内快轮询，静止后回落 ----
            let interval = if last_change.elapsed() < Duration::from_millis(FOLLOW_SETTLE_MS) {
                FOLLOW_ACTIVE_MS
            } else {
                FOLLOW_IDLE_MS
            };
            std::thread::sleep(Duration::from_millis(interval));
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        loop {
            if app
                .try_state::<crate::AppState>()
                .map(|s| s.watcher_stopped.load(std::sync::atomic::Ordering::SeqCst))
                .unwrap_or(true)
            {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(FOLLOW_IDLE_MS));
        }
    }
}

/// 用户主动隐藏意图标志：经托盘/热键/隐藏按钮隐藏时不做自愈
static OVERLAY_USER_HIDDEN: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
/// 浮层页面加载完成标志：此前为启动初始态不得自愈抢显（避免白屏闪烁），
/// 此后若窗口仍隐藏即属异常（显示握手竞态），允许自愈恢复
static OVERLAY_PAGE_READY: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// 记录用户主动隐藏意图（托盘/热键/隐藏按钮路径调用）
pub fn set_overlay_user_hidden(hidden: bool) {
    OVERLAY_USER_HIDDEN.store(hidden, std::sync::atomic::Ordering::SeqCst);
}

/// 标记浮层页面加载完成（on_page_load Finished 时由 lib.rs 调用）
pub fn set_overlay_page_ready() {
    OVERLAY_PAGE_READY.store(true, std::sync::atomic::Ordering::SeqCst);
}

/// 显示悬浮窗并重申不抢焦点样式
///
/// show() 可能让 tao/WebView2 重新应用窗口样式（实测顶层 WS_EX_NOACTIVATE
/// 会被覆盖丢失），导致后续点击悬浮窗激活窗口、目标输入框失焦。
/// 所有显示悬浮窗的路径统一走此函数：显示后立即重申样式。
/// 同时清除用户隐藏意图（自愈机制依据：此后隐藏即属异常）。
pub fn show_overlay_with_styles(app: &AppHandle) {
    if let Some(win) = get_overlay_window(app) {
        let _ = win.show();
    }
    OVERLAY_USER_HIDDEN.store(false, std::sync::atomic::Ordering::SeqCst);
    let _ = apply_overlay_styles(app);
}

/// 原生显示窗口（跨线程可靠；tao 的 show() 仅主线程生效）
///
/// 实测（0.10.1 自愈风暴诊断）：跟随线程调用 WebviewWindow::show() 不产生
/// 效果（事件派发链在非主线程断裂），窗口保持隐藏。Win32 ShowWindow
/// 经消息队列投递，任意线程调用均可靠，且自带 NOACTIVATE 语义不打扰焦点。
#[cfg(target_os = "windows")]
fn show_window_native(hwnd: isize) -> bool {
    use windows::Win32::Foundation::HWND;
    use windows::Win32::UI::WindowsAndMessaging::{ShowWindow, SHOW_WINDOW_CMD};
    if hwnd == 0 {
        return false;
    }
    let h = HWND(hwnd as *mut std::ffi::c_void);
    // SW_SHOW = 5
    unsafe { ShowWindow(h, SHOW_WINDOW_CMD(5)) }.as_bool()
}

#[cfg(not(target_os = "windows"))]
fn show_window_native(_hwnd: isize) -> bool {
    false
}

/// 悬浮窗健康自检：非用户意图的隐藏或样式丢失时自动恢复
///
/// 故障形态（0.10.1 实测捕获）：
/// - 进程重启后显示握手竞态 → 窗口停留初始隐藏态，切换布局/重置均只改
///   几何不调 show，悬浮窗"消失"且重置无效
/// - 扩展样式（NOACTIVATE/TOOLWINDOW）被外部重置
/// 自愈策略：跟随线程每轮以一两次 GetWindowLongPtrW（微秒级）检查
/// 可见性与关键样式，异常时原生恢复（ShowWindow/SetWindowLong 均可
/// 跨线程），并输出诊断日志供溯源。返回 true 表示执行了恢复。
pub fn ensure_overlay_sane(app: &AppHandle) -> bool {
    // 启动初始态（页面未加载完成，避免白屏抢显）或用户主动隐藏：不干预
    if OVERLAY_USER_HIDDEN.load(std::sync::atomic::Ordering::SeqCst)
        || !OVERLAY_PAGE_READY.load(std::sync::atomic::Ordering::SeqCst)
    {
        return false;
    }

    #[cfg(target_os = "windows")]
    {
        use windows::Win32::Foundation::HWND;
        use windows::Win32::UI::WindowsAndMessaging::{
            GetWindowLongPtrW, GWL_STYLE, WS_VISIBLE,
        };
        let Some(h) = overlay_hwnd(app) else {
            return false;
        };
        let hwnd = HWND(h as *mut std::ffi::c_void);
        let style = unsafe { GetWindowLongPtrW(hwnd, GWL_STYLE) };
        let visible = (style & (WS_VISIBLE.0 as isize)) != 0;
        if !visible {
            eprintln!("[overlay-sane] 检测到悬浮窗非预期隐藏，原生恢复显示");
            show_window_native(h);
            let _ = apply_overlay_styles(app);
            return true;
        }
        if !native_style_ok(h) {
            eprintln!("[overlay-sane] 检测到扩展样式丢失（NOACTIVATE/TOOLWINDOW），重申样式");
            let _ = apply_overlay_styles(app);
            return true;
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        let _ = app;
    }

    false
}

/// 窗口物理样式快速检查（不走 Tauri IPC，直接 Win32；自愈热路径用）
#[cfg(target_os = "windows")]
fn native_style_ok(hwnd: isize) -> bool {
    use windows::Win32::Foundation::HWND;
    use windows::Win32::UI::WindowsAndMessaging::{
        GetWindowLongPtrW, GWL_EXSTYLE, WS_EX_NOACTIVATE, WS_EX_TOOLWINDOW,
    };
    if hwnd == 0 {
        return false;
    }
    let h = HWND(hwnd as *mut std::ffi::c_void);
    let ex = unsafe { GetWindowLongPtrW(h, GWL_EXSTYLE) };
    (ex & (WS_EX_NOACTIVATE.0 as isize)) != 0 && (ex & (WS_EX_TOOLWINDOW.0 as isize)) != 0
}

#[cfg(not(target_os = "windows"))]
fn native_style_ok(_hwnd: isize) -> bool {
    true
}

/// 重置悬浮窗位置与大小：清除两布局的记忆几何与吸附记忆，恢复默认几何
///
/// 托盘菜单"重置悬浮窗位置和大小"入口。清除记忆后立即应用默认
/// 位置（屏幕右上角留边距）与默认尺寸。
pub fn reset_overlay_geometry(app: &AppHandle) {
    if get_overlay_window(app).is_none() {
        return;
    }
    // 清除记忆几何与吸附记忆并保存
    if let Some(state) = app.try_state::<crate::AppState>() {
        if let Ok(mut mgr) = state.config_manager.lock() {
            if let Some(overlay) = mgr.config_mut().overlay.as_mut() {
                overlay.vertical_x = None;
                overlay.vertical_y = None;
                overlay.vertical_w = None;
                overlay.vertical_h = None;
                overlay.horizontal_x = None;
                overlay.horizontal_y = None;
                overlay.horizontal_w = None;
                overlay.horizontal_h = None;
                // 吸附记忆一并清除：否则 apply_overlay_geometry 会按吸附边
                // 立即把悬浮窗贴回边缘，重置形同无效
                overlay.snap_memory = None;
            }
            let _ = mgr.save();
        }
    }
    // 重置语义即"找回悬浮窗"：清除隐藏意图并确保可见（含样式重申），
    // 覆盖窗口被异常隐藏导致"重置后仍不出现"的故障形态
    set_overlay_user_hidden(false);
    if get_overlay_window(app)
        .map(|w| w.is_visible().unwrap_or(false))
        .unwrap_or(false)
    {
        let _ = apply_overlay_styles(app);
    } else {
        show_overlay_with_styles(app);
    }
    // 应用当前布局的默认几何（含屏幕内钳制）
    apply_overlay_geometry(app, &current_layout(app));
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

/// 递归为窗口及全部子窗口添加 WS_EX_NOACTIVATE（阻止 WebView2 子窗口点击激活）
#[cfg(target_os = "windows")]
fn set_no_activate_recursive(hwnd: windows::Win32::Foundation::HWND) {
    use windows::Win32::Foundation::{BOOL, HWND, LPARAM};
    use windows::Win32::UI::WindowsAndMessaging::*;

    // EnumChildWindows 只枚举直接子窗口，递归下钻覆盖整棵 HWND 树
    unsafe extern "system" fn collect_child(h: HWND, l: LPARAM) -> BOOL {
        let v = unsafe { &mut *(l.0 as *mut Vec<HWND>) };
        v.push(h);
        BOOL(1)
    }

    let mut children: Vec<HWND> = Vec::new();
    unsafe {
        let ex = GetWindowLongPtrW(hwnd, GWL_EXSTYLE);
        SetWindowLongPtrW(hwnd, GWL_EXSTYLE, ex | (WS_EX_NOACTIVATE.0 as isize));
        EnumChildWindows(
            hwnd,
            Some(collect_child),
            LPARAM(&mut children as *mut Vec<HWND> as isize),
        );
    }
    for child in children {
        set_no_activate_recursive(child);
    }
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

            // WebView2 子窗口链也加 WS_EX_NOACTIVATE：
            // 真实鼠标点击命中的是 WebView2 的子 HWND（Chrome_WidgetWin_* /
            // RenderWidgetHost），子窗口未设置 NOACTIVATE 时，其 mousedown
            // 默认处理会触发 WM_MOUSEACTIVATE → 激活顶层悬浮窗 → 目标窗口
            // 失去焦点。递归为全部子窗口补上该样式，从源头阻止激活。
            set_no_activate_recursive(hwnd);

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

#[cfg(test)]
mod tests {
    use super::anchored_new_y;

    // 高度 100→160（增长 60）时各锚定方向的新 Y（当前 y=200）
    #[test]
    fn test_anchored_new_y_growth() {
        // 保顶（win-bottom/screen-top 吸附：贴合边在顶，向下扩展）
        assert_eq!(anchored_new_y(Some("top"), false, 200, 100, 160), 200);
        // 保底（win-top/screen-bottom 吸附：贴合边在底，向上扩展）
        assert_eq!(anchored_new_y(Some("bottom"), false, 200, 100, 160), 140);
        // 居中（左右侧吸附：对称扩展，上下各让 30）
        assert_eq!(anchored_new_y(Some("center"), false, 200, 100, 160), 170);
    }

    // 高度收缩 160→100 时贴合边同样不动
    #[test]
    fn test_anchored_new_y_shrink() {
        assert_eq!(anchored_new_y(Some("top"), false, 200, 160, 100), 200);
        assert_eq!(anchored_new_y(Some("bottom"), false, 200, 160, 100), 260);
        assert_eq!(anchored_new_y(Some("center"), false, 200, 160, 100), 230);
    }

    // 无吸附锚定：回退策略由 fallback_keep_top 决定（首次保顶/之后保底）
    #[test]
    fn test_anchored_new_y_fallback() {
        assert_eq!(anchored_new_y(None, true, 200, 100, 160), 200);
        assert_eq!(anchored_new_y(None, false, 200, 100, 160), 140);
        // 未知锚定值按无锚定处理
        assert_eq!(anchored_new_y(Some("diagonal"), true, 200, 100, 160), 200);
    }
}