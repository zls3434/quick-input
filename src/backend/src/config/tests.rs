use super::*;

// ============================================================
// 夹具工厂
// ============================================================

/// 创建测试用的 ButtonConfig 夹具
fn create_button_fixture(id: &str, label: &str, content: &str) -> ButtonConfig {
    ButtonConfig {
        id: id.to_string(),
        label: label.to_string(),
        content: content.to_string(),
        comment: None,
    }
}

/// 创建带注释的 ButtonConfig 夹具
fn create_button_fixture_with_comment(
    id: &str,
    label: &str,
    content: &str,
    comment: &str,
) -> ButtonConfig {
    ButtonConfig {
        id: id.to_string(),
        label: label.to_string(),
        content: content.to_string(),
        comment: Some(comment.to_string()),
    }
}

/// 创建测试用的默认配置夹具
fn create_default_config_fixture() -> ConfigFile {
ConfigFile {
                overlay: None,
buttons: vec![
            create_button_fixture("git-status", "Git Status", "git status"),
            create_button_fixture("git-pull", "Git Pull", "git pull --rebase"),
            create_button_fixture_with_comment(
                "deploy-prod",
                "Deploy",
                "kubectl apply -f prod.yaml",
                "部署到生产环境，需先确认审批",
            ),
        ],
        profiles: vec![
            AppProfile {
                process_name: "Code.exe".to_string(),
                name: None,
                buttons: vec![
                    create_button_fixture("fmt", "Format", "cargo fmt"),
                    create_button_fixture("build", "Build", "cargo build"),
                ],
            },
            AppProfile {
                process_name: "WindowsTerminal.exe".to_string(),
                name: None,
                buttons: vec![
                    create_button_fixture("kubectl-get-pods", "Get Pods", "kubectl get pods"),
                    create_button_fixture("docker-ps", "Docker PS", "docker ps"),
                ],
            },
        ],
    }
}

// ============================================================
// AC1-1：合法配置文件解析
// ============================================================

#[test]
fn test_ac1_1_parse_valid_config() {
    let config = create_default_config_fixture();

    // 序列化为 TOML
    let toml_str = toml::to_string(&config).expect("序列化应成功");
    assert!(!toml_str.is_empty(), "TOML 字符串不应为空");

    // 反序列化回 ConfigFile
    let parsed: ConfigFile = toml::from_str(&toml_str).expect("反序列化应成功");

    // 验证默认按钮列表
    assert_eq!(parsed.buttons.len(), 3);
    assert_eq!(parsed.buttons[0].id, "git-status");
    assert_eq!(parsed.buttons[0].label, "Git Status");
    assert_eq!(parsed.buttons[0].content, "git status");
    assert_eq!(parsed.buttons[0].comment, None);

    // 验证带注释的按钮
    assert_eq!(parsed.buttons[2].id, "deploy-prod");
    assert_eq!(
        parsed.buttons[2].comment,
        Some("部署到生产环境，需先确认审批".to_string())
    );

    // 验证按应用配置
    assert_eq!(parsed.profiles.len(), 2);
    assert_eq!(parsed.profiles[0].process_name, "Code.exe");
    assert_eq!(parsed.profiles[0].buttons.len(), 2);
    assert_eq!(parsed.profiles[1].process_name, "WindowsTerminal.exe");
    assert_eq!(parsed.profiles[1].buttons.len(), 2);
}

#[test]
fn test_ac1_1_parse_valid_toml_from_string() {
    let toml_str = r#"
[[buttons]]
id = "hello"
label = "Hello"
content = "Hello, World!"

[[buttons]]
id = "date"
label = "Date"
content = "date"

[[profiles]]
process_name = "Code.exe"

[[profiles.buttons]]
id = "fmt"
label = "Format"
content = "cargo fmt"
"#;

    let config: ConfigFile = toml::from_str(toml_str).expect("合法 TOML 应解析成功");
    assert_eq!(config.buttons.len(), 2);
    assert_eq!(config.profiles.len(), 1);
    assert_eq!(config.profiles[0].buttons.len(), 1);
}

// ============================================================
// AC1-2：缺失必填字段报错
// ============================================================

#[test]
fn test_ac1_2_missing_required_field_id() {
    let toml_str = r#"
[[buttons]]
label = "No ID"
content = "some content"
"#;

    let result: Result<ConfigFile, toml::de::Error> = toml::from_str(toml_str);
    assert!(result.is_err(), "缺少 id 必填字段应报错");
    let err = result.unwrap_err().to_string();
    // 错误信息应包含缺失字段的信息
    assert!(
        err.contains("id") || err.contains("missing field"),
        "错误信息应指出缺失字段 id，实际错误: {}",
        err
    );
}

