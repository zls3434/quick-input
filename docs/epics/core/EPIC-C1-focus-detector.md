---
状态: 草案
作者: zls3434
更新日期: 2026-08-10
层级: core
对应模块: M3
---

# EPIC-C1: 焦点感知与配置切换

## 概述

实现 QuickInput 的焦点感知能力——识别当前焦点窗口所属进程，自动加载并切换至对应配置。"按当前应用自动切换配置文件"（V3）是 QuickInput 的核心差异化优势。

## 业务价值

- 支撑核心价值 V3：按应用自动切换配置，用户"0 配置切换"
- 不同应用（IDE、终端、浏览器、邮件）各自维护专属按钮集，互不干扰
- 焦点切换事件驱动 M5 刷新按钮列表，实现配置无缝切换

## 验收标准

- [ ] AC1: 能正确识别当前焦点窗口所属进程名（如 `Code.exe`、`WindowsTerminal.exe`）
- [ ] AC2: 焦点切换时自动从 M4 加载匹配配置
- [ ] AC3: 无匹配配置时自动回退默认配置
- [ ] AC4: 焦点切换后通知 M5 刷新按钮列表（通过事件总线）
- [ ] AC5: 首期实现 Windows 平台（GetForegroundWindow + GetWindowThreadProcessId）

## 范围

- 焦点进程/窗口识别
- 进程名 → 配置的映射与查询
- 焦点切换事件派发
- **不含**：配置文件的读写（M4）、按钮 UI 渲染（M5）、输入注入（M2）

## 技术要点

- Windows：`GetForegroundWindow()` → `GetWindowThreadProcessId()` → 通过进程 ID 获取进程名
- macOS：`NSWorkspace.frontmostApplication` → `bundleIdentifier` / `CGWindowList`
- Linux：`_NET_ACTIVE_WINDOW` 原子查询 → `xdotool`/`x11rb`
- 轮询 vs 事件监听：Windows 用 `SetWinEventHook(EVENT_SYSTEM_FOREGROUND)` 监听焦点变化，避免轮询