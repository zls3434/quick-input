---
状态: 草案
作者: zls3434
更新日期: 2026-08-12
所属 Epic: EPIC-C1
对应模块: M3
优先级: P1
依赖: S140
---

# S141 用户故事：焦点切换监听

**用户故事**：作为用户，我希望工具监听前台窗口变化，以便焦点切换时自动感知并触发配置更新。

## 验收标准（Given-When-Then）

- **AC6-1**：Given 前台窗口从应用 A 切换到应用 B，When 监听开启，Then 收到前台切换通知（含新前台窗口句柄）。
- **AC6-2**：Given 监听运行中，When 无前台变化，Then 不产生任何通知（事件驱动，非轮询）。
- **AC6-3**：Given 监听启动，When 应用退出，Then 监听线程安全终止（无泄漏、无 panic）。
- **AC6-4**：Given 监听回调中调用 S140 识别进程名，When 前台切换，Then 能获取新进程名。

## 任务清单

1. 新建 `focus_watcher.rs` 模块，实现 `FocusWatcher` 结构
2. 使用 `SetWinEventHook(EVENT_SYSTEM_FOREGROUND)` 注册前台切换回调
3. 专用线程运行消息循环（`GetMessageW`/`DispatchMessageW`）
4. 回调中通过 `std::sync::mpsc` 通道发送前台窗口句柄
5. 实现 `stop()` 安全终止（`UnhookWinEvent` + 发送退出消息）
6. 编写单元测试验证监听生命周期

## 关联需求

- V3 按应用自动切换配置
- N6 按当前焦点进程/窗口自动切换配置文件

## 技术路由

- 后端 Rust 程序员：Win32 事件钩子 + 消息循环线程
- 依赖 S140 进程名识别

## 估算

- 2 人日（中）