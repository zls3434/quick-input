//! 全局热键模块 (S161)
//!
//! 注册默认全局热键 `Ctrl+Shift+Space`，复用 S160 的浮层切换逻辑。

use tauri::{AppHandle, Manager};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};

/// 默认全局热键
pub const DEFAULT_SHORTCUT: &str = "CTRL+SHIFT+SPACE";

/// 注册全局热键
///
/// 热键按下时切换 overlay 窗口显示状态；注册失败返回明确错误，
/// 不影响应用其他功能。
pub fn setup_global_shortcut(app: &AppHandle) -> tauri::Result<()> {
    let shortcut: Shortcut = DEFAULT_SHORTCUT
        .parse()
        .map_err(|e| tauri::Error::Anyhow(anyhow::anyhow!("热键解析失败: {e}")))?;

    app.global_shortcut()
        .on_shortcut(shortcut, move |app, _shortcut, event| {
            if event.state == ShortcutState::Pressed {
                toggle_overlay(app);
            }
        })
        .map_err(|e| tauri::Error::Anyhow(anyhow::anyhow!("全局热键注册失败: {e}")))?;

    Ok(())
}

/// 注销全局热键
///
/// 当前热键由 `tauri-plugin-global-shortcut` 托管，应用退出时自动注销，
/// 因此此函数暂未在运行时调用。保留作为手动注销的公共 API。
#[allow(dead_code)]
pub fn teardown_global_shortcut(app: &AppHandle) -> tauri::Result<()> {
    app.global_shortcut()
        .unregister(DEFAULT_SHORTCUT)
        .map_err(|e| tauri::Error::Anyhow(anyhow::anyhow!("全局热键注销失败: {e}")))?;
    Ok(())
}

/// 切换浮层窗口可见性
fn toggle_overlay(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("overlay") {
        if window.is_visible().unwrap_or(false) {
            let _ = window.hide();
        } else {
            let _ = window.show();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_shortcut_is_defined() {
        assert_eq!(DEFAULT_SHORTCUT, "CTRL+SHIFT+SPACE");
    }

    #[test]
    fn test_default_shortcut_parses() {
        let shortcut: Result<Shortcut, _> = DEFAULT_SHORTCUT.parse();
        assert!(shortcut.is_ok());
    }
}