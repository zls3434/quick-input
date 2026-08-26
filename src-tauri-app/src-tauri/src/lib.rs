// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod focus_detector;
mod focus_guard;
mod focus_watcher;
mod global_shortcut;
mod inject;
#[cfg(target_os = "linux")]
mod inject_linux;
#[cfg(target_os = "macos")]
mod inject_macos;
mod inject_windows;
mod process_list;
mod target_window;
mod tray;
mod window;
mod floater;
mod placeholder;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

use focus_watcher::FocusWatcher;
use global_shortcut::setup_global_shortcut;
use inject::{Injector, PlatformInjector};
use quickinput_config::config::model::AppProfile;
use quickinput_config::config::model::ButtonConfig;
use quickinput_config::config::ConfigManager;
use tauri::{Emitter, Manager};
use tauri_plugin_autostart::MacosLauncher;
use tauri_plugin_autostart::ManagerExt as _;
use tauri_plugin_dialog::DialogExt;
use tray::setup_tray;
use window::apply_overlay_styles;
use floater::{
    floater_action, floater_debug, floater_pull_pending, floater_ready, hide_floater, show_floater,
};

/// Tauri 应用状态
struct AppState {
    config_manager: Mutex<ConfigManager>,
    /// 当前活动进程名（大小写不敏感的按钮集匹配依据）
    current_process: Mutex<String>,
    /// 焦点监听器
    watcher: Mutex<Option<FocusWatcher>>,
    /// 标记监听线程是否停止
    watcher_stopped: AtomicBool,
    /// 已收到的前台切换事件数（focus_debug 诊断用）
    focus_events: std::sync::atomic::AtomicU64,
}

/// 悬浮窗按钮分组视图：分组 Tab + 默认分组按钮
#[derive(serde::Serialize)]
struct GroupView {
    name: String,
    buttons: Vec<ButtonConfig>,
}

/// 悬浮窗按钮视图：groups 非空时前端渲染 Tab 栏；
/// default_buttons 为未分组按钮（「默认」标签内容）。
#[derive(serde::Serialize)]
struct ButtonsView {
    groups: Vec<GroupView>,
    default_buttons: Vec<ButtonConfig>,
}

/// 获取当前配置的按钮分组视图（按当前活动进程匹配）
#[tauri::command]
fn get_buttons(state: tauri::State<AppState>) -> Result<ButtonsView, String> {
    let mgr = state.config_manager.lock().map_err(|e| e.to_string())?;
    let process = state.current_process.lock().map_err(|e| e.to_string())?;
    let config = mgr.config();
    // 命中画像：悬浮窗 Tab 依据画像自定义分组；画像 buttons 为默认分组
    if let Some(profile) = config
        .profiles
        .iter()
        .find(|p| p.process_name.eq_ignore_ascii_case(&process))
    {
        return Ok(ButtonsView {
            groups: profile
                .groups
                .iter()
                .map(|g| GroupView {
                    name: g.name.clone(),
                    buttons: g.buttons.clone(),
                })
                .collect(),
            default_buttons: profile.buttons.clone(),
        });
    }
    // 未命中画像：回退默认映射/默认按钮，按按钮 group 字段聚合分组视图
    // （全局/默认按钮的分组同样驱动悬浮窗 Tab，语义与画像分组一致）
    let btns = if !config.default_buttons.is_empty() {
        &config.default_buttons
    } else {
        &config.buttons
    };
    let (groups, ungrouped) =
        quickinput_config::config::model::AppProfile::regroup(btns.clone());
    Ok(ButtonsView {
        groups: groups
            .into_iter()
            .map(|g| GroupView {
                name: g.name,
                buttons: g.buttons,
            })
            .collect(),
        default_buttons: ungrouped,
    })
}

/// 调试：返回画像切换内部状态（当前进程名与已收到的前台事件数）
#[tauri::command]
fn focus_debug(state: tauri::State<AppState>) -> Result<String, String> {
    let p = state.current_process.lock().map_err(|e| e.to_string())?;
    Ok(format!(
        "process=[{}] events={} hook={} ticks={} cbhits={}",
        p,
        state.focus_events.load(std::sync::atomic::Ordering::SeqCst),
        focus_watcher::HOOK_STATUS.load(std::sync::atomic::Ordering::SeqCst),
        focus_watcher::LOOP_TICKS.load(std::sync::atomic::Ordering::SeqCst),
        focus_watcher::CALLBACK_HITS.load(std::sync::atomic::Ordering::SeqCst),
    ))
}

/// 注入文本到当前焦点输入框（async：粘贴注入含等待，避免阻塞主线程）
///
/// cursor_back：注入完成后发送 N 个左方向键（模板按钮左键输出时占位符
/// 位置留空，光标需回退到占位符处，如 git commit -m "" 光标在引号中间）
///
/// mode：注入模式。"paste"（默认，剪贴板粘贴）或 "keystroke"（扫描码
/// 按键模拟，面向老游戏——DirectInput/自绘输入框不响应粘贴与 Unicode 注入）
///
/// 注入前展开动态占位符（{date} / {time} / {clipboard}），支持时间戳与
/// 剪贴板内容动态输入。{input} 不在此展开（前端模板交互专用）。
#[tauri::command]
async fn inject_text(
    text: String,
    cursor_back: Option<u32>,
    mode: Option<String>,
) -> Result<(), String> {
    // 展开动态占位符（剪贴板读取在注入线程内进行，避免阻塞主线程）
    let expanded = {
        let now = chrono::Local::now();
        let clipboard = crate::placeholder::read_clipboard_text();
        crate::placeholder::expand_placeholders(&text, now, &clipboard)
    };
    let injector = PlatformInjector::new();
    let mode = mode.unwrap_or_else(|| "paste".to_string());
    tauri::async_runtime::spawn_blocking(move || {
        injector.inject_text_mode(&expanded, cursor_back.unwrap_or(0), &mode)
    })
    .await
    .map_err(|e| e.to_string())?
    .map_err(|e| e.to_string())
}

