//! 配置默认样例

use super::model::{AppProfile, ButtonConfig, ConfigFile};

/// 创建首次启动时的默认配置样例
///
/// 提供几个示例按钮，方便用户首启后直接体验 QuickInput 的核心功能。
/// 用户可手动编辑 `default.toml` 自定义。
pub fn default_config() -> ConfigFile {
    ConfigFile {
        buttons: vec![
            ButtonConfig {
                id: "git-status".to_string(),
                label: "Git Status".to_string(),
                content: "git status".to_string(),
                comment: Some("查看当前工作区状态".to_string()),
            },
            ButtonConfig {
                id: "git-commit".to_string(),
                label: "Git Commit".to_string(),
                content: "git commit -m \"\"".to_string(),
                comment: Some("创建提交（在引号内补充说明）".to_string()),
            },
            ButtonConfig {
                id: "kubectl-pods".to_string(),
                label: "Get Pods".to_string(),
                content: "kubectl get pods".to_string(),
                comment: Some("列出所有 Pod".to_string()),
            },
        ],
        profiles: vec![
            AppProfile {
                process_name: "Code.exe".to_string(),
                name: None,
                buttons: vec![
                    ButtonConfig {
                        id: "fmt".to_string(),
                        label: "Format".to_string(),
                        content: "cargo fmt".to_string(),
                        comment: Some("格式化 Rust 代码".to_string()),
                    },
                    ButtonConfig {
                        id: "build".to_string(),
                        label: "Build".to_string(),
                        content: "cargo build".to_string(),
                        comment: Some("编译项目".to_string()),
                    },
                ],
            },
            AppProfile {
                process_name: "WindowsTerminal.exe".to_string(),
                name: None,
                buttons: vec![
                    ButtonConfig {
                        id: "docker-ps".to_string(),
                        label: "Docker PS".to_string(),
                        content: "docker ps".to_string(),
                        comment: Some("列出运行中的容器".to_string()),
                    },
                    ButtonConfig {
                        id: "tail-log".to_string(),
                        label: "Tail Log".to_string(),
                        content: "tail -f app.log".to_string(),
                        comment: Some("实时查看日志".to_string()),
                    },
                ],
            },
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config_has_buttons_and_profiles() {
        let config = default_config();
        assert!(!config.buttons.is_empty(), "默认配置应有按钮");
        assert!(!config.profiles.is_empty(), "默认配置应有应用画像");
    }

    #[test]
    fn test_default_config_roundtrip_serialization() {
        let config = default_config();
        let toml_str = toml::to_string(&config).unwrap();
        let parsed: ConfigFile = toml::from_str(&toml_str).unwrap();
        assert_eq!(config, parsed, "默认配置应能序列化并反序列化一致");
    }
}