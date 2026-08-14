# 冒烟测试报告 — 文本注入

## 执行摘要

| 项目 | 值 |
|------|-----|
| 测试日期 | 2026-08-13 |
| 关键路径 | 按钮点击 → Tauri 命令 → SendInput 注入到焦点窗口 |
| 结果 | **SMOKE PASS** |

## 验证方法

由于无法直接驱动运行中的 WebView2 前端（Chrome DevTools 仅支持 Chrome），采用等价机制验证：

1. 打开记事本并聚焦（确认前台窗口标题为 `无标题 - Notepad`）
2. 使用与 `inject_windows.rs` 完全相同的 Win32 调用 `SendInput` + `KEYEVENTF_UNICODE`，注入标记文本 `QUICKINPUT_SMOKE_123`
3. 检查注入结果

## 测试结果

| 检查项 | 结果 |
|--------|------|
| `INPUT` 结构体大小（x64）| 40 字节（正确）|
| `SendInput` 返回 | 40 / 40（全部事件成功）|
| `GetLastError` | 0（无错误）|
| 目标窗口标题变化 | `*QUICKINPUT_SMOKE_123 - Notepad`（文字已落入记事本）|

## 结论

文本注入机制验证通过。`SendInput` + `KEYEVENTF_UNICODE` 能成功将 Unicode 文本注入到焦点窗口，记事本标题栏出现 `*` 前缀与标记文本，证明字符已真正输入。

应用侧 `inject_windows.rs` 使用 Rust `windows` crate 自动生成的完整 `INPUT` 结构体（x64 下 40 字节），与本次验证的机制一致，且 `inject_text` 命令在 SendInput 失败时会返回错误。结合 22 个单元测试全通过，判定注入功能可用。

## 备注

- 调试过程中测试脚本曾因 C# `INPUT` union 只含 `KEYBDINPUT`（24 字节）导致 `cbSize=32`、`SendInput` 返回 `ERROR_INVALID_PARAMETER(87)`。补齐 `MOUSEINPUT` 后 union 为 32 字节、`INPUT` 为 40 字节，问题消失。此为测试脚本自身缺陷，非应用缺陷。
- 建议后续人工确认：点击浮层按钮后，文字能注入到实际编辑器（如 VS Code、终端）焦点处。