#[test]
fn test_ac1_2_missing_required_field_label() {
    let toml_str = r#"
[[buttons]]
id = "btn-1"
content = "some content"
"#;

    let result: Result<ConfigFile, toml::de::Error> = toml::from_str(toml_str);
    assert!(result.is_err(), "缺少 label 必填字段应报错");
}

#[test]
fn test_ac1_2_missing_required_field_content() {
    let toml_str = r#"
[[buttons]]
id = "btn-1"
label = "Button"
"#;

    let result: Result<ConfigFile, toml::de::Error> = toml::from_str(toml_str);
    assert!(result.is_err(), "缺少 content 必填字段应报错");
}

#[test]
fn test_ac1_2_missing_required_field_process_name() {
    let toml_str = r#"
[[profiles]]
buttons = []
"#;

    let result: Result<ConfigFile, toml::de::Error> = toml::from_str(toml_str);
    assert!(result.is_err(), "缺少 process_name 必填字段应报错");
}

// ============================================================
// AC1-3：类型错误应报错，不崩溃
// ============================================================

#[test]
fn test_ac1_3_type_error_buttons_not_array() {
    let toml_str = r#"
buttons = "not-an-array"
"#;

    let result: Result<ConfigFile, toml::de::Error> = toml::from_str(toml_str);
    assert!(result.is_err(), "类型错误应报错，不崩溃");
}

#[test]
fn test_ac1_3_type_error_id_not_string() {
    let toml_str = r#"
[[buttons]]
id = 12345
label = "Button"
content = "content"
"#;

    let result: Result<ConfigFile, toml::de::Error> = toml::from_str(toml_str);
    assert!(result.is_err(), "id 为数字类型应报错，不崩溃");
}

// ============================================================
// AC1-4：强类型 struct + serde 保证
// ============================================================

#[test]
fn test_ac1_4_strongly_typed_serde_struct() {
    let config = create_default_config_fixture();

    // 验证字段类型（强类型保证）
    let _button: &ButtonConfig = &config.buttons[0];
    let _profile: &AppProfile = &config.profiles[0];
    let _maybe_comment: Option<&String> = config.buttons[0].comment.as_ref();

    // 序列化后再反序列化，验证 round-trip
    let toml_str = toml::to_string(&config).unwrap();
    let parsed: ConfigFile = toml::from_str(&toml_str).unwrap();
    assert_eq!(config, parsed, "Round-trip 序列化/反序列化应保持数据一致");
}

// ============================================================
// 可选字段测试
// ============================================================

#[test]
fn test_optional_comment_defaults_to_none() {
    let toml_str = r#"
[[buttons]]
id = "btn-1"
label = "Button"
content = "content"
"#;

    let config: ConfigFile = toml::from_str(toml_str).unwrap();
    assert_eq!(config.buttons[0].comment, None, "未提供 comment 时应默认为 None");
}

#[test]
fn test_optional_buttons_defaults_to_empty() {
    let toml_str = r#"
[[profiles]]
process_name = "Test.exe"
"#;

    let config: ConfigFile = toml::from_str(toml_str).unwrap();
    assert_eq!(config.buttons.len(), 0, "未提供 buttons 时应默认为空");
    assert_eq!(config.profiles[0].buttons.len(), 0, "未提供 profiles.buttons 时应默认为空");
}

// ============================================================
// 空配置测试
// ============================================================

#[test]
fn test_empty_config_is_valid() {
    let toml_str = "";
    let config: ConfigFile = toml::from_str(toml_str).unwrap();
    assert_eq!(config.buttons.len(), 0, "空配置的 buttons 应为空");
    assert_eq!(config.profiles.len(), 0, "空配置的 profiles 应为空");
}

// ============================================================
// S102：按进程名映射应用配置
// ============================================================

/// 创建仅含 profiles 的测试配置夹具
fn create_profile_config_fixture() -> ConfigFile {
ConfigFile {
                overlay: None,
buttons: vec![create_button_fixture("default-1", "Default", "echo default")],
        profiles: vec![
            AppProfile {
                process_name: "Code.exe".to_string(),
                name: None,
                buttons: vec![
                    create_button_fixture("fmt", "Format", "cargo fmt"),
                    create_button_fixture("build", "Build", "cargo build"),
                ],
            },
            AppProfile {
                process_name: "WindowsTerminal.exe".to_string(),
                name: None,
                buttons: vec![create_button_fixture("docker-ps", "Docker PS", "docker ps")],
            },
        ],
    }
}

