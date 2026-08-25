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
/// 发送 N 个左方向键（光标回退）
///
/// - `scan_mode=false`：虚拟键注入（现代应用通用）
/// - `scan_mode=true`：扫描码注入（老游戏 DirectInput/自绘输入框只认扫描码；
///   方向键为扩展键，须带 KEYEVENTF_EXTENDEDKEY，否则被识别为小键盘 4）
fn send_left_keys(n: u32, scan_mode: bool) -> Result<(), InjectError> {
    if n == 0 {
        return Ok(());
    }
    let mut inputs: Vec<INPUT> = Vec::with_capacity((n as usize) * 2);
    for _ in 0..n {
        if scan_mode {
            inputs.push(scancode_input_ext(SCAN_LEFT, false));
            inputs.push(scancode_input_ext(SCAN_LEFT, true));
        } else {
            inputs.push(vk_left_input(false));
            inputs.push(vk_left_input(true));
        }
    }
    let sent = unsafe { SendInput(&inputs, std::mem::size_of::<INPUT>() as i32) };
    if sent != inputs.len() as u32 {
        return Err(InjectError::Unknown("SendInput 方向键失败".into()));
    }
    Ok(())
}

/// 构造左方向键虚拟键事件（现代应用路径）
#[inline]
fn vk_left_input(up: bool) -> INPUT {
    INPUT {
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
    }
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

/// 构造扫描码键盘事件（KEYEVENTF_SCANCODE：走物理键盘输入路径）
#[inline]
fn scancode_input(scan: u16, up: bool) -> INPUT {
    use windows::Win32::UI::Input::KeyboardAndMouse::KEYEVENTF_SCANCODE;
    INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: VIRTUAL_KEY(0),
                wScan: scan,
                dwFlags: if up {
                    KEYEVENTF_SCANCODE | KEYEVENTF_KEYUP
                } else {
                    KEYEVENTF_SCANCODE
                },
                time: 0,
                dwExtraInfo: 0,
            },
        },
    }
}

/// 左 Shift 扫描码（Set 1）
const SCAN_LSHIFT: u16 = 0x2A;

/// 主键盘 Enter 扫描码（Set 1）
const SCAN_ENTER: u16 = 0x1C;

/// 左方向键扫描码（Set 1，扩展键：需 KEYEVENTF_EXTENDEDKEY）
const SCAN_LEFT: u16 = 0x4B;

/// 构造扩展键扫描码键盘事件（方向键等：KEYEVENTF_SCANCODE | EXTENDEDKEY）
///
/// 不带 EXTENDEDKEY 时扫描码 0x4B 会被目标应用解释为小键盘 4。
#[inline]
fn scancode_input_ext(scan: u16, up: bool) -> INPUT {
    use windows::Win32::UI::Input::KeyboardAndMouse::KEYEVENTF_EXTENDEDKEY;
    let mut i = scancode_input(scan, up);
    unsafe {
        i.Anonymous.ki.dwFlags |= KEYEVENTF_EXTENDEDKEY;
    }
    i
}

/// 按键序列描述：一个字符对应的扫描码与修饰键需求
#[derive(Debug, PartialEq)]
struct KeyPlan {
    scan: u16,
    need_shift: bool,
}

/// 把单个字符解析为按键计划（当前键盘布局反查）
///
/// 返回 None 表示布局无对应键位（如中文、生僻符号）或需要
/// Ctrl/Alt 组合（模拟会误触游戏快捷键）——调用方回退 Unicode 注入。
fn plan_char(ch: u16, hkl: windows::Win32::UI::Input::KeyboardAndMouse::HKL) -> Option<KeyPlan> {
    use windows::Win32::UI::Input::KeyboardAndMouse::{MapVirtualKeyExW, VkKeyScanExW, MAPVK_VK_TO_VSC};
    unsafe {
        let vks = VkKeyScanExW(ch, hkl);
        if vks == -1 {
            return None;
        }
        // 高字节修饰键状态：bit0=Shift bit1=Ctrl bit2=Alt（取 0x07 全三位）
        let state = ((vks >> 8) & 0x07) as u16;
        if state & 0x06 != 0 {
            // 需要 Ctrl/Alt 组合的字符拒绝模拟（会误触目标应用快捷键）
            return None;
        }
        let vk = (vks & 0xFF) as u32;
        let scan = MapVirtualKeyExW(vk, MAPVK_VK_TO_VSC, hkl) as u16;
        if scan == 0 {
            return None;
        }
        Some(KeyPlan {
            scan,
            need_shift: state & 0x01 != 0,
        })
    }
}

