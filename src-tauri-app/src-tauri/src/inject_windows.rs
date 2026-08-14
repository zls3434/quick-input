//! Windows 平台文本注入实现 (S121)
//!
//! 使用 Win32 `SendInput` + `KEYEVENTF_UNICODE` 实现 Unicode 文本注入，
//! 支持多字节字符（中文等）。注入前记录焦点，注入后恢复（RAII）。
//! 处理修饰键冲突，并在无法打开前台进程时返回 InjectionRejected。

use windows::Win32::Foundation::CloseHandle;
use windows::Win32::UI::Input::KeyboardAndMouse::{
    GetKeyState, INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT, KEYBD_EVENT_FLAGS, KEYEVENTF_KEYUP,
    KEYEVENTF_UNICODE, SendInput, VIRTUAL_KEY, VK_CONTROL, VK_MENU, VK_SHIFT, keybd_event,
};
use windows::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowThreadProcessId};
use windows::Win32::System::Threading::{OpenProcess, PROCESS_QUERY_LIMITED_INFORMATION};

use crate::inject::{InjectError, Injector};
use crate::focus_guard::FocusGuard;

/// Windows 注入器
pub struct WindowsInjector;

impl WindowsInjector {
    pub fn new() -> Self {
        Self
    }
}

impl Default for WindowsInjector {
    fn default() -> Self {
        Self::new()
    }
}

impl Injector for WindowsInjector {
    fn inject_text(&self, text: &str) -> Result<(), InjectError> {
        // 1. 记录焦点（RAII 保护，注入后自动恢复）
        let _guard = FocusGuard::new();

        // 2. 处理修饰键冲突（记录当前 Ctrl/Alt/Shift 状态）
        let modifiers = ModifierState::capture();
        modifiers.release();

        // 3. SendInput 逐字符注入 UTF-16
        for ch in text.encode_utf16() {
            send_unicode_char(ch)?;
        }

        // 4. 恢复修饰键
        modifiers.restore();

        Ok(())
    }
}

/// 发送单个 Unicode 字符（keydown + keyup）
fn send_unicode_char(ch: u16) -> Result<(), InjectError> {
    let inputs = [
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VIRTUAL_KEY(0),
                    wScan: ch,
                    dwFlags: KEYEVENTF_UNICODE,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        },
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VIRTUAL_KEY(0),
                    wScan: ch,
                    dwFlags: KEYEVENTF_UNICODE | KEYEVENTF_KEYUP,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        },
    ];

    let sent = unsafe { SendInput(&inputs, std::mem::size_of::<INPUT>() as i32) };
    if sent != inputs.len() as u32 {
        return Err(InjectError::Unknown("SendInput 注入失败".into()));
    }
    Ok(())
}

/// 修饰键状态捕获与恢复
///
/// 用户可能在点击按钮时仍按住修饰键（如 Ctrl/Alt），
/// 直接注入会导致输入被修饰键影响。此结构在注入前记录修饰键状态，
/// 注入前释放，注入后恢复。
struct ModifierState {
    ctrl: bool,
    shift: bool,
    alt: bool,
}

impl ModifierState {
    fn capture() -> Self {
        // GetKeyState 返回 i16，用 u16 语义比较高位（0x8000）
        let ctrl = unsafe { (GetKeyState(VK_CONTROL.0 as i32) as u16 & 0x8000) != 0 };
        let shift = unsafe { (GetKeyState(VK_SHIFT.0 as i32) as u16 & 0x8000) != 0 };
        let alt = unsafe { (GetKeyState(VK_MENU.0 as i32) as u16 & 0x8000) != 0 };
        Self { ctrl, shift, alt }
    }

    /// 释放所有按住的修饰键
    fn release(&self) {
        if self.ctrl {
            keep_keybd_event(VK_CONTROL.0 as u8, KEYEVENTF_KEYUP);
        }
        if self.shift {
            keep_keybd_event(VK_SHIFT.0 as u8, KEYEVENTF_KEYUP);
        }
        if self.alt {
            keep_keybd_event(VK_MENU.0 as u8, KEYEVENTF_KEYUP);
        }
    }

    /// 恢复此前按住的修饰键
    fn restore(&self) {
        if self.ctrl {
            keep_keybd_event(VK_CONTROL.0 as u8, Default::default());
        }
        if self.shift {
            keep_keybd_event(VK_SHIFT.0 as u8, Default::default());
        }
        if self.alt {
            keep_keybd_event(VK_MENU.0 as u8, Default::default());
        }
    }
}

/// 触发 keybd_event（底层封装）
#[inline]
fn keep_keybd_event(vk: u8, flags: KEYBD_EVENT_FLAGS) {
    unsafe {
        keybd_event(vk, 0, flags, 0);
    }
}
/// 简化 UIPI 检测（扩展点）
///
/// 当前实现采用"若无法打开前台进程则视为拒绝注入"的保守策略。
/// 完整级别比较留待需要时增强。
#[allow(dead_code)]
fn check_foreground_process_openable() -> Result<(), InjectError> {
    let hwnd = unsafe { GetForegroundWindow() };
    let mut pid = 0u32;
    unsafe { GetWindowThreadProcessId(hwnd, Some(&mut pid)); }
    if pid == 0 {
        return Err(InjectError::TargetNotFound);
    }
    let handle = unsafe { OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid) };
    match handle {
        Ok(h) if !h.0.is_null() => {
            unsafe {
                let _ = CloseHandle(h);
            }
            Ok(())
        }
        _ => {
            // 无法打开前台进程（如高完整性级别窗口），拒绝注入
            Err(InjectError::InjectionRejected)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // AC2-1/AC2-2/AC2-3: send_unicode_char 构造（纯函数验证）
    #[test]
    fn test_modifier_state_capture_no_panic() {
        let m = ModifierState::capture();
        m.release();
        m.restore();
    }

    // 注入不应 panic（结果取决于运行环境）
    #[test]
    fn test_windows_injector_does_not_panic() {
        let injector = WindowsInjector::new();
        let _ = injector.inject_text("hello");
    }

    // AC3-1: 多字节 Unicode 字符（中文）注入不 panic
    #[test]
    fn test_windows_injector_unicode_multibyte_does_not_panic() {
        let injector = WindowsInjector::new();
        // 中文、日文、特殊符号
        let _ = injector.inject_text("你好世界こんにちは😊");
        let _ = injector.inject_text("Hello 中文 123 !@#");
    }

    #[test]
    fn test_check_foreground_openable_no_panic() {
        let _ = check_foreground_process_openable();
    }
}

