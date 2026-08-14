---
状态: 草案
作者: zls3434
更新日期: 2026-08-10
层级: foundation
对应模块: M2
---

# EPIC-F3: 输入注入引擎

## 概述

实现 QuickInput 的核心能力——跨平台文本注入。将按钮预设内容注入到当前焦点输入框，且不改变输入焦点。是核心价值"即点即输，不打断输入焦点"（V1）的直接实现。

## 业务价值

- 支撑核心价值 V1：单击即向焦点输入框注入预设内容
- "不改变焦点"是核心差异化，注入引擎必须保证
- 跨平台注入一致性是产品跨平台能力（N9）的关键

## 验收标准

- [ ] AC1: 能将预设文本注入到当前焦点输入框
- [ ] AC2: 注入过程不改变当前输入焦点
- [ ] AC3: 支持中文等多字节字符（Unicode）注入
- [ ] AC4: 注入对普通输入框、终端、文本编辑器等均可用
- [ ] AC5: 首期实现 Windows 平台（SendInput），预留 macOS(MacOS)/Linux 扩展接口

## 范围

- 文本注入到焦点输入框
- 注入时焦点保持
- 跨平台注入接口抽象与各平台实现
- **不含**：按钮 UI、配置读取、焦点识别（由其他 Epic 负责）

## 技术要点

- Windows：`SendInput` + `KEYEVENTF_UNICODE`（UTF-16 字符注入）
- macOS：`CGEvent` + `CGEventKeyboardSetUnicodeString`（需 Accessibility 权限，参考 `cgevents`）
- Linux：`xdotool type`（X11）/ `wtype`（Wayland）
- 抽象统一注入接口，各平台独立实现，Feature 开关控制
- 注意 UIPI 权限（Windows 高完整性窗口注入限制）