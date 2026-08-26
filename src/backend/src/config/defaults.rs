//! 配置默认样例

use super::model::{AppProfile, ButtonConfig, ConfigFile};

/// 创建首次启动时的默认配置样例
///
/// 提供常用按钮，方便用户首启后直接体验 QuickInput 的核心功能：
/// - 默认按钮组：常见聊天会话回复短语（未匹配任何进程时显示）
/// - WindowsTerminal.exe：Git 常用操作 + 常用 Linux 命令
/// - Code.exe：常用开发命令
/// 用户可手动编辑 `default.toml` 自定义。
pub fn default_config() -> ConfigFile {
    ConfigFile {
        default_buttons: vec![],
        default_inject_mode: None,
        overlay: None,
        shortcuts: None,
        buttons: vec![
            ButtonConfig {
                id: "reply-ok".to_string(),
                label: "好的".to_string(),
                content: "好的".to_string(),
                comment: Some("简洁确认回复".to_string()),
                ..Default::default()
            },
            ButtonConfig {
                id: "reply-ok-handle".to_string(),
                label: "收到处理".to_string(),
                content: "好的，收到，马上处理".to_string(),
                comment: Some("确认收到并开始处理".to_string()),
                ..Default::default()
            },
            ButtonConfig {
                id: "reply-thanks".to_string(),
                label: "感谢".to_string(),
                content: "谢谢！麻烦你了".to_string(),
                comment: Some("致谢回复".to_string()),
                ..Default::default()
            },
            ButtonConfig {
                id: "reply-no-problem".to_string(),
                label: "没问题".to_string(),
                content: "没问题，可以的".to_string(),
                comment: Some("同意/确认可行".to_string()),
                ..Default::default()
            },
            ButtonConfig {
                id: "reply-wait".to_string(),
                label: "稍等".to_string(),
                content: "稍等，我看一下".to_string(),
                comment: Some("需要时间查看".to_string()),
                ..Default::default()
            },
            ButtonConfig {
                id: "reply-later".to_string(),
                label: "稍后回复".to_string(),
                content: "收到，稍后回复你".to_string(),
                comment: Some("延迟回复场景".to_string()),
                ..Default::default()
            },
            ButtonConfig {
                id: "reply-noted".to_string(),
                label: "已记下".to_string(),
                content: "好的，我先记下了".to_string(),
                comment: Some("记录待办/要点".to_string()),
                ..Default::default()
            },
            ButtonConfig {
                id: "reply-done".to_string(),
                label: "已完成".to_string(),
                content: "已经处理好了，你可以看看".to_string(),
                comment: Some("告知任务完成".to_string()),
                ..Default::default()
            },
            ButtonConfig {
                id: "reply-confirm".to_string(),
                label: "确认中".to_string(),
                content: "这个问题我需要确认一下，稍后答复你".to_string(),
                comment: Some("需要核实后答复".to_string()),
                ..Default::default()
            },
            ButtonConfig {
                id: "reply-sorry".to_string(),
                label: "致歉".to_string(),
                content: "抱歉，刚才没看到消息".to_string(),
                comment: Some("回复不及时致歉".to_string()),
                ..Default::default()
            },
        ],
        profiles: vec![
            AppProfile {
                process_name: "WindowsTerminal.exe".to_string(),
                name: Some("终端".to_string()),
                inject_mode: None,
                buttons: vec![
                    ButtonConfig {
                        id: "git-status".to_string(),
                        label: "Git Status".to_string(),
                        content: "git status".to_string(),
                        comment: Some("查看工作区状态".to_string()),
                        ..Default::default()
                    },
                    ButtonConfig {
                        id: "git-add".to_string(),
                        label: "Git Add".to_string(),
                        content: "git add .".to_string(),
                        comment: Some("暂存所有变更".to_string()),
                        ..Default::default()
                    },
                    ButtonConfig {
                        id: "git-commit".to_string(),
                        label: "Git Commit".to_string(),
                        content: "git commit -m \"{input}\"".to_string(),
                        comment: Some("提交（{input} 为模板位：左键光标停在此处，右键模板输入）".to_string()),
                        ..Default::default()
                    },
                    ButtonConfig {
                        id: "git-push".to_string(),
                        label: "Git Push".to_string(),
                        content: "git push".to_string(),
                        comment: Some("推送到远程仓库".to_string()),
                        ..Default::default()
                    },
                    ButtonConfig {
                        id: "git-pull".to_string(),
                        label: "Git Pull".to_string(),
                        content: "git pull".to_string(),
                        comment: Some("拉取远程更新".to_string()),
                        ..Default::default()
                    },
                    ButtonConfig {
                        id: "git-log".to_string(),
                        label: "Git Log".to_string(),
                        content: "git log --oneline -10".to_string(),
                        comment: Some("最近 10 条提交记录".to_string()),
                        ..Default::default()
                    },
                    ButtonConfig {
                        id: "git-branch".to_string(),
                        label: "Git Branch".to_string(),
                        content: "git branch -a".to_string(),
                        comment: Some("查看全部分支".to_string()),
                        ..Default::default()
                    },
                    ButtonConfig {
                        id: "git-checkout".to_string(),
                        label: "Git Checkout".to_string(),
                        content: "git checkout -b {input}".to_string(),
                        comment: Some("新建并切换分支（{input} 填分支名）".to_string()),
                        ..Default::default()
                    },
                    ButtonConfig {
                        id: "git-diff".to_string(),
                        label: "Git Diff".to_string(),
                        content: "git diff".to_string(),
                        comment: Some("查看工作区差异".to_string()),
                        ..Default::default()
                    },
                    ButtonConfig {
                        id: "git-stash".to_string(),
                        label: "Git Stash".to_string(),
                        content: "git stash".to_string(),
                        comment: Some("暂存工作区改动".to_string()),
                        ..Default::default()
                    },
                    ButtonConfig {
                        id: "git-amend".to_string(),
                        label: "Git Amend".to_string(),
                        content: "git commit --amend".to_string(),
                        comment: Some("修改最近一次提交信息".to_string()),
                        ..Default::default()
                    },
                    ButtonConfig {
                        id: "docker-ps".to_string(),
                        label: "Docker PS".to_string(),
                        content: "docker ps".to_string(),
                        comment: Some("列出运行中的容器".to_string()),
                        ..Default::default()
                    },
                    ButtonConfig {
                        id: "tail-log".to_string(),
                        label: "Tail Log".to_string(),
                        content: "tail -f app.log".to_string(),
                        comment: Some("实时查看日志".to_string()),
                        ..Default::default()
                    },
                    ButtonConfig {
                        id: "linux-ls".to_string(),
                        label: "ls -la".to_string(),
                        content: "ls -la".to_string(),
                        comment: Some("列出文件详情".to_string()),
                        ..Default::default()
                    },
                    ButtonConfig {
                        id: "linux-pwd".to_string(),
                        label: "pwd".to_string(),
                        content: "pwd".to_string(),
                        comment: Some("显示当前目录".to_string()),
                        ..Default::default()
                    },
                    ButtonConfig {
                        id: "linux-clear".to_string(),
                        label: "清屏".to_string(),
                        content: "clear".to_string(),
                        comment: Some("清空终端输出".to_string()),
                        ..Default::default()
                    },
                    ButtonConfig {
                        id: "linux-ps".to_string(),
                        label: "ps grep".to_string(),
                        content: "ps aux | grep {input}".to_string(),
                        comment: Some("查看进程（{input} 填关键字）".to_string()),
                        ..Default::default()
                    },
                    ButtonConfig {
                        id: "linux-df".to_string(),
                        label: "df -h".to_string(),
                        content: "df -h".to_string(),
                        comment: Some("磁盘使用情况".to_string()),
                        ..Default::default()
                    },
                    ButtonConfig {
                        id: "linux-du".to_string(),
                        label: "du -sh".to_string(),
                        content: "du -sh *".to_string(),
                        comment: Some("各目录占用大小".to_string()),
                        ..Default::default()
                    },
                    ButtonConfig {
                        id: "linux-free".to_string(),
                        label: "free -h".to_string(),
                        content: "free -h".to_string(),
                        comment: Some("内存使用情况".to_string()),
                        ..Default::default()
                    },
                    ButtonConfig {
                        id: "linux-top".to_string(),
                        label: "top".to_string(),
                        content: "top".to_string(),
                        comment: Some("实时进程监控（q 退出）".to_string()),
                        ..Default::default()
                    },
                    ButtonConfig {
                        id: "linux-tail".to_string(),
                        label: "tail -f".to_string(),
                        content: "tail -f {input}".to_string(),
                        comment: Some("实时查看日志（{input} 填文件路径）".to_string()),
                        ..Default::default()
                    },
                    ButtonConfig {
                        id: "linux-chmod".to_string(),
                        label: "chmod +x".to_string(),
                        content: "chmod +x {input}".to_string(),
                        comment: Some("赋予执行权限（{input} 填文件）".to_string()),
                        ..Default::default()
                    },
                    ButtonConfig {
                        id: "linux-curl".to_string(),
                        label: "curl -s".to_string(),
                        content: "curl -s {input}".to_string(),
                        comment: Some("发起请求（{input} 填 URL）".to_string()),
                        ..Default::default()
                    },
                    ButtonConfig {
                        id: "linux-kill".to_string(),
                        label: "kill -9".to_string(),
                        content: "kill -9 {input}".to_string(),
                        comment: Some("强制结束进程（{input} 填 PID）".to_string()),
                        ..Default::default()
                    },
                    ButtonConfig {
                        id: "linux-history".to_string(),
                        label: "history".to_string(),
                        content: "history | tail -20".to_string(),
                        comment: Some("最近 20 条命令记录".to_string()),
                        ..Default::default()
                    },
                ],
                groups: vec![],
            },
            AppProfile {
                process_name: "Code.exe".to_string(),
                name: Some("VS Code".to_string()),
                inject_mode: None,
                buttons: vec![
                    ButtonConfig {
                        id: "fmt".to_string(),
                        label: "Format".to_string(),
                        content: "cargo fmt".to_string(),
                        comment: Some("格式化 Rust 代码".to_string()),
                        ..Default::default()
                    },
                    ButtonConfig {
                        id: "build".to_string(),
                        label: "Build".to_string(),
                        content: "cargo build".to_string(),
                        comment: Some("编译项目".to_string()),
                        ..Default::default()
                    },
                ],
                groups: vec![],
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

    #[test]
    fn test_default_config_has_chat_replies() {
        let config = default_config();
        // 默认按钮组应包含聊天常用回复
        assert!(
            config.buttons.iter().any(|b| b.id.starts_with("reply-")),
            "默认按钮应包含聊天回复短语"
        );
    }

    #[test]
    fn test_default_config_has_terminal_git_and_linux() {
        let config = default_config();
        let term = config
            .profiles
            .iter()
            .find(|p| p.process_name.eq_ignore_ascii_case("WindowsTerminal.exe"))
            .expect("应有 WindowsTerminal 画像");
        assert!(
            term.buttons.iter().any(|b| b.id.starts_with("git-")),
            "终端画像应包含 Git 操作按钮"
        );
        assert!(
            term.buttons.iter().any(|b| b.id.starts_with("linux-")),
            "终端画像应包含 Linux 命令按钮"
        );
    }

    #[test]
    fn test_default_config_button_ids_unique() {
        let config = default_config();
        let mut seen = std::collections::HashSet::new();
        for b in &config.buttons {
            assert!(seen.insert(b.id.as_str()), "默认按钮 ID 重复: {}", b.id);
        }
        for p in &config.profiles {
            for b in &p.buttons {
                assert!(
                    seen.insert(b.id.as_str()),
                    "画像按钮 ID 与默认按钮重复: {}",
                    b.id
                );
            }
        }
    }

    #[test]
    fn test_default_config_validates() {
        assert!(default_config().validate().is_ok(), "默认配置应通过校验");
    }

    /// 仓库根目录的 config.example.toml 必须能被当前 schema 解析并通过校验，
    /// 防止示例文件与代码演进脱节。
    #[test]
    fn test_example_config_file_parses_and_validates() {
        let path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../config.example.toml"
        );
        let content = std::fs::read_to_string(path)
            .expect("应存在 config.example.toml（仓库根目录）");
        let parsed: ConfigFile = toml::from_str(&content)
            .expect("config.example.toml 应符合配置 schema");
        assert!(parsed.validate().is_ok(), "config.example.toml 应通过校验");
        assert!(
            !parsed.buttons.is_empty(),
            "config.example.toml 应包含全局按钮"
        );
        // 示例应演示画像自定义分组（[[profiles.groups]] 嵌套）
        assert!(
            parsed.profiles.iter().any(|p| !p.groups.is_empty()),
            "config.example.toml 应包含画像分组示例"
        );
        // 示例应演示 {input} 模板占位符
        assert!(
            parsed
                .buttons
                .iter()
                .chain(parsed.profiles.iter().flat_map(|p| p.buttons.iter()))
                .any(|b| b.content.contains("{input}")),
            "config.example.toml 应包含 {{input}} 模板按钮示例"
        );
        // 示例应演示动态占位符（{date}/{time}/{clipboard}，后端注入时展开）
        let dynamic = ["{date}", "{time}", "{clipboard}"];
        for ph in dynamic {
            assert!(
                parsed
                    .buttons
                    .iter()
                    .chain(parsed.profiles.iter().flat_map(|p| p.buttons.iter()))
                    .any(|b| b.content.contains(ph)),
                "config.example.toml 应包含 {ph} 动态占位符按钮示例"
            );
        }
    }
}
