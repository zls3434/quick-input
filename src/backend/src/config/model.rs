use serde::{Deserialize, Serialize};

/// 配置校验错误
#[derive(Debug, Clone, PartialEq)]
pub struct ValidationError {
    pub field: String,
    pub message: String,
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.field, self.message)
    }
}

/// 单个快捷按钮配置
///
/// 每个按钮包含：唯一标识 `id`、显示标签 `label`、待输入内容 `content`、
/// 以及可选的悬浮注释 `comment`。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ButtonConfig {
    /// 唯一标识符（必填）
    pub id: String,
    /// 按钮显示名称（必填）
    pub label: String,
    /// 点击后注入的文本/命令内容（必填）
    pub content: String,
    /// 悬浮注释说明（可选，缺失时默认为 None）
    #[serde(default)]
    pub comment: Option<String>,
}

/// 按应用（进程）的配置画像
///
/// 一个进程名对应一组按钮配置，进程名大小写不敏感。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AppProfile {
    /// 进程名称（必填），如 `"Code.exe"`、`"WindowsTerminal.exe"`
    pub process_name: String,
    /// 映射自定义名称（可选），用于设置界面展示；缺失时显示进程名
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 该进程下的快捷按钮列表（必填，但可空）
    #[serde(default)]
    pub buttons: Vec<ButtonConfig>,
}

/// 悬浮窗设置（布局与各布局的记忆位置）
///
/// 位置使用逻辑坐标存储，按布局分别记忆；未记忆时使用各布局默认位置。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(default)]
pub struct OverlaySettings {
    /// 布局："vertical"（竖向，默认）或 "horizontal"（横向）
    pub layout: String,
    /// 竖向布局记忆位置 X（逻辑坐标）
    pub vertical_x: Option<i32>,
    /// 竖向布局记忆位置 Y（逻辑坐标）
    pub vertical_y: Option<i32>,
    /// 横向布局记忆位置 X（逻辑坐标）
    pub horizontal_x: Option<i32>,
    /// 横向布局记忆位置 Y（逻辑坐标）
    pub horizontal_y: Option<i32>,
}

impl OverlaySettings {
    pub const LAYOUT_VERTICAL: &'static str = "vertical";
    pub const LAYOUT_HORIZONTAL: &'static str = "horizontal";

    /// 生效布局（空值视为竖向默认）
    pub fn effective_layout(&self) -> &'static str {
        if self.layout == Self::LAYOUT_HORIZONTAL {
            Self::LAYOUT_HORIZONTAL
        } else {
            Self::LAYOUT_VERTICAL
        }
    }

    /// 读取指定布局的记忆位置
    pub fn saved_position(&self, layout: &str) -> Option<(i32, i32)> {
        if layout == Self::LAYOUT_HORIZONTAL {
            match (self.horizontal_x, self.horizontal_y) {
                (Some(x), Some(y)) => Some((x, y)),
                _ => None,
            }
        } else {
            match (self.vertical_x, self.vertical_y) {
                (Some(x), Some(y)) => Some((x, y)),
                _ => None,
            }
        }
    }

    /// 记录指定布局的位置
    pub fn set_position(&mut self, layout: &str, x: i32, y: i32) {
        if layout == Self::LAYOUT_HORIZONTAL {
            self.horizontal_x = Some(x);
            self.horizontal_y = Some(y);
        } else {
            self.vertical_x = Some(x);
            self.vertical_y = Some(y);
        }
    }
}

/// 顶层配置文件
///
/// 包含默认按钮列表与可选的按应用配置画像。
/// 当焦点进程无匹配的 `AppProfile` 时，回退使用 `buttons` 默认列表。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ConfigFile {
    /// 默认配置的按钮列表（可选，默认为空）
    #[serde(default)]
    pub buttons: Vec<ButtonConfig>,
    /// 按应用配置画像列表（可选，默认为空）
    #[serde(default)]
    pub profiles: Vec<AppProfile>,
    /// 悬浮窗设置（可选）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overlay: Option<OverlaySettings>,
}

