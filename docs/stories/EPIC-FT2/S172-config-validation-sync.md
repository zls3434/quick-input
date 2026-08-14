---
状态: 草案
作者: zls3434
更新日期: 2026-08-13
所属 Epic: EPIC-FT2
对应模块: M7, M4
优先级: P2
依赖: S170, S171
---

# S172 用户故事：配置校验、保存与同步

**用户故事**：作为用户，我希望配置修改后能自动校验并保存，确保配置不失效，且主浮层同步刷新。

## 验收标准（Given-When-Then）

- **AC13-1**：Given 配置修改，When 保存，Then 后端统一校验（ID 唯一性、必填字段、进程名格式）后再写入。
- **AC13-2**：Given 校验失败，When 保存，Then 返回明确错误信息，配置不写入。
- **AC13-3**：Given 校验通过，When 保存，Then 原子写入 TOML 文件（不损坏已有配置）。
- **AC13-4**：Given 保存成功，When 完成，Then 发射 `ConfigSwitched` 事件，主浮层自动刷新。

## 任务清单

1. 后端添加 `validate_config` 校验函数（检查按钮 ID 唯一性、必填字段、进程名格式）
2. 添加 `validate_config` Tauri 命令暴露给前端
3. 在 `add_button`、`update_button`、`add_profile`、`update_profile` 中调用校验
4. 前端设置页面在保存前调用校验，显示错误信息
5. 确认 `ConfigSwitched` 在所有保存路径上正确发射
6. 编写单元测试覆盖校验逻辑

## 关联需求

- AC4 修改后自动保存到 TOML 配置文件

## 技术路由

- 后端 Rust 程序员：配置校验函数 + 集成到 CRUD 命令
- 前端 Svelte 程序员：校验错误展示
- 依赖 S170/S171 的 CRUD 命令

## 估算

- 1 人日（小）