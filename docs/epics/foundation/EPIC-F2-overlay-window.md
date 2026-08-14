---
状态: 草案
作者: zls3434
更新日期: 2026-08-10
层级: foundation
对应模块: M1
---

# EPIC-F2: 浮层窗口系统

## 概述

实现 QuickInput 的置顶浮动窗口系统，提供常驻屏幕顶层、不抢焦点、可拖动、可调整大小的系统级窗口。这是按钮面板 UI（M5）的宿主，也是工具的核心交互载体。

## 业务价值

- 支撑"置顶透明浮层，随时可达"（V2）与"置顶浮动层常驻显示"（N1）
- "点击不改变焦点"（N3）是核心差异化，必须在窗口系统层保证
- 提供常驻浮层的技术基础，作为 M5 的渲染宿主

## 验收标准

- [ ] AC1: 浮层窗口常驻屏幕顶层（always-on-top），不随焦点变化隐藏
- [ ] AC2: 点击浮层按钮不改变当前输入焦点（不抢焦点）
- [ ] AC3: 浮层可拖动位置、可调整大小
- [ ] AC4: 支持多显示器环境正确保持置顶
- [ ] AC5: 首期在 Windows 平台验证，架构预留 macOS/Linux 扩展

## 范围

- 置顶窗口创建与行为控制
- 不激活（non-activating）行为
- 拖动与缩放支持
- **不含**：按钮 UI 渲染（M5）、输入注入（M2）、焦点识别（M3）

## 技术要点

- Windows：`WS_EX_NOACTIVATE | WS_EX_TOPMOST | WS_EX_TOOLWINDOW`，`SetWindowPos(HWND_TOPMOST)`，`SW_SHOWNOACTIVATE`
- macOS：`NSPanel` + `.nonactivatingPanel` + `becomesKeyOnlyIfNeeded`
- Linux：`override_redirect` + `_NET_WM_STATE_ABOVE`
- 通过 Tauri 窗口 API + `raw-window-handle` 访问原生句柄