//! 浮层窗口模块 (M2)
//!
//! 负责 QuickInput 的 tooltip 与右键菜单外置浮层窗口：
//! 内容渲染在独立透明窗口（floater）中，定位在悬浮窗外、屏幕工作区内，
//! 彻底摆脱悬浮窗尺寸约束（WebView2 内容在窗口物理边界处裁剪）。

use crate::window::get_overlay_window;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use tauri::{AppHandle, Emitter, Manager, WebviewWindow};

/// 浮层窗口 label（与 tauri.conf.json 中的 label 一致）
pub const FLOATER_WINDOW_LABEL: &str = "floater";

/// 浮层与锚点之间的间距（物理像素）
const FLOATER_GAP: i32 = 8;

/// 浮层类型
#[derive(Clone, Copy, PartialEq)]
pub enum FloaterKind {
    /// 提示气泡：鼠标穿透、不抢焦点
    Tooltip,
    /// 右键菜单：可点击
    Menu,
    /// 顶栏浮层（悬浮窗上方外侧右对齐）：控制按钮组 + 分组标签
    Toolbar,
}

/// 计算浮层窗口左上角位置（纯函数，物理像素，可单测）
///
/// 输入（均物理像素）：
/// - `anchor`: 锚点矩形（悬浮窗内按钮）`(left, top, right, bottom)`
/// - `size`: 浮层内容尺寸 `(width, height)`
/// - `work_area`: 悬浮窗所在显示器工作区 `(left, top, right, bottom)`
/// - `kind`: 浮层类型（决定垂直优先方向与水平对齐方式）
///
/// 规则：
/// - 垂直：tooltip 优先显示在锚点上方（保持原视觉）；menu 优先显示在
///   锚点下方（右键位置惯例）。一侧空间不足则翻转到另一侧；两侧都不足
///   则贴工作区底部完整显示（内容高于工作区时贴顶）。
/// - 水平：menu 左对齐锚点左缘；tooltip 相对锚点水平居中；越界时
///   钳制到工作区，保证浮层完整可见。
/// - 最终位置整体钳制在工作区内。
pub fn compute_floater_placement(
    anchor: (i32, i32, i32, i32),
    size: (i32, i32),
    work_area: (i32, i32, i32, i32),
    kind: FloaterKind,
) -> (i32, i32) {
    let (al, at, ar, ab) = anchor;
    let (w, h) = size;
    let (wal, wat, war, wab) = work_area;

    let above = at - FLOATER_GAP - h;
    let below = ab + FLOATER_GAP;
    let y = match kind {
        FloaterKind::Tooltip => {
            if above >= wat {
                above
            } else if below + h <= wab {
                below
            } else {
                (wab - h).max(wat)
            }
        }
        FloaterKind::Menu => {
            if below + h <= wab {
                below
            } else if above >= wat {
                above
            } else {
                (wab - h).max(wat)
            }
        }
        FloaterKind::Toolbar => {
            // 顶栏：固定悬浮窗上方外侧；空间不足贴工作区顶部
            if above >= wat {
                above
            } else {
                wat
            }
        }
    };

    let x = match kind {
        FloaterKind::Menu => al,
        FloaterKind::Tooltip => {
            let cx = al + (ar - al) / 2;
            cx - w / 2
        }
        FloaterKind::Toolbar => ar - w, // 右对齐悬浮窗右缘
    };

    // 整体钳制到工作区：保证完整可见（浮层比工作区大时贴左/上）
    let x_min = wal;
    let x_max = (war - w).max(x_min);
    let y_min = wat;
    let y_max = (wab - h).max(y_min);
    (x.clamp(x_min, x_max), y.clamp(y_min, y_max))
}

/// 菜单项（overlay 前端下发）
#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct FloaterMenuItem {
    pub id: String,
    pub label: String,
    pub disabled: bool,
    pub hint: Option<String>,
}

