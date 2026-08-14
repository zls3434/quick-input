---
状态: 草案
作者: zls3434
更新日期: 2026-08-10
所属 Epic: EPIC-F2
对应模块: M1
优先级: P0
---

# S110 用户故事：置顶浮层窗口创建

**用户故事**：作为用户，我希望浮层窗口常驻屏幕顶层，以便随时访问快捷按钮。

## 验收标准（Given-When-Then）

- **AC1-1**：Given 应用启动，When 创建浮层窗口，Then 窗口出现在屏幕顶层（always-on-top），不随其他窗口切换而隐藏。
- **AC1-2**：Given 浮层窗口已创建，When 切换到其他应用，Then 浮层仍保持在顶层可见。
- **AC1-3**：Given 浮层窗口，When 窗口尺寸设置，Then 默认宽高适中（如 300x400），可调整。
- **AC1-4**：Given 多显示器环境，When 浮层显示，Then 在主显示器上正确置顶。
- **AC1-5**：Given 首期实现，When 平台为 Windows，Then 置顶功能正常（macOS/Linux 预留扩展点）。

## 任务清单

1. 使用 Tauri API 创建无边框窗口，设置 `always_on_top: true`
2. 通过 `raw-window-handle` 获取 HWND，调用 Win32 API 确保正确置顶（`SetWindowPos(HWND_TOPMOST)`）
3. 设置窗口默认尺寸与初始位置（屏幕右下角或居中）
4. 实现窗口创建/销毁生命周期管理
5. 编写集成测试验证窗口置顶行为

## 关联需求

- V2 置顶透明浮层，随时可达
- N1 置顶浮动层常驻显示

## 技术路由

- 后端 Rust 程序员：Tauri 窗口 API + Win32 置顶
- 前端 Svelte：仅作为窗口宿主，无 UI 渲染

## 估算

- 2 人日（中）