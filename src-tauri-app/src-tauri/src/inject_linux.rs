//! Linux 平台文本注入实现 (S122)
//!
//! X11 环境使用 `xdotool type` 实现文本注入。
//! Wayland 环境使用 `wtype` 实现文本注入。
//! 自动检测当前显示服务器，优先使用对应工具。

use std::process::Command;
use crate::inject::{InjectError, Injector};

/// Linux 注入器
pub struct LinuxInjector;

impl LinuxInjector {
    pub fn new() -> Self {
        Self
    }
}

impl Default for LinuxInjector {
    fn default() -> Self {
        Self::new()
    }
}

impl Injector for LinuxInjector {
    fn inject_text(&self, text: &str) -> Result<(), InjectError> {
        // 检测当前显示服务器
        let wayland_display = std::env::var("WAYLAND_DISPLAY").ok();

        if wayland_display.is_some() {
            self.inject_via_wtype(text)
        } else {
            self.inject_via_xdotool(text)
        }
    }
}

impl LinuxInjector {
    /// 通过 xdotool type 注入（X11）
    fn inject_via_xdotool(&self, text: &str) -> Result<(), InjectError> {
        let output = Command::new("xdotool")
            .args(["type", "--clearmodifiers", "--", text])
            .output()
            .map_err(|e| {
                if e.kind() == std::io::ErrorKind::NotFound {
                    InjectError::PermissionDenied
                } else {
                    InjectError::Unknown(format!("xdotool 执行失败: {e}"))
                }
            })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(InjectError::Unknown(format!("xdotool 注入失败: {stderr}")));
        }

        Ok(())
    }

    /// 通过 wtype 注入（Wayland）
    fn inject_via_wtype(&self, text: &str) -> Result<(), InjectError> {
        let output = Command::new("wtype")
            .arg(text)
            .output()
            .map_err(|e| {
                if e.kind() == std::io::ErrorKind::NotFound {
                    InjectError::PermissionDenied
                } else {
                    InjectError::Unknown(format!("wtype 执行失败: {e}"))
                }
            })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(InjectError::Unknown(format!("wtype 注入失败: {stderr}")));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linux_injector_constructs() {
        let injector = LinuxInjector::new();
        let _ = injector.inject_text("test");
        // 在非 Linux 环境上，xdotool/wtype 不存在，应返回错误但不 panic
    }
}