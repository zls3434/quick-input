---
状态: 评审中
作者: zls3434
更新日期: 2026-08-10
---

# 技术偏好文档

> 本文档记录 QuickInput 项目的技术栈选型与关键技术决策，是后续架构设计与开发的依据。

## 1. 技术栈总览

| 层级 | 选型 | 版本 | 说明 |
|------|------|------|------|
| 桌面框架 | Tauri | 2.x | 跨平台桌面应用框架，Rust 核心 + Web 前端 |
| 后端语言 | Rust | 最新稳定版 | 系统级能力（焦点/窗口/输入注入/置顶） |
| 前端框架 | Svelte | 最新稳定版 | 浮层 UI，体积小、性能优、适合轻量工具 |
| 前端语言 | TypeScript | 5.x | 类型安全 |
| 配置格式 | TOML | ron 兼容 | 配置文件持久化与批量自定义 |
| 目标平台 | Windows / macOS / Linux | — | 跨平台，Windows 为 MVP 首要验证平台 |

## 2. 关键技术决策

### 2.1 置顶浮层且点击不抢焦点

- **Windows**：设置 `WS_EX_NOACTIVATE | WS_EX_TOPMOST | WS_EX_TOOLWINDOW` 扩展样式，`SetWindowPos(HWND_TOPMOST)` 置顶，`ShowWindow(SW_SHOWNOACTIVATE)` + 处理 `WM_MOUSEACTIVATE` 返回 `MA_NOACTIVATE`。
- **macOS**：使用 `NSPanel`（社区插件 `tauri-nspanel`）设置 `.nonactivatingPanel` + `becomesKeyOnlyIfNeeded`，用 `orderFrontRegardless()` 显示。
- **Linux**：X11 下用 `override_redirect` + `_NET_WM_STATE_ABOVE`；Wayland 作为受限降级。

### 2.2 识别当前焦点窗口/进程

- 分平台实现统一抽象接口：Windows `GetForegroundWindow` + `GetWindowThreadProcessId`；macOS `NSWorkspace.frontmostApplication` / `CGWindowList`；Linux `_NET_ACTIVE_WINDOW`（x11rb / xdotool）。
- 提供"未匹配则回退默认配置"的兜底策略。

### 2.3 向焦点输入框注入文本且不改变焦点

- **Windows**：`SendInput` + `KEYEVENTF_UNICODE`（发送 UTF-16 字符），参考 `win-text-inject` crate 处理剪贴板与修饰键冲突。
- **macOS**：`CGEvent` + `CGEventKeyboardSetUnicodeString`（需 Accessibility 权限），参考 `cgevents` crate。
- **Linux**：X11 用 `xdotool type`，Wayland 用 `wtype`（可降级）。

### 2.4 配置持久化与按应用切换

- 使用 TOML 配置文件；支持多配置文件按焦点进程自动加载，默认配置兜底。
- MVP 阶段配置手动编辑即可，可视化配置界面为 P2。

## 3. 技术风险控制

| 风险 | 应对 |
|------|------|
| 注入不改变焦点 | 窗口不激活 + 注入前记录原焦点 + `SendInput`/`CGEvent` 直接注入 |
| 跨平台注入差异 | 抽象统一「注入层」接口，各平台独立实现 |
| macOS 权限 | 首次运行引导授权，检查 `CGPreflightPostEventAccess` |
| Wayland/特殊环境 | 明确不支持范围，XWayland 回退或降级说明 |
| 开机自启差异 | 分平台实现（注册表 / LaunchAgent / 用户自启动） |

## 4. 推荐依赖

见架构阶段细化。核心依赖：`tauri 2`、`raw-window-handle`、`windows 0.58`（Win32_UI 系列）、`objc2`/`cgevents`（macOS）、`x11rb`/`xdotool`（Linux）、`active-win-pos-rs`（跨平台焦点检测）、`toml`（配置）。

## 5. 构建与工具

- 构建系统：`cargo` + Tauri CLI
- 包管理：`cargo`（Rust）/ `npm`（前端）
- 版本控制：Git，主干开发模式
- 配置来源：环境变量 + TOML 文件，密钥禁止硬编码
