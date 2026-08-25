//! 目标窗口探测与边缘吸附几何计算
//!
//! 职责：
//! 1. 探测当前前台目标窗口（进程名、矩形、最大化/全屏状态、所在显示器）
//! 2. 纯函数计算吸附判定与吸附位置（可单测，不依赖真实窗口）
//!
//! 吸附策略（与配置 `overlay.snap_memory` 的边值对应）：
//! - 目标全屏/最大化：悬浮窗可吸附目标所在显示器工作区的四边（screen-*）
//! - 目标窗口化：悬浮窗可吸附目标窗口外侧四边（win-*，留 SNAP_GAP 间距）
//! - 目标全屏/最大化时记忆的 win-* 边自动映射为同朝向 screen-* 边

use windows::Win32::Foundation::{CloseHandle, HWND, RECT};
use windows::Win32::Graphics::Gdi::{
    GetMonitorInfoW, MonitorFromWindow, MONITORINFO, MONITOR_DEFAULTTONEAREST,
};
use windows::Win32::System::Threading::{
    OpenProcess, PROCESS_NAME_FORMAT, PROCESS_QUERY_INFORMATION, QueryFullProcessImageNameW,
};
use windows::Win32::UI::WindowsAndMessaging::{
    GetForegroundWindow, GetWindowLongPtrW, GetWindowRect, GetWindowThreadProcessId, IsZoomed,
    GWL_STYLE, WS_CAPTION,
};

/// 吸附判定阈值（逻辑像素）：拖动结束时悬浮窗贴合边距目标边该距离内即吸附
pub const SNAP_THRESHOLD: f64 = 40.0;
/// 窗口边吸附间距（逻辑像素）：吸附在应用窗口外侧时与窗口边缘保持的间隙
pub const SNAP_GAP: f64 = 8.0;

/// 前台目标窗口信息（矩形均为物理像素）
#[derive(Debug, Clone, PartialEq)]
pub struct TargetWindowInfo {
    /// 前台窗口所属进程名（含 .exe）
    pub process_name: String,
    /// 窗口矩形 (left, top, right, bottom)
    pub rect: (i32, i32, i32, i32),
    /// 是否最大化
    pub is_maximized: bool,
    /// 是否无边框全屏（无标题栏且覆盖整个显示器）
    pub is_fullscreen: bool,
    /// 目标所在显示器全屏矩形（物理像素）
    pub monitor_rect: (i32, i32, i32, i32),
    /// 目标所在显示器工作区（物理像素，排除任务栏）
    pub work_area: (i32, i32, i32, i32),
}

impl TargetWindowInfo {
    /// 目标是否处于全屏或最大化状态（吸附走屏幕边）
    pub fn is_screen_mode(&self) -> bool {
        self.is_maximized || self.is_fullscreen
    }
}

/// 本应用进程名（探测时跳过自身：模板输入弹窗等场景前台会是本进程）
const SELF_PROCESS: &str = "quickinput.exe";

/// 探测当前前台目标窗口（= 前台句柄 + 全量探测）
///
/// 返回 None 的情形：无前台窗口 / 前台是本应用 / 进程信息或矩形获取失败。
#[cfg(target_os = "windows")]
pub fn current_target_window() -> Option<TargetWindowInfo> {
    probe_target(foreground_hwnd()?)
}

/// 非 Windows 平台占位实现
#[cfg(not(target_os = "windows"))]
pub fn current_target_window() -> Option<TargetWindowInfo> {
    None
}

// ---- 跟随线程低开销接口 ----
// 常驻跟随线程按"句柄不变 → 增量刷新几何"工作：进程名探测（OpenProcess/
// QueryFullProcessImageName）从前台每轮执行降为仅前台切换时执行，
// 稳态 CPU 趋近于零；几何刷新只需 GetWindowRect/IsZoomed/样式位等
// 微秒级调用，快轮询期间也不构成负载。

/// 前台窗口句柄（无前台窗口返回 None）
#[cfg(target_os = "windows")]
pub fn foreground_hwnd() -> Option<isize> {
    let hwnd = unsafe { GetForegroundWindow() };
    if hwnd.0.is_null() {
        None
    } else {
        Some(hwnd.0 as isize)
    }
}

/// 非 Windows 平台占位实现
#[cfg(not(target_os = "windows"))]
pub fn foreground_hwnd() -> Option<isize> {
    None
}

