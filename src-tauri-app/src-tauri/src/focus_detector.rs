//! 焦点识别模块 (M3)
//!
//! 识别当前前台窗口所属进程名，供按应用自动切换配置使用。
//! 首期实现 Windows 平台：`GetForegroundWindow` → `GetWindowThreadProcessId`
//! → `QueryFullProcessImageName` 提取进程名。

/// 焦点识别错误类型
#[derive(Debug, Clone, PartialEq)]
pub enum FocusDetectError {
    /// 无法获取前台窗口
    NoForegroundWindow,
    /// 无法获取窗口所属进程 ID
    NoProcessId,
    /// 无法打开进程句柄
    OpenProcessFailed,
    /// 无法获取进程映像路径
    QueryImageNameFailed,
    /// 进程路径无法转为 UTF-8
    InvalidName(String),
}

impl std::fmt::Display for FocusDetectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FocusDetectError::NoForegroundWindow => write!(f, "无法获取前台窗口"),
            FocusDetectError::NoProcessId => write!(f, "无法获取窗口所属进程 ID"),
            FocusDetectError::OpenProcessFailed => write!(f, "无法打开进程句柄"),
            FocusDetectError::QueryImageNameFailed => write!(f, "无法获取进程映像路径"),
            FocusDetectError::InvalidName(name) => write!(f, "进程路径无法转 UTF-8: {name}"),
        }
    }
}

impl std::error::Error for FocusDetectError {}

/// 识别当前前台窗口所属进程名
///
/// 返回含 `.exe` 后缀的完整进程名（如 `"Code.exe"`、`"WindowsTerminal.exe"`），
/// 与 M4 配置的 `process_name` 字段匹配约定一致（大小写敏感比较由调用方决定，
/// 但 M4 的匹配已做大小写不敏感处理）。
///
/// 失败时返回明确错误，不 panic。
#[cfg(target_os = "windows")]
pub fn current_foreground_process() -> Result<String, FocusDetectError> {
    use windows::Win32::Foundation::CloseHandle;
    use windows::Win32::System::Threading::{
        OpenProcess, PROCESS_NAME_FORMAT, PROCESS_QUERY_INFORMATION, QueryFullProcessImageNameW,
    };
    use windows::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowThreadProcessId};

    // 1. 获取前台窗口
    let hwnd = unsafe { GetForegroundWindow() };
    if hwnd.0.is_null() {
        return Err(FocusDetectError::NoForegroundWindow);
    }

    // 2. 获取窗口所属进程 ID
    let mut pid = 0u32;
    unsafe { GetWindowThreadProcessId(hwnd, Some(&mut pid)); }
    if pid == 0 {
        return Err(FocusDetectError::NoProcessId);
    }

    // 3. 打开进程句柄
    let handle = unsafe { OpenProcess(PROCESS_QUERY_INFORMATION, false, pid) };
    match handle {
        Ok(h) if !h.0.is_null() => {
            // 4. 获取进程映像完整路径
            let mut buffer = vec![0u16; 4096];
            let mut size = buffer.len() as u32;
            let ok = unsafe {
                QueryFullProcessImageNameW(
                    h,
                    PROCESS_NAME_FORMAT(0),
                    windows::core::PWSTR(buffer.as_mut_ptr()),
                    &mut size,
                )
            };
            unsafe {
                let _ = CloseHandle(h);
            }
            if ok.is_err() {
                return Err(FocusDetectError::QueryImageNameFailed);
            }

            // 5. 转为 UTF-8 字符串
            let path = String::from_utf16(&buffer[..size as usize])
                .map_err(|e| FocusDetectError::InvalidName(e.to_string()))?;

            // 6. 提取文件名（含 .exe）
            let file_name = std::path::Path::new(&path)
                .file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_default();
            if file_name.is_empty() {
                return Err(FocusDetectError::InvalidName(path));
            }
            Ok(file_name)
        }
        _ => Err(FocusDetectError::OpenProcessFailed),
    }
}

/// 非 Windows 平台占位实现
#[cfg(not(target_os = "windows"))]
pub fn current_foreground_process() -> Result<String, FocusDetectError> {
    Err(FocusDetectError::NoForegroundWindow)
}

#[cfg(test)]
mod tests {
    use super::*;

    // AC5-1/AC5-2: 前台进程识别在本机可运行（不 panic）
    #[test]
    fn test_current_foreground_process_does_not_panic() {
        // 在真实 Windows 环境中获取前台进程
        let result = current_foreground_process();
        // 不应 panic。结果可能是 Ok(进程名) 或 Err（取决于环境）
        let _ = result;
    }

    // 错误类型可比较、可 Display
    #[test]
    fn test_focus_detect_error_display() {
        let e = FocusDetectError::NoForegroundWindow;
        assert_eq!(e.to_string(), "无法获取前台窗口");
        assert_eq!(e, FocusDetectError::NoForegroundWindow);
    }

    // 错误类型实现 std::error::Error
    #[test]
    fn test_focus_detect_error_implements_std_error() {
        fn assert_error<E: std::error::Error>() {}
        assert_error::<FocusDetectError>();
    }
}