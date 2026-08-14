---
状态: 草案
作者: zls3434
更新日期: 2026-08-10
---

# 用户故事索引

> 本文档索引 QuickInput 第一批（EPIC-F1/F2/F3/C2）的用户故事。

## 故事总览

| Story ID | 名称 | 所属 Epic | 对应模块 | 优先级 | 依赖 | 估算 |
|----------|------|-----------|----------|--------|------|------|
| S100 | 配置数据结构与校验规则 | EPIC-F1 | M4 | P0 | 无 | 1 人日 |
| S101 | 配置读取与保存 | EPIC-F1 | M4 | P0 | S100 | 2 人日 |
| S102 | 按进程名映射应用配置 | EPIC-F1 | M4, M3 | P1 | S100 | 2 人日 |
| S103 | 默认兜底与配置切换事件 | EPIC-F1 | M4, M3 | P1 | S102 | 2 人日 |
| S110 | 置顶浮层窗口创建 | EPIC-F2 | M1 | P0 | 无 | 2 人日 |
| S111 | 点击不抢焦点 | EPIC-F2 | M1 | P0 | S110 | 2 人日 |
| S112 | 浮层拖动与调整大小 | EPIC-F2 | M1 | P0 | S110 | 1 人日 |
| S120 | 统一跨平台注入接口 | EPIC-F3 | M2 | P0 | 无 | 1 人日 |
| S121 | Windows 文本注入实现 | EPIC-F3 | M2 | P0 | S120 | 3 人日 |
| S122 | Unicode 支持与跨平台扩展 | EPIC-F3 | M2 | P0 | S121 | 1 人日 |
| S130 | 按钮面板渲染 | EPIC-C2 | M5 | P0 | S101, S110 | 2 人日 |
| S131 | 悬浮注释显示 | EPIC-C2 | M5 | P1 | S130 | 1 人日 |
| S132 | 单击按钮触发注入 | EPIC-C2 | M5, M2 | P0 | S130, S121 | 2 人日 |
| S133 | 配置切换时自动刷新按钮列表 | EPIC-C2 | M5, M4 | P1 | S130, S103 | 1 人日 |

## 开发顺序建议

### 第一批（最小核心验证）

> 目标：完成"单默认配置的置顶浮层，可点击注入"

| 顺序 | Story | 说明 |
|------|-------|------|
| 1 | S100 | 数据模型（基础） |
| 2 | S101 | 配置加载/保存 |
| 3 | S110 | 浮层窗口创建 |
| 4 | S120 | 注入接口定义 |
| 5 | S121 | Windows 注入实现 |
| 6 | S130 | 按钮面板渲染 |
| 7 | S132 | 单击注入（核心链路闭环） |
| 8 | S112 | 拖动/调整大小（可后置） |

### 第二批（增强）

| 顺序 | Story | 说明 |
|------|-------|------|
| 9 | S111 | 点击不抢焦点（重要，可提前验证） |
| 10 | S102 | 按进程映射配置 |
| 11 | S103 | 默认兜底 + 切换事件 |
| 12 | S131 | 悬浮注释 |
| 13 | S133 | 配置切换刷新 |
| 14 | S122 | 跨平台扩展骨架 |

## 文件清单

- `docs/stories/EPIC-F1/S100-config-schema.md`
- `docs/stories/EPIC-F1/S101-config-load-save.md`
- `docs/stories/EPIC-F1/S102-config-app-mapping.md`
- `docs/stories/EPIC-F1/S103-config-default-fallback.md`
- `docs/stories/EPIC-F2/S110-overlay-window-create.md`
- `docs/stories/EPIC-F2/S111-overlay-non-focus-stealing.md`
- `docs/stories/EPIC-F2/S112-overlay-drag-resize.md`
- `docs/stories/EPIC-F3/S120-inject-interface.md`
- `docs/stories/EPIC-F3/S121-inject-windows.md`
- `docs/stories/EPIC-F3/S122-inject-unicode-mac-linux.md`
- `docs/stories/EPIC-C2/S130-panel-render.md`
- `docs/stories/EPIC-C2/S131-panel-tooltip.md`
- `docs/stories/EPIC-C2/S132-panel-inject-click.md`
- `docs/stories/EPIC-C2/S133-panel-config-switch.md`