/// 获取当前前台进程生效的注入模式（paste / keystroke）
///
/// 供悬浮窗前端在点击按钮时随按钮一起取用：命中画像用画像模式，
/// 否则用 default_inject_mode，缺省 paste。
#[tauri::command]
fn get_current_inject_mode(state: tauri::State<AppState>) -> Result<String, String> {
    let mgr = state.config_manager.lock().map_err(|e| e.to_string())?;
    let process = state.current_process.lock().map_err(|e| e.to_string())?;
    Ok(mgr.config().inject_mode_for_process(&process).to_string())
}

/// 向当前焦点输入框发送回车键（长按输入后回车交互）
///
/// mode 跟随前台应用的注入模式：keystroke（兼容模式）发真实扫描码，
/// paste/缺省发虚拟键（现代应用通用）。
#[tauri::command]
async fn inject_enter(mode: Option<String>) -> Result<(), String> {
    let injector = PlatformInjector::new();
    let mode = mode.unwrap_or_else(|| "paste".to_string());
    tauri::async_runtime::spawn_blocking(move || injector.inject_enter_mode(&mode))
        .await
        .map_err(|e| e.to_string())?
        .map_err(|e| e.to_string())
}

/// 恢复点击悬浮窗前的目标前台窗口（mouseup 后兜底，防焦点丢失）
#[tauri::command]
fn restore_focus() -> Result<(), String> {
    crate::inject_windows::restore_target_foreground();
    Ok(())
}

/// 切换悬浮窗"可输入"模式：模板输入弹窗打开时临时移除 NOACTIVATE 并激活
/// 窗口（获得键盘输入焦点），关闭时恢复 NOACTIVATE 并把前台还给原窗口。
#[tauri::command]
fn set_overlay_focusable(app: tauri::AppHandle, enabled: bool) -> Result<(), String> {
    window::set_overlay_focusable(&app, enabled)
}

/// 查询开机自启是否启用
#[tauri::command]
fn is_autostart_enabled(app: tauri::AppHandle) -> Result<bool, String> {
    app.autolaunch().is_enabled().map_err(|e| e.to_string())
}

/// 切换开机自启启用/禁用
#[tauri::command]
fn toggle_autostart(app: tauri::AppHandle, enable: bool) -> Result<(), String> {
    if enable {
        app.autolaunch().enable().map_err(|e| e.to_string())
    } else {
        app.autolaunch().disable().map_err(|e| e.to_string())
    }
}

/// 显示设置窗口（托盘菜单与前端命令共用）
///
/// 设置窗口在 tauri.conf.json 中已配置初始 URL 为 `/settings`，
/// 此处只需显示并聚焦。
pub fn show_settings_window(app: &tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("settings") {
        let _ = window.show();
        let _ = window.set_focus();
    }
}

/// 打开设置窗口
#[tauri::command]
fn open_settings(app: tauri::AppHandle) -> Result<(), String> {
    show_settings_window(&app);
    Ok(())
}

/// 获取所有默认按钮（供设置窗口编辑）
#[tauri::command]
fn get_all_buttons(state: tauri::State<AppState>) -> Result<Vec<ButtonConfig>, String> {
    let mgr = state.config_manager.lock().map_err(|e| e.to_string())?;
    Ok(mgr.config().buttons.clone())
}

/// 添加一个新按钮
#[tauri::command]
fn add_button(
    app: tauri::AppHandle,
    state: tauri::State<AppState>,
    id: String,
    label: String,
    content: String,
    comment: Option<String>,
    group: Option<String>,
) -> Result<(), String> {
    let mut mgr = state.config_manager.lock().map_err(|e| e.to_string())?;
    let config = mgr.config_mut();
    if config.buttons.iter().any(|b| b.id == id) {
        return Err(format!("按钮 ID '{id}' 已存在"));
    }
    // 先在副本上修改并校验，校验通过后才应用到实际配置（避免失败时污染内存配置）
    let mut probe = config.clone();
    probe.buttons.push(ButtonConfig {
        id,
        label,
        content,
        comment,
        group,
        ..Default::default()
    });
    probe.validate().map_err(|e| e.to_string())?;
    config.buttons = probe.buttons;
    mgr.save().map_err(|e| e.to_string())?;
    app.emit("ConfigSwitched", ()).map_err(|e| e.to_string())?;
    Ok(())
}

