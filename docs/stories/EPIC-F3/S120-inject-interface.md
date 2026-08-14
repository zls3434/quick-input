---
状态: 草案
作者: zls3434
更新日期: 2026-08-10
所属 Epic: EPIC-F3
对应模块: M2
优先级: P0
---

# S120 用户故事：统一跨平台注入接口

**用户故事**：作为开发者，我希望定义统一的跨平台注入接口，以便各平台独立实现。

## 验收标准（Given-When-Then）

- **AC1-1**：Given 注入接口定义，When 设计 trait，Then 包含 `fn inject_text(text: &str) -> Result<(), InjectError>` 方法。
- **AC1-2**：Given 注入接口，When 不同平台实现，Then 各平台 struct 实现同一 trait，通过 Feature Flag 编译时选择。
- **AC1-3**：Given 注入失败，When 调用返回，Then `InjectError` 枚举包含「权限不足」「注入被拒」「未知错误」等变体。
- **AC1-4**：Given 注入接口，When 注入前，Then 自动记录当前焦点句柄，注入后自动恢复焦点。

## 任务清单

1. 定义 `Injector` trait（`fn inject_text(&self, text: &str) -> Result<(), InjectError>`）
2. 定义 `InjectError` 错误枚举
3. 定义 `InjectGuard` 结构体（RAII 风格：在构造时记录焦点，在析构时恢复焦点）
4. 定义 Feature Flag：`#[cfg(windows)]`、`#[cfg(target_os = "macos")]`、`#[cfg(target_os = "linux")]`
5. 编写单元测试验证接口契约

## 关联需求

- V1 即点即输，不打断输入焦点
- N2 单击按钮即输入到当前焦点输入框
- N9 跨平台支持

## 技术路由

- 后端 Rust 程序员：trait 设计与跨平台抽象

## 估算

- 1 人日（小）