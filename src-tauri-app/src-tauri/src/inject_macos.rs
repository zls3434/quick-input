//! macOS 平台文本注入实现 (S122)
//!
//! 使用 `osascript` 模拟键盘输入，将文本注入到当前焦点应用。
//! 需用户授予 Accessibility 权限（系统偏好设置 → 隐私与安全性 → 辅助功能）。
//!
//! 注：原生实现可使用 Core Graphics `CGEvent` + `CGEventKeyboardSetUnicodeString`，
//! 但 `osascript` 方式无需额外编译依赖，且可在 CI 上编译验证。

use std::process::Command;
use crate::inject::{InjectError, Injector};

/// macOS 注入器
pub struct MacosInjector;

impl MacosInjector {
    pub fn new() -> Self {
        Self
    }
}

impl Default for MacosInjector {
    fn default() -> Self {
        Self::new()
    }
}

impl Injector for MacosInjector {
    fn inject_text(&self, text: &str) -> Result<(), InjectError> {
        // 使用 osascript 模拟键盘输入
        // 转义文本中的特殊字符
        let escaped = text
            .replace('\\', "\\\\")
            .replace('"', "\\\"")
            .replace('\n', "\\n")
            .replace('\r', "\\r")
            .replace('\t', "\\t");

        let script = format!(
            r#"tell application "System Events"
                keystroke "{}"
            end tell"#,
            escaped
        );

        let output = Command::new("osascript")
            .arg("-e")
            .arg(&script)
            .output()
            .map_err(|e| {
                if e.kind() == std::io::ErrorKind::NotFound {
                    InjectError::PermissionDenied
                } else {
                    InjectError::Unknown(format!("osascript 执行失败: {e}"))
                }
            })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            if stderr.contains("not allowed") || stderr.contains("permission") {
                return Err(InjectError::PermissionDenied);
            }
            return Err(InjectError::Unknown(format!("osascript 注入失败: {stderr}")));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_macos_injector_constructs() {
        let injector = MacosInjector::new();
        let _ = injector.inject_text("test");
        // 在非 macOS 环境上，osascript 不存在，应返回 PermissionDenied
        // 但不 panic
    }
}