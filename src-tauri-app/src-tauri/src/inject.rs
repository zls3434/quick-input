//! 输入注入引擎抽象层 (M2)
//!
//! 定义统一的跨平台注入接口 `Injector` trait，
//! 各平台独立实现，通过 Feature Flag 编译时选择。
//! 注入时自动使用 `FocusGuard` 保护焦点（AC1-4）。

use std::fmt;

/// 注入错误类型
#[derive(Debug, Clone, PartialEq)]
pub enum InjectError {
    /// 权限不足（如 macOS Accessibility 未授权）
    /// 该变体供 macOS 平台使用，Windows 实现当前不构造
    #[allow(dead_code)]
    PermissionDenied,
    /// 注入被系统拒绝（如 Windows UIPI 高完整性窗口）
    InjectionRejected,
    /// 未找到目标输入框
    TargetNotFound,
    /// 未知错误
    Unknown(String),
}

impl fmt::Display for InjectError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InjectError::PermissionDenied => write!(f, "权限不足，无法注入"),
            InjectError::InjectionRejected => write!(f, "注入被系统拒绝"),
            InjectError::TargetNotFound => write!(f, "未找到目标输入框"),
            InjectError::Unknown(msg) => write!(f, "注入错误: {msg}"),
        }
    }
}

impl std::error::Error for InjectError {}

/// 统一注入器接口
///
/// 定义跨平台文本注入的抽象契约。
/// 各平台实现（Windows / macOS / Linux）通过 `Injector` trait 统一调用。
pub trait Injector {
    /// 将 text 注入到当前焦点输入框
    ///
    /// 实现者需确保：
    /// - 注入前通过 `FocusGuard` 记录当前焦点
    /// - 注入后自动恢复焦点（RAII 析构或显式恢复）
    fn inject_text(&self, text: &str) -> Result<(), InjectError>;

    /// 注入文本后发送 N 个左方向键（光标回退）
    ///
    /// 模板按钮左键输出时占位符位置留空，光标需回退到占位符处
    /// （如 git commit -m "" 输出后光标落在引号中间）。
    /// 默认实现退化为普通注入（不支持平台忽略回退）。
    fn inject_text_ext(&self, text: &str, cursor_back: u32) -> Result<(), InjectError> {
        let _ = cursor_back;
        self.inject_text(text)
    }

    /// 按指定注入模式注入文本（paste=剪贴板粘贴 / keystroke=按键模拟）
    ///
    /// keystroke 模式面向老游戏（DirectInput/自绘输入框）：粘贴与
    /// KEYEVENTF_UNICODE 均无效时，用真实扫描码模拟物理键盘。
    /// 平台不支持的模式回退默认注入。
    fn inject_text_mode(
        &self,
        text: &str,
        cursor_back: u32,
        mode: &str,
    ) -> Result<(), InjectError> {
        let _ = mode;
        self.inject_text_ext(text, cursor_back)
    }

    /// 向当前焦点输入框发送回车键
    ///
    /// 供"长按输入后回车"交互使用：文本已注入完毕后补发一次 Enter。
    /// 默认实现返回不支持，各平台可按需覆盖。
    fn inject_enter(&self) -> Result<(), InjectError> {
        Err(InjectError::Unknown("当前平台不支持回车注入".into()))
    }

    /// 按指定注入模式发送回车键（paste / keystroke）
    ///
    /// keystroke 模式面向老游戏：纯虚拟键（wVk）注入会被
    /// DirectInput/自绘输入框忽略，须发真实扫描码。
    /// 默认实现回退 inject_enter（平台不支持模式时行为不变）。
    fn inject_enter_mode(&self, mode: &str) -> Result<(), InjectError> {
        let _ = mode;
        self.inject_enter()
    }
}

// ============================================================
// 平台相关注入器类型别名
// ============================================================

/// Windows 注入器（使用 SendInput + KEYEVENTF_UNICODE）
#[cfg(target_os = "windows")]
pub type PlatformInjector = WindowsInjector;

/// macOS 注入器（使用 CGEvent，需 Accessibility 权限）
#[cfg(target_os = "macos")]
pub type PlatformInjector = MacosInjector;

/// Linux 注入器（使用 xdotool / wtype）
#[cfg(target_os = "linux")]
pub type PlatformInjector = LinuxInjector;

// ============================================================
// Windows 注入器（真实实现，来自 inject_windows 模块）
// ============================================================

/// Windows 注入器
///
/// 使用 Win32 `SendInput` + `KEYEVENTF_UNICODE` 实现 Unicode 文本注入。
/// 注入前通过 `FocusGuard` 记录焦点，注入后自动恢复。
#[cfg(target_os = "windows")]
pub use crate::inject_windows::WindowsInjector;

// ============================================================
// macOS 注入器（S122 实现，来自 inject_macos 模块）
// ============================================================

/// macOS 注入器
///
/// 使用 `osascript` 模拟键盘输入实现文本注入。
#[cfg(target_os = "macos")]
pub use crate::inject_macos::MacosInjector;

// ============================================================
// Linux 注入器（S122 实现，来自 inject_linux 模块）
// ============================================================

/// Linux 注入器
///
/// X11 使用 `xdotool type`，Wayland 使用 `wtype` 实现注入。
#[cfg(target_os = "linux")]
pub use crate::inject_linux::LinuxInjector;

// ============================================================
// 单元测试
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    // AC1-1: Injector trait 包含 inject_text 方法
    #[test]
    fn test_ac1_1_injector_trait_exists() {
        // 编译时验证：trait 定义了 inject_text 方法
        fn _assert_trait<T: Injector>() {}
        // 仅验证 trait 可编译
    }

    // AC1-2: 平台注入器实现同一 trait
    #[test]
    fn test_ac1_2_platform_injector_implements_trait() {
        // 在当前平台创建注入器，验证它实现了 Injector trait
        let injector = PlatformInjector::new();
        // 验证它实现了 Injector
        fn takes_injector(_i: &dyn Injector) {}
        takes_injector(&injector);
    }

    // AC1-3: InjectError 枚举变体
    #[test]
    fn test_ac1_3_inject_error_variants() {
        let err1 = InjectError::PermissionDenied;
        let err2 = InjectError::InjectionRejected;
        let err3 = InjectError::TargetNotFound;
        let err4 = InjectError::Unknown("test".into());

        // Display 实现
        assert_eq!(err1.to_string(), "权限不足，无法注入");
        assert_eq!(err2.to_string(), "注入被系统拒绝");
        assert_eq!(err3.to_string(), "未找到目标输入框");
        assert!(err4.to_string().contains("test"));

        // PartialEq 比较
        assert_eq!(err1, InjectError::PermissionDenied);
        assert_ne!(err1, err2);
    }

    // AC1-4: 注入器在真实环境中不应 panic（结果取决于环境）
    #[test]
    fn test_ac1_4_injector_does_not_panic() {
        let injector = PlatformInjector::new();
        // 在真实 Windows 环境中，注入可能成功或失败，但不应 panic
        let _ = injector.inject_text("hello");
    }

    // 跨平台编译：所有平台注入器均可创建
    #[test]
    fn test_platform_injector_constructs() {
        let _inj = PlatformInjector::new();
    }

    // InjectError 实现 std::error::Error
    #[test]
    fn test_inject_error_implements_std_error() {
        fn assert_error<E: std::error::Error>() {}
        assert_error::<InjectError>();
    }
}