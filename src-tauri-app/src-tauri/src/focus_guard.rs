//! 焦点保护模块
//!
//! 提供 `FocusGuard`，在进入"注入/交互"操作前记录当前前台窗口与键盘焦点窗口，
//! 在操作结束时（RAII 析构）自动恢复，确保"点击不抢焦点"的核心体验。
//!
//! 记录两类焦点：
//! - 前台窗口（`GetForegroundWindow`）：注入后恢复窗口激活状态
//! - 键盘焦点窗口（`GetGUIThreadInfo.hwndFocus`）：注入后恢复键盘输入焦点，
//!   确保后续输入继续落在原输入框（配合前端 mousedown 拦截，双保险）

/// 焦点保护器（Windows）
///
/// 使用 RAII 模式：构造时记录前台窗口与键盘焦点窗口，析构时恢复。
#[cfg(target_os = "windows")]
pub struct FocusGuard {
    /// 记录的前台窗口句柄
    original_foreground: Option<windows::Win32::Foundation::HWND>,
    /// 记录的键盘焦点窗口句柄
    original_keyboard_focus: Option<windows::Win32::Foundation::HWND>,
}

#[cfg(target_os = "windows")]
impl FocusGuard {
    /// 记录当前前台窗口与键盘焦点窗口，创建焦点保护器
    pub fn new() -> Self {
        use windows::Win32::UI::WindowsAndMessaging::{
            GetForegroundWindow, GetGUIThreadInfo, GetWindowThreadProcessId, GUITHREADINFO,
        };

        unsafe {
            let fg = GetForegroundWindow();
            let mut keyboard_focus = None;

            // 通过前台窗口所在线程的 GUI 线程信息获取键盘焦点窗口
            if !fg.0.is_null() {
                let mut pid = 0u32;
                let tid = GetWindowThreadProcessId(fg, Some(&mut pid));
                if tid != 0 {
                    let mut gti = GUITHREADINFO {
                        cbSize: std::mem::size_of::<GUITHREADINFO>() as u32,
                        ..Default::default()
                    };
                    if GetGUIThreadInfo(tid, &mut gti).is_ok() && !gti.hwndFocus.0.is_null() {
                        keyboard_focus = Some(gti.hwndFocus);
                    }
                }
            }

            Self {
                original_foreground: if fg.0.is_null() { None } else { Some(fg) },
                original_keyboard_focus: keyboard_focus,
            }
        }
    }

    /// 主动恢复前台焦点与键盘焦点（可提前调用，不依赖析构）
    pub fn restore(&mut self) {
        use windows::Win32::UI::Input::KeyboardAndMouse::SetFocus;
        use windows::Win32::UI::WindowsAndMessaging::SetForegroundWindow;

        if let Some(hwnd) = self.original_foreground.take() {
            unsafe {
                let _ = SetForegroundWindow(hwnd);
            }
        }
        if let Some(hwnd) = self.original_keyboard_focus.take() {
            unsafe {
                let _ = SetFocus(hwnd);
            }
        }
    }
}

// FocusGuard 当前由 S121 注入引擎接入，此处标注避免未使用警告
#[allow(dead_code)]
#[cfg(target_os = "windows")]
impl FocusGuard {
    // 占位 impl，实际用途见 S121 注入引擎接入
}

#[cfg(target_os = "windows")]
impl Drop for FocusGuard {
    fn drop(&mut self) {
        self.restore();
    }
}

/// 跨平台占位实现
#[cfg(not(target_os = "windows"))]
pub struct FocusGuard;

#[cfg(not(target_os = "windows"))]
impl FocusGuard {
    pub fn new() -> Self {
        Self
    }
    pub fn restore(&mut self) {}
}

impl Default for FocusGuard {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_focus_guard_constructs() {
        // FocusGuard 应能正常创建，不 panic
        let _guard = FocusGuard::new();
    }

    #[test]
    fn test_focus_guard_restore_does_not_panic() {
        // 手动调用 restore 不应 panic
        let mut guard = FocusGuard::new();
        guard.restore();
        // 二次调用也应安全（original_focus 已 take，无害）
        guard.restore();
    }

    #[test]
    fn test_focus_guard_drop_does_not_panic() {
        // 析构时自动恢复焦点，不应 panic
        {
            let _guard = FocusGuard::new();
        }
        // 作用域退出后 guard 已 drop
    }
}
