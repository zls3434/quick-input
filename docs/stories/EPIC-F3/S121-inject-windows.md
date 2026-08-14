---
状态: 草案
作者: zls3434
更新日期: 2026-08-10
所属 Epic: EPIC-F3
对应模块: M2
优先级: P0
依赖: S120
---

# S121 用户故事：Windows 文本注入实现

**用户故事**：作为用户，我希望在 Windows 上单击按钮能把文本注入焦点输入框且不改变焦点，以便快速输入预设内容。

## 验收标准（Given-When-Then）

- **AC2-1**：Given 当前焦点在输入框，When 调用注入，Then 文本被输入到该输入框。
- **AC2-2**：Given 注入文本含英文字母、数字、符号，When 注入，Then 全部正确输入。
- **AC2-3**：Given 注入文本含中文等多字节字符，When 注入，Then 全部正确输入。
- **AC2-4**：Given 注入过程中，When 焦点变化，Then 注入到目标输入框而非新焦点窗口（注入前记录目标）。
- **AC2-5**：Given 注入后，When 检查焦点，Then 焦点仍在原输入框。
- **AC2-6**：Given 目标窗口为高完整性级别（如管理员终端），When 注入，Then 处理 UIPI 限制（返回明确错误或降级方案）。

## 任务清单

1. 实现 `WindowsInjector` struct，实现 `Injector` trait
2. 使用 `SendInput` + `KEYEVENTF_UNICODE` 逐字符注入 UTF-16
3. 实现注入前记录焦点句柄（`GetForegroundWindow`），注入后恢复
4. 处理 UIPI 权限：检测 `Target::accepts_injection`，失败时返回可读错误
5. 处理修饰键冲突（用户按着 Ctrl/Alt 时，先释放再注入）
6. 编写单元测试与手动验证

## 关联需求

- V1 即点即输，不打断输入焦点
- N2 单击按钮即输入到当前焦点输入框
- TR1 单击注入不改变焦点

## 技术路由

- 后端 Rust 程序员：Win32 `SendInput` 实现
- 依赖 S120 接口定义

## 估算

- 3 人日（大）