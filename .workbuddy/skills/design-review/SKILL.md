---
name: design-review
description: "审查设计文档，验证8必需章节、用户故事完整性、边界情况覆盖。"
license: MIT
metadata:
  model: sonnet
  argument-hint: "[文档路径]"
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
    windsurf: {enabled: true, trigger: /design-review}
    trae: {enabled: true}
    hermes: {enabled: true, platforms: [macos, linux, windows]}
    workbuddy: {enabled: true}
---

# design-review — 设计文档审查

## 技能目的

审查指定设计文档（架构文档、UX 规范、API 规范等）的结构完整性与内容质量，验证 8 个必需章节是否齐全、用户故事是否完整、边界情况是否覆盖。

## 参数说明

- `[文档路径]`：待审查的设计文档路径。

## 分阶段工作流

### 阶段 1：读取文档

- **输入**：文档路径参数
- **处理**：使用 Read 读取目标文档，使用 Grep 提取章节标题
- **输出**：文档内容 + 章节结构

### 阶段 2：验证必需章节

- **输入**：章节结构
- **处理**：对照 8 个必需章节清单检查是否存在：上下文、目标、约束、决策、方案、接口、风险、追溯
- **输出**：章节完整性表

### 阶段 3：用户故事完整性

- **输入**：文档内容 + 关联 SRS
- **处理**：
  1. 使用 Grep 提取文档中引用的用户故事
  2. 使用 Read 读取关联 SRS 中的用户故事列表
  3. 验证所有用户故事是否在设计中被覆盖
- **输出**：用户故事覆盖矩阵

### 阶段 4：边界情况覆盖

- **输入**：文档内容
- **处理**：检查是否覆盖异常路径、空状态、权限不足、并发、超时等边界情况
- **输出**：边界情况覆盖清单

### 阶段 5：输出审查报告

- **输入**：以上所有检查结果
- **处理**：通过 AskUserQuestion 确认审查结论，使用 Write 写入审查报告
- **输出**：审查报告文件

## 协作协议引用

- 遵循 `.claude/docs/templates/collaborative-protocols/design-agent-protocol.md` 审查规范
- 参考 `.claude/docs/templates/collaborative-protocols/design-agent-protocol.md` 设计文档规范

## 推荐下一步

根据审查报告修复缺失章节与未覆盖项，修复后重新运行本技能验证，或进入下一阶段。