/// 更新一个已有按钮
#[tauri::command]
fn update_button(
    app: tauri::AppHandle,
    state: tauri::State<AppState>,
    id: String,
    label: String,
    content: String,
    comment: Option<String>,
    group: Option<String>,
) -> Result<(), String> {
    let mut mgr = state.config_manager.lock().map_err(|e| e.to_string())?;
    let config = mgr.config_mut();
    // 先在副本上修改并校验，校验通过后才应用到实际配置
    let mut probe = config.clone();
    let btn = probe
        .buttons
        .iter_mut()
        .find(|b| b.id == id)
        .ok_or_else(|| format!("按钮 ID '{id}' 不存在"))?;
    btn.label = label;
    btn.content = content;
    btn.comment = comment;
    btn.group = group;
    probe.validate().map_err(|e| e.to_string())?;
    config.buttons = probe.buttons;
    mgr.save().map_err(|e| e.to_string())?;
    app.emit("ConfigSwitched", ()).map_err(|e| e.to_string())?;
    Ok(())
}

/// 删除一个按钮
#[tauri::command]
fn delete_button(
    app: tauri::AppHandle,
    state: tauri::State<AppState>,
    id: String,
) -> Result<(), String> {
    let mut mgr = state.config_manager.lock().map_err(|e| e.to_string())?;
    let config = mgr.config_mut();
    // 先在副本上删除并校验，校验通过后才应用到实际配置
    let mut probe = config.clone();
    let len_before = probe.buttons.len();
    probe.buttons.retain(|b| b.id != id);
    if probe.buttons.len() == len_before {
        return Err(format!("按钮 ID '{id}' 不存在"));
    }
    probe.validate().map_err(|e| e.to_string())?;
    config.buttons = probe.buttons;
    mgr.save().map_err(|e| e.to_string())?;
    app.emit("ConfigSwitched", ()).map_err(|e| e.to_string())?;
    Ok(())
}

/// 获取所有应用画像（供设置窗口编辑；groups 已展平进 buttons 并携带 group 值）
#[tauri::command]
fn get_profiles(state: tauri::State<AppState>) -> Result<Vec<AppProfile>, String> {
    let mgr = state.config_manager.lock().map_err(|e| e.to_string())?;
    let mut profiles = mgr.config().profiles.clone();
    for p in &mut profiles {
        p.buttons = p.flattened_buttons();
        p.groups = vec![];
    }
    Ok(profiles)
}

/// 默认映射数据（按钮列表 + 注入模式），供设置窗口编辑回显
#[derive(serde::Serialize)]
struct DefaultProfilePayload {
    buttons: Vec<ButtonConfig>,
    /// 生效注入模式（None = 默认 paste，未显式配置）
    inject_mode: Option<String>,
}

/// 获取默认映射（未匹配任何应用画像时使用；空按钮表示回退默认按钮组）
#[tauri::command]
fn get_default_profile(state: tauri::State<AppState>) -> Result<DefaultProfilePayload, String> {
    let mgr = state.config_manager.lock().map_err(|e| e.to_string())?;
    Ok(DefaultProfilePayload {
        buttons: mgr.config().default_buttons.clone(),
        inject_mode: mgr.config().default_inject_mode.clone(),
    })
}

/// 更新默认映射（按钮列表 + 注入模式，保存顺序即悬浮窗按钮顺序）
#[tauri::command]
fn update_default_profile(
    app: tauri::AppHandle,
    state: tauri::State<AppState>,
    buttons: Vec<ButtonConfig>,
    inject_mode: Option<String>,
) -> Result<(), String> {
    let mut mgr = state.config_manager.lock().map_err(|e| e.to_string())?;
    let config = mgr.config_mut();
    // 先在副本上修改并校验，校验通过后才应用到实际配置
    let mut probe = config.clone();
    probe.default_buttons = buttons;
    probe.default_inject_mode = inject_mode;
    probe.validate().map_err(|e| e.to_string())?;
    config.default_buttons = probe.default_buttons;
    config.default_inject_mode = probe.default_inject_mode;
    mgr.save().map_err(|e| e.to_string())?;
    app.emit("ConfigSwitched", ()).map_err(|e| e.to_string())?;
    Ok(())
}

/// 新增一个应用画像
#[tauri::command]
fn add_profile(
    app: tauri::AppHandle,
    state: tauri::State<AppState>,
    process_name: String,
    buttons: Vec<ButtonConfig>,
    name: Option<String>,
    inject_mode: Option<String>,
) -> Result<(), String> {
    let mut mgr = state.config_manager.lock().map_err(|e| e.to_string())?;
    let config = mgr.config_mut();
    if config
        .profiles
        .iter()
        .any(|p| p.process_name.eq_ignore_ascii_case(&process_name))
    {
        return Err(format!("进程 '{process_name}' 已有映射"));
    }
    // 先在副本上修改并校验，校验通过后才应用到实际配置
    let mut probe = config.clone();
    // 存储前归一化：按按钮 group 值聚合为分组 + 未分组
    let (groups, buttons) = AppProfile::regroup(buttons);
    probe.profiles.push(AppProfile {
        process_name,
        name,
        buttons,
        inject_mode,
        groups,
    });
    probe.validate().map_err(|e| e.to_string())?;
    config.profiles = probe.profiles;
    mgr.save().map_err(|e| e.to_string())?;
    app.emit("ConfigSwitched", ()).map_err(|e| e.to_string())?;
    Ok(())
}

