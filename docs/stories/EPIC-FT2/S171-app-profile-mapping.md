---
状态: 草案
作者: zls3434
更新日期: 2026-08-13
所属 Epic: EPIC-FT2
对应模块: M7, M4
优先级: P2
依赖: S170
---

# S171 用户故事：应用映射管理

**用户故事**：作为用户，我希望在设置界面中管理应用映射（进程名与按钮集的对应），以便不同应用自动加载各自按钮。

## 验收标准（Given-When-Then）

- **AC12-1**：Given 设置窗口，When 查看，Then 显示所有应用画像（进程名 + 按钮数）。
- **AC12-2**：Given 应用画像，When 新增进程映射，Then 可创建新进程名对应的画像。
- **AC12-3**：Given 已存在的画像，When 编辑，Then 可修改进程名及其按钮集。
- **AC12-4**：Given 已存在的画像，When 删除并确认，Then 移除该映射。
- **AC12-5**：Given 修改保存，When 完成，Then 通知主浮层刷新（`ConfigSwitched`）。

## 任务清单

1. 添加 Rust 命令：`get_profiles`、`add_profile`、`update_profile`、`delete_profile`
2. 设置窗口增加"应用映射"标签页
3. 支持新增/编辑/删除应用画像（进程名 + 按钮列表）
4. 保存后发射 `ConfigSwitched`
5. 编写单元测试覆盖画像 CRUD

## 关联需求

- AC3 支持管理应用映射（哪个进程用哪个配置）

## 技术路由

- 后端 Rust 程序员：Rust 画像 CRUD 命令
- 前端 Svelte 程序员：设置窗口应用映射 UI
- 依赖 S170 设置窗口框架

## 估算

- 1 人日（小）