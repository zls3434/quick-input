//! 全局热键模块 (S161)
//!
//! 注册"显示/隐藏悬浮窗"全局热键（默认 `Ctrl+Shift+Space`），
//! 快捷键可在配置管理的快捷键配置中修改；修改时校验格式并检测
//! 是否与其他软件冲突（Windows 用 RegisterHotKey 探测）。

use tauri::{AppHandle, Manager};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};

/// 注册全局热键（从配置读取快捷键，缺省使用默认值）
///
/// 热键按下时切换 overlay 窗口显示状态；注册失败返回明确错误，
/// 不影响应用其他功能。
pub fn setup_global_shortcut(app: &AppHandle) -> tauri::Result<()> {
    let shortcut_str = current_shortcut(app);
    let shortcut: Shortcut = shortcut_str
        .parse()
        .map_err(|e| tauri::Error::Anyhow(anyhow::anyhow!("热键解析失败 [{shortcut_str}]: {e}")))?;

    app.global_shortcut()
        .on_shortcut(shortcut, move |app, _shortcut, event| {
            if event.state == ShortcutState::Pressed {
                toggle_overlay(app);
            }
        })
        .map_err(|e| {
            tauri::Error::Anyhow(anyhow::anyhow!(
                "全局热键注册失败 [{shortcut_str}]（可能与其他软件冲突）: {e}"
            ))
        })?;

    Ok(())
}

/// 更新"显示/隐藏悬浮窗"全局热键：注销旧键并注册新键
///
/// 供配置管理"修改快捷键"使用。调用方需先通过 `is_shortcut_available`
/// 确认新键未被占用；此处失败（如被抢占）返回错误且不保存配置。
pub fn update_global_shortcut(app: &AppHandle, new_shortcut: &str) -> tauri::Result<()> {
    let parsed: Shortcut = new_shortcut
        .parse()
        .map_err(|e| tauri::Error::Anyhow(anyhow::anyhow!("热键格式无效 [{new_shortcut}]: {e}")))?;

    let plugin = app.global_shortcut();

    // 注销旧的（若与当前注册不一致）
    let old = current_shortcut(app);
    if old != new_shortcut {
        if let Ok(old_parsed) = old.parse::<Shortcut>() {
            let _ = plugin.unregister(old_parsed);
        }
    }

    // 注册新热键（失败即冲突/无效）
    plugin
        .on_shortcut(parsed, move |app, _shortcut, event| {
            if event.state == ShortcutState::Pressed {
                toggle_overlay(app);
            }
        })
        .map_err(|e| {
            tauri::Error::Anyhow(anyhow::anyhow!(
                "热键注册失败 [{new_shortcut}]（可能与其他软件冲突）: {e}"
            ))
        })?;

    Ok(())
}

/// 读取配置中的"显示/隐藏悬浮窗"热键（缺省默认）
fn current_shortcut(app: &AppHandle) -> String {
    app.try_state::<crate::AppState>()
        .and_then(|state| {
            state
                .config_manager
                .lock()
                .ok()
                .and_then(|mgr| mgr.config().shortcuts.clone())
        })
        .map(|s| s.effective_show_overlay())
        .unwrap_or_else(|| {
            quickinput_config::config::model::ShortcutSettings::DEFAULT_SHOW_OVERLAY.to_string()
        })
}