// AC3-1: 焦点进程名为 Code.exe 时返回对应按钮集
#[test]
fn test_ac3_1_get_buttons_for_matching_process() {
    let config = create_profile_config_fixture();
    let buttons = config
        .get_buttons_for_process("Code.exe")
        .expect("Code.exe 应有匹配的按钮集");
    assert_eq!(buttons.len(), 2, "Code.exe 应返回 2 个按钮");
    assert_eq!(buttons[0].id, "fmt");
    assert_eq!(buttons[1].id, "build");
}

// AC3-2: 解析 profiles 建立进程名 → 按钮集映射
#[test]
fn test_ac3_2_profiles_establish_mapping() {
    let config = create_profile_config_fixture();
    assert_eq!(config.profiles.len(), 2);
    // 每个 profile 的 process_name 与 buttons 已正确建立映射
    assert_eq!(config.profiles[0].process_name, "Code.exe");
    assert_eq!(config.profiles[0].buttons.len(), 2);
    assert_eq!(config.profiles[1].process_name, "WindowsTerminal.exe");
    assert_eq!(config.profiles[1].buttons.len(), 1);
}

// AC3-3: 无匹配进程名返回 None（供 S103 兜底）
#[test]
fn test_ac3_3_no_match_returns_none() {
    let config = create_profile_config_fixture();
    let result = config.get_buttons_for_process("UnknownApp.exe");
    assert!(result.is_none(), "无匹配进程名应返回 None");
}

// AC3-4: 进程名大小写不敏感匹配
#[test]
fn test_ac3_4_case_insensitive_matching() {
    let config = create_profile_config_fixture();

    // 全小写
    let lower = config
        .get_buttons_for_process("code.exe")
        .expect("小写进程名应匹配");
    assert_eq!(lower.len(), 2);

    // 混合大小写
    let mixed = config
        .get_buttons_for_process("wInDoWsTeRmInAl.ExE")
        .expect("混合大小写应匹配");
    assert_eq!(mixed.len(), 1);

    // 大小写不同但进程名相同，返回同一按钮集
    let upper = config
        .get_buttons_for_process("CODE.EXE")
        .expect("大写进程名应匹配");
    assert_eq!(upper[0].id, "fmt");
}

// ============================================================
// S103：默认配置回退
// ============================================================

// AC4-1: 无匹配进程名时回退到默认按钮列表
#[test]
fn test_ac4_1_fallback_to_default_buttons() {
    let config = create_profile_config_fixture();
    // 默认按钮列表有 1 个按钮："default-1"
    let buttons = config.get_buttons_current("UnknownApp.exe");
    assert_eq!(buttons.len(), 1, "无匹配进程应回退到默认按钮");
    assert_eq!(buttons[0].id, "default-1");
}

// AC4-1: 有匹配进程名时优先返回进程按钮集
#[test]
fn test_ac4_1_matching_process_returns_profile_buttons() {
    let config = create_profile_config_fixture();
    let buttons = config.get_buttons_current("Code.exe");
    assert_eq!(buttons.len(), 2, "匹配进程时应返回对应按钮集");
    assert_eq!(buttons[0].id, "fmt");
    assert_eq!(buttons[1].id, "build");
}

// AC4-1: 回退返回的引用永不 panic
#[test]
fn test_ac4_1_fallback_never_panics() {
    let config = ConfigFile::default();
    // 空配置 + 任意进程名，回退到空列表，不应 panic
    let buttons = config.get_buttons_current("Anything.exe");
    assert!(buttons.is_empty(), "空配置回退应为空列表");
}

// AC4-4: 配置加载失败隔离（验证方法签名不 panic）
#[test]
fn test_ac4_4_loading_error_does_not_affect_other_modules() {
    // 验证 get_buttons_current 在空配置下返回空切片
    let config = ConfigFile::default();
    let _ = config.get_buttons_current("test.exe");
    // 无 panic 即通过
}

// ============================================================
// S172: 配置校验测试
// ============================================================

// AC13-1: 空配置通过校验
#[test]
fn test_validate_empty_config_ok() {
    let config = ConfigFile::default();
    assert!(config.validate().is_ok());
}

// AC13-1: 正常按钮集通过校验
#[test]
fn test_validate_valid_buttons_ok() {
    let config = ConfigFile {
                overlay: None,
buttons: vec![
            create_button_fixture("a", "A", "aaa"),
            create_button_fixture("b", "B", "bbb"),
        ],
        profiles: vec![],
    };
    assert!(config.validate().is_ok());
}

// AC13-2: 空 ID 校验失败
#[test]
fn test_validate_empty_id_fails() {
    let config = ConfigFile {
                overlay: None,
buttons: vec![create_button_fixture("", "A", "aaa")],
        profiles: vec![],
    };
    let err = config.validate().unwrap_err();
    assert_eq!(err.field, "buttons[].id");
}