/// isize 句柄 → HWND
#[cfg(target_os = "windows")]
fn hwnd_from(hwnd: isize) -> HWND {
    HWND(hwnd as *mut std::ffi::c_void)
}

/// 窗口是否属于本进程（跟随线程快速过滤自身，免进程名探测）
#[cfg(target_os = "windows")]
pub fn is_self_window(hwnd: isize) -> bool {
    use windows::Win32::System::Threading::GetCurrentProcessId;
    if hwnd == 0 {
        return true;
    }
    let mut pid = 0u32;
    unsafe { GetWindowThreadProcessId(hwnd_from(hwnd), Some(&mut pid)) };
    pid != 0 && pid == unsafe { GetCurrentProcessId() }
}

/// 非 Windows 平台占位实现
#[cfg(not(target_os = "windows"))]
pub fn is_self_window(_hwnd: isize) -> bool {
    true
}

/// 由句柄全量探测目标（含进程名；句柄无效/属于本进程/探测失败返回 None）
#[cfg(target_os = "windows")]
pub fn probe_target(hwnd: isize) -> Option<TargetWindowInfo> {
    if hwnd == 0 {
        return None;
    }
    let h = hwnd_from(hwnd);
    let process_name = foreground_process_name(h)?;
    if process_name.eq_ignore_ascii_case(SELF_PROCESS) {
        return None;
    }
    let rect = window_rect(h)?;
    let is_maximized = unsafe { IsZoomed(h) }.as_bool();
    let (monitor_rect, work_area) = monitor_rects(h)?;
    let is_fullscreen = !is_maximized && is_borderless_fullscreen(h, rect);
    Some(TargetWindowInfo {
        process_name,
        rect,
        is_maximized,
        is_fullscreen,
        monitor_rect,
        work_area,
    })
}

/// 非 Windows 平台占位实现
#[cfg(not(target_os = "windows"))]
pub fn probe_target(_hwnd: isize) -> Option<TargetWindowInfo> {
    None
}

/// 增量刷新目标几何状态（不重探进程名）：任何字段变化返回 true 并写回
#[cfg(target_os = "windows")]
pub fn refresh_geometry(hwnd: isize, info: &mut TargetWindowInfo) -> bool {
    if hwnd == 0 {
        return false;
    }
    let h = hwnd_from(hwnd);
    let Some(rect) = window_rect(h) else {
        return false;
    };
    let is_maximized = unsafe { IsZoomed(h) }.as_bool();
    let Some((monitor_rect, work_area)) = monitor_rects(h) else {
        return false;
    };
    let is_fullscreen = !is_maximized && is_borderless_fullscreen(h, rect);
    let changed = info.rect != rect
        || info.is_maximized != is_maximized
        || info.is_fullscreen != is_fullscreen
        || info.monitor_rect != monitor_rect
        || info.work_area != work_area;
    if changed {
        info.rect = rect;
        info.is_maximized = is_maximized;
        info.is_fullscreen = is_fullscreen;
        info.monitor_rect = monitor_rect;
        info.work_area = work_area;
    }
    changed
}

/// 非 Windows 平台占位实现
#[cfg(not(target_os = "windows"))]
pub fn refresh_geometry(_hwnd: isize, _info: &mut TargetWindowInfo) -> bool {
    false
}

/// 读取窗口物理矩形（isize 句柄版，低延迟路径用）
#[cfg(target_os = "windows")]
pub fn phys_rect(hwnd: isize) -> Option<(i32, i32, i32, i32)> {
    if hwnd == 0 {
        return None;
    }
    window_rect(hwnd_from(hwnd))
}

/// 非 Windows 平台占位实现
#[cfg(not(target_os = "windows"))]
pub fn phys_rect(_hwnd: isize) -> Option<(i32, i32, i32, i32)> {
    None
}

/// 由窗口句柄取进程名（含 .exe）
#[cfg(target_os = "windows")]
fn foreground_process_name(hwnd: HWND) -> Option<String> {
    let mut pid = 0u32;
    unsafe { GetWindowThreadProcessId(hwnd, Some(&mut pid)) };
    if pid == 0 {
        return None;
    }
    let handle = unsafe { OpenProcess(PROCESS_QUERY_INFORMATION, false, pid) }.ok()?;
    let mut buffer = vec![0u16; 1024];
    let mut size = buffer.len() as u32;
    let ok = unsafe {
        QueryFullProcessImageNameW(
            handle,
            PROCESS_NAME_FORMAT(0),
            windows::core::PWSTR(buffer.as_mut_ptr()),
            &mut size,
        )
    };
    unsafe {
        let _ = CloseHandle(handle);
    }
    if ok.is_err() {
        return None;
    }
    let path = String::from_utf16(&buffer[..size as usize]).ok()?;
    let name = std::path::Path::new(&path)
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())?;
    if name.is_empty() {
        None
    } else {
        Some(name)
    }
}