/// 探测快捷键是否可用（未被其他软件注册）
///
/// Windows 下通过 RegisterHotKey 探测：成功注册即未被占用，随即注销；
/// 失败（ERROR_HOTKEY_ALREADY_REGISTERED）即被其他软件占用。
#[cfg(target_os = "windows")]
pub fn is_shortcut_available(shortcut_str: &str) -> Result<bool, String> {
    use windows::Win32::Foundation::{GetLastError, HWND};
    use windows::Win32::UI::Input::KeyboardAndMouse::{
        RegisterHotKey, UnregisterHotKey, HOT_KEY_MODIFIERS, MOD_ALT, MOD_CONTROL, MOD_SHIFT,
        MOD_WIN,
    };

    const HOTKEY_ID: i32 = 0x5100;

    // 解析 "CTRL+SHIFT+SPACE" -> 修饰键 + 虚拟键
    let parts: Vec<String> = shortcut_str
        .split('+')
        .map(|s| s.trim().to_uppercase())
        .collect();
    if parts.is_empty() {
        return Err("快捷键不能为空".into());
    }
    let mut mods: u32 = 0;
    let mut vk_name: Option<String> = None;
    for p in &parts {
        match p.as_str() {
            "CTRL" | "CONTROL" => mods |= MOD_CONTROL.0,
            "SHIFT" => mods |= MOD_SHIFT.0,
            "ALT" => mods |= MOD_ALT.0,
            "WIN" | "META" | "CMD" => mods |= MOD_WIN.0,
            other => {
                if vk_name.is_some() {
                    return Err(format!(
                        "快捷键格式无效 [{shortcut_str}]：存在多个按键名 [{other}]"
                    ));
                }
                vk_name = Some(other.to_string());
            }
        }
    }
    let Some(name) = vk_name else {
        return Err(format!("快捷键格式无效 [{shortcut_str}]：缺少按键"));
    };
    let vk = map_vk(&name)?;

    unsafe {
        // 探测：成功注册即未占用；立即注销释放
        if RegisterHotKey(HWND(std::ptr::null_mut()), HOTKEY_ID, HOT_KEY_MODIFIERS(mods), vk as u32)
            .is_ok()
        {
            let _ = UnregisterHotKey(HWND(std::ptr::null_mut()), HOTKEY_ID);
            Ok(true)
        } else {
            let err = GetLastError();
            if err.0 == 1409 {
                // ERROR_HOTKEY_ALREADY_REGISTERED
                Ok(false)
            } else {
                Err(format!("热键探测失败: Win32 error {}", err.0))
            }
        }
    }
}

/// 未冲突时的默认结果（非 Windows 平台不检测，视为可用）
#[cfg(not(target_os = "windows"))]
pub fn is_shortcut_available(_shortcut_str: &str) -> Result<bool, String> {
    Ok(true)
}

#[cfg(target_os = "windows")]
fn map_vk(name: &str) -> Result<u16, String> {
    use windows::Win32::UI::Input::KeyboardAndMouse::*;
    let vk = match name {
        "SPACE" => VK_SPACE.0 as u16,
        "TAB" => VK_TAB.0 as u16,
        "ESC" | "ESCAPE" => VK_ESCAPE.0 as u16,
        "ENTER" | "RETURN" => VK_RETURN.0 as u16,
        "BACKSPACE" => VK_BACK.0 as u16,
        "DELETE" | "DEL" => VK_DELETE.0 as u16,
        "INSERT" | "INS" => VK_INSERT.0 as u16,
        "HOME" => VK_HOME.0 as u16,
        "END" => VK_END.0 as u16,
        "PAGEUP" => VK_PRIOR.0 as u16,
        "PAGEDOWN" => VK_NEXT.0 as u16,
        "LEFT" => VK_LEFT.0 as u16,
        "UP" => VK_UP.0 as u16,
        "RIGHT" => VK_RIGHT.0 as u16,
        "DOWN" => VK_DOWN.0 as u16,
        "F1" => VK_F1.0 as u16,
        "F2" => VK_F2.0 as u16,
        "F3" => VK_F3.0 as u16,
        "F4" => VK_F4.0 as u16,
        "F5" => VK_F5.0 as u16,
        "F6" => VK_F6.0 as u16,
        "F7" => VK_F7.0 as u16,
        "F8" => VK_F8.0 as u16,
        "F9" => VK_F9.0 as u16,
        "F10" => VK_F10.0 as u16,
        "F11" => VK_F11.0 as u16,
        "F12" => VK_F12.0 as u16,
        _ if name.len() == 1 && name.chars().next().unwrap().is_ascii_digit() => {
            ('0' as u16) + (name.parse::<u16>().unwrap_or(0))
        }
        _ if name.len() == 1 && name.chars().next().unwrap().is_ascii_uppercase() => {
            name.chars().next().unwrap() as u16
        }
        _ => return Err(format!("不支持的按键 [{name}]")),
    };
    Ok(vk)
}

/// 切换浮层窗口可见性
fn toggle_overlay(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("overlay") {
        if window.is_visible().unwrap_or(false) {
            // 记录用户隐藏意图，避免自愈机制误判抢显
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

    #[test]
    fn test_default_shortcut_is_defined() {
        assert_eq!(
            quickinput_config::config::model::ShortcutSettings::DEFAULT_SHOW_OVERLAY,
            "CTRL+SHIFT+SPACE"
        );
    }

    #[test]
    fn test_default_shortcut_parses() {
        let shortcut: Result<Shortcut, _> =
            quickinput_config::config::model::ShortcutSettings::DEFAULT_SHOW_OVERLAY.parse();
        assert!(shortcut.is_ok());
    }
}
