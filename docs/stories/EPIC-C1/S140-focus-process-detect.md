---
状态: 草案
作者: zls3434
更新日期: 2026-08-12
所属 Epic: EPIC-C1
对应模块: M3
优先级: P1
依赖: 无
---

# S140 用户故事：焦点进程识别

**用户故事**：作为用户，我希望工具识别当前焦点窗口所属进程，以便后续按应用自动切换配置。

## 验收标准（Given-When-Then）

- **AC5-1**：Given 当前前台窗口属于 `Code.exe`，When 调用识别接口，Then 返回 `Code.exe`（含进程名）。
- **AC5-2**：Given 当前前台窗口属于 `WindowsTerminal.exe`，When 调用识别接口，Then 返回 `WindowsTerminal.exe`。
- **AC5-3**：Given 返回的进程名，When 与 M4 配置匹配，Then 大小写不敏感（进程名统一提取为含 `.exe` 的完整名称）。
- **AC5-4**：Given 无法获取前台窗口（罕见情况下系统异常），When 识别失败，Then 返回明确错误而非 panic。

## 任务清单

1. 新建 `focus_detector.rs` 模块，实现 `current_foreground_process() -> Result<String, FocusDetectError>`
2. Windows 实现：`GetForegroundWindow()` → `GetWindowThreadProcessId()` → `QueryFullProcessImageName()` 提取进程名
3. 定义 `FocusDetectError` 错误类型（目标窗口缺失等）
4. 进程名统一含 `.exe` 后缀，与配置 `process_name` 匹配约定一致
5. 编写单元测试覆盖识别逻辑

## 关联需求

- V3 按应用自动切换配置
- N6 按当前焦点进程/窗口自动切换配置文件

## 技术路由

- 后端 Rust 程序员：Win32 焦点窗口 → 进程名提取
- 依赖 S102/S103 已有配置匹配接口

## 估算

- 2 人日（中）
