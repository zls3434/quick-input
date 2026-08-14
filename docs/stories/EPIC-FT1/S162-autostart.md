---
状态: 草案
作者: zls3434
更新日期: 2026-08-13
所属 Epic: EPIC-FT1
对应模块: M6
优先级: P2
依赖: 无
---

# S162 用户故事：开机自启

**用户故事**：作为用户，我希望应用能随系统启动自动运行，以便工具常驻可用。

## 验收标准（Given-When-Then）

- **AC10-1**：Given 应用安装完成，When 启动，Then 默认注册开机自启（用户可配置）。
- **AC10-2**：Given 开机自启已注册，When 卸载应用，Then 自启注册自动清理（无残留）。
- **AC10-3**：Given 开机自启功能，When 调用接口，Then 可查询当前启用/禁用状态。
- **AC10-4**：Given 开机自启注册失败，When 启动，Then 输出错误日志，不影响应用运行。

## 任务清单

1. 在 `Cargo.toml` 添加 `tauri-plugin-autostart` 依赖
2. 在 `lib.rs` 注册插件
3. 在 `setup()` 中启用开机自启
4. 暴露 Tauri 命令 `is_autostart_enabled` / `toggle_autostart`
5. 编写单元测试验证自启注册与状态查询

## 关联需求

- N8 开机自启确保工具常驻

## 技术路由

- 后端 Rust 程序员：Tauri Autostart 插件
- 独立模块，无依赖

## 估算

- 1 人日（小）