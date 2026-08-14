//! 配置管理器 — 负责配置文件的加载、保存与默认生成

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use super::defaults::default_config;
use super::model::ConfigFile;

/// 配置管理错误类型
#[derive(Debug)]
pub enum ConfigError {
    /// IO 错误（文件不存在、无权限等）
    Io(io::Error),
    /// TOML 解析错误（格式非法）
    Parse(toml::de::Error),
    /// TOML 序列化错误
    Serialize(toml::ser::Error),
    /// 配置文件路径未能确定
    PathNotSet,
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::Io(e) => write!(f, "配置文件 IO 错误: {e}"),
            ConfigError::Parse(e) => write!(f, "配置文件解析错误: {e}"),
            ConfigError::Serialize(e) => write!(f, "配置文件序列化错误: {e}"),
            ConfigError::PathNotSet => write!(f, "配置文件路径未设置"),
        }
    }
}

impl std::error::Error for ConfigError {}

impl From<io::Error> for ConfigError {
    fn from(e: io::Error) -> Self {
        ConfigError::Io(e)
    }
}

impl From<toml::de::Error> for ConfigError {
    fn from(e: toml::de::Error) -> Self {
        ConfigError::Parse(e)
    }
}

impl From<toml::ser::Error> for ConfigError {
    fn from(e: toml::ser::Error) -> Self {
        ConfigError::Serialize(e)
    }
}

/// 配置管理器
///
/// 负责从配置目录加载 `default.toml`、原子写入保存、并在首次启动时生成默认配置。
/// 配置目录路径由调用方注入，保持库的跨平台与可测试性。
pub struct ConfigManager {
    /// 配置目录
    config_dir: PathBuf,
    /// 内存中当前配置
    config: ConfigFile,
}

impl ConfigManager {
    /// 创建配置管理器
    ///
    /// 初始化一个空配置（尚未从磁盘加载）。调用 [`ConfigManager::load`] 或
    /// [`ConfigManager::ensure_default`] 后才会填充实际配置。
    pub fn new(config_dir: &Path) -> Self {
        Self {
            config_dir: config_dir.to_path_buf(),
            config: ConfigFile::default(),
        }
    }

    /// 配置文件完整路径
    pub fn config_path(&self) -> PathBuf {
        self.config_dir.join("default.toml")
    }

    /// 从配置目录加载 `default.toml`
    ///
    /// 若文件不存在则返回 [`ConfigError::Io`]（NotFound），调用方可据此触发默认生成。
    pub fn load(&mut self) -> Result<(), ConfigError> {
        let path = self.config_path();
        let content = fs::read_to_string(&path).map_err(ConfigError::Io)?;
        self.config = toml::from_str(&content)?;
        Ok(())
    }

    /// 原子写入当前配置到 `default.toml`
    ///
    /// 采用「写入临时文件 → 重命名覆盖」策略，避免写坏已有配置（最小化写入）。
    #[allow(clippy::needless_return)]
    pub fn save(&self) -> Result<(), ConfigError> {
        let path = self.config_path();
        let toml_str = toml::to_string(&self.config)?;

        // 确保配置目录存在
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(ConfigError::Io)?;
        }

        // 原子写：先写临时文件，再重命名覆盖
        let tmp_path = path.with_extension("toml.tmp");
        fs::write(&tmp_path, &toml_str).map_err(ConfigError::Io)?;
        fs::rename(&tmp_path, &path).map_err(ConfigError::Io)?;
        Ok(())
    }

    /// 确保配置文件存在，首次启动时生成默认样例
    ///
    /// 若 `default.toml` 不存在，则用 [`default_config`] 生成并写入。
    /// 返回配置文件的完整路径。
    pub fn ensure_default(&self) -> Result<PathBuf, ConfigError> {
        let path = self.config_path();
        if !path.exists() {
            let default_cfg = default_config();
            let toml_str = toml::to_string(&default_cfg)?;
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent).map_err(ConfigError::Io)?;
            }
            let tmp_path = path.with_extension("toml.tmp");
            fs::write(&tmp_path, &toml_str).map_err(ConfigError::Io)?;
            fs::rename(&tmp_path, &path).map_err(ConfigError::Io)?;
        }
        Ok(path)
    }

    /// 加载配置；若文件不存在则先创建默认配置再加载
    ///
    /// 组合 [`ConfigManager::ensure_default`] 与 [`ConfigManager::load`]，
    /// 为调用方提供"一劳永逸"的启动加载入口。
    pub fn load_or_init(&mut self) -> Result<(), ConfigError> {
        self.ensure_default()?;
        self.load()
    }

    /// 访问当前配置（只读）
    pub fn config(&self) -> &ConfigFile {
        &self.config
    }

    /// 修改当前配置（可变引用）
    pub fn config_mut(&mut self) -> &mut ConfigFile {
        &mut self.config
    }
}