/// 锚点矩形（overlay 前端上报，逻辑像素，getBoundingClientRect 值）
#[derive(serde::Deserialize)]
pub struct FloaterAnchor {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

/// 待定位数据（floater_ready 时消费）
struct PendingPlacement {
    anchor: (i32, i32, i32, i32),
    work_area: (i32, i32, i32, i32),
    kind: FloaterKind,
}

static PENDING: Mutex<Option<PendingPlacement>> = Mutex::new(None);
static CURRENT_KIND: Mutex<Option<FloaterKind>> = Mutex::new(None);
/// 浮层页面是否加载完成（页面未就绪时 show 事件缓存，加载后补发）
static FLOATER_PAGE_READY: AtomicBool = AtomicBool::new(false);
static PENDING_SHOW: Mutex<Option<serde_json::Value>> = Mutex::new(None);

/// 标记浮层页面加载完成并补发缓存的显示事件（on_page_load 调用）
pub fn set_floater_page_ready(webview: &tauri::Webview) {
    FLOATER_PAGE_READY.store(true, Ordering::SeqCst);
    if let Ok(mut slot) = PENDING_SHOW.lock() {
        if let Some(v) = slot.take() {
            let _ = webview.emit("floater://show", v);
        }
    }
}

fn get_floater_window(app: &AppHandle) -> Option<WebviewWindow> {
    app.get_webview_window(FLOATER_WINDOW_LABEL)
}

/// 获取悬浮窗所在显示器工作区（物理像素，排除任务栏）
#[cfg(target_os = "windows")]
fn overlay_monitor_work_area(overlay: &WebviewWindow) -> Option<(i32, i32, i32, i32)> {
    use raw_window_handle::{HasWindowHandle, RawWindowHandle};
    use windows::Win32::Foundation::HWND;
    use windows::Win32::Graphics::Gdi::{
        GetMonitorInfoW, MonitorFromWindow, MONITORINFO, MONITOR_DEFAULTTONEAREST,
    };

    let handle = overlay.window_handle().ok()?;
    let raw = handle.as_raw();
    let hwnd = match raw {
        RawWindowHandle::Win32(w) => HWND(w.hwnd.get() as *mut std::ffi::c_void),
        _ => return None,
    };
    let mon = unsafe { MonitorFromWindow(hwnd, MONITOR_DEFAULTTONEAREST) };
    if mon.0.is_null() {
        return None;
    }
    let mut info = MONITORINFO::default();
    info.cbSize = std::mem::size_of::<MONITORINFO>() as u32;
    let ok = unsafe { GetMonitorInfoW(mon, &mut info) };
    if ok.as_bool() {
        Some((info.rcWork.left, info.rcWork.top, info.rcWork.right, info.rcWork.bottom))
    } else {
        None
    }
}

#[cfg(not(target_os = "windows"))]
fn overlay_monitor_work_area(_overlay: &WebviewWindow) -> Option<(i32, i32, i32, i32)> {
    None
}

/// 设置浮层窗口鼠标穿透与不抢焦点（tooltip 用；menu 时清除）
#[cfg(target_os = "windows")]
pub fn set_floater_click_through(app: &AppHandle, enabled: bool) -> Result<(), String> {
    use raw_window_handle::{HasWindowHandle, RawWindowHandle};
    use windows::Win32::Foundation::HWND;
    use windows::Win32::UI::WindowsAndMessaging::{
        GetWindowLongPtrW, SetWindowLongPtrW, GWL_EXSTYLE, WS_EX_NOACTIVATE, WS_EX_TRANSPARENT,
    };

    let window = get_floater_window(app).ok_or_else(|| "浮层窗口 (floater) 未找到".to_string())?;
    let handle = window.window_handle().map_err(|e| e.to_string())?;
    let raw = handle.as_raw();
    let hwnd = match raw {
        RawWindowHandle::Win32(w) => HWND(w.hwnd.get() as *mut std::ffi::c_void),
        _ => return Ok(()),
    };

    unsafe {
        let ex = GetWindowLongPtrW(hwnd, GWL_EXSTYLE);
        let transparent = WS_EX_TRANSPARENT.0 as isize;
        let no_activate = WS_EX_NOACTIVATE.0 as isize;
        let new_ex = if enabled {
            ex | transparent | no_activate
        } else {
            ex & !(transparent | no_activate)
        };
        SetWindowLongPtrW(hwnd, GWL_EXSTYLE, new_ex);
    }
    Ok(())
}

#[cfg(not(target_os = "windows"))]
pub fn set_floater_click_through(_app: &AppHandle, _enabled: bool) -> Result<(), String> {
    Ok(())
}

/// 设置浮层窗口不抢焦点（toolbar 用：可点击但不打断当前注入目标焦点）
#[cfg(target_os = "windows")]
pub fn set_floater_no_activate(app: &AppHandle, enabled: bool) -> Result<(), String> {
    use raw_window_handle::{HasWindowHandle, RawWindowHandle};
    use windows::Win32::Foundation::HWND;
    use windows::Win32::UI::WindowsAndMessaging::{
        GetWindowLongPtrW, SetWindowLongPtrW, GWL_EXSTYLE, WS_EX_NOACTIVATE,
    };

    let window = get_floater_window(app).ok_or_else(|| "浮层窗口 (floater) 未找到".to_string())?;
    let handle = window.window_handle().map_err(|e| e.to_string())?;
    let raw = handle.as_raw();
    let hwnd = match raw {
        RawWindowHandle::Win32(w) => HWND(w.hwnd.get() as *mut std::ffi::c_void),
        _ => return Ok(()),
    };

    unsafe {
        let ex = GetWindowLongPtrW(hwnd, GWL_EXSTYLE);
        let no_activate = WS_EX_NOACTIVATE.0 as isize;
        let new_ex = if enabled {
            ex | no_activate
        } else {
            ex & !no_activate
        };
        SetWindowLongPtrW(hwnd, GWL_EXSTYLE, new_ex);
    }
    Ok(())
}

#[cfg(not(target_os = "windows"))]
pub fn set_floater_no_activate(_app: &AppHandle, _enabled: bool) -> Result<(), String> {
    Ok(())
}

/// 显示浮层（两阶段）：记录锚点与内容 → 触发前端渲染与测量，
/// 前端上报 `floater_ready` 后定位显示，避免定位闪烁。
#[tauri::command]
pub fn show_floater(
    app: AppHandle,
    kind: String,
    text: Option<String>,
    items: Option<Vec<FloaterMenuItem>>,
    anchor: FloaterAnchor,
    toolbar: Option<serde_json::Value>,
) -> Result<(), String> {
    let fkind = match kind.as_str() {
        "tooltip" => FloaterKind::Tooltip,
        "menu" => FloaterKind::Menu,
        "toolbar" => FloaterKind::Toolbar,
        _ => return Err(format!("未知浮层类型: {kind}")),
    };
    let overlay = get_overlay_window(&app).ok_or_else(|| "悬浮窗 (overlay) 未找到".to_string())?;

    let anchor_px = if fkind == FloaterKind::Toolbar {
        // 顶栏锚点 = 悬浮窗窗口矩形（物理像素，不依赖前端锚点）
        let pos = overlay.inner_position().map_err(|e| e.to_string())?;
        let size = overlay.inner_size().map_err(|e| e.to_string())?;
        let (ox, oy) = (pos.x as i32, pos.y as i32);
        let (w, h) = (size.width as i32, size.height as i32);
        (ox, oy, ox + w, oy + h)
    } else {
        // 锚点逻辑像素 → 物理像素。前端 getBoundingClientRect 是客户区相对坐标，
        // 须用客户区原点（inner_position）；若用 outer_position（外框），透明窗口
        // 的系统边框/阴影（实测左 8px、上 1px）会使浮层整体向左上偏移。
        let pos = overlay.inner_position().map_err(|e| e.to_string())?;
        let scale = overlay.scale_factor().map_err(|e| e.to_string())?;
        let (ox, oy) = (pos.x as f64, pos.y as f64);
        (
            (ox + anchor.x * scale) as i32,
            (oy + anchor.y * scale) as i32,
            (ox + (anchor.x + anchor.w) * scale) as i32,
            (oy + (anchor.y + anchor.h) * scale) as i32,
        )
    };

    let work_area = overlay_monitor_work_area(&overlay).unwrap_or((0, 0, 1920, 1040));

    {
        let mut slot = PENDING.lock().map_err(|e| e.to_string())?;
        *slot = Some(PendingPlacement { anchor: anchor_px, work_area, kind: fkind });
    }
    {
        let mut slot = CURRENT_KIND.lock().map_err(|e| e.to_string())?;
        *slot = Some(fkind);
    }

    let payload = serde_json::json!({ "kind": kind, "text": text, "items": items, "toolbar": toolbar });
    // 双通道投递：事件优先送达（页面已就绪时），PENDING_SHOW 缓存供
    // 页面 onMount 主动 pull 兜底（页面 JS 因后台节流挂起时事件会丢失）。
    {
        let mut slot = PENDING_SHOW.lock().map_err(|e| e.to_string())?;
        *slot = Some(payload.clone());
    }
    app.emit_to("floater", "floater://show", payload)
        .map_err(|e| e.to_string())
}

/// 拉取待显示内容（floater 页面 onMount 兜底调用：页面就绪前的事件
/// 可能丢失，页面启动后主动取回最近一次显示请求）
#[tauri::command]
pub fn floater_pull_pending() -> Option<serde_json::Value> {
    PENDING_SHOW.lock().ok().and_then(|mut s| s.take())
}

/// 诊断：浮层模块状态（排查用）
#[tauri::command]
pub fn floater_debug() -> String {
    let kind = CURRENT_KIND
        .lock()
        .ok()
        .and_then(|s| *s)
        .map(|k| match k {
            FloaterKind::Tooltip => "tooltip",
            FloaterKind::Menu => "menu",
            FloaterKind::Toolbar => "toolbar",
        })
        .unwrap_or("none");
    format!(
        "page_ready={} pending_show={} pending_placement={} current_kind={}",
        FLOATER_PAGE_READY.load(Ordering::SeqCst),
        PENDING_SHOW.lock().map(|s| s.is_some()).unwrap_or(false),
        PENDING.lock().map(|s| s.is_some()).unwrap_or(false),
        kind
    )
}

/// 浮层内容渲染完成：按上报尺寸定位并显示
#[tauri::command]
pub fn floater_ready(app: AppHandle, width: f64, height: f64) -> Result<(), String> {
    let pending = PENDING
        .lock()
        .map_err(|e| e.to_string())?
        .take()
        .ok_or_else(|| "无待定位的浮层数据".to_string())?;
    let floater = get_floater_window(&app).ok_or_else(|| "浮层窗口 (floater) 未找到".to_string())?;

    let scale = floater.scale_factor().map_err(|e| e.to_string())?;
    let size = ((width * scale) as i32, (height * scale) as i32);
    let (x, y) = compute_floater_placement(pending.anchor, size, pending.work_area, pending.kind);

    floater.set_size(tauri::LogicalSize::new(width, height)).map_err(|e| e.to_string())?;

    // compute 结果为内容（客户区）左上角，但 set_position 设置的是外框位置。
    // 透明窗口仍可能带系统边框/阴影（实测左 8px、上 1px），若直接按外框
    // 定位，内容会整体向右下偏移。用 inner_position 与 outer_position 的
    // 差值修正，使客户区精确落在目标位置。
    let outer = floater.outer_position().map_err(|e| e.to_string())?;
    let inner = floater.inner_position().map_err(|e| e.to_string())?;
    let (fx, fy) = ((inner.x - outer.x) as i32, (inner.y - outer.y) as i32);
    floater.set_position(tauri::PhysicalPosition::new(x - fx, y - fy)).map_err(|e| e.to_string())?;

    // tooltip 鼠标穿透 + 不抢焦点；menu 可点击；toolbar 可点击但不抢焦点
    set_floater_click_through(&app, pending.kind == FloaterKind::Tooltip)?;
    if pending.kind == FloaterKind::Toolbar {
        set_floater_no_activate(&app, true)?;
    }

    floater.show().map_err(|e| e.to_string())?;

    // 菜单浮层获得焦点：点击其他窗口/桌面时触发 Focused(false) → 自动关闭。
    // tooltip 保持不抢焦点（NOACTIVATE）；toolbar 同样不抢焦点（点击顶栏
    // 按钮不打断当前注入目标焦点）。
    if pending.kind == FloaterKind::Menu {
        let _ = floater.set_focus();
    }

    // 通知前端最终方向（tooltip 箭头翻转：浮层在锚点上方 → 箭头朝下指按钮）
    let above = pending.kind == FloaterKind::Tooltip && y + size.1 <= pending.anchor.1;
    let _ = floater.emit("floater://orient", serde_json::json!({ "above": above }));

    Ok(())
}

/// 隐藏浮层（幂等）
///
/// 不真正隐藏窗口、不移到屏幕外：两者都会导致 WebView2 判定页面不可见，
/// 挂起页面 JS 执行（background throttling），后续 `floater://show` 事件
/// 到达时监听器未运行而丢失。改为把窗口移回屏幕左上角（0,0）保持
/// 1x1 可见，页面持续运行，事件可靠投递。
#[tauri::command]
pub fn hide_floater(app: AppHandle) -> Result<(), String> {
    if let Some(floater) = get_floater_window(&app) {
        let _ = floater.set_size(tauri::LogicalSize::new(1.0, 1.0));
        let _ = floater.set_position(tauri::PhysicalPosition::new(0, 0));
        let _ = floater.emit("floater://hide", ());
    }
    if let Ok(mut slot) = CURRENT_KIND.lock() {
        *slot = None;
    }
    Ok(())
}

/// 事件回调用隐藏（忽略错误）
pub fn hide_floater_quiet(app: &AppHandle) {
    let _ = hide_floater(app.clone());
}

/// 菜单项点击：隐藏浮层并转发动作到悬浮窗前端
#[tauri::command]
pub fn floater_action(app: AppHandle, id: String) -> Result<(), String> {
    hide_floater(app.clone())?;
    app.emit_to("overlay", "floater-menu-action", serde_json::json!({ "id": id }))
        .map_err(|e| e.to_string())
}

/// 顶栏浮层悬停状态上报（floater 页面鼠标悬停/移出时调用）：
/// 悬停期间悬浮窗前端取消自动隐藏定时器，移出后恢复延迟隐藏。
#[tauri::command]
pub fn floater_hover(app: AppHandle, hovering: bool) -> Result<(), String> {
    app.emit_to("overlay", "floater-hover", serde_json::json!({ "hovering": hovering }))
        .map_err(|e| e.to_string())
}

/// 顶栏浮层分组标签切换：不隐藏浮层，仅转发到悬浮窗前端切换当前分组
/// （悬浮窗内容变化后浮层保持显示，高亮由浮层页面本地更新）。
#[tauri::command]
pub fn floater_tab_switch(app: AppHandle, name: String) -> Result<(), String> {
    app.emit_to("overlay", "floater-tab-switch", serde_json::json!({ "name": name }))
        .map_err(|e| e.to_string())
}

/// 顶栏浮层淡出（自动隐藏前调用）：浮层页面播放淡出动画后自行隐藏。
/// 与 hide_floater 不同：不立即移走窗口，给前端留出淡出过渡的可视时间。
#[tauri::command]
pub fn floater_fade_out(app: AppHandle) -> Result<(), String> {
    if let Some(floater) = get_floater_window(&app) {
        let _ = floater.emit("floater://fadeout", ());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_menu_places_below_when_room() {
        // 锚点 (400,300)-(500,340)，工作区 0,0~1920,1040，菜单 160x90
        let pos = compute_floater_placement((400, 300, 500, 340), (160, 90), (0, 0, 1920, 1040), FloaterKind::Menu);
        assert_eq!(pos, (400, 348)); // 340 + 8
    }

    #[test]
    fn test_menu_flips_above_when_below_crowded() {
        // 锚点底部紧贴工作区底：下方放不下，翻上方
        // （计划原数据 940 下方实际放得下：948+90=1038<=1040，故修正为 943 使放不下成立）
        let pos = compute_floater_placement((400, 900, 500, 943), (160, 90), (0, 0, 1920, 1040), FloaterKind::Menu);
        assert_eq!(pos, (400, 802)); // 900 - 8 - 90
    }

    #[test]
    fn test_menu_sticks_to_bottom_when_both_crowded() {
        // 工作区高度 100，菜单 90 高：上方 20 < 90、下方不足 → 贴底
        let pos = compute_floater_placement((0, 0, 100, 100), (160, 90), (0, 0, 1000, 100), FloaterKind::Menu);
        assert_eq!(pos, (0, 10)); // 100-90=10 完整显示
    }

    #[test]
    fn test_menu_clamps_horizontal_when_overflows_right() {
        // 锚点贴工作区右缘：左对齐越界 → 钳制回工作区内
        let pos = compute_floater_placement((1900, 300, 1920, 340), (160, 90), (0, 0, 1920, 1040), FloaterKind::Menu);
        assert_eq!(pos, (1760, 348)); // x: 1920-160
    }

    #[test]
    fn test_tooltip_places_above_when_room() {
        // tooltip 优先上方：100x40
        let pos = compute_floater_placement((400, 300, 500, 340), (100, 40), (0, 0, 1920, 1040), FloaterKind::Tooltip);
        assert_eq!(pos, (400, 252)); // 300-8-40；水平居中 450-50
    }

    #[test]
    fn test_tooltip_flips_below_when_above_crowded() {
        // 锚点贴工作区顶部：上方放不下 → 翻下方
        let pos = compute_floater_placement((400, 0, 500, 40), (100, 40), (0, 0, 1920, 1040), FloaterKind::Tooltip);
        assert_eq!(pos, (400, 48)); // 40 + 8
    }

    #[test]
    fn test_tooltip_clamps_horizontal_when_center_overflows_left() {
        // 锚点贴工作区左缘，tooltip 居中越左界 → 钳制回 0
        let pos = compute_floater_placement((0, 300, 100, 340), (120, 40), (0, 0, 1920, 1040), FloaterKind::Tooltip);
        assert_eq!(pos.0, 0); // 50-60=-10 → 0
    }

    #[test]
    fn test_float_larger_than_work_area_sticks_to_top_left() {
        // 浮层 2000x1100 大于工作区 1920x1040 → 贴 (0,0)
        let pos = compute_floater_placement((100, 100, 200, 140), (2000, 1100), (0, 0, 1920, 1040), FloaterKind::Tooltip);
        assert_eq!(pos, (0, 0));
    }
}