/// 为整段文本构造扫描码按键序列（纯函数，便于单测）
///
/// - Shift 域管理：进入大写/符号区前按下左 Shift，离开时释放，结尾统一释放
/// - 无法映射的字符（中文等）回退 KEYEVENTF_UNICODE 逐字符
fn build_keystroke_inputs(
    text: &str,
    hkl: windows::Win32::UI::Input::KeyboardAndMouse::HKL,
) -> Vec<INPUT> {
    let mut inputs: Vec<INPUT> = Vec::with_capacity(text.len() * 2 + 4);
    let mut shift_down = false;
    for ch in text.encode_utf16() {
        match plan_char(ch, hkl) {
            Some(plan) => {
                if plan.need_shift && !shift_down {
                    inputs.push(scancode_input(SCAN_LSHIFT, false));
                    shift_down = true;
                } else if !plan.need_shift && shift_down {
                    inputs.push(scancode_input(SCAN_LSHIFT, true));
                    shift_down = false;
                }
                inputs.push(scancode_input(plan.scan, false));
                inputs.push(scancode_input(plan.scan, true));
            }
            None => {
                // 离开 Shift 域后再注入 Unicode 字符，避免组合污染
                if shift_down {
                    inputs.push(scancode_input(SCAN_LSHIFT, true));
                    shift_down = false;
                }
                inputs.push(unicode_input(ch, false));
                inputs.push(unicode_input(ch, true));
            }
        }
    }
    if shift_down {
        inputs.push(scancode_input(SCAN_LSHIFT, true));
    }
    inputs
}

/// 检测指定进程是否以管理员（提升）权限运行
fn is_process_elevated(pid: u32) -> bool {
    use windows::Win32::Foundation::{CloseHandle, HANDLE};
    use windows::Win32::Security::{
        GetTokenInformation, TokenElevation, TOKEN_ELEVATION, TOKEN_QUERY,
    };
    use windows::Win32::System::Threading::{
        OpenProcess, OpenProcessToken, PROCESS_QUERY_LIMITED_INFORMATION,
    };

    unsafe {
        let Ok(hproc) = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid) else {
            return false;
        };
        let mut token = HANDLE::default();
        let ok = OpenProcessToken(hproc, TOKEN_QUERY, &mut token).is_ok();
        let _ = CloseHandle(hproc);
        if !ok {
            return false;
        }
        let mut elev = TOKEN_ELEVATION::default();
        let mut ret = 0u32;
        let ok = GetTokenInformation(
            token,
            TokenElevation,
            Some(&mut elev as *mut _ as *mut std::ffi::c_void),
            std::mem::size_of::<TOKEN_ELEVATION>() as u32,
            &mut ret,
        )
        .is_ok();
        let _ = CloseHandle(token);
        ok && elev.TokenIsElevated != 0
    }
}

/// 检测自身（QuickInput 进程）是否以管理员权限运行
fn is_self_elevated() -> bool {
    use windows::Win32::Foundation::{CloseHandle, HANDLE};
    use windows::Win32::Security::{
        GetTokenInformation, TokenElevation, TOKEN_ELEVATION, TOKEN_QUERY,
    };
    use windows::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken};

    unsafe {
        let mut token = HANDLE::default();
        if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token).is_err() {
            return false;
        }
        let mut elev = TOKEN_ELEVATION::default();
        let mut ret = 0u32;
        let ok = GetTokenInformation(
            token,
            TokenElevation,
            Some(&mut elev as *mut _ as *mut std::ffi::c_void),
            std::mem::size_of::<TOKEN_ELEVATION>() as u32,
            &mut ret,
        )
        .is_ok();
        let _ = CloseHandle(token);
        ok && elev.TokenIsElevated != 0
    }
}

