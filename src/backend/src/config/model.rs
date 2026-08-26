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

/// 注入模式：剪贴板粘贴（默认，绕过输入法，适合现代应用）
pub const INJECT_MODE_PASTE: &str = "paste";
/// 注入模式：按键模拟（真实扫描码，适合老游戏/DirectInput/自绘输入框）
pub const INJECT_MODE_KEYSTROKE: &str = "keystroke";

/// 校验注入模式取值合法（None 视为默认 paste）
fn validate_inject_mode(field: &str, value: &Option<String>) -> Result<(), ValidationError> {
    if let Some(m) = value {
        if m != INJECT_MODE_PASTE && m != INJECT_MODE_KEYSTROKE {
            return Err(ValidationError {
                field: field.into(),
                message: format!("注入模式 '{m}' 无效（应为 paste 或 keystroke）"),
            });
        }
    }
    Ok(())
}

/// 单个快捷按钮配置
///
/// 每个按钮包含：唯一标识 `id`、显示标签 `label`、待输入内容 `content`、
/// 以及可选的悬浮注释 `comment`。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
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
    /// 管理维度分组名（可选）：仅设置界面按此归类展示按钮，不参与悬浮窗 Tab。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
}

/// 按钮分组（画像级）：组名 + 组内按钮。
/// 悬浮窗 Tab 标签的数据来源（仅画像定义的分组生效）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ButtonGroup {
    /// 分组名（必填，画像内大小写不敏感唯一）
    pub name: String,
    /// 该分组下的按钮列表（必填，可空）
    #[serde(default)]
    pub buttons: Vec<ButtonConfig>,
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
    /// 注入模式（可选）："paste"（默认）或 "keystroke"（老游戏按键模拟）。
    /// 老游戏（DirectInput/自绘输入框）不响应剪贴板粘贴与 Unicode 注入，
    /// 需用真实扫描码按键模拟。缺失时默认 paste。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inject_mode: Option<String>,
    /// 自定义分组（可选）：悬浮窗 Tab 标签依据。旧配置无此字段兼容为空。
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub groups: Vec<ButtonGroup>,
}

impl AppProfile {
    /// 生效的注入模式（未配置视为 paste）
    pub fn effective_inject_mode(&self) -> &'static str {
        if self.inject_mode.as_deref() == Some(INJECT_MODE_KEYSTROKE) {
            INJECT_MODE_KEYSTROKE
        } else {
            INJECT_MODE_PASTE
        }
    }

    /// 把按钮列表（含管理维度 group 值）聚合为「分组 + 未分组」存储结构。
    ///
    /// 分组按按钮 group 值首次出现顺序聚合；空串/空白的 group 视为未分组。
    /// 归入分组的按钮其 `group` 字段归一化为 `None`（组名由 `ButtonGroup.name`
    /// 承载，避免磁盘冗余），保证与 `flattened_buttons` 展平后 round-trip 等价。
    pub fn regroup(buttons: Vec<ButtonConfig>) -> (Vec<ButtonGroup>, Vec<ButtonConfig>) {
        let mut groups: Vec<ButtonGroup> = Vec::new();
        let mut ungrouped: Vec<ButtonConfig> = Vec::new();
        for mut btn in buttons {
            // 转 String 持有所有权，避免借用 btn.group 后无法就地修改
            let g = btn.group.as_deref().map(str::trim).unwrap_or("").to_string();
            if g.is_empty() {
                ungrouped.push(btn);
            } else {
                // 组名已由 ButtonGroup.name 承载，归一化冗余字段
                btn.group = None;
                if let Some(grp) = groups.iter_mut().find(|grp| grp.name == g) {
                    grp.buttons.push(btn);
                } else {
                    groups.push(ButtonGroup {
                        name: g,
                        buttons: vec![btn],
                    });
                }
            }
        }
        (groups, ungrouped)
    }

    /// 展平：把 groups 与 buttons 合并为「按钮 + group 值」管理视图（设置界面回显用）。
    ///
    /// 分组内按钮携带其组名（`group: Some(name)`），未分组按钮保持 `group: None`。
    pub fn flattened_buttons(&self) -> Vec<ButtonConfig> {
        let mut out = Vec::new();
        for g in &self.groups {
            for mut b in g.buttons.clone() {
                b.group = Some(g.name.clone());
                out.push(b);
            }
        }
        out.extend(self.buttons.clone());
        out
    }
}

