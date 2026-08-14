---
状态: 草案
作者: zls3434
更新日期: 2026-08-12
所属 Epic: EPIC-C1
对应模块: M3, M4, M5
优先级: P1
依赖: S140, S141, S103
---

# S142 用户故事：配置自动切换

**用户故事**：作为用户，我希望焦点窗口切换时按钮列表自动更新，以便在不同应用看到对应按钮集。

## 验收标准（Given-When-Then）

- **AC7-1**：Given 焦点从应用 A 切换到应用 B，When B 在配置中有对应的 `[[profiles]]`，Then 按钮列表自动切换为 B 的按钮集。
- **AC7-2**：Given 焦点切换到无匹配配置的应用，When 切换，Then 按钮列表回退为默认按钮。
- **AC7-3**：Given 焦点在同一应用内切换（如两个 Code 窗口），When 切换，Then 不重复发射事件（避免无谓刷新）。
- **AC7-4**：Given 焦点切换且配置不同，When 切换，Then 发射 `ConfigSwitched` 事件，前端收到后刷新按钮列表。
- **AC7-5**：Given 焦点监听线程运行中，When 应用退出，Then 线程安全终止。

## 任务清单

1. 修改 `AppState`：添加 `current_process` 字段记录当前活动进程名
2. 修改 `get_buttons`：使用 `get_buttons_current(process_name)` 按进程返回按钮
3. 在 `setup()` 中启动 `FocusWatcher` 并保存到 `AppState`
4. 启动消费线程监听前台变化，更新 `current_process` 并发射 `ConfigSwitched`
5. 移除 `focus_detector.rs` 和 `focus_watcher.rs` 的 `#![allow(dead_code)]`
6. 编写集成测试验证焦点切换 → 配置切换流程

## 关联需求

- V3 按应用自动切换配置
- N6 按当前焦点进程/窗口自动切换配置文件

## 技术路由

- 后端 Rust 程序员：S140 + S141 集成 + Tauri 事件发射
- 依赖 S140 进程识别、S141 焦点监听、S103 配置映射

## 估算

- 1 人日（小）