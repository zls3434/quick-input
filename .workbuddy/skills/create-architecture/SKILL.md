---
name: create-architecture
description: "编写主架构文档，读取所有SRS生成architecture-doc.md，列出必需ADR。"
license: MIT
metadata:
  model: sonnet
  argument-hint: ""
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
    windsurf: {enabled: true, trigger: /create-architecture}
    trae: {enabled: true}
    hermes: {enabled: true, platforms: [macos, linux, windows]}
    workbuddy: {enabled: true}
---

# create-architecture — 主架构文档编写

## 技能目的

读取所有 SRS 与产品概念，综合生成 `architecture-doc.md` 主架构文档，并列出需要创建的 ADR（架构决策记录）清单。

## 参数说明

本技能无参数。自动读取所有 SRS。

## 分阶段工作流

### 阶段 1：收集需求输入

- **输入**：项目目录
- **处理**：
  1. 使用 Glob 扫描 `docs/srs/*.md`
  2. 使用 Read 读取所有 SRS
  3. 使用 Read 读取 `docs/product-concept.md` 与 `docs/technical-preferences.md`
- **输出**：需求输入集合

### 阶段 2：识别架构决策点

- **输入**：需求输入集合
- **处理**：
  1. 识别需要架构决策的技术点（数据存储、通信协议、部署模式等）
  2. 通过 AskUserQuestion 与用户讨论关键决策方向
- **输出**：架构决策点清单

### 阶段 3：生成架构文档

- **输入**：需求输入 + 决策点
- **处理**：使用 Write 创建 `docs/architecture-doc.md`，包含：系统上下文、容器视图、组件视图、部署视图、数据流、安全模型
- **输出**：architecture-doc.md 文件

### 阶段 4：列出必需 ADR

- **输入**：架构决策点清单
- **处理**：为每个决策点生成 ADR 条目，使用 Write 写入 `docs/adr-index.md`
- **输出**：ADR 索引文件

## 协作协议引用

- 遵循 `.claude/docs/templates/collaborative-protocols/design-agent-protocol.md` 架构文档规范
- 参考 `.claude/docs/templates/architecture-doc.md` 模板

## 推荐下一步

对每个必需 ADR 使用 `/architecture-decision` 逐个创建。全部完成后使用 `/architecture-review` 进行架构审查。