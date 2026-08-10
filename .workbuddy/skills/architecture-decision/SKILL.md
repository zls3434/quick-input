---
name: architecture-decision
description: "创建架构决策记录(ADR)，使用 architecture-decision-record.md 模板引导式编写，经用户批准后写入文件。"
license: MIT
metadata:
  model: sonnet
  argument-hint: "[决策主题]"
  user-invocable: true
  allowed-tools:
    - Read
    - Glob
    - Grep
    - Write
    - AskUserQuestion
  platforms:
    claude-code: {enabled: true}
    cursor: {enabled: true}
    codex: {enabled: true}
    windsurf: {enabled: true, trigger: /architecture-decision}
    trae: {enabled: true}
    hermes: {enabled: true, platforms: [macos, linux, windows]}
    workbuddy: {enabled: true}
---

# architecture-decision — 架构决策记录创建

## 技能目的

为指定决策主题创建架构决策记录（ADR），使用模板引导式编写，分析上下文、呈现备选方案、经用户选择与批准后写入文件，确保关键技术决策有据可查且可追溯。

## 参数说明

- `[决策主题]`：待决策的架构主题，例如"数据存储方案"、"通信协议选择"。需与 `docs/adr-index.md` 中的条目对应。

## 分阶段工作流

### 阶段 1：读取架构文档与 ADR 清单

- **输入**：决策主题参数
- **处理**：
  1. 使用 Read 读取 `docs/architecture-doc.md` 获取架构上下文
  2. 使用 Read 读取 `docs/adr-index.md` 获取必需 ADR 列表
  3. 使用 Glob 查找 `.claude/docs/templates/architecture-decision-record.md` 模板
  4. 确认决策主题存在于 ADR 清单中
- **输出**：架构上下文 + ADR 模板

### 阶段 2：分析决策上下文

- **输入**：架构文档 + 决策主题
- **处理**：
  1. 使用 Grep 在所有 SRS 中检索与该决策相关的需求
  2. 使用 Read 读取相关 SRS 章节
  3. 使用 Read 读取 `docs/technical-preferences.md` 获取技术偏好约束
  4. 识别决策的约束条件与影响范围
- **输出**：决策上下文摘要

### 阶段 3：呈现备选方案

- **输入**：决策上下文
- **处理**：
  1. 基于约束条件列出 2~4 个备选方案
  2. 为每个方案分析优缺点、成本、风险
  3. 通过 AskUserQuestion 向用户呈现方案选项
- **输出**：备选方案对比表

### 阶段 4：用户选择与 ADR 起草

- **输入**：用户选择的方案
- **处理**：
  1. 基于用户选择填充 ADR 模板各章节
  2. 包含：背景、决策、状态、上下文、备选方案、后果、实施指南
  3. 向用户展示 ADR 草稿
- **输出**：ADR 草稿

### 阶段 5：批准与写入

- **输入**：ADR 草稿
- **处理**：
  1. 通过 AskUserQuestion 请求用户批准
  2. 批准后使用 Write 写入 `docs/adr/ADR-NNN-{主题slug}.md`
  3. 更新 `docs/adr-index.md` 中该条目的状态为 Accepted
- **输出**：ADR 文件 + 更新后的索引

## 协作协议引用

- 遵循 `.claude/docs/templates/collaborative-protocols/design-agent-protocol.md` 架构文档规范
- 参考 `.claude/docs/templates/architecture-decision-record.md` 模板
- 写入前必须获得用户明确批准

## 推荐下一步

继续创建下一个必需 ADR。全部 ADR 完成后使用 `/architecture-review` 验证架构完整性，随后使用 `/create-control-manifest` 生成控制清单。