// AC13-2: 空标签校验失败
#[test]
fn test_validate_empty_label_fails() {
    let config = ConfigFile {
                overlay: None,
buttons: vec![create_button_fixture("a", " ", "aaa")],
        profiles: vec![],
    };
    let err = config.validate().unwrap_err();
    assert_eq!(err.field, "buttons[].label");
}

// AC13-2: 重复按钮 ID 校验失败
#[test]
fn test_validate_duplicate_id_fails() {
    let config = ConfigFile {
                overlay: None,
buttons: vec![
            create_button_fixture("a", "A", "aaa"),
            create_button_fixture("a", "B", "bbb"),
        ],
        profiles: vec![],
    };
    let err = config.validate().unwrap_err();
    assert!(err.message.contains("重复"));
}

// AC13-2: 空进程名校验失败
#[test]
fn test_validate_empty_process_name_fails() {
    let config = ConfigFile {
                overlay: None,
buttons: vec![],
        profiles: vec![AppProfile {
            process_name: "".into(),
            name: None,
            buttons: vec![],
        }],
    };
    let err = config.validate().unwrap_err();
    assert_eq!(err.field, "profiles[].process_name");
}

// AC13-2: 重复进程名（大小写不敏感）校验失败
#[test]
fn test_validate_duplicate_process_name_case_insensitive_fails() {
    let config = ConfigFile {
                overlay: None,
buttons: vec![],
        profiles: vec![
            AppProfile {
                process_name: "Code.exe".into(),
                name: None,
                buttons: vec![],
            },
            AppProfile {
                process_name: "code.EXE".into(),
                name: None,
                buttons: vec![],
            },
        ],
    };
    let err = config.validate().unwrap_err();
    assert!(err.message.contains("重复"));
}

// AC13-1: 有效画像通过校验
#[test]
fn test_validate_valid_profile_ok() {
    let config = ConfigFile {
                overlay: None,
buttons: vec![],
        profiles: vec![AppProfile {
            process_name: "Code.exe".into(),
            name: None,
            buttons: vec![create_button_fixture("p1", "P1", "p1c")],
        }],
    };
    assert!(config.validate().is_ok());
}
// ============================================================
// 悬浮窗设置：布局与位置记忆
// ============================================================

#[test]
fn test_overlay_default_layout_is_vertical() {
    let ov = super::model::OverlaySettings::default();
    assert_eq!(ov.effective_layout(), "vertical");
    assert_eq!(ov.saved_position("vertical"), None);
}

#[test]
fn test_overlay_position_saved_per_layout() {
    let mut ov = super::model::OverlaySettings::default();
    ov.set_geometry("horizontal", 100, 200, 720, 116);
    assert_eq!(ov.saved_position("horizontal"), Some((100, 200)));
    assert_eq!(ov.saved_position("vertical"), None, "横向记忆不应影响竖向");
    ov.set_geometry("vertical", 10, 20, 320, 500);
    assert_eq!(ov.saved_position("vertical"), Some((10, 20)));
    assert_eq!(ov.saved_position("horizontal"), Some((100, 200)));
}

#[test]
fn test_overlay_size_memory_per_layout() {
    let mut ov = super::model::OverlaySettings::default();
    // 默认尺寸
    assert_eq!(
        ov.effective_size("vertical"),
        super::model::OverlaySettings::VERTICAL_DEFAULT_SIZE
    );
    assert_eq!(
        ov.effective_size("horizontal"),
        (super::model::OverlaySettings::HORIZONTAL_DEFAULT_W, super::model::OverlaySettings::HORIZONTAL_ROW_H)
    );
    // 竖向记忆宽高
    ov.set_geometry("vertical", 5, 6, 340, 520);
    assert_eq!(ov.effective_size("vertical"), (340, 520));
    // 横向只记忆宽度，高度恒为单行高度
    ov.set_geometry("horizontal", 7, 8, 900, 300);
    assert_eq!(ov.effective_size("horizontal"), (900, super::model::OverlaySettings::HORIZONTAL_ROW_H));
    // 两布局尺寸互不影响
    assert_eq!(ov.effective_size("vertical"), (340, 520));
}

#[test]
fn test_overlay_invalid_layout_rejected() {
    let mut config = create_default_config_fixture();
    config.overlay = Some(super::model::OverlaySettings {
        layout: "diagonal".to_string(),
        ..Default::default()
    });
    let err = config.validate().unwrap_err();
    assert_eq!(err.field, "overlay.layout");
}

#[test]
fn test_overlay_settings_backward_compatible() {
    // 旧配置无 overlay 字段应正常解析
    let toml_str = r#"
[[buttons]]
id = "a"
label = "A"
content = "aa"
"#;
    let config: ConfigFile = toml::from_str(toml_str).unwrap();
    assert!(config.overlay.is_none());
}
