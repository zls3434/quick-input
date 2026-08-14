---
状态: 草案
作者: zls3434
更新日期: 2026-08-12
所属 Epic: EPIC-FT1
对应模块: M6
优先级: P2
依赖: S160
---

# S161 用户故事：全局热键呼出/隐藏

**用户故事**：作为用户，我希望通过全局快捷键呼出/隐藏浮层，以便快速访问工具，无需鼠标操作。

## 验收标准（Given-When-Then）

- **AC9-1**：Given 应用运行中，When 按下 `Ctrl+Shift+Space`，Then 浮层窗口切换显示/隐藏（显示时不抢焦点）。
- **AC9-2**：Given 浮层已显示，When 再次按下热键，Then 浮层隐藏。
- **AC9-3**：Given 热键注册，When 应用退出，Then 热键自动注销（无残留）。
- **AC9-4**：Given 热键冲突，When 注册失败，Then 输出错误日志，不影响应用其他功能。

## 任务清单

1. 在 `Cargo.toml` 添加 `tauri-plugin-global-shortcut` 依赖
2. 在 `lib.rs` 注册插件
3. 在 `setup()` 中注册 `Ctrl+Shift+Space` 热键
4. 热键回调中切换浮层可见性（复用 S160 `toggle_overlay`）
5. 编写单元测试验证热键注册与注销

## 关联需求

- N8 开机自启（热键是常驻工具的必要入口）
- AC3 全局热键呼出/隐藏浮层

## 技术路由

- 后端 Rust 程序员：Tauri GlobalShortcut 插件
- 依赖 S160 托盘（热键切换复用 `toggle_overlay`）

## 估算

- 1 人日（小）