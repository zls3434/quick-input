//! 动态占位符展开 (M2 增强)
//!
//! 注入文本中的动态占位符在注入时展开为实际值：
//! - `{date}`      → 当前日期（YYYY-MM-DD，如 2026-08-26）
//! - `{time}`      → 当前时间（HH:MM:SS，24 小时制，如 14:30:05）
//! - `{clipboard}` → 当前剪贴板文本（为空则替换为空字符串）
//!
//! `{input}` 不在此展开：它由前端交互处理（左键光标占位 / 右键模板输入），
//! 后端注入时保持原样，避免破坏模板按钮语义。
//!
//! 展开为纯函数：时间与剪贴板内容由调用方传入，便于确定性单元测试。

use chrono::Timelike;

/// 展开文本中的动态占位符
///
/// 未知占位符（如 `{foo}`）与 `{input}` 原样保留。
pub fn expand_placeholders(
    text: &str,
    now: chrono::DateTime<chrono::Local>,
    clipboard: &str,
) -> String {
    let date = now.format("%Y-%m-%d").to_string();
    let time = format!("{:02}:{:02}:{:02}", now.hour(), now.minute(), now.second());
    text.replace("{date}", &date)
        .replace("{time}", &time)
        .replace("{clipboard}", clipboard)
}

/// 读取系统剪贴板文本（占位符 `{clipboard}` 展开用）
///
/// Windows 复用注入引擎的 Win32 剪贴板实现；其他平台暂返回空
/// （`{clipboard}` 展开为空字符串，不影响注入）。
pub fn read_clipboard_text() -> String {
    #[cfg(target_os = "windows")]
    {
        crate::inject_windows::get_clipboard_text().unwrap_or_default()
    }
    #[cfg(not(target_os = "windows"))]
    {
        String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn fixed_now() -> chrono::DateTime<chrono::Local> {
        // 固定时间：2026-08-26 14:30:05（本地时区）
        chrono::Local
            .with_ymd_and_hms(2026, 8, 26, 14, 30, 5)
            .single()
            .expect("固定时间应有效")
    }

    #[test]
    fn test_expand_date_placeholder() {
        assert_eq!(
            expand_placeholders("今天 {date}", fixed_now(), ""),
            "今天 2026-08-26"
        );
    }

    #[test]
    fn test_expand_time_placeholder() {
        assert_eq!(
            expand_placeholders("现在 {time}", fixed_now(), ""),
            "现在 14:30:05"
        );
    }

    #[test]
    fn test_expand_clipboard_placeholder() {
        assert_eq!(
            expand_placeholders("粘贴 {clipboard}", fixed_now(), "hello world"),
            "粘贴 hello world"
        );
    }

    #[test]
    fn test_expand_mixed_placeholders() {
        assert_eq!(
            expand_placeholders("{date} {time} {clipboard}", fixed_now(), "abc"),
            "2026-08-26 14:30:05 abc"
        );
    }

    #[test]
    fn test_expand_unknown_placeholder_kept() {
        assert_eq!(
            expand_placeholders("未知 {foo} 占位", fixed_now(), ""),
            "未知 {foo} 占位"
        );
    }

    #[test]
    fn test_expand_input_placeholder_kept() {
        // {input} 由前端模板交互处理，后端不得展开
        assert_eq!(
            expand_placeholders("git commit -m \"{input}\"", fixed_now(), ""),
            "git commit -m \"{input}\""
        );
    }

    #[test]
    fn test_expand_no_placeholder_unchanged() {
        assert_eq!(
            expand_placeholders("纯文本内容", fixed_now(), "anything"),
            "纯文本内容"
        );
    }

    #[test]
    fn test_expand_empty_clipboard_becomes_empty() {
        assert_eq!(
            expand_placeholders("值: {clipboard}", fixed_now(), ""),
            "值: "
        );
    }

    #[test]
    fn test_expand_placeholder_appears_multiple_times() {
        assert_eq!(
            expand_placeholders("{date} 与 {date}", fixed_now(), ""),
            "2026-08-26 与 2026-08-26"
        );
    }

    #[test]
    fn test_expand_empty_text() {
        assert_eq!(expand_placeholders("", fixed_now(), ""), "");
    }
}
