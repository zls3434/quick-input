//! 配置管理器（S101）单元测试
//!
//! 覆盖配置加载、保存、首次启动默认生成、错误处理等验收标准。
//! 测试使用临时目录，隔离于真实用户配置。

use std::fs;

use super::model::{ButtonConfig, ConfigFile};
use super::manager::ConfigManager;

/// 创建临时配置目录的辅助函数，测试结束自动清理
fn temp_dir(tag: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("quickinput_test_{tag}_{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("应能创建临时目录");
    dir
}

/// 快速构造 ButtonConfig 的测试夹具
fn button(id: &str, content: &str) -> ButtonConfig {
    ButtonConfig {
        id: id.to_string(),
        label: id.to_string(),
        content: content.to_string(),
        comment: None,
    }
}

// ============================================================
// AC2-4：首次启动生成默认配置
// ============================================================

#[test]
fn test_ac2_4_ensure_default_creates_file_when_missing() {
    let dir = temp_dir("ensure_default");
    let mut mgr = ConfigManager::new(&dir);

    // 初始时文件不存在
    assert!(!mgr.config_path().exists(), "测试开始前文件不应存在");

    // 首次生成
    let path = mgr.ensure_default().expect("ensure_default 应成功");
    assert!(path.exists(), "默认配置文件应被创建");
    assert_eq!(path, mgr.config_path());
    assert!(mgr.config_path().exists(), "config_path 应存在");

    // 生成的默认配置应包含样例按钮
    mgr.load().expect("加载生成的默认配置应成功");
    assert!(!mgr.config().buttons.is_empty(), "默认配置应有按钮");

    fs::remove_dir_all(&dir).ok();
}

// ============================================================
// AC2-1：从配置目录加载 default.toml
// ============================================================

#[test]
fn test_ac2_1_load_config_from_directory() {
    let dir = temp_dir("load");
    let mut mgr = ConfigManager::new(&dir);

    // 构造一个配置文件写入
    let mut config = ConfigFile::default();
    config.buttons.push(button("hello", "Hello World"));
    let toml_str = toml::to_string(&config).unwrap();
    fs::write(mgr.config_path(), toml_str).unwrap();

    // 加载
    mgr.load().expect("加载应成功");
    assert_eq!(mgr.config().buttons.len(), 1);
    assert_eq!(mgr.config().buttons[0].content, "Hello World");

    fs::remove_dir_all(&dir).ok();
}

#[test]
fn test_ac2_1_load_or_init_works_when_missing() {
    let dir = temp_dir("load_or_init");
    let mut mgr = ConfigManager::new(&dir);

    // 文件不存在时 load_or_init 应自动创建默认并加载
    mgr.load_or_init().expect("load_or_init 应成功");
    assert!(mgr.config_path().exists(), "应创建默认配置");
    assert!(!mgr.config().buttons.is_empty(), "应加载到默认按钮");

    fs::remove_dir_all(&dir).ok();
}

// ============================================================
// AC2-2：修改配置后保存，重新加载反映最新配置
// ============================================================

#[test]
fn test_ac2_2_modify_save_then_reload() {
    let dir = temp_dir("modify");
    let mut mgr = ConfigManager::new(&dir);
    mgr.ensure_default().unwrap();

    // 修改配置：追加一个按钮
    mgr.config_mut().buttons.push(button("custom", "zk status"));
    let custom_id = mgr.config().buttons.len(); // 记录数量
    mgr.save().expect("保存应成功");

    // 重新初始化一个 manager 并加载，验证最新配置
    let mut mgr2 = ConfigManager::new(&dir);
    mgr2.load().expect("重新加载应成功");
    assert_eq!(mgr2.config().buttons.len(), custom_id, "修改后的配置应被持久化");

    fs::remove_dir_all(&dir).ok();
}

// ============================================================
// AC2-3：保存不破坏已有其他字段（最小化写入）
// ============================================================

#[test]
fn test_ac2_3_save_preserves_existing_fields() {
    let dir = temp_dir("preserve");
    let mut mgr = ConfigManager::new(&dir);

    // 构造含 buttons 与 profiles 的完整配置
    let mut config = ConfigFile::default();
    config.buttons.push(button("b1", "content-1"));
    config.profiles.push(super::model::AppProfile {
        process_name: "Code.exe".to_string(),
        buttons: vec![button("p1", "fmt")],
    });
    mgr.config_mut().buttons = config.buttons.clone();
    mgr.config_mut().profiles = config.profiles.clone();

    // 保存
    mgr.save().expect("保存应成功");

    // 重新加载，验证 buttons 和 profiles 都保留
    let mut mgr2 = ConfigManager::new(&dir);
    mgr2.load().expect("重新加载应成功");
    assert_eq!(mgr2.config().buttons.len(), 1, "buttons 应被保留");
    assert_eq!(mgr2.config().profiles.len(), 1, "profiles 应被保留");
    assert_eq!(mgr2.config().profiles[0].process_name, "Code.exe");

    fs::remove_dir_all(&dir).ok();
}

// ============================================================
// 错误处理：失败返回错误，不 panic
// ============================================================

#[test]
fn test_error_load_broken_config_returns_error_not_panic() {
    let dir = temp_dir("broken");
    let mut mgr = ConfigManager::new(&dir);
    fs::write(mgr.config_path(), "this is not valid toml [ broken").unwrap();

    let result = mgr.load();
    assert!(result.is_err(), "损坏的配置应返回错误而非 panic");
    assert!(
        matches!(result, Err(super::manager::ConfigError::Parse(_))),
        "应为解析错误"
    );

    fs::remove_dir_all(&dir).ok();
}

#[test]
fn test_error_load_missing_file_returns_error() {
    let dir = temp_dir("missing_load");
    let mut mgr = ConfigManager::new(&dir);
    // 文件不存在
    let result = mgr.load();
    assert!(result.is_err(), "文件不存在应返回错误");

    fs::remove_dir_all(&dir).ok();
}

// ============================================================
// AC2-5：跨平台路径 — 配置目录由调用方注入
// ============================================================

#[test]
fn test_ac2_5_path_is_based_on_injected_dir() {
    let dir = temp_dir("path_check");
    let mgr = ConfigManager::new(&dir);
    assert_eq!(
        mgr.config_path(),
        dir.join("default.toml"),
        "配置路径应基于注入的目录"
    );

    fs::remove_dir_all(&dir).ok();
}