/// 窗口矩形（物理像素）
#[cfg(target_os = "windows")]
fn window_rect(hwnd: HWND) -> Option<(i32, i32, i32, i32)> {
    let mut rect = RECT::default();
    if unsafe { GetWindowRect(hwnd, &mut rect) }.is_err() {
        return None;
    }
    Some((rect.left, rect.top, rect.right, rect.bottom))
}

/// 无边框全屏判定：无标题栏样式 且 窗口矩形覆盖整个显示器
///
/// 全屏游戏典型特征：WS_CAPTION 被移除，窗口矩形铺满显示器（含任务栏区域）。
#[cfg(target_os = "windows")]
fn is_borderless_fullscreen(hwnd: HWND, rect: (i32, i32, i32, i32)) -> bool {
    let style = unsafe { GetWindowLongPtrW(hwnd, GWL_STYLE) };
    let has_caption = (style as u32) & WS_CAPTION.0 != 0;
    if has_caption {
        return false;
    }
    let Some((mon, _)) = monitor_rects(hwnd) else {
        return false;
    };
    // 覆盖判定留 2px 容差（部分驱动/缩放下的取整误差）
    rect.0 <= mon.0 + 2
        && rect.1 <= mon.1 + 2
        && rect.2 >= mon.2 - 2
        && rect.3 >= mon.3 - 2
}

/// 窗口所在显示器的全屏矩形与工作区（物理像素）
#[cfg(target_os = "windows")]
fn monitor_rects(hwnd: HWND) -> Option<((i32, i32, i32, i32), (i32, i32, i32, i32))> {
    let monitor = unsafe { MonitorFromWindow(hwnd, MONITOR_DEFAULTTONEAREST) };
    let mut info = MONITORINFO {
        cbSize: std::mem::size_of::<MONITORINFO>() as u32,
        ..Default::default()
    };
    if unsafe { GetMonitorInfoW(monitor, &mut info) }.as_bool() {
        let mon = info.rcMonitor;
        let wa = info.rcWork;
        Some((
            (mon.left, mon.top, mon.right, mon.bottom),
            (wa.left, wa.top, wa.right, wa.bottom),
        ))
    } else {
        None
    }
}

// ============================================================
// 吸附几何纯函数（逻辑坐标，可单测）
// ============================================================

/// 物理像素矩形 → 逻辑像素 (left, top, right, bottom)
fn logical_rect(r: (i32, i32, i32, i32), scale: f64) -> (f64, f64, f64, f64) {
    (
        r.0 as f64 / scale,
        r.1 as f64 / scale,
        r.2 as f64 / scale,
        r.3 as f64 / scale,
    )
}

/// 两区间 [a1,a2] 与 [b1,b2] 是否有重叠（用于平行轴对齐校验）
fn spans_overlap(a1: f64, a2: f64, b1: f64, b2: f64) -> bool {
    a1 < b2 && b1 < a2
}

/// 目标状态变化时的边映射：全屏/最大化下 win-* 退化为同朝向 screen-*
///
/// 最大化窗口矩形 ≈ 工作区（含不可见边框外溢），win-* 的外侧定位无意义；
/// 恢复窗口化后由跟随线程按记忆的 win-* 边归位。
pub fn map_edge_for_target<'a>(edge: &'a str, info: &TargetWindowInfo) -> &'a str {
    if !info.is_screen_mode() {
        return edge;
    }
    match edge {
        "win-left" => "screen-left",
        "win-right" => "screen-right",
        "win-top" => "screen-top",
        "win-bottom" => "screen-bottom",
        other => other,
    }
}

