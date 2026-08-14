---
状态: 草案
作者: zls3434
更新日期: 2026-08-13
所属 Epic: EPIC-FT3
对应模块: M8, M4
优先级: P3
依赖: S172
---

# S180 用户故事：配置导出与导入

**用户故事**：作为用户，我希望将配置导出为 TOML 文件分享给他人，并能导入他人的配置，以便团队共享标准化命令集。

## 验收标准（Given-When-Then）

- **AC14-1**：Given 设置窗口，When 点击"导出配置"，Then 弹出系统保存对话框，导出 TOML 文件。
- **AC14-2**：Given 设置窗口，When 点击"导入配置"，Then 弹出系统打开对话框，选择 TOML 文件后导入。
- **AC14-3**：Given 导入 TOML 文件，When 格式校验失败，Then 返回明确错误信息，不覆盖当前配置。
- **AC14-4**：Given 导入成功，When 完成，Then 写入配置目录，发射 `ConfigSwitched` 刷新主浮层。

## 任务清单

1. 添加 Rust 命令：`export_config`、`import_config`
2. `export_config`：序列化当前配置 → 弹出保存对话框 → 写入文件
3. `import_config`：弹出打开对话框 → 读取文件 → 解析 TOML → 校验 → 写入配置目录
4. 设置窗口工具栏增加"导出"/"导入"按钮
5. 编写单元测试覆盖导出/导入流程

## 关联需求

- AC1 支持导出 TOML 文件
- AC2 支持导入 TOML 文件
- AC3 导入时校验格式

## 技术路由

- 后端 Rust 程序员：`export_config` / `import_config` 命令
- 前端 Svelte 程序员：设置窗口按钮
- 依赖 M4 配置读写、S172 校验

## 估算

- 1 人日（小）