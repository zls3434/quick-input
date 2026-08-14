---
状态: 草案
作者: zls3434
更新日期: 2026-08-10
---

# Epic 索引

> 本文档索引 QuickInput 项目所有 Epic，按三层结构组织。

## 三层结构

```
Foundation（基础设施层）
├── EPIC-F1: 配置管理基础设施
├── EPIC-F2: 浮层窗口系统
└── EPIC-F3: 输入注入引擎

Core（核心业务层）
├── EPIC-C1: 焦点感知与配置切换
└── EPIC-C2: 按钮面板交互

Feature（功能层）
├── EPIC-FT1: 应用生命周期管理（P2）
├── EPIC-FT2: 可视化配置管理（P2，扩展）
└── EPIC-FT3: 配置共享与协作（P3，扩展）
```

## Epic 详表

| 编号 | 名称 | 层级 | 对应模块 | 优先级 | 依赖 |
|------|------|------|----------|--------|------|
| EPIC-F1 | 配置管理基础设施 | foundation | M4 | P0 | 无 |
| EPIC-F2 | 浮层窗口系统 | foundation | M1 | P0 | 无 |
| EPIC-F3 | 输入注入引擎 | foundation | M2 | P0 | 无 |
| EPIC-C1 | 焦点感知与配置切换 | core | M3 | P1 | EPIC-F1 |
| EPIC-C2 | 按钮面板交互 | core | M5 | P0 | EPIC-F1, EPIC-F2, EPIC-F3 |
| EPIC-FT1 | 应用生命周期管理 | feature | M6 | P2 | EPIC-F2, EPIC-F1 |
| EPIC-FT2 | 可视化配置管理 | feature | M7 | P2 | EPIC-F1, EPIC-C2 |
| EPIC-FT3 | 配置共享与协作 | feature | M8 | P3 | EPIC-F1 |

## 文件清单

- `docs/epics/index.md` — 本索引文件
- `docs/epics/foundation/EPIC-F1-config-manager.md`
- `docs/epics/foundation/EPIC-F2-overlay-window.md`
- `docs/epics/foundation/EPIC-F3-input-injector.md`
- `docs/epics/core/EPIC-C1-focus-detector.md`
- `docs/epics/core/EPIC-C2-button-panel.md`
- `docs/epics/feature/EPIC-FT1-app-lifecycle.md`
- `docs/epics/feature/EPIC-FT2-config-gui.md`
- `docs/epics/feature/EPIC-FT3-config-sharing.md`

## 开发批次与 Epic 对应关系

| 批次 | 包含 Epic | 产出目标 |
|------|-----------|----------|
| 第一批 | EPIC-F1, EPIC-F2, EPIC-F3, EPIC-C2 | 最小核心验证：单默认配置的置顶浮层，可点击注入 |
| 第二批 | EPIC-C1 + EPIC-F1/EPIC-C2 增强 | MVP 完成：按应用自动切换配置 |
| 第三批 | EPIC-FT1 | 体验增强：开机自启、热键呼出 |
| 扩展 | EPIC-FT2, EPIC-FT3 | 后续规划 |