/// 快捷键配置
///
/// 全局快捷键使用 `tauri-plugin-global-shortcut` 的字符串格式，
/// 如 `"CTRL+SHIFT+SPACE"`。键名大小写不敏感。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(default)]
pub struct ShortcutSettings {
    /// 显示/隐藏悬浮窗热键（None = 默认 CTRL+SHIFT+SPACE）
    pub show_overlay: Option<String>,
}

impl ShortcutSettings {
    /// 默认"显示/隐藏悬浮窗"热键
    pub const DEFAULT_SHOW_OVERLAY: &'static str = "CTRL+SHIFT+SPACE";

    /// 生效的"显示/隐藏悬浮窗"热键
    pub fn effective_show_overlay(&self) -> String {
        self.show_overlay
            .clone()
            .filter(|s| !s.trim().is_empty())
            .unwrap_or_else(|| Self::DEFAULT_SHOW_OVERLAY.to_string())
    }
}

/// 悬浮窗设置（布局与各布局的记忆几何）
///
/// 位置/尺寸使用逻辑坐标存储，按布局分别记忆；未记忆时使用各布局默认值。
/// 横向布局高度由内容行数自适应，不记忆高度。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(default)]
pub struct OverlaySettings {
    /// 布局："vertical"（竖向，默认）或 "horizontal"（横向）
    pub layout: String,
    /// 竖向布局记忆位置 X（逻辑坐标）
    pub vertical_x: Option<i32>,
    /// 竖向布局记忆位置 Y（逻辑坐标）
    pub vertical_y: Option<i32>,
    /// 竖向布局记忆宽度（逻辑像素）
    pub vertical_w: Option<i32>,
    /// 竖向布局记忆高度（逻辑像素）
    pub vertical_h: Option<i32>,
    /// 横向布局记忆位置 X（逻辑坐标）
    pub horizontal_x: Option<i32>,
    /// 横向布局记忆位置 Y（逻辑坐标）
    pub horizontal_y: Option<i32>,
    /// 横向布局记忆宽度（逻辑像素，高度自适应不记忆）
    pub horizontal_w: Option<i32>,
    /// 横向布局高度缓存（逻辑像素）。
    /// 仅作为下次启动的初始高度，避免"先高后自适应"的闪烁与位移；
    /// 运行期高度仍由内容行数决定。
    pub horizontal_h: Option<i32>,
    /// 悬浮窗透明度百分比（20~100，默认 85；前端 CSS 级实现，None = 默认 85）。
    /// 存整数避免 f32 序列化出 0.44999998807907104 之类的脏值。
    pub opacity: Option<u8>,
    /// 悬浮窗是否置顶（None = 置顶）
    pub always_on_top: Option<bool>,
    /// 按钮长按触发回车的阈值（毫秒，200~5000，None = 默认 1000）。
    /// 按住超过该时长补发回车；范围内松开仅输入不回车。
    pub hold_threshold_ms: Option<u32>,
    /// 边缘吸附记忆：按进程名记忆每个应用各布局的吸附边（可选）。
    /// 键为进程名（如 "notepad.exe"，大小写不敏感匹配），值为该应用
    /// 横/竖布局各自记忆的吸附边。与画像表独立，任意前台应用均可记忆，
    /// 无需建立画像。TOML 形如：
    /// `[overlay.snap_memory."notepad.exe"]` → `horizontal = "win-bottom"`。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snap_memory: Option<std::collections::HashMap<String, SnapEdgeSettings>>,
}