/// 拖动结束时的吸附判定：悬浮窗当前几何（逻辑坐标，外框）距哪条候选边最近
///
/// 候选边集合：
/// - 目标全屏/最大化：显示器工作区四边（贴合边 0 间距）
/// - 目标窗口化：目标窗口外侧四边（贴合后留 SNAP_GAP 间距）
/// 判定条件：垂直于边的方向距离 ≤ SNAP_THRESHOLD，且平行方向与目标边有重叠
/// （避免悬浮窗在边的延长线远处误吸）。多个候选时取距离最小者。
pub fn detect_snap_edge(
    ox: f64,
    oy: f64,
    ow: f64,
    oh: f64,
    info: &TargetWindowInfo,
    scale: f64,
) -> Option<&'static str> {
    let wa = logical_rect(info.work_area, scale);
    let (wa_l, wa_t, wa_r, wa_b) = (wa.0, wa.1, wa.2, wa.3);

    // 候选：(边名, 垂直方向目标贴合位置, 距离, 水平跨度, 垂直跨度)
    let mut best: Option<(&'static str, f64)> = None;
    let mut consider = |edge: &'static str, dist: f64| {
        if dist > SNAP_THRESHOLD {
            return;
        }
        if best.map(|(_, d)| dist < d).unwrap_or(true) {
            best = Some((edge, dist));
        }
    };

    if info.is_screen_mode() {
        // 屏幕四边：悬浮窗始终在屏内，平行轴重叠必然满足
        consider("screen-left", (ox - wa_l).abs());
        consider("screen-right", ((ox + ow) - wa_r).abs());
        consider("screen-top", (oy - wa_t).abs());
        consider("screen-bottom", ((oy + oh) - wa_b).abs());
    } else {
        let wr = logical_rect(info.rect, scale);
        let (wl, wt, wrr, wb) = (wr.0, wr.1, wr.2, wr.3);
        // 窗口外侧四边：贴合位置与窗口边相距 SNAP_GAP，且需平行轴重叠
        if spans_overlap(oy, oy + oh, wt, wb) {
            consider("win-left", ((ox + ow) - (wl - SNAP_GAP)).abs());
            consider("win-right", (ox - (wrr + SNAP_GAP)).abs());
        }
        if spans_overlap(ox, ox + ow, wl, wrr) {
            consider("win-top", ((oy + oh) - (wt - SNAP_GAP)).abs());
            consider("win-bottom", (oy - (wb + SNAP_GAP)).abs());
        }
    }
    best.map(|(edge, _)| edge)
}

/// 计算吸附边的悬浮窗位置（逻辑坐标，返回外框左上角）
///
/// - screen-*：贴合工作区边（0 间距），沿边在工作区内居中
/// - win-*：贴合窗口外侧（SNAP_GAP 间距），沿边在窗口投影内居中
/// - 最终位置钳制到工作区内（窗口贴屏幕边时外侧空间不足则内收）
///
/// 非法边返回 None（调用方回退到记忆位置/默认位置）。
pub fn snapped_position(
    edge: &str,
    ow: f64,
    oh: f64,
    info: &TargetWindowInfo,
    scale: f64,
) -> Option<(f64, f64)> {
    let wa = logical_rect(info.work_area, scale);
    let (wa_l, wa_t, wa_r, wa_b) = (wa.0, wa.1, wa.2, wa.3);
    let wr = logical_rect(info.rect, scale);
    let (wl, wt, wrr, wb) = (wr.0, wr.1, wr.2, wr.3);

    let (x, y) = match edge {
        "screen-left" => (wa_l, wa_t + (wa_b - wa_t - oh) / 2.0),
        "screen-right" => (wa_r - ow, wa_t + (wa_b - wa_t - oh) / 2.0),
        "screen-top" => (wa_l + (wa_r - wa_l - ow) / 2.0, wa_t),
        "screen-bottom" => (wa_l + (wa_r - wa_l - ow) / 2.0, wa_b - oh),
        "win-left" => (wl - SNAP_GAP - ow, wt + (wb - wt - oh) / 2.0),
        "win-right" => (wrr + SNAP_GAP, wt + (wb - wt - oh) / 2.0),
        "win-top" => (wl + (wrr - wl - ow) / 2.0, wt - SNAP_GAP - oh),
        "win-bottom" => (wl + (wrr - wl - ow) / 2.0, wb + SNAP_GAP),
        _ => return None,
    };

    // 钳制到工作区：保证悬浮窗完整可见（窗口比工作区宽/高时贴左/上）
    let x_min = wa_l;
    let x_max = (wa_r - ow).max(x_min);
    let y_min = wa_t;
    let y_max = (wa_b - oh).max(y_min);
    Some((x.clamp(x_min, x_max), y.clamp(y_min, y_max)))
}

