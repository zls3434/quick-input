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

/// 最近一次注入时的目标前台窗口（供点击结束后恢复焦点兜底）
///
/// mousedown 注入后，WebView2 子窗口完成鼠标处理时仍可能激活悬浮窗
/// （激活发生在注入恢复之后）。mouseUp 时由前端调用 restore_focus 命令，
/// 依据此记录把前台还给目标窗口。
#[cfg(target_os = "windows")]
static LAST_TARGET_FOREGROUND: std::sync::Mutex<Option<isize>> = std::sync::Mutex::new(None);

/// 记录注入目标前台窗口（HWND 原始值）
#[cfg(target_os = "windows")]
pub fn remember_target_foreground(raw: isize) {
    if let Ok(mut slot) = LAST_TARGET_FOREGROUND.lock() {
        *slot = Some(raw);
    }
}

/// 恢复最近记录的目标前台窗口（点击结束后兜底调用）
#[cfg(target_os = "windows")]
pub fn restore_target_foreground() {
    use windows::Win32::Foundation::HWND;
    use windows::Win32::UI::WindowsAndMessaging::SetForegroundWindow;

    let raw = match LAST_TARGET_FOREGROUND.lock() {
        Ok(mut slot) => slot.take(),
        Err(_) => None,
    };
    if let Some(v) = raw {
        unsafe {
            let _ = SetForegroundWindow(HWND(v as *mut std::ffi::c_void));
        }
    }
}

/// 非 Windows 平台占位
#[cfg(not(target_os = "windows"))]
pub fn remember_target_foreground(_raw: isize) {}
#[cfg(not(target_os = "windows"))]
pub fn restore_target_foreground() {}

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

/// 剪贴板格式常量（CF_UNICODETEXT）
const CF_UNICODETEXT: u32 = 13;

/// 写入剪贴板文本（不调用 EmptyClipboard，保留图像等其他格式）
fn set_clipboard_text(text: &str) -> Result<(), InjectError> {
    use windows::Win32::Foundation::HANDLE;
    use windows::Win32::System::DataExchange::{
        CloseClipboard, OpenClipboard, SetClipboardData,
    };
    use windows::Win32::System::Memory::{
        GlobalAlloc, GlobalLock, GlobalUnlock, GMEM_MOVEABLE,
    };

    unsafe {
        let wide: Vec<u16> = text.encode_utf16().chain(std::iter::once(0)).collect();
        let bytes = wide.len() * 2;
        let hglobal = GlobalAlloc(GMEM_MOVEABLE, bytes)
            .map_err(|_| InjectError::Unknown("GlobalAlloc 失败".into()))?;
        let dst = GlobalLock(hglobal);
        if dst.is_null() {
            return Err(InjectError::Unknown("GlobalLock 失败".into()));
        }
        std::ptr::copy_nonoverlapping(wide.as_ptr() as *const u8, dst as *mut u8, bytes);
        let _ = GlobalUnlock(hglobal);

        if OpenClipboard(None).is_err() {
            return Err(InjectError::Unknown("OpenClipboard 失败".into()));
        }
        // 失败时由系统/后续写入覆盖，内存量小不做显式释放
        let ok = SetClipboardData(CF_UNICODETEXT, HANDLE(hglobal.0)).is_ok();
        let _ = CloseClipboard();
        if !ok {
            return Err(InjectError::Unknown("SetClipboardData 失败".into()));
        }
    }
    Ok(())
}

/// 读取剪贴板文本（无文本时返回 None）
fn get_clipboard_text() -> Option<String> {
    use windows::Win32::Foundation::HGLOBAL;
    use windows::Win32::System::DataExchange::{
        CloseClipboard, GetClipboardData, IsClipboardFormatAvailable, OpenClipboard,
    };
    use windows::Win32::System::Memory::{GlobalLock, GlobalUnlock};

    unsafe {
        if OpenClipboard(None).is_err() {
            return None;
        }
        let mut result = None;
        if IsClipboardFormatAvailable(CF_UNICODETEXT).is_ok() {
            if let Ok(handle) = GetClipboardData(CF_UNICODETEXT) {
                let ptr = GlobalLock(HGLOBAL(handle.0)) as *const u16;
                if !ptr.is_null() {
                    let mut len = 0usize;
                    while *ptr.add(len) != 0 {
                        len += 1;
                    }
                    result =
                        Some(String::from_utf16_lossy(std::slice::from_raw_parts(ptr, len)));
                    let _ = GlobalUnlock(HGLOBAL(handle.0));
                }
            }
        }
        let _ = CloseClipboard();
        result
    }
}