/// 单个应用的吸附边记忆（按布局分别记录）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct SnapEdgeSettings {
    /// 横向布局吸附边（可选，值见 `OverlaySettings::SNAP_EDGES`）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub horizontal: Option<String>,
    /// 竖向布局吸附边（可选）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vertical: Option<String>,
    /// 横向布局沿边方向偏移比例（0.0~1.0；None = 0.5 居中）。
    /// 垂直边沿水平方向、水平边沿垂直方向插值定位。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub horizontal_offset: Option<f64>,
    /// 竖向布局沿边方向偏移比例（0.0~1.0；None = 0.5 居中）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vertical_offset: Option<f64>,
}

impl OverlaySettings {
    pub const LAYOUT_VERTICAL: &'static str = "vertical";
    pub const LAYOUT_HORIZONTAL: &'static str = "horizontal";

    /// 竖向布局默认尺寸（宽高均可拖动调整）
    pub const VERTICAL_DEFAULT_SIZE: (i32, i32) = (300, 400);
    /// 横向布局默认宽度与单行高度初值
    /// （单行高度与前端渲染一致约 56；有缓存时优先用缓存，见 horizontal_h）
    pub const HORIZONTAL_DEFAULT_W: i32 = 720;
    pub const HORIZONTAL_ROW_H: i32 = 56;