/// 更新一个应用画像
#[tauri::command]
fn update_profile(
    app: tauri::AppHandle,
    state: tauri::State<AppState>,
    process_name: String,
    buttons: Vec<ButtonConfig>,
    name: Option<String>,
    inject_mode: Option<String>,
) -> Result<(), String> {
    let mut mgr = state.config_manager.lock().map_err(|e| e.to_string())?;
    let config = mgr.config_mut();
    // 先在副本上修改并校验，校验通过后才应用到实际配置
    let mut probe = config.clone();
    let profile = probe
        .profiles
        .iter_mut()
        .find(|p| p.process_name.eq_ignore_ascii_case(&process_name))
        .ok_or_else(|| "进程映射不存在".to_string())?;
    profile.process_name = process_name;
    profile.name = name;
    // 存储前归一化：按按钮 group 值聚合为分组 + 未分组
    let (groups, buttons) = AppProfile::regroup(buttons);
    profile.buttons = buttons;
    profile.groups = groups;
    profile.inject_mode = inject_mode;
    probe.validate().map_err(|e| e.to_string())?;
    config.profiles = probe.profiles;
    mgr.save().map_err(|e| e.to_string())?;
    app.emit("ConfigSwitched", ()).map_err(|e| e.to_string())?;
    Ok(())
}

/// 枚举拥有可见窗口的运行进程（供配置管理绑定进程选择）
#[tauri::command]
fn list_window_processes() -> Result<Vec<process_list::RunningProcess>, String> {
    Ok(process_list::list_window_processes())
}

/// 获取悬浮窗设置（无配置时返回默认值：竖向布局）
#[tauri::command]
fn get_overlay_settings(
    state: tauri::State<AppState>,
) -> Result<quickinput_config::config::model::OverlaySettings, String> {
    let mgr = state.config_manager.lock().map_err(|e| e.to_string())?;
    Ok(mgr.config().overlay.clone().unwrap_or_default())
}

/// 切换悬浮窗布局（保存配置并立即应用窗口几何）
#[tauri::command]
fn set_overlay_layout(
    app: tauri::AppHandle,
    state: tauri::State<AppState>,
    layout: String,
) -> Result<(), String> {
    if layout != "vertical" && layout != "horizontal" {
        return Err(format!("布局 '{layout}' 无效（应为 vertical 或 horizontal）"));
    }
    {
        let mut mgr = state.config_manager.lock().map_err(|e| e.to_string())?;
        let config = mgr.config_mut();
        let overlay = config.overlay.get_or_insert_with(Default::default);
        overlay.layout = layout.clone();
        // 先在副本上校验，校验通过后才保存
        let probe = config.clone();
        probe.validate().map_err(|e| e.to_string())?;
        config.overlay = probe.overlay;
        mgr.save().map_err(|e| e.to_string())?;
    }
    // 应用新布局的尺寸与位置（优先该布局的记忆位置）
    window::apply_overlay_geometry(&app, &layout);
    app.emit("ConfigSwitched", ()).map_err(|e| e.to_string())?;
    Ok(())
}

/// 切换悬浮窗透明度（保存配置；视觉应用由前端 CSS 完成）
///
/// opacity 为百分比（20~100，100=不透明），存整数保证 TOML 可读。
#[tauri::command]
fn set_overlay_opacity(
    state: tauri::State<AppState>,
    opacity: u8,
) -> Result<u8, String> {
    let clamped = opacity.clamp(20, 100);
    let mut mgr = state.config_manager.lock().map_err(|e| e.to_string())?;
    let config = mgr.config_mut();
    let overlay = config.overlay.get_or_insert_with(Default::default);
    overlay.opacity = Some(clamped);
    let probe = config.clone();
    probe.validate().map_err(|e| e.to_string())?;
    config.overlay = probe.overlay;
    mgr.save().map_err(|e| e.to_string())?;
    Ok(clamped)
}

/// 设置按钮长按触发回车的时间阈值（毫秒，200~5000，保存并返回生效值）
///
/// 滑动条步进 100ms；数值输入框可传更精确值，统一夹取到合法范围。
#[tauri::command]
fn set_hold_threshold(state: tauri::State<AppState>, ms: u32) -> Result<u32, String> {
    use quickinput_config::config::model::OverlaySettings;
    let clamped = ms.clamp(
        OverlaySettings::HOLD_THRESHOLD_MIN_MS,
        OverlaySettings::HOLD_THRESHOLD_MAX_MS,
    );
    let mut mgr = state.config_manager.lock().map_err(|e| e.to_string())?;
    let config = mgr.config_mut();
    let overlay = config.overlay.get_or_insert_with(Default::default);
    overlay.hold_threshold_ms = Some(clamped);
    let probe = config.clone();
    probe.validate().map_err(|e| e.to_string())?;
    config.overlay = probe.overlay;
    mgr.save().map_err(|e| e.to_string())?;
    Ok(clamped)
}

/// 切换悬浮窗置顶（保存配置并立即应用到窗口 Z-order）
#[tauri::command]
fn set_overlay_always_on_top(
    app: tauri::AppHandle,
    state: tauri::State<AppState>,
    enabled: bool,
) -> Result<(), String> {
    {
        let mut mgr = state.config_manager.lock().map_err(|e| e.to_string())?;
        let config = mgr.config_mut();
        let overlay = config.overlay.get_or_insert_with(Default::default);
        overlay.always_on_top = Some(enabled);
        let probe = config.clone();
        probe.validate().map_err(|e| e.to_string())?;
        config.overlay = probe.overlay;
        mgr.save().map_err(|e| e.to_string())?;
    }
    // 立即生效（同步更新扩展样式与 Z-order）
    let _ = window::apply_overlay_styles(&app);
    Ok(())
}

