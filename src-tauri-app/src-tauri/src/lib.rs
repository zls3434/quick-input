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
mod tray;
mod window;

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

/// Tauri 应用状态
struct AppState {
    config_manager: Mutex<ConfigManager>,
    /// 当前活动进程名（大小写不敏感的按钮集匹配依据）
    current_process: Mutex<String>,
    /// 焦点监听器
    watcher: Mutex<Option<FocusWatcher>>,
    /// 标记监听线程是否停止
    watcher_stopped: AtomicBool,
}

/// 获取当前配置的按钮列表（按当前活动进程匹配）
#[tauri::command]
fn get_buttons(state: tauri::State<AppState>) -> Result<Vec<quickinput_config::config::model::ButtonConfig>, String> {
    let mgr = state.config_manager.lock().map_err(|e| e.to_string())?;
    let process = state.current_process.lock().map_err(|e| e.to_string())?;
    // 按进程匹配，无匹配回退默认
    let buttons = mgr
        .config()
        .get_buttons_current(&process)
        .iter()
        .cloned()
        .collect();
    Ok(buttons)
}

/// 注入文本到当前焦点输入框
#[tauri::command]
fn inject_text(text: String) -> Result<(), String> {
    let injector = PlatformInjector::new();
    injector.inject_text(&text).map_err(|e| e.to_string())
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
/// 设置窗口在启动时默认加载根页面（浮层页），打开前必须先导航到
/// `/settings`，否则窗口会显示浮层按钮列表而非配置管理界面。
pub fn show_settings_window(app: &tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("settings") {
        let url = if cfg!(debug_assertions) {
            "http://localhost:1420/settings".to_string()
        } else {
            "http://tauri.localhost/settings".to_string()
        };
        if let Ok(url) = url.parse::<tauri::Url>() {
            let _ = window.navigate(url);
        }
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

/// 获取所有应用画像（供设置窗口编辑）
#[tauri::command]
fn get_profiles(state: tauri::State<AppState>) -> Result<Vec<AppProfile>, String> {
    let mgr = state.config_manager.lock().map_err(|e| e.to_string())?;
    Ok(mgr.config().profiles.clone())
}

/// 新增一个应用画像
#[tauri::command]
fn add_profile(
    app: tauri::AppHandle,
    state: tauri::State<AppState>,
    process_name: String,
    buttons: Vec<ButtonConfig>,
    name: Option<String>,
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
    probe.profiles.push(AppProfile {
        process_name,
        name,
        buttons,
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
    profile.buttons = buttons;
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

/// 导出配置到 TOML 文件（弹出保存对话框）
#[tauri::command]
fn export_config(app: tauri::AppHandle, state: tauri::State<AppState>) -> Result<(), String> {
    // 序列化当前配置
    let mgr = state.config_manager.lock().map_err(|e| e.to_string())?;
    let toml_str = toml::to_string(mgr.config()).map_err(|e| e.to_string())?;
    drop(mgr);

    // 弹出保存对话框（阻塞版本）
    let file_path = app
        .dialog()
        .file()
        .add_filter("TOML", &["toml"])
        .set_file_name("quickinput-config.toml")
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

/// 消费焦点监听事件，焦点切换时更新当前进程并发射 ConfigSwitched
fn run_focus_listener(app: tauri::AppHandle) {
    #[cfg(target_os = "windows")]
    {
        use focus_detector::current_foreground_process;

        // 从受管状态获取 AppState
        let state = app.state::<AppState>();

        // 从 watcher 取出接收端
        let receiver = {
            let mut watcher_guard = state.watcher.lock().unwrap();
            watcher_guard.as_mut().and_then(|w| w.take_receiver())
        };

        if let Some(receiver) = receiver {
            while !state.watcher_stopped.load(Ordering::SeqCst) {
                match receiver.recv_timeout(std::time::Duration::from_millis(500)) {
                    Ok(hwnd) => {
                        let _ = hwnd;
                        // 通过 S140 获取当前前台进程名
                        if let Ok(process) = current_foreground_process() {
                            let should_switch = {
                                let mut current = state.current_process.lock().unwrap();
                                if current.eq_ignore_ascii_case(&process) {
                                    false // AC7-3: 同一进程不重复发射
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
                    Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                        // 无事件，继续循环
                    }
                    Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => {
                        // 发送端已被丢弃，监听线程结束
                        break;
                    }
                }
            }
        }
    }
}

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
            inject_text,
            reload_config,
            is_autostart_enabled,
            toggle_autostart,
            open_settings,
            get_all_buttons,
            add_button,
            update_button,
            delete_button,
            get_profiles,
            add_profile,
            list_window_processes,
            update_profile,
            delete_profile,
            validate_config,
            export_config,
            import_config,
        ])
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
            };

            app.manage(app_state);

            // 启动焦点监听消费线程
            let listener_handle = app.handle().clone();
            std::thread::spawn(move || {
                run_focus_listener(listener_handle);
            });

            // 应用系统级置顶/不抢焦点样式
            apply_overlay_styles(app.handle())?;

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