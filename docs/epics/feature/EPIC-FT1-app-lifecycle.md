---
状态: 草案
作者: zls3434
更新日期: 2026-08-10
层级: feature
对应模块: M6
---

# EPIC-FT1: 应用生命周期管理

## 概述

实现 QuickInput 的应用生命周期管理能力，包括开机自启、系统托盘图标、全局热键呼出/隐藏等增强体验功能。属于 P2 优先级，在 Foundation 与 Core 层完成后开发。

## 业务价值

- 支撑"开机自启"（N8），确保工具常驻可用
- 系统托盘图标提供最小化/退出控制
- 全局热键呼出/隐藏浮层，减少屏幕占用

## 验收标准

- [ ] AC1: 应用支持开机自启（系统启动时自动运行）
- [ ] AC2: 系统托盘显示图标，右键菜单包含"显示/隐藏""退出"选项
- [ ] AC3: 支持全局热键呼出/隐藏浮层（默认 `Ctrl+Shift+Space`）
- [ ] AC4: 托盘图标退出时确认关闭

## 范围

- 开机自启注册/取消（各平台实现）
- 系统托盘图标与菜单
- 全局热键注册与处理
- **不含**：配置管理（M4）、浮层窗口（M1）具体行为（仅控制显示/隐藏）

## 技术要点

- Windows：注册表 `HKCU\Software\Microsoft\Windows\CurrentVersion\Run` 或启动文件夹
- macOS：`LaunchAgent` plist
- Linux：`XDG Autostart` `.desktop` 文件
- 全局热键：Tauri 插件 `tauri-plugin-global-shortcut`