// ============================================================
// 单元测试（纯函数，构造虚拟目标窗口）
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// 构造测试用目标窗口：1920x1080 物理屏（工作区 0,0~1920,1040）
    fn windowed_target() -> TargetWindowInfo {
        TargetWindowInfo {
            process_name: "app.exe".to_string(),
            rect: (400, 300, 1200, 800),
            is_maximized: false,
            is_fullscreen: false,
            monitor_rect: (0, 0, 1920, 1080),
            work_area: (0, 0, 1920, 1040),
        }
    }

    fn maximized_target() -> TargetWindowInfo {
        TargetWindowInfo {
            process_name: "app.exe".to_string(),
            rect: (-8, -8, 1928, 1048),
            is_maximized: true,
            is_fullscreen: false,
            monitor_rect: (0, 0, 1920, 1080),
            work_area: (0, 0, 1920, 1040),
        }
    }

    // ---- detect_snap_edge ----

    #[test]
    fn test_detect_windowed_near_window_left() {
        let info = windowed_target();
        // 悬浮窗右缘贴近窗口左侧外侧（400-8=392），垂直方向与窗口重叠
        let edge = detect_snap_edge(292.0, 500.0, 100.0, 60.0, &info, 1.0);
        assert_eq!(edge, Some("win-left"));
    }

    #[test]
    fn test_detect_windowed_near_window_right() {
        let info = windowed_target();
        // 悬浮窗左缘贴近窗口右侧外侧（1200+8=1208）
        let edge = detect_snap_edge(1210.0, 500.0, 100.0, 60.0, &info, 1.0);
        assert_eq!(edge, Some("win-right"));
    }

    #[test]
    fn test_detect_windowed_near_window_bottom() {
        let info = windowed_target();
        // 悬浮窗顶缘贴近窗口底边外侧（800+8=808），水平方向与窗口重叠
        let edge = detect_snap_edge(700.0, 805.0, 100.0, 60.0, &info, 1.0);
        assert_eq!(edge, Some("win-bottom"));
    }

    #[test]
    fn test_detect_windowed_far_away_returns_none() {
        let info = windowed_target();
        // 距窗口边 300 逻辑像素，远离屏幕边
        assert_eq!(detect_snap_edge(600.0, 400.0, 100.0, 60.0, &info, 1.0), None);
    }

    #[test]
    fn test_detect_windowed_parallel_no_overlap_returns_none() {
        let info = windowed_target();
        // 距窗口左缘距离满足，但垂直方向不与窗口重叠（在窗口下方 100px 处）
        let edge = detect_snap_edge(292.0, 900.0, 100.0, 60.0, &info, 1.0);
        assert_eq!(edge, None);
    }

    #[test]
    fn test_detect_windowed_screen_edge_not_candidate() {
        let info = windowed_target();
        // 贴屏幕底边（y+oh=1040）：窗口化目标不吸屏幕边
        let edge = detect_snap_edge(900.0, 980.0, 100.0, 60.0, &info, 1.0);
        assert_eq!(edge, None);
    }

    #[test]
    fn test_detect_maximized_near_screen_bottom() {
        let info = maximized_target();
        let edge = detect_snap_edge(900.0, 980.0, 100.0, 60.0, &info, 1.0);
        assert_eq!(edge, Some("screen-bottom"));
    }

    #[test]
    fn test_detect_maximized_near_screen_right() {
        let info = maximized_target();
        // 悬浮窗右缘 1920（工作区右边界）
        let edge = detect_snap_edge(1820.0, 500.0, 100.0, 60.0, &info, 1.0);
        assert_eq!(edge, Some("screen-right"));
    }

    #[test]
    fn test_detect_maximized_window_edge_not_candidate() {
        let info = maximized_target();
        // 最大化时窗口边（≈工作区±边框外溢）不作为候选：
        // 贴工作区左侧 0 处应吸 screen-left 而非 win-left
        let edge = detect_snap_edge(0.0, 500.0, 100.0, 60.0, &info, 1.0);
        assert_eq!(edge, Some("screen-left"));
    }

    #[test]
    fn test_detect_prefers_nearest_edge() {
        let info = windowed_target();
        // 同时贴近窗口左缘（距 18）与窗口底缘（距 28）：取更近的 win-left
        // （ox+ow=410 既满足 win-left 距离，又与窗口水平跨度重叠使 win-bottom 入选）
        let edge = detect_snap_edge(310.0, 780.0, 100.0, 60.0, &info, 1.0);
        assert_eq!(edge, Some("win-left"));
    }

    #[test]
    fn test_detect_scale_conversion() {
        // 150% 缩放：物理工作区 0,0~2880,1560 → 逻辑 0,0~1920,1040
        let info = TargetWindowInfo {
            process_name: "app.exe".to_string(),
            rect: (600, 450, 1800, 1200),
            is_maximized: false,
            is_fullscreen: false,
            monitor_rect: (0, 0, 2880, 1620),
            work_area: (0, 0, 2880, 1560),
        };
        // 窗口逻辑矩形 (400,300,1200,800)，左缘外侧吸附位 392 逻辑
        let edge = detect_snap_edge(292.0, 500.0, 100.0, 60.0, &info, 1.5);
        assert_eq!(edge, Some("win-left"));
    }

    // ---- snapped_position ----

    #[test]
    fn test_snapped_win_right_position() {
        let info = windowed_target();
        // 窗口右缘 1200 + gap 8 = 1208；垂直居中于窗口 300+(500-60)/2=520
        let (x, y) = snapped_position("win-right", 100.0, 60.0, &info, 1.0).unwrap();
        assert_eq!(x, 1208.0);
        assert_eq!(y, 520.0);
    }

    #[test]
    fn test_snapped_win_bottom_position() {
        let info = windowed_target();
        // 窗口底 800 + gap 8 = 808；水平居中于窗口 400+(800-100)/2=750
        let (x, y) = snapped_position("win-bottom", 100.0, 60.0, &info, 1.0).unwrap();
        assert_eq!(x, 750.0);
        assert_eq!(y, 808.0);
    }

    #[test]
    fn test_snapped_screen_bottom_position() {
        let info = maximized_target();
        // 工作区底 1040：贴合（0 间距）；水平居中于工作区 (1920-100)/2=910
        let (x, y) = snapped_position("screen-bottom", 100.0, 60.0, &info, 1.0).unwrap();
        assert_eq!(x, 910.0);
        assert_eq!(y, 980.0);
    }

    #[test]
    fn test_snapped_clamped_into_work_area() {
        let info = TargetWindowInfo {
            process_name: "app.exe".to_string(),
            // 窗口右半超出屏幕（right=1900），悬浮窗吸右侧放不下
            rect: (1000, 300, 1900, 800),
            is_maximized: false,
            is_fullscreen: false,
            monitor_rect: (0, 0, 1920, 1080),
            work_area: (0, 0, 1920, 1040),
        };
        // win-right 目标 x=1908，但钳制到工作区内 x_max=1920-100=1820
        let (x, _) = snapped_position("win-right", 100.0, 60.0, &info, 1.0).unwrap();
        assert_eq!(x, 1820.0);
    }

    #[test]
    fn test_snapped_invalid_edge_returns_none() {
        let info = windowed_target();
        assert!(snapped_position("diagonal", 100.0, 60.0, &info, 1.0).is_none());
    }

    // ---- map_edge_for_target ----

    #[test]
    fn test_map_edge_win_to_screen_when_maximized() {
        let info = maximized_target();
        assert_eq!(map_edge_for_target("win-bottom", &info), "screen-bottom");
        assert_eq!(map_edge_for_target("win-left", &info), "screen-left");
        // screen-* 保持不变
        assert_eq!(map_edge_for_target("screen-right", &info), "screen-right");
    }

    #[test]
    fn test_map_edge_unchanged_when_windowed() {
        let info = windowed_target();
        assert_eq!(map_edge_for_target("win-bottom", &info), "win-bottom");
        assert_eq!(map_edge_for_target("screen-right", &info), "screen-right");
    }

    // ---- current_target_window 冒烟（真实环境不 panic） ----

    #[test]
    fn test_current_target_window_does_not_panic() {
        let _ = current_target_window();
    }

    // ---- 跟随线程低开销接口：非法句柄安全、真实环境不 panic ----

    #[test]
    fn test_follow_helpers_invalid_hwnd_safe() {
        // 句柄 0：各接口安全返回，不 panic
        assert_eq!(probe_target(0), None);
        assert_eq!(phys_rect(0), None);
        assert!(is_self_window(0));
        let mut info = windowed_target();
        assert!(!refresh_geometry(0, &mut info));
        // 原状态不被破坏
        assert_eq!(info.rect, (400, 300, 1200, 800));
    }

    #[test]
    fn test_follow_helpers_real_environment_smoke() {
        // 真实环境：前台句柄可获取（桌面环境必有），探测不 panic
        if let Some(hwnd) = foreground_hwnd() {
            let _ = is_self_window(hwnd);
            let _ = probe_target(hwnd);
            let _ = phys_rect(hwnd);
        }
    }
}
