---
状态: 草案
作者: zls3434
更新日期: 2026-08-10
所属 Epic: EPIC-F3
对应模块: M2
优先级: P0
依赖: S121
---

# S122 用户故事：Unicode 支持与跨平台扩展

**用户故事**：作为用户，我希望注入支持中文等多字节字符并预留 macOS/Linux 扩展，以便跨平台与中文输入。

## 验收标准（Given-When-Then）

- **AC3-1**：Given 注入文本含中文、日文等 Unicode 字符，When 在 Windows 上注入，Then 全部正确输入（`KEYEVENTF_UNICODE` 已实现）。
- **AC3-2**：Given macOS 平台，When 注入接口被调用，Then `MacosInjector` 编译通过并返回权限检测结果（未授权时返回 `InjectError::PermissionDenied`）。
- **AC3-3**：Given Linux 平台，When 注入接口被调用，Then `LinuxInjector` 编译通过（调用 `xdotool` 或返回降级提示）。
- **AC3-4**：Given 跨平台，When 编译，Then 通过 Feature Flag 正确选择对应平台实现。

## 任务清单

1. 完善 Windows `SendInput` 对多字节 Unicode 的全面测试
2. 实现 `MacosInjector` 骨架（`CGEvent` 路径，返回权限检测），预留 macOS 实现
3. 实现 `LinuxInjector` 骨架（`xdotool type` 路径），预留 Linux 实现
4. 编写跨平台编译验证（CI 或本地交叉编译检查）
5. 编写 Windows 多字节字符注入测试用例

## 关联需求

- N9 跨平台支持
- TR2 跨平台输入注入实现差异

## 技术路由

- 后端 Rust 程序员：跨平台 Feature Flag 与骨架实现
- 依赖 S121 Windows 实现

## 估算

- 1 人日（小）