impl ConfigFile {
    /// 校验配置合法性
    ///
    /// 检查规则：
    /// - 每个按钮必须填写 `id`、`label`、`content`（非空）
    /// - `id` 在默认按钮列表内唯一
    /// - 每个 `AppProfile` 的 `process_name` 非空
    /// - `process_name` 在画像列表中唯一（大小写不敏感）
    ///
    /// 有任一违规时返回第一个 `ValidationError`。
    pub fn validate(&self) -> Result<(), ValidationError> {
        // 校验默认按钮
        let mut seen_ids: Vec<&str> = Vec::new();
        for btn in &self.buttons {
            if btn.id.trim().is_empty() {
                return Err(ValidationError {
                    field: "buttons[].id".into(),
                    message: "按钮 ID 不能为空".into(),
                });
            }
            if btn.label.trim().is_empty() {
                return Err(ValidationError {
                    field: "buttons[].label".into(),
                    message: format!("按钮 '{}' 的标签不能为空", btn.id),
                });
            }
            if btn.content.trim().is_empty() {
                return Err(ValidationError {
                    field: "buttons[].content".into(),
                    message: format!("按钮 '{}' 的内容不能为空", btn.id),
                });
            }
            if seen_ids.iter().any(|s| **s == btn.id) {
                return Err(ValidationError {
                    field: "buttons[].id".into(),
                    message: format!("按钮 ID '{}' 重复", btn.id),
                });
            }
            seen_ids.push(&btn.id);
        }

        // 校验画像
        let mut seen_profiles: Vec<&str> = Vec::new();
        for profile in &self.profiles {
            if profile.process_name.trim().is_empty() {
                return Err(ValidationError {
                    field: "profiles[].process_name".into(),
                    message: "进程名不能为空".into(),
                });
            }
            if seen_profiles
                .iter()
                .any(|s| s.eq_ignore_ascii_case(&profile.process_name))
            {
                return Err(ValidationError {
                    field: "profiles[].process_name".into(),
                    message: format!("进程 '{}' 重复", profile.process_name),
                });
            }
            seen_profiles.push(&profile.process_name);

            // 校验画像内按钮 ID 唯一
            let mut p_seen: Vec<&str> = Vec::new();
            for btn in &profile.buttons {
                if btn.id.trim().is_empty() {
                    return Err(ValidationError {
                        field: format!("profiles[{}].buttons[].id", profile.process_name),
                        message: "按钮 ID 不能为空".into(),
                    });
                }
                if p_seen.iter().any(|s| **s == btn.id) {
                    return Err(ValidationError {
                        field: format!("profiles[{}].buttons[].id", profile.process_name),
                        message: format!("按钮 ID '{}' 重复", btn.id),
                    });
                }
                p_seen.push(&btn.id);
            }
        }

        // 校验悬浮窗设置
        if let Some(overlay) = &self.overlay {
            let layout = overlay.layout.trim();
            if !layout.is_empty()
                && layout != OverlaySettings::LAYOUT_VERTICAL
                && layout != OverlaySettings::LAYOUT_HORIZONTAL
            {
                return Err(ValidationError {
                    field: "overlay.layout".into(),
                    message: format!("悬浮窗布局 '{layout}' 无效（应为 vertical 或 horizontal）"),
                });
            }
        }

        Ok(())
    }

    /// 按进程名查询匹配的按钮配置（大小写不敏感）
    ///
    /// 从 `profiles` 中查找与给定进程名匹配的 `AppProfile`，
    /// 匹配采用大小写不敏感比较（`eq_ignore_ascii_case`）。
    ///
    /// 返回 `Option<&[ButtonConfig]>`：
    /// - 匹配成功时返回该进程的按钮列表切片
    /// - 无匹配时返回 `None`（供 S103 兜底回退到默认按钮）
    pub fn get_buttons_for_process(&self, process_name: &str) -> Option<&[ButtonConfig]> {
        self.profiles
            .iter()
            .find(|p| p.process_name.eq_ignore_ascii_case(process_name))
            .map(|p| p.buttons.as_slice())
    }

    /// 获取当前进程的按钮列表（优先匹配进程，无匹配回退默认）
    ///
    /// S103 兜底逻辑：调用 [`ConfigFile::get_buttons_for_process`] 优先按进程匹配；
    /// 若匹配成功返回该进程的按钮集，否则回退到 `buttons` 默认列表。
    /// 返回切片引用，永不返回 `None`。
    pub fn get_buttons_current(&self, process_name: &str) -> &[ButtonConfig] {
        self.get_buttons_for_process(process_name)
            .unwrap_or(&self.buttons)
    }
}