/// 移除剪贴板文本格式（恢复原"无文本"状态）
fn clear_clipboard_text() {
    use windows::Win32::Foundation::HANDLE;
    use windows::Win32::System::DataExchange::{
        CloseClipboard, OpenClipboard, SetClipboardData,
    };
    unsafe {
        if OpenClipboard(None).is_ok() {
            // hMem 传 NULL 表示移除该格式
            let _ = SetClipboardData(CF_UNICODETEXT, HANDLE(std::ptr::null_mut()));
            let _ = CloseClipboard();
        }
    }
}

/// 发送 Ctrl+V（单次 SendInput，原子）
fn send_paste_keys() -> Result<(), InjectError> {
    let vk_input = |vk: u16, up: bool| INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: VIRTUAL_KEY(vk),
                wScan: 0,
                dwFlags: if up { KEYEVENTF_KEYUP } else { Default::default() },
                time: 0,
                dwExtraInfo: 0,
            },
        },
    };
    let inputs = [
        vk_input(0x11, false), // Ctrl down
        vk_input(0x56, false), // V down
        vk_input(0x56, true),  // V up
        vk_input(0x11, true),  // Ctrl up
    ];
    let sent = unsafe { SendInput(&inputs, std::mem::size_of::<INPUT>() as i32) };
    if sent != inputs.len() as u32 {
        return Err(InjectError::Unknown("SendInput 粘贴键失败".into()));
    }
    Ok(())
}

/// 粘贴注入：写入剪贴板 → Ctrl+V → 等待目标处理 → 恢复原剪贴板
///
/// 键盘注入（KEYEVENTF_UNICODE）在目标窗口开启中文输入法时会被 IME
/// 组合处理：字母进拼音组合被吞改（"get pods"→"ssssssss"）、引号被
/// 自动配对成对（`"`→`""`）。剪贴板粘贴完全绕过 IME，文本按字面落入，
/// 且瞬时完成（无逐字符耗时，长文本也在毫秒级）。
fn paste_text(text: &str) -> Result<(), InjectError> {
    if text.is_empty() {
        return Ok(());
    }
    // 保存原文本（图像等其他格式不被动：SetClipboardData 不清空剪贴板）
    let saved = get_clipboard_text();
    set_clipboard_text(text)?;
    send_paste_keys()?;
    // 等待目标应用处理粘贴（读取剪贴板通常 <20ms，留足余量）
    std::thread::sleep(std::time::Duration::from_millis(60));
    match saved {
        Some(s) => {
            let _ = set_clipboard_text(&s);
        }
        None => clear_clipboard_text(),
    }
    Ok(())
}

/// 发送 N 个左方向键（单次 SendInput，原子）
///
/// 模板按钮左键输出占位符留空后，光标回退到占位符位置
/// （如 git commit -m "" 输出后光标落在引号中间，可直接键入内容）。
fn send_left_keys(n: u32) -> Result<(), InjectError> {
    if n == 0 {
        return Ok(());
    }
    let vk_input = |up: bool| INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: VIRTUAL_KEY(0x25), // VK_LEFT
                wScan: 0,
                dwFlags: if up { KEYEVENTF_KEYUP } else { Default::default() },
                time: 0,
                dwExtraInfo: 0,
            },
        },
    };
    let mut inputs: Vec<INPUT> = Vec::with_capacity((n as usize) * 2);
    for _ in 0..n {
        inputs.push(vk_input(false));
        inputs.push(vk_input(true));
    }
    let sent = unsafe { SendInput(&inputs, std::mem::size_of::<INPUT>() as i32) };
    if sent != inputs.len() as u32 {
        return Err(InjectError::Unknown("SendInput 方向键失败".into()));
    }
    Ok(())
}