/// 按键模拟注入：把文本逐字符映射为真实键盘扫描码事件（KEYEVENTF_SCANCODE）
///
/// 老游戏（DirectInput 轮询 / 自绘输入框只处理 WM_KEYDOWN 的 VK 与扫描码 /
/// 只认扫描码的客户端）既不响应剪贴板粘贴，也不认识 KEYEVENTF_UNICODE
/// 产生的 VK_PACKET(0xE7)。此模式用当前键盘布局反查每个字符的虚拟键与
/// Shift 状态，再以扫描码事件模拟物理键盘，走与真实打字完全相同的输入
/// 路径。无法映射的字符（如中文）回退 Unicode 逐字符注入。
///
/// UIPI 保护：目标窗口以管理员运行而本程序不是时，SendInput 会被系统
/// 静默丢弃——提前检测并返回明确错误，提示用户以管理员身份运行。
fn send_keystroke_text(text: &str) -> Result<(), InjectError> {
    use windows::Win32::UI::Input::KeyboardAndMouse::GetKeyboardLayout;

    // UIPI 检测：前台窗口提升运行且自身未提升 → 直接报错（提示用户）
    unsafe {
        let hwnd = GetForegroundWindow();
        if !hwnd.0.is_null() {
            let mut pid = 0u32;
            GetWindowThreadProcessId(hwnd, Some(&mut pid));
            if pid != 0 && is_process_elevated(pid) && !is_self_elevated() {
                return Err(InjectError::Unknown(
                    "目标窗口以管理员身份运行，注入被系统拦截；请以管理员身份重启 QuickInput".into(),
                ));
            }
        }
    }

    let inputs = unsafe { build_keystroke_inputs(text, GetKeyboardLayout(0)) };
    if inputs.is_empty() {
        return Ok(());
    }
    let sent = unsafe { SendInput(&inputs, std::mem::size_of::<INPUT>() as i32) };
    if sent != inputs.len() as u32 {
        return Err(InjectError::Unknown("SendInput 按键注入失败".into()));
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
            .and_then(|_| send_left_keys(cursor_back, false));

        // 4. 恢复修饰键
        modifiers.restore();

        result
    }

    fn inject_text_mode(&self, text: &str, cursor_back: u32, mode: &str) -> Result<(), InjectError> {
        if mode != "keystroke" {
            // paste 及未知模式走默认链路（粘贴 → Unicode 回退）
            return self.inject_text_ext(text, cursor_back);
        }
        // keystroke：扫描码按键模拟（老游戏）。不做粘贴（自绘输入框
        // 不响应 Ctrl+V），不逐字符回退 Unicode（VK_PACKET 不被识别）。
        let _guard = FocusGuard::new();
        remember_target_foreground(unsafe {
            use windows::Win32::UI::WindowsAndMessaging::GetForegroundWindow;
            GetForegroundWindow().0 as isize
        });
        let modifiers = ModifierState::capture();
        modifiers.release();
        let result = send_keystroke_text(text).and_then(|_| send_left_keys(cursor_back, true));
        modifiers.restore();
        result
    }

    fn inject_enter_mode(&self, mode: &str) -> Result<(), InjectError> {
        if mode != "keystroke" {
            return self.inject_enter();
        }
        // keystroke：真实扫描码回车（老游戏只认扫描码，纯虚拟键被忽略）
        let _guard = FocusGuard::new();
        remember_target_foreground(unsafe {
            use windows::Win32::UI::WindowsAndMessaging::GetForegroundWindow;
            GetForegroundWindow().0 as isize
        });
        let modifiers = ModifierState::capture();
        modifiers.release();
        let inputs = [
            scancode_input(SCAN_ENTER, false),
            scancode_input(SCAN_ENTER, true),
        ];
        let sent = unsafe { SendInput(&inputs, std::mem::size_of::<INPUT>() as i32) };
        let result = if sent != inputs.len() as u32 {
            Err(InjectError::Unknown("SendInput 回车注入失败（扫描码）".into()))
        } else {
            Ok(())
        };
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

    /// 真实 SendInput 测试的全局锁：并行注入 + FocusGuard 焦点线程操作
    /// 会相互干扰（曾致测试进程堆损坏），必须串行执行
    static SEND_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

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
        let _g = SEND_LOCK.lock().unwrap();
        let injector = WindowsInjector::new();
        let _ = injector.inject_text("hello");
    }

    // AC3-1: 多字节 Unicode 字符（中文）注入不 panic
    #[test]
    fn test_windows_injector_unicode_multibyte_does_not_panic() {
        let _g = SEND_LOCK.lock().unwrap();
        let injector = WindowsInjector::new();
        // 中文、日文、特殊符号
        let _ = injector.inject_text("你好世界こんにちは😊");
        let _ = injector.inject_text("Hello 中文 123 !@#");
    }

    #[test]
    fn test_check_foreground_openable_no_panic() {
        let _ = check_foreground_process_openable();
    }

    // ===== 扫描码注入（keystroke 模式）单元测试 =====

    use windows::Win32::UI::Input::KeyboardAndMouse::GetKeyboardLayout;

    /// 美式布局下常见字符可映射为扫描码计划
    #[test]
    fn test_plan_char_ascii_mappable() {
        let hkl = unsafe { GetKeyboardLayout(0) };
        // 小写 a：无 Shift，扫描码 0x1E
        assert_eq!(
            plan_char('a' as u16, hkl),
            Some(KeyPlan { scan: 0x1E, need_shift: false })
        );
        // 大写 A / 感叹号：需要 Shift
        assert_eq!(
            plan_char('A' as u16, hkl).map(|p| p.need_shift),
            Some(true)
        );
        assert_eq!(
            plan_char('!' as u16, hkl).map(|p| p.need_shift),
            Some(true)
        );
    }

    /// 键盘布局无键位的字符（中文）返回 None
    #[test]
    fn test_plan_char_chinese_unmappable() {
        let hkl = unsafe { GetKeyboardLayout(0) };
        assert_eq!(plan_char('中' as u16, hkl), None);
    }

    /// 小写文本：每字符恰好 down+up 两个事件，无 Shift
    #[test]
    fn test_build_keystroke_inputs_lowercase() {
        let hkl = unsafe { GetKeyboardLayout(0) };
        let inputs = build_keystroke_inputs("abc", hkl);
        assert_eq!(inputs.len(), 6); // 3 字符 × (down + up)
        // 全部为扫描码事件（wVk=0），无 Shift（0x2A）
        assert!(inputs
            .iter()
            .all(|i| unsafe { i.Anonymous.ki }.wVk.0 == 0));
        assert!(!inputs.iter().any(|i| unsafe { i.Anonymous.ki }.wScan == SCAN_LSHIFT));
    }

    /// 大写/符号混合：Shift 按下一次、结尾释放（域管理）
    #[test]
    fn test_build_keystroke_inputs_shift_domain() {
        let hkl = unsafe { GetKeyboardLayout(0) };
        let inputs = build_keystroke_inputs("aB!c", hkl);
        // a:2 + [shift down] B:2 [shift? 仍需] !:2 [shift up] c:2
        let shift_events = inputs
            .iter()
            .filter(|i| unsafe { i.Anonymous.ki }.wScan == SCAN_LSHIFT)
            .count();
        // 'B' 与 '!' 连续需要 Shift：down 1 次；'c' 前释放 1 次 = 2 个事件
        assert_eq!(shift_events, 2);
        // 第一个 Shift 事件必须是按下
        let first = inputs
            .iter()
            .find(|i| unsafe { i.Anonymous.ki }.wScan == SCAN_LSHIFT)
            .unwrap();
        let flags = unsafe { first.Anonymous.ki }.dwFlags;
        assert_eq!(flags.0 & 0x0002, 0); // 无 KEYUP → 按下
    }

    /// 全大写文本：结尾必须释放 Shift（不留按下态）
    #[test]
    fn test_build_keystroke_inputs_shift_released_at_end() {
        let hkl = unsafe { GetKeyboardLayout(0) };
        let inputs = build_keystroke_inputs("AB", hkl);
        let last_shift = inputs
            .iter()
            .filter(|i| unsafe { i.Anonymous.ki }.wScan == SCAN_LSHIFT)
            .last()
            .expect("应存在 Shift 释放事件");
        let flags = unsafe { last_shift.Anonymous.ki }.dwFlags;
        assert_ne!(flags.0 & 0x0002, 0); // 含 KEYUP → 释放
    }

    /// 中英混合：中文回退 Unicode 注入（KEYEVENTF_UNICODE=0x0004）
    #[test]
    fn test_build_keystroke_inputs_chinese_fallback_unicode() {
        let hkl = unsafe { GetKeyboardLayout(0) };
        let inputs = build_keystroke_inputs("a中b", hkl);
        let unicode_events = inputs
            .iter()
            .filter(|i| unsafe { i.Anonymous.ki }.dwFlags.0 & 0x0004 != 0)
            .count();
        assert_eq!(unicode_events, 2); // '中' 的 down + up
    }

    /// keystroke 模式注入不 panic（结果取决于运行环境）
    #[test]
    fn test_inject_text_mode_keystroke_no_panic() {
        let _g = SEND_LOCK.lock().unwrap();
        let injector = WindowsInjector::new();
        let _ = injector.inject_text_mode("Hello123", 0, "keystroke");
        let _ = injector.inject_text_mode("a中b", 0, "keystroke");
    }

    /// 未知模式回退默认链路
    #[test]
    fn test_inject_text_mode_unknown_falls_back() {
        let _g = SEND_LOCK.lock().unwrap();
        let injector = WindowsInjector::new();
        let _ = injector.inject_text_mode("hi", 0, "whatever");
    }

    // ===== 回车/方向键扫描码注入（keystroke 模式长按回车）=====

    /// 扩展键事件：扫描码正确且带 KEYEVENTF_EXTENDEDKEY（0x0001），
    /// 否则方向键 0x4B 会被目标应用解释为小键盘 4
    #[test]
    fn test_scancode_input_ext_has_extended_flag() {
        let down = scancode_input_ext(SCAN_LEFT, false);
        let up = scancode_input_ext(SCAN_LEFT, true);
        let fd = unsafe { down.Anonymous.ki }.dwFlags.0;
        let fu = unsafe { up.Anonymous.ki }.dwFlags.0;
        assert_eq!(unsafe { down.Anonymous.ki }.wScan, 0x4B);
        assert_ne!(fd & 0x0001, 0, "down 须含 EXTENDEDKEY");
        assert_ne!(fd & 0x0008, 0, "down 须含 SCANCODE");
        assert_ne!(fu & 0x0001, 0, "up 须含 EXTENDEDKEY");
        assert_ne!(fu & 0x0002, 0, "up 须含 KEYUP");
        assert_ne!(fu & 0x0008, 0, "up 须含 SCANCODE");
    }

    /// 主键盘回车扫描码事件：0x1C，非扩展键（不得带 EXTENDEDKEY）
    #[test]
    fn test_scan_enter_not_extended() {
        let down = scancode_input(SCAN_ENTER, false);
        assert_eq!(unsafe { down.Anonymous.ki }.wScan, 0x1C);
        assert_eq!(
            unsafe { down.Anonymous.ki }.dwFlags.0 & 0x0001,
            0,
            "主键盘 Enter 不是扩展键"
        );
    }

    /// 回车模式注入不 panic：keystroke 与 paste 路径均应可执行
    #[test]
    fn test_inject_enter_mode_no_panic() {
        let _g = SEND_LOCK.lock().unwrap();
        let injector = WindowsInjector::new();
        let _ = injector.inject_enter_mode("keystroke");
        let _ = injector.inject_enter_mode("paste");
    }
}