/// 保存悬浮窗几何（位置 + 尺寸；拖动/缩放结束后由前端调用，按布局记忆）
///
/// 横向布局的高度作为下次启动的初始高度缓存，避免启动闪烁与位移。
///
/// `user_drag=true`（拖动结束）时执行吸附判定：
/// - 命中吸附边：记忆该应用本布局的吸附边，位置保存为吸附后位置并立即贴齐
/// - 未命中（目标存在）：清除吸附记忆（拖离边缘 = 解除吸附）
/// - 无可用目标（前台为本应用等）：不动吸附记忆
/// `user_drag=false`（缩放等）仅更新几何，吸附记忆保持，由跟随线程按新尺寸重贴。
#[tauri::command]
fn save_overlay_geometry(
    app: tauri::AppHandle,
    state: tauri::State<AppState>,
    layout: String,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    user_drag: Option<bool>,
) -> Result<(), String> {
    let mut mgr = state.config_manager.lock().map_err(|e| e.to_string())?;
    let config = mgr.config_mut();
    let overlay = config.overlay.get_or_insert_with(Default::default);

    let mut final_x = x;
    let mut final_y = y;
    let mut snapped_pos: Option<(i32, i32)> = None;
    if user_drag.unwrap_or(false) {
        match window::evaluate_drag_snap(&app) {
            window::DragSnapOutcome::Snapped { process, edge, offset, pos } => {
                overlay.set_snap_edge(&process, &layout, Some((edge, offset)));
                final_x = pos.0;
                final_y = pos.1;
                snapped_pos = Some(pos);
            }
            window::DragSnapOutcome::Missed { process } => {
                overlay.set_snap_edge(&process, &layout, None);
            }
            window::DragSnapOutcome::NoTarget => {}
        }
    }
    overlay.set_geometry(&layout, final_x, final_y, w, h);
    let probe = config.clone();
    probe.validate().map_err(|e| e.to_string())?;
    config.overlay = probe.overlay;
    mgr.save().map_err(|e| e.to_string())?;
    // 命中吸附：立即贴齐到吸附位置（不等跟随线程的下一轮）
    if let Some((sx, sy)) = snapped_pos {
        window::move_overlay(&app, sx, sy);
    }
    Ok(())
}