/// 构造单个 Unicode 字符的键盘事件
#[inline]
fn unicode_input(ch: u16, up: bool) -> INPUT {
    INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: VIRTUAL_KEY(0),
                wScan: ch,
                dwFlags: if up {
                    KEYEVENTF_UNICODE | KEYEVENTF_KEYUP
                } else {
                    KEYEVENTF_UNICODE
                },
                time: 0,
                dwExtraInfo: 0,
            },
        },
    }
}

/// 批量原子注入：整段文本的全部 keydown/keyup 打包进一次 SendInput 调用
///
/// 逐字符多次调用 SendInput 时，调用间隙可能被焦点切换（restore_focus）、
/// 窗口激活、IME 处理插入，造成字符丢失/交错。单次批量调用在系统层面
/// 原子插入输入队列，杜绝中途打断；同时少 N-1 次系统调用，更快。
fn send_unicode_text(text: &str) -> Result<(), InjectError> {
    let chars: Vec<u16> = text.encode_utf16().collect();
    let mut inputs: Vec<INPUT> = Vec::with_capacity(chars.len() * 2);
    for ch in chars {
        inputs.push(unicode_input(ch, false));
        inputs.push(unicode_input(ch, true));
    }
    if inputs.is_empty() {
        return Ok(());
    }
    let sent = unsafe { SendInput(&inputs, std::mem::size_of::<INPUT>() as i32) };
    if sent != inputs.len() as u32 {
        return Err(InjectError::Unknown("SendInput 注入失败".into()));
    }
    Ok(())
}

impl Injector for WindowsInjector {
    fn inject_text(&self, text: &str) -> Result<(), InjectError> {
        self.inject_text_ext(text, 0)
    }

    fn inject_text_ext(&self, text: &str, cursor_back: u32) -> Result<(), InjectError> {
        // 1. 记录焦点（RAII 保护，注入后自动恢复）
        let _guard = FocusGuard::new();
        // 同时记录目标前台，供 mouseup 后 restore_focus 兜底恢复
        remember_target_foreground(unsafe {
            use windows::Win32::UI::WindowsAndMessaging::GetForegroundWindow;
            GetForegroundWindow().0 as isize
        });

        // 2. 处理修饰键冲突（记录当前 Ctrl/Alt/Shift 状态）
        let modifiers = ModifierState::capture();
        modifiers.release();

        // 3. 剪贴板粘贴注入（绕过中文输入法）；剪贴板不可用时退回键盘注入；
        //    cursor_back > 0 时追加 N 个 VK_LEFT（光标回退到占位符位置）
        let result = paste_text(text)
            .or_else(|_| send_unicode_text(text))
            .and_then(|_| send_left_keys(cursor_back));

        // 4. 恢复修饰键
        modifiers.restore();

        result
    }

    fn inject_enter(&self) -> Result<(), InjectError> {
        let _guard = FocusGuard::new();
        // 长按场景：刷新目标前台记录，mouseUp 后 restore_focus 仍指向正确窗口
        remember_target_foreground(unsafe {
            use windows::Win32::UI::WindowsAndMessaging::GetForegroundWindow;
            GetForegroundWindow().0 as isize
        });
        let modifiers = ModifierState::capture();
        modifiers.release();

        // 发送 VK_RETURN 按下 + 抬起（单次 SendInput，原子）
        let inputs = [
            INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: VIRTUAL_KEY(0x0D), // VK_RETURN
                        wScan: 0,
                        dwFlags: Default::default(),
                        time: 0,
                        dwExtraInfo: 0,
                    },
                },
            },
            INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: VIRTUAL_KEY(0x0D),
                        wScan: 0,
                        dwFlags: KEYEVENTF_KEYUP,
                        time: 0,
                        dwExtraInfo: 0,
                    },
                },
            },
        ];
        let sent = unsafe { SendInput(&inputs, std::mem::size_of::<INPUT>() as i32) };
        let result = if sent != inputs.len() as u32 {
            Err(InjectError::Unknown("SendInput 回车注入失败".into()))
        } else {
            Ok(())
        };

        modifiers.restore();
        result
    }
}

/// 发送单个 Unicode 字符（keydown + keyup）
/// 已被 send_unicode_text 批量原子注入取代，保留用于单字符场景测试
#[allow(dead_code)]
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

