---
name: requirement-spec
description: "编写逐模块需求规格(SRS)，使用 requirement-spec.md 模板引导式逐章编写，每章批准后立即写入文件。"
license: MIT
metadata:
  model: sonnet
  argument-hint: "[模块名]"
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
    windsurf: {enabled: true, trigger: /requirement-spec}
    trae: {enabled: true}
    hermes: {enabled: true, platforms: [macos, linux, windows]}
    workbuddy: {enabled: true}
---

# requirement-spec — 逐模块需求规格编写

## 技能目的

为指定模块编写详细的需求规格说明书（SRS），使用模板引导式逐章编写，每完成一章经用户批准后立即写入文件，确保需求完整、可追溯。

## 参数说明

- `[模块名]`：目标模块名称，需与 `systems-index.md` 中的模块名一致。

## 分阶段工作流

### 阶段 1：加载上下文

- **输入**：模块名参数
- **处理**：
  1. 使用 Read 读取 `docs/systems-index.md` 获取模块定义与依赖
  2. 使用 Read 读取 `docs/product-concept.md` 获取相关概念
  3. 使用 Glob 查找 `.claude/docs/templates/requirement-spec.md` 模板
- **输出**：模块上下文 + SRS 模板

### 阶段 2：引导式逐章编写

- **输入**：SRS 模板章节列表
- **处理**：对每一章：
  1. 通过 AskUserQuestion 提出针对该章的问题
  2. 根据回答生成章节内容
  3. 请用户批准该章内容
  4. 批准后立即使用 Write 追加写入 `docs/srs/{模块名}.md`
- **输出**：逐章已写入的内容

### 阶段 3：交叉引用检查

- **输入**：完整 SRS 文件
- **处理**：使用 Grep 检查 SRS 内引用的模块是否存在、引用的 ADR 是否已创建
- **输出**：交叉引用检查报告

### 阶段 4：更新索引

- **输入**：SRS 文件路径
- **处理**：使用 Write 更新 `docs/systems-index.md` 中该模块的 SRS 路径字段
- **输出**：更新后的索引

## 协作协议引用

- 遵循 `.claude/docs/templates/collaborative-protocols/design-agent-protocol.md` SRS 编写规范
- 参考 `.claude/docs/templates/requirement-spec.md` 模板

## 推荐下一步

对下一模块重复执行本技能，或使用 `/consistency-check` 进行跨 SRS 一致性检查。全部 SRS 完成后使用 `/review-all-srs` 进行总审查。