/// 隐藏悬浮窗（前端隐藏按钮入口）
///
/// 记录用户隐藏意图（自愈机制不抢显）后隐藏窗口；
/// 再次显示经托盘菜单/全局热键/本命令的显示分支。
#[tauri::command]
fn hide_overlay(app: tauri::AppHandle) -> Result<(), String> {
    window::set_overlay_user_hidden(true);
    if let Some(win) = app.get_webview_window(window::OVERLAY_WINDOW_LABEL) {
        win.hide().map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// 设置悬浮窗拖动会话标志（前端拖动开始时置位；结束由后端按左键状态检测）
///
/// 跟随线程在会话期间暂停重定位，避免与系统模态移动循环抢位。
#[tauri::command]
fn set_overlay_dragging(dragging: bool) {
    window::set_overlay_dragging(dragging);
}

/// 横排高度自适应：单次原子调整（位置 + 尺寸一次原生 SetWindowPos 生效）
///
/// 前端测量出目标客户区高度（物理像素）后调用。锚定方向（吸附边 →
/// 保顶/保底/居中）由后端统一解析，无吸附时按 fallback_keep_top 回退
/// （首次调整保顶边、之后保底边）。
#[tauri::command]
fn apply_overlay_height(
    app: tauri::AppHandle,
    target_inner_h: u32,
    fallback_keep_top: Option<bool>,
) -> Result<(), String> {
    window::apply_overlay_height_anchored(&app, target_inner_h, fallback_keep_top.unwrap_or(true))
}

/// 删除一个应用画像
#[tauri::command]
fn delete_profile(
    app: tauri::AppHandle,
    state: tauri::State<AppState>,
    process_name: String,
) -> Result<(), String> {
    let mut mgr = state.config_manager.lock().map_err(|e| e.to_string())?;
    let config = mgr.config_mut();
    // 先在副本上删除并校验，校验通过后才应用到实际配置
    let mut probe = config.clone();
    let len_before = probe.profiles.len();
    probe
        .profiles
        .retain(|p| !p.process_name.eq_ignore_ascii_case(&process_name));
    if probe.profiles.len() == len_before {
        return Err(format!("进程 '{process_name}' 映射不存在"));
    }
    probe.validate().map_err(|e| e.to_string())?;
    config.profiles = probe.profiles;
    mgr.save().map_err(|e| e.to_string())?;
    app.emit("ConfigSwitched", ()).map_err(|e| e.to_string())?;
    Ok(())
}

/// 校验当前配置（供前端在保存前调用）
#[tauri::command]
fn validate_config(state: tauri::State<AppState>) -> Result<(), String> {
    let mgr = state.config_manager.lock().map_err(|e| e.to_string())?;
    mgr.config().validate().map_err(|e| e.to_string())?;
    Ok(())
}

/// 导出配置到 TOML 文件（弹出保存对话框，文件名带时间后缀便于区分版本）
#[tauri::command]
fn export_config(app: tauri::AppHandle, state: tauri::State<AppState>) -> Result<(), String> {
    // 序列化当前配置
    let mgr = state.config_manager.lock().map_err(|e| e.to_string())?;
    let toml_str = toml::to_string(mgr.config()).map_err(|e| e.to_string())?;
    drop(mgr);

    // 文件名带本地时间后缀（秒级精度），导出多次互不覆盖，便于区分版本
    let stamp = chrono::Local::now().format("%Y%m%d-%H%M%S");
    let file_name = format!("quickinput-config-{stamp}.toml");

    // 弹出保存对话框（阻塞版本）
    let file_path = app
        .dialog()
        .file()
        .add_filter("TOML", &["toml"])
        .set_file_name(&file_name)
        .blocking_save_file();

    let file_path = match file_path {
        Some(p) => p,
        None => return Ok(()), // 用户取消
    };

    // 写入文件
    std::fs::write(file_path.as_path().unwrap(), &toml_str).map_err(|e| e.to_string())?;
    Ok(())
}

/// 从 TOML 文件导入配置（弹出打开对话框）
#[tauri::command]
fn import_config(app: tauri::AppHandle, state: tauri::State<AppState>) -> Result<(), String> {
    // 弹出打开对话框（阻塞版本）
    let file_path = app
        .dialog()
        .file()
        .add_filter("TOML", &["toml"])
        .blocking_pick_file();

    let file_path = match file_path {
        Some(p) => p,
        None => return Ok(()), // 用户取消
    };

    // 读取文件内容
    let content = std::fs::read_to_string(file_path.as_path().unwrap()).map_err(|e| e.to_string())?;

    // 解析 TOML
    let config: quickinput_config::config::model::ConfigFile =
        toml::from_str(&content).map_err(|e| format!("TOML 解析失败: {e}"))?;

    // 校验配置
    config.validate().map_err(|e| e.to_string())?;

    // 写入配置目录
    let mut mgr = state.config_manager.lock().map_err(|e| e.to_string())?;
    *mgr.config_mut() = config;
    mgr.save().map_err(|e| e.to_string())?;

    app.emit("ConfigSwitched", ()).map_err(|e| e.to_string())?;
    Ok(())
}

/// 重新加载配置，发射 ConfigSwitched 事件通知前端刷新
#[tauri::command]
fn reload_config(app: tauri::AppHandle, state: tauri::State<AppState>) -> Result<(), String> {
    let mut mgr = state.config_manager.lock().map_err(|e| e.to_string())?;
    mgr.load().map_err(|e| e.to_string())?;
    app.emit("ConfigSwitched", ()).map_err(|e| e.to_string())?;
    Ok(())
}

/// 读取快捷键配置（缺省返回默认热键）
#[tauri::command]
fn get_shortcuts(state: tauri::State<AppState>) -> Result<quickinput_config::config::model::ShortcutSettings, String> {
    let mgr = state.config_manager.lock().map_err(|e| e.to_string())?;
    Ok(mgr.config().shortcuts.clone().unwrap_or_default())
}

/// 设置快捷键：校验格式 → 检测冲突 → 保存配置 → 更新全局热键注册
///
/// 冲突检测：新键与当前已注册热键（包括本应用正在使用的旧键）相同时视为可用；
/// 与其他软件冲突时返回错误，配置不保存。
#[tauri::command]
fn set_shortcut(
    app: tauri::AppHandle,
    state: tauri::State<AppState>,
    name: String,
    value: String,
) -> Result<String, String> {
    if name != "show_overlay" {
        return Err(format!("不支持的快捷键项 [{name}]"));
    }
    let value = value.trim().to_uppercase();

    // 1. 格式校验 + 冲突检测（探测前先校验格式，避免无效键名注册报错）
    let parsed: tauri_plugin_global_shortcut::Shortcut = value
        .parse()
        .map_err(|e| format!("快捷键格式无效 [{value}]: {e}"))?;
    let _ = parsed;

    // 2. 冲突检测：若新键等于当前生效键则跳过（用户未改）；否则探测占用
    let current = {
        let mgr = state.config_manager.lock().map_err(|e| e.to_string())?;
        mgr.config()
            .shortcuts
            .clone()
            .unwrap_or_default()
            .effective_show_overlay()
    };
    if value != current {
        let available = global_shortcut::is_shortcut_available(&value)
            .map_err(|e| format!("冲突检测失败: {e}"))?;
        if !available {
            return Err(format!(
                "快捷键 [{value}] 已被其他软件占用，请更换组合键"
            ));
        }
    }

    // 3. 保存配置
    {
        let mut mgr = state.config_manager.lock().map_err(|e| e.to_string())?;
        let config = mgr.config_mut();
        let shortcuts = config.shortcuts.get_or_insert_with(Default::default);
        shortcuts.show_overlay = Some(value.clone());
        let probe = config.clone();
        probe.validate().map_err(|e| e.to_string())?;
        config.shortcuts = probe.shortcuts;
        mgr.save().map_err(|e| e.to_string())?;
    }

    // 4. 更新全局热键注册（注册失败回滚配置，避免配置与运行不一致）
    if let Err(e) = global_shortcut::update_global_shortcut(&app, &value) {
        // 回滚配置
        if let Ok(mut mgr) = state.config_manager.lock() {
            let _ = mgr.load();
        }
        return Err(format!("热键注册失败: {e}"));
    }

    Ok(value)
}

/// 检测快捷键是否可用（是否与其他软件冲突）
///
/// 返回 true = 未被占用可安全使用；false = 已被其他软件注册。
#[tauri::command]
fn check_shortcut_available(shortcut: String) -> Result<bool, String> {
    // 先做格式校验
    let _: tauri_plugin_global_shortcut::Shortcut = shortcut
        .parse()
        .map_err(|e| format!("快捷键格式无效 [{shortcut}]: {e}"))?;
    global_shortcut::is_shortcut_available(&shortcut)
}

/// 重置悬浮窗位置和大小（托盘菜单入口；此处注册命令便于调用/测试）
#[tauri::command]
fn reset_overlay_geometry_command(app: tauri::AppHandle) -> Result<(), String> {
    window::reset_overlay_geometry(&app);
    Ok(())
}

/// 恢复默认按钮与进程映射（保留悬浮窗/快捷键设置）
///
/// 用于程序版本更新后默认按钮变化、或用户希望回到出厂按钮配置的场景。
/// 仅重置 `buttons` 与 `profiles`，`[overlay]`/`[shortcuts]` 个性化设置保留。
#[tauri::command]
fn reset_config_to_default(
    app: tauri::AppHandle,
    state: tauri::State<AppState>,
) -> Result<(), String> {
    {
        let mut mgr = state.config_manager.lock().map_err(|e| e.to_string())?;
        let default = quickinput_config::config::defaults::default_config();
        let config = mgr.config_mut();
        config.buttons = default.buttons;
        config.profiles = default.profiles;
        mgr.save().map_err(|e| e.to_string())?;
    }
    app.emit("ConfigSwitched", ()).map_err(|e| e.to_string())?;
    Ok(())
}

/// 消费焦点监听事件，焦点切换时更新当前进程并发射 ConfigSwitched
fn run_focus_listener(app: tauri::AppHandle) {
    #[cfg(target_os = "windows")]
    {
        use focus_detector::current_foreground_process;

        let state = app.state::<AppState>();

        // 说明：原实现经 mpsc 消费 FocusWatcher（SetWinEventHook
        // EVENT_SYSTEM_FOREGROUND）事件。实测在 GUI 子系统（windows_subsystem
        // = "windows"）的 Tauri 进程内，钩子注册成功（HOOK_STATUS=2）但回调
        // 永不派发（独立测试二进制则正常），事件驱动不可用。
        // 改为 500ms 轮询前台进程名：稳定可靠，OpenProcess +
        // QueryFullProcessImageNameW 开销可忽略（<0.1ms/次）。
        // 同一进程不重复发射（AC7-3）；首次检测立即执行（覆盖启动初始态）。
        loop {
            if state.watcher_stopped.load(Ordering::SeqCst) {
                break;
            }
            if let Ok(process) = current_foreground_process() {
                state
                    .focus_events
                    .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                // 悬浮窗自身激活（模板输入弹窗期间）不触发画像切换：
                // 保持当前按钮组不变，避免弹窗期间按钮闪动
                let is_self = process.eq_ignore_ascii_case("quickinput.exe");
                if !is_self {
                    let should_switch = {
                        let mut current = state.current_process.lock().unwrap();
                        if current.eq_ignore_ascii_case(&process) {
                            false // 同一进程不重复发射
                        } else {
                            *current = process;
                            true
                        }
                    };
                    if should_switch {
                        let _ = app.emit("ConfigSwitched", ());
                    }
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(500));
        }
    }
}

/// 浮层待显示标志：on_page_load 早于 setup 触发时置位，setup 应用几何后补显示
static OVERLAY_SHOW_PENDING: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(
            tauri_plugin_autostart::init(MacosLauncher::LaunchAgent, Some(vec![])),
        )
        .invoke_handler(tauri::generate_handler![
            get_buttons,
            get_current_inject_mode,
            focus_debug,
            inject_text,
            inject_enter,
            restore_focus,
            set_overlay_focusable,
            reload_config,
            is_autostart_enabled,
            toggle_autostart,
            open_settings,
            get_all_buttons,
            add_button,
            update_button,
            delete_button,
            get_profiles,
            get_default_profile,
            update_default_profile,
            add_profile,
            list_window_processes,
            get_overlay_settings,
            set_overlay_layout,
            set_overlay_opacity,
            set_hold_threshold,
            set_overlay_always_on_top,
            save_overlay_geometry,
            set_overlay_dragging,
            apply_overlay_height,
            hide_overlay,
            reset_overlay_geometry_command,
            update_profile,
            delete_profile,
            validate_config,
            export_config,
            import_config,
            get_shortcuts,
            set_shortcut,
            check_shortcut_available,
            reset_config_to_default,
            show_floater,
            hide_floater,
            floater_ready,
            floater_action,
            floater_pull_pending,
            floater_debug,
        ])
        // 浮层页面加载完成后：先应用样式与几何，再显示窗口。
        // 窗口初始隐藏（tauri.conf.json visible:false）以避免 WebView2 内容未就绪时
        // 的白屏闪烁；隐藏窗口中 WebView2 会挂起页面定时器，前端 setTimeout(show)
        // 不可靠，故由 Rust 侧在 PageLoadEvent::Finished 时显示。
        // 注意：
        // 1. 必须对 WebviewWindow（宿主窗口）调用 show——Webview::show() 仅显示
        //    WebView 控件，不影响窗口可见性。
        // 2. on_page_load 可能在 setup 钩子之前触发（窗口创建期间 WebView2 消息泵
        //    会同步派发加载完成事件）。此时配置尚未加载、AppState 未管理：
        //    - 不能访问 state（会 panic），用 try_state 判断
        //    - 不能立即显示（几何未知会默认位置闪现），标记待显示，
        //      由 setup 加载配置并应用几何后再显示
        .on_page_load(|webview, payload| {
            use tauri::webview::PageLoadEvent;
            if payload.event() == PageLoadEvent::Finished && webview.label() == "overlay" {
                use tauri::Manager;
                let app = webview.app_handle();
                // 页面就绪后允许自愈：此后窗口若停留隐藏态（显示握手竞态，
                // 如进程重启轮换时序异常），跟随线程将原生恢复显示
                window::set_overlay_page_ready();
                let _ = window::apply_overlay_styles(app);
                let state_ready = app.try_state::<crate::AppState>().is_some();
                if state_ready {
                    let layout = window::current_layout(app);
                    window::apply_overlay_geometry(app, &layout);
                    // 显示后立即重申样式（show 可能重置顶层 NOACTIVATE）
                    window::show_overlay_with_styles(app);
                } else {
                    // setup 未执行：推迟显示（setup 末尾检查此标志）
                    OVERLAY_SHOW_PENDING.store(true, std::sync::atomic::Ordering::SeqCst);
                }
            }
            if payload.event() == PageLoadEvent::Finished && webview.label() == "floater" {
                floater::set_floater_page_ready(webview);
            }
        })
        .on_window_event(|window, event| {
            use tauri::WindowEvent;
            match window.label() {
                "overlay" => {
                    // 悬浮窗移动/缩放时隐藏浮层，避免浮层悬空错位
                    if matches!(event, WindowEvent::Moved(_) | WindowEvent::Resized(_)) {
                        floater::hide_floater_quiet(window.app_handle());
                    }
                }
                "floater" => {
                    // 菜单浮层失去焦点（点击其他应用/桌面）→ 关闭
                    if matches!(event, WindowEvent::Focused(false)) {
                        floater::hide_floater_quiet(window.app_handle());
                    }
                }
                _ => {}
            }
        })
        .setup(|app| {
            // 初始化配置管理器
            let config_dir = app.path().app_config_dir().unwrap_or_else(|_| {
                // 回退到用户 home 目录下的 .quickinput
                let home = std::env::var("USERPROFILE")
                    .or_else(|_| std::env::var("HOME"))
                    .unwrap_or_else(|_| ".".to_string());
                std::path::PathBuf::from(home).join(".quickinput")
            });

            let mut config_manager = ConfigManager::new(&config_dir);
            if let Err(e) = config_manager.load_or_init() {
                eprintln!("配置加载失败: {e}，使用空配置");
            }

            // 启动焦点监听器
            let watcher = FocusWatcher::start();

            let app_state = AppState {
                config_manager: Mutex::new(config_manager),
                // 初始为默认（空进程名 → 回退默认按钮）
                current_process: Mutex::new(String::new()),
                watcher: Mutex::new(Some(watcher)),
                watcher_stopped: AtomicBool::new(false),
                focus_events: std::sync::atomic::AtomicU64::new(0),
            };

            app.manage(app_state);

            // 启动焦点监听消费线程
            let listener_handle = app.handle().clone();
            std::thread::spawn(move || {
                run_focus_listener(listener_handle);
            });

            // 启动吸附跟随线程（150ms 轮询，按吸附记忆跟随目标窗口重定位）
            let follow_handle = app.handle().clone();
            std::thread::spawn(move || {
                window::run_snap_follow(follow_handle);
            });

            // 应用系统级置顶/不抢焦点样式
            apply_overlay_styles(app.handle())?;

            // 按配置应用悬浮窗布局几何（尺寸 + 记忆位置或默认位置）。
            // 此时窗口仍隐藏（visible:false），调整几何不可见、无闪烁。
            {
                let layout = window::current_layout(app.handle());
                window::apply_overlay_geometry(app.handle(), &layout);
            }

            // 页面加载完成早于 setup 时（WebView2 消息泵同步派发），此时已加载
            // 配置并应用几何，补上被推迟的窗口显示（含样式重申）
            if OVERLAY_SHOW_PENDING.load(std::sync::atomic::Ordering::SeqCst) {
                window::show_overlay_with_styles(app.handle());
            }

            // 设置系统托盘图标与菜单
            if let Err(e) = setup_tray(app.handle()) {
                eprintln!("托盘创建失败: {e}");
            }

            // 注册全局热键
            if let Err(e) = setup_global_shortcut(app.handle()) {
                eprintln!("全局热键注册失败: {e}");
            }

            // 启用开机自启
            if let Err(e) = app.autolaunch().enable() {
                eprintln!("开机自启注册失败: {e}");
            }
            match app.autolaunch().is_enabled() {
                Ok(true) => eprintln!("开机自启状态: 已启用"),
                Ok(false) => eprintln!("开机自启状态: 未启用"),
                Err(e) => eprintln!("开机自启查询失败: {e}"),
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}