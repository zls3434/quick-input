<!-- Software Engineering Studios -->
# 平台适配指南

本文件介绍 Software Engineering Studios 的多平台适配架构和使用方法。

## 三层架构

### 第一层：规范源层（.studio/）

平台无关的单一真相源，所有配置在此维护：

- `.studio/project/` — 项目概述文档
- `.studio/agents/` — 40 个 Agent 定义
- `.studio/skills/` — 72 个技能定义（Agent Skills 标准格式）
- `.studio/rules/` — 11 个路径规则
- `.studio/hooks/` — 12 个 Hook 脚本
- `.studio/docs/` — 规范文档
- `.studio/templates/` — 文档模板

### 第二层：适配层（tools/adapters/）

同步脚本将规范源转换为各平台格式：

- `sync-all.sh` — 同步所有平台
- `sync-claude-code.sh` — 同步 Claude Code
- `sync-codex.sh` — 同步 Codex
- `sync-cursor.sh` — 同步 Cursor
- `sync-windsurf.sh` — 同步 Windsurf
- `sync-trae.sh` — 同步 Trae IDE
- `sync-hermes.sh` — 同步 Hermes Agent
- `sync-workbuddy.sh` — 同步 WorkBuddy

### 第三层：输出层

各平台原生配置目录，由同步脚本自动生成。

## 开放标准

### AGENTS.md

被 Codex、Cursor、Windsurf、Trae 等广泛支持的跨平台主配置标准。项目根目录的 `AGENTS.md` 是所有平台的共享入口。

### Agent Skills

Anthropic 主导的技能开放标准，被 Claude Code、Cursor、Codex、Trae 等支持。`.studio/skills/` 下的 SKILL.md 遵循此标准。

## 格式转换

### Agent 定义

| 平台 | 转换方式 |
|---|---|
| Claude Code | 直接使用（YAML frontmatter + body） |
| Cursor | 转为 Agent Requested 类型 .mdc 文件 |
| Windsurf | 转为 .windsurf/workflows/agent-*.md 工作流 |
| Trae IDE | 转为 .trae/agents/ 定义文件 |
| Hermes | 浓缩到 SOUL.md 身份配置 |
| Codex/WorkBuddy | 浓缩到 AGENTS.md |

### 技能定义

| 平台 | 转换方式 |
|---|---|
| Claude Code | metadata 字段提升为顶层 |
| Cursor | 直接复制（已兼容） |
| Windsurf | 转为 .windsurf/workflows/*.md |
| Trae IDE | 直接复制（已兼容） |
| Hermes | 添加 platforms、metadata.hermes 字段 |
| WorkBuddy | 直接复制（已兼容） |

### Hook 系统

Hook 仅 Claude Code 支持，其他平台降级为 AGENTS.md 中的文档指令。详见 `.studio/hooks/_manifest.md`。

## 常见操作

### 修改 Agent 定义

1. 编辑 `.studio/agents/对应文件.md`
2. 运行 `bash tools/adapters/sync-all.sh`
3. 提交变更

### 新增技能

1. 在 `.studio/skills/新技能名/` 创建 SKILL.md
2. 更新 `.studio/skills/_catalog.yaml`
3. 运行同步
4. 提交变更

### 修改规则

1. 编辑 `.studio/rules/对应文件.md`
2. 运行同步
3. 提交变更