    /// 生效布局（空值视为竖向默认）
    pub fn effective_layout(&self) -> &'static str {
        if self.layout == Self::LAYOUT_HORIZONTAL {
            Self::LAYOUT_HORIZONTAL
        } else {
            Self::LAYOUT_VERTICAL
        }
    }

    /// 生效透明度（0.2~1.0）：百分比夹取到 20~100 后换算，默认 85%
    pub fn effective_opacity(&self) -> f32 {
        self.opacity.unwrap_or(85).clamp(20, 100) as f32 / 100.0
    }

    /// 生效置顶状态（None 视为置顶，保持既有行为）
    pub fn effective_always_on_top(&self) -> bool {
        self.always_on_top.unwrap_or(true)
    }

    /// 长按触发阈值下限（毫秒）
    pub const HOLD_THRESHOLD_MIN_MS: u32 = 200;
    /// 长按触发阈值上限（毫秒）
    pub const HOLD_THRESHOLD_MAX_MS: u32 = 5000;
    /// 长按触发阈值默认值（毫秒）
    pub const HOLD_THRESHOLD_DEFAULT_MS: u32 = 1000;

    /// 生效的长按触发阈值（毫秒）：缺省 1000，夹取 200~5000
    pub fn effective_hold_threshold_ms(&self) -> u32 {
        self.hold_threshold_ms
            .unwrap_or(Self::HOLD_THRESHOLD_DEFAULT_MS)
            .clamp(Self::HOLD_THRESHOLD_MIN_MS, Self::HOLD_THRESHOLD_MAX_MS)
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

    /// 读取指定布局的记忆尺寸：
    /// - 竖向：返回记忆的宽高（缺省用默认 300x400）
    /// - 横向：返回记忆宽度（缺省 720）+ 高度缓存（缺省单行初值 64），
    ///   高度仅是启动初值，运行期由前端按行数自适应修正
    pub fn effective_size(&self, layout: &str) -> (i32, i32) {
        if layout == Self::LAYOUT_HORIZONTAL {
            (
                self.horizontal_w.unwrap_or(Self::HORIZONTAL_DEFAULT_W),
                self.horizontal_h.unwrap_or(Self::HORIZONTAL_ROW_H),
            )
        } else {
            (
                self.vertical_w.unwrap_or(Self::VERTICAL_DEFAULT_SIZE.0),
                self.vertical_h.unwrap_or(Self::VERTICAL_DEFAULT_SIZE.1),
            )
        }
    }

    /// 记录指定布局的几何（位置 + 尺寸）。
    /// 横向布局的高度作为启动缓存保存（运行期仍自适应）。
    pub fn set_geometry(&mut self, layout: &str, x: i32, y: i32, w: i32, h: i32) {
        if layout == Self::LAYOUT_HORIZONTAL {
            self.horizontal_x = Some(x);
            self.horizontal_y = Some(y);
            self.horizontal_w = Some(w);
            self.horizontal_h = Some(h);
        } else {
            self.vertical_x = Some(x);
            self.vertical_y = Some(y);
            self.vertical_w = Some(w);
            self.vertical_h = Some(h);
        }
    }

    // ---- 边缘吸附记忆 ----

    /// 合法吸附边集合：屏幕边（screen-*，目标所在显示器工作区边缘）与
    /// 应用窗口边（win-*，窗口化目标的外侧边缘）
    pub const SNAP_EDGES: [&'static str; 8] = [
        "screen-left",
        "screen-right",
        "screen-top",
        "screen-bottom",
        "win-left",
        "win-right",
        "win-top",
        "win-bottom",
    ];

    /// 判断是否为合法吸附边
    pub fn is_valid_snap_edge(edge: &str) -> bool {
        Self::SNAP_EDGES.contains(&edge)
    }

    /// 读取指定进程在某布局的吸附边记忆（进程名大小写不敏感）
    pub fn snap_edge(&self, process: &str, layout: &str) -> Option<String> {
        let mem = self.snap_memory.as_ref()?;
        let entry = mem
            .iter()
            .find(|(k, _)| k.eq_ignore_ascii_case(process))
            .map(|(_, v)| v)?;
        if layout == Self::LAYOUT_HORIZONTAL {
            entry.horizontal.clone()
        } else {
            entry.vertical.clone()
        }
    }

    /// 读取指定进程在某布局的沿边偏移比例（无记忆返回 None）
    pub fn snap_offset(&self, process: &str, layout: &str) -> Option<f64> {
        let mem = self.snap_memory.as_ref()?;
        let entry = mem
            .iter()
            .find(|(k, _)| k.eq_ignore_ascii_case(process))
            .map(|(_, v)| v)?;
        if layout == Self::LAYOUT_HORIZONTAL {
            entry.horizontal_offset
        } else {
            entry.vertical_offset
        }
    }

    /// 读取指定进程在某布局的吸附边与沿边偏移比例。
    /// 返回 (边名, 偏移)；无偏移字段时偏移回退 0.5（居中，兼容旧配置）。
    pub fn snap_edge_offset(&self, process: &str, layout: &str) -> Option<(String, f64)> {
        let edge = self.snap_edge(process, layout)?;
        let offset = self.snap_offset(process, layout).unwrap_or(0.5);
        Some((edge, offset))
    }

    /// 记录/清除指定进程在某布局的吸附边与沿边偏移记忆。
    /// edge 为 None 时清除该布局的记忆（含偏移）；清除后条目两布局均为空
    /// 则删除整条，避免配置残留空表。
    pub fn set_snap_edge(&mut self, process: &str, layout: &str, edge: Option<(&str, f64)>) {
        let mem = self.snap_memory.get_or_insert_with(Default::default);
        // 大小写不敏感：命中已有键则复用，避免同一进程多条目
        let key = mem
            .keys()
            .find(|k| k.eq_ignore_ascii_case(process))
            .cloned()
            .unwrap_or_else(|| process.to_string());
        let entry = mem.entry(key).or_default();
        match edge {
            Some((e, off)) => {
                if layout == Self::LAYOUT_HORIZONTAL {
                    entry.horizontal = Some(e.to_string());
                    entry.horizontal_offset = Some(off);
                } else {
                    entry.vertical = Some(e.to_string());
                    entry.vertical_offset = Some(off);
                }
            }
            None => {
                if layout == Self::LAYOUT_HORIZONTAL {
                    entry.horizontal = None;
                    entry.horizontal_offset = None;
                } else {
                    entry.vertical = None;
                    entry.vertical_offset = None;
                }
            }
        }
        if entry.horizontal.is_none() && entry.vertical.is_none() {
            // 条目已空：移除（key 已被 entry 独占借用结束，重新查找）
            mem.retain(|k, v| !(k.eq_ignore_ascii_case(process) && v.horizontal.is_none() && v.vertical.is_none()));
        }
    }
}

