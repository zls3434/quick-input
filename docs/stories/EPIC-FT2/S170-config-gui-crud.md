---
状态: 草案
作者: zls3434
更新日期: 2026-08-13
所属 Epic: EPIC-FT2
对应模块: M7, M4, M5
优先级: P2
依赖: M4, M5
---

# S170 用户故事：配置编辑窗口与默认按钮 CRUD

**用户故事**：作为用户，我希望在可视化界面中增删改默认按钮，无需手动编辑 TOML 文件。

## 验收标准（Given-When-Then）

- **AC11-1**：Given 托盘菜单，When 点击"配置管理"，Then 打开独立设置窗口。
- **AC11-2**：Given 设置窗口，When 加载，Then 列出所有默认按钮（id、label、content、comment）。
- **AC11-3**：Given 按钮列表，When 点击"新增"，Then 弹出表单，填写后保存。
- **AC11-4**：Given 按钮列表，When 点击编辑，Then 修改各字段，保存后更新。
- **AC11-5**：Given 按钮列表，When 点击删除并确认，Then 移除该按钮。
- **AC11-6**：Given 配置修改保存，When 完成，Then 通知主浮层刷新按钮列表（`ConfigSwitched`）。

## 任务清单

1. 修改 `tauri.conf.json` 添加 settings 窗口
2. 新建 `src/routes/settings/+page.svelte` 设置页面（按钮列表 + 编辑表单）
3. 添加 Rust 命令：`save_config`、`add_button`、`update_button`、`delete_button`
4. 托盘菜单增加"配置管理"项，点击打开设置窗口
5. 保存后发射 `ConfigSwitched` 事件通知主浮层刷新
6. 编写单元测试覆盖按钮 CRUD

## 关联需求

- AC1 配置 GUI 界面列出所有按钮，支持增删改
- AC2 支持编辑按钮的标签、内容、注释

## 技术路由

- 后端 Rust 程序员：Rust 命令 + settings 窗口控制
- 前端 Svelte 程序员：设置页面 UI
- 依赖 M4 配置读写、M5 浮层刷新

## 估算

- 2 人日（中）