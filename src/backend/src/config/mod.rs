//! QuickInput 配置模块 — 定义 TOML 配置的数据结构与校验规则
//!
//! 本模块定义了 `ConfigFile`、`ButtonConfig`、`AppProfile` 三个核心数据结构，
//! 使用 `serde` 实现序列化/反序列化，支持必填字段校验与可选字段默认值。
//! 并提供 `ConfigManager` 进行配置文件的加载、保存、默认生成。

pub mod defaults;
pub mod manager;
pub mod model;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_manager;

pub use manager::*;
pub use model::*;