/// 顶层配置文件
///
/// 包含默认按钮列表与可选的按应用配置画像。
/// 当焦点进程无匹配的 `AppProfile` 时，回退顺序：`default_buttons`（默认映射，
/// 若配置了非空按钮）→ `buttons` 默认列表。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ConfigFile {
    /// 默认配置的按钮列表（可选，默认为空）
    #[serde(default)]
    pub buttons: Vec<ButtonConfig>,
    /// 默认映射的按钮列表：未匹配任何应用画像时使用（可选，空则回退 `buttons`）
    #[serde(default)]
    pub default_buttons: Vec<ButtonConfig>,
    /// 按应用配置画像列表（可选，默认为空）
    #[serde(default)]
    pub profiles: Vec<AppProfile>,
    /// 默认注入模式（可选）：无匹配画像时使用（管 default_buttons/buttons 场景）。
    /// 命中画像时以画像自身的 inject_mode 为准，不继承此值。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_inject_mode: Option<String>,
    /// 悬浮窗设置（可选）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overlay: Option<OverlaySettings>,
    /// 快捷键设置（可选）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shortcuts: Option<ShortcutSettings>,
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

            validate_inject_mode(
                &format!("profiles[{}].inject_mode", profile.process_name),
                &profile.inject_mode,
            )?;

            // 校验画像内按钮 ID 唯一（groups[].buttons 与 buttons 共用同一集合，跨组唯一）
            let mut p_seen: Vec<&str> = Vec::new();

            // 分组名校验（非空、大小写不敏感唯一）
            let mut group_names: Vec<&str> = Vec::new();
            for group in &profile.groups {
                let gname = group.name.trim();
                if gname.is_empty() {
                    return Err(ValidationError {
                        field: format!("profiles[{}].groups[].name", profile.process_name),
                        message: "分组名不能为空".into(),
                    });
                }
                if group_names
                    .iter()
                    .any(|s| s.eq_ignore_ascii_case(gname))
                {
                    return Err(ValidationError {
                        field: format!("profiles[{}].groups[].name", profile.process_name),
                        message: format!("分组名 '{}' 重复", group.name),
                    });
                }
                group_names.push(gname);
                // 组内按钮：id 非空 + 与画像全部按钮（含平铺）跨组唯一
                for btn in &group.buttons {
                    if btn.id.trim().is_empty() {
                        return Err(ValidationError {
                            field: format!("profiles[{}].groups[].buttons[].id", profile.process_name),
                            message: "按钮 ID 不能为空".into(),
                        });
                    }
                    if p_seen.iter().any(|s| **s == btn.id) {
                        return Err(ValidationError {
                            field: format!("profiles[{}].groups[].buttons[].id", profile.process_name),
                            message: format!("按钮 ID '{}' 重复", btn.id),
                        });
                    }
                    p_seen.push(&btn.id);
                }
            }

            // 校验画像内按钮 ID 唯一（平铺按钮；与分组按钮共用 p_seen 防跨组重复）
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

        // 校验默认映射按钮 ID 唯一
        let mut d_seen: Vec<&str> = Vec::new();
        for btn in &self.default_buttons {
            if btn.id.trim().is_empty() {
                return Err(ValidationError {
                    field: "default_buttons[].id".into(),
                    message: "按钮 ID 不能为空".into(),
                });
            }
            if btn.label.trim().is_empty() {
                return Err(ValidationError {
                    field: "default_buttons[].label".into(),
                    message: format!("按钮 '{}' 的标签不能为空", btn.id),
                });
            }
            if btn.content.trim().is_empty() {
                return Err(ValidationError {
                    field: "default_buttons[].content".into(),
                    message: format!("按钮 '{}' 的内容不能为空", btn.id),
                });
            }
            if d_seen.iter().any(|s| **s == btn.id) {
                return Err(ValidationError {
                    field: "default_buttons[].id".into(),
                    message: format!("按钮 ID '{}' 重复", btn.id),
                });
            }
            d_seen.push(&btn.id);
        }

        // 校验默认注入模式
        validate_inject_mode("default_inject_mode", &self.default_inject_mode)?;

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
            if let Some(ms) = overlay.hold_threshold_ms {
                if !(OverlaySettings::HOLD_THRESHOLD_MIN_MS..=OverlaySettings::HOLD_THRESHOLD_MAX_MS)
                    .contains(&ms)
                {
                    return Err(ValidationError {
                        field: "overlay.hold_threshold_ms".into(),
                        message: format!(
                            "长按触发时间 {ms}ms 超出范围（{}~{}ms）",
                            OverlaySettings::HOLD_THRESHOLD_MIN_MS,
                            OverlaySettings::HOLD_THRESHOLD_MAX_MS
                        ),
                    });
                }
            }
            // 校验吸附记忆的边值合法性与沿边偏移范围
            if let Some(mem) = &overlay.snap_memory {
                for (process, edges) in mem {
                    for (layout_name, edge, offset) in [
                        ("horizontal", &edges.horizontal, &edges.horizontal_offset),
                        ("vertical", &edges.vertical, &edges.vertical_offset),
                    ] {
                        if let Some(e) = edge {
                            if !OverlaySettings::is_valid_snap_edge(e) {
                                return Err(ValidationError {
                                    field: format!("overlay.snap_memory.{process}.{layout_name}"),
                                    message: format!(
                                        "吸附边 '{e}' 无效（应为 {} 之一）",
                                        OverlaySettings::SNAP_EDGES.join(" / ")
                                    ),
                                });
                            }
                        }
                        if let Some(off) = offset {
                            if !(0.0..=1.0).contains(off) {
                                return Err(ValidationError {
                                    field: format!("overlay.snap_memory.{process}.{layout_name}_offset"),
                                    message: format!("沿边偏移比例 {off} 超出范围（0.0~1.0）"),
                                });
                            }
                        }
                    }
                }
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

    /// 获取当前进程的按钮列表（优先匹配进程，无匹配回退默认映射/默认按钮）
    ///
    /// S103 兜底逻辑：调用 [`ConfigFile::get_buttons_for_process`] 优先按进程匹配；
    /// 若匹配成功返回该进程的按钮集，否则回退顺序：
    /// 1. `default_buttons`（默认映射，配置了非空按钮时）
    /// 2. `buttons` 默认列表
    /// 返回切片引用，永不返回 `None`。
    pub fn get_buttons_current(&self, process_name: &str) -> &[ButtonConfig] {
        if let Some(btns) = self.get_buttons_for_process(process_name) {
            return btns;
        }
        if !self.default_buttons.is_empty() {
            return &self.default_buttons;
        }
        &self.buttons
    }

    /// 查询当前进程生效的注入模式（与 get_buttons_current 同一匹配/回退逻辑）
    ///
    /// - 命中画像：返回画像自身的 `inject_mode`（未配置视为 paste）
    /// - 未命中：返回 `default_inject_mode`（未配置视为 paste）
    pub fn inject_mode_for_process(&self, process_name: &str) -> &'static str {
        if let Some(p) = self
            .profiles
            .iter()
            .find(|p| p.process_name.eq_ignore_ascii_case(process_name))
        {
            return p.effective_inject_mode();
        }
        if self.default_inject_mode.as_deref() == Some(INJECT_MODE_KEYSTROKE) {
            INJECT_MODE_KEYSTROKE
        } else {
            INJECT_MODE_PASTE
        }
    }
}