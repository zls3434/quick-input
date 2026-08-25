//! 系统托盘模块 (S160)
//!
//! 创建系统托盘图标与右键菜单（"显示/隐藏浮层"、"退出"），
//! 使用 Tauri 2 内置 `TrayIconBuilder` + `Menu` 实现。
//! 退出前通过 `tauri-plugin-dialog` 弹出确认对话框。

use tauri::menu::{MenuBuilder, MenuItemBuilder};
use tauri::tray::TrayIconBuilder;
use tauri::{AppHandle, Manager};

/// 托盘菜单项 ID：显示/隐藏浮层
const MENU_TOGGLE: &str = "toggle";
/// 托盘菜单项 ID：配置管理
const MENU_SETTINGS: &str = "settings";
/// 托盘菜单项 ID：重置悬浮窗位置和大小
const MENU_RESET_GEOMETRY: &str = "reset_geometry";
/// 托盘菜单项 ID：退出
const MENU_QUIT: &str = "quit";

/// 创建并注册系统托盘
///
/// 托盘图标复用应用图标（`icons/icon.ico`），提供右键菜单：
/// - "显示/隐藏浮层"：切换 overlay 窗口可见性
/// - "重置悬浮窗位置和大小"：清除两布局的记忆几何，恢复默认位置尺寸
/// - "配置管理"：打开设置窗口
/// - "退出"：弹出确认对话框，确认后退出应用
pub fn setup_tray(app: &AppHandle) -> tauri::Result<()> {
    // 构建菜单
    let toggle_item = MenuItemBuilder::with_id(MENU_TOGGLE, "显示/隐藏浮层").build(app)?;
    let reset_item = MenuItemBuilder::with_id(MENU_RESET_GEOMETRY, "重置悬浮窗位置和大小").build(app)?;
    let settings_item = MenuItemBuilder::with_id(MENU_SETTINGS, "配置管理").build(app)?;
    let quit_item = MenuItemBuilder::with_id(MENU_QUIT, "退出").build(app)?;

    let menu = MenuBuilder::new(app)
        .item(&toggle_item)
        .item(&reset_item)
        .separator()
        .item(&settings_item)
        .separator()
        .item(&quit_item)
        .build()?;

    // 创建托盘图标
    let _tray = TrayIconBuilder::with_id("quickinput-tray")
        .icon(app.default_window_icon().cloned().unwrap_or_else(|| {
            // 无默认图标时使用 1x1 透明像素占位
            tauri::image::Image::new_owned(vec![0, 0, 0, 0], 1, 1)
        }))
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(move |app, event| match event.id().as_ref() {
            MENU_TOGGLE => toggle_overlay(app),
            MENU_RESET_GEOMETRY => {
                crate::window::reset_overlay_geometry(app);
            }
            MENU_SETTINGS => {
                // 与 open_settings 命令共用：先导航到 /settings 再显示，
                // 否则窗口会显示浮层页（无配置管理按钮）
                crate::show_settings_window(app);
            }
            MENU_QUIT => {
                // 弹出退出确认对话框
                let app_clone = app.clone();
                tauri::async_runtime::spawn(async move {
                    use tauri_plugin_dialog::DialogExt;
                    let confirmed = app_clone
                        .dialog()
                        .message("确定要退出 QuickInput 吗？")
                        .title("退出确认")
                        .kind(tauri_plugin_dialog::MessageDialogKind::Warning)
                        .buttons(tauri_plugin_dialog::MessageDialogButtons::OkCancelCustom(
                            "退出".into(),
                            "取消".into(),
                        ))
                        .show(move |confirmed| {
                            if confirmed {
                                app_clone.exit(0);
                            }
                        });
                    let _ = confirmed;
                });
            }
            _ => {}
        })
        .build(app)?;

    Ok(())
}

/// 切换浮层窗口可见性
///
/// 若窗口当前可见则隐藏，否则显示（不抢焦点，显示后重申样式）。
/// 隐藏前记录用户意图，避免自愈机制把主动隐藏误判为异常而抢显。
fn toggle_overlay(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("overlay") {
        if window.is_visible().unwrap_or(false) {
            crate::window::set_overlay_user_hidden(true);
            let _ = window.hide();
        } else {
            crate::window::show_overlay_with_styles(app);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // 菜单项 ID 常量不应为空
    #[test]
    fn test_menu_ids_are_defined() {
        assert!(!MENU_TOGGLE.is_empty());
        assert!(!MENU_RESET_GEOMETRY.is_empty());
        assert!(!MENU_SETTINGS.is_empty());
        assert!(!MENU_QUIT.is_empty());
    }
}