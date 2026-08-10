---
name: propagate-design-change
description: "分析 SRS 变更影响，找到受影响的 ADR 与 Story，列出需更新的文档清单。"
license: MIT
metadata:
  model: sonnet
  argument-hint: "[变更的SRS路径]"
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
    windsurf: {enabled: true, trigger: /propagate-design-change}
    trae: {enabled: true}
    hermes: {enabled: true, platforms: [macos, linux, windows]}
    workbuddy: {enabled: true}
---

# propagate-design-change — 设计变更传播分析

## 技能目的

当某个 SRS 发生变更时，分析依赖图，找到受影响的 ADR、架构文档、Epic、Story 及其他文档，生成需更新的文档清单，确保变更影响全面覆盖。

## 参数说明

- `[变更的SRS路径]`：发生变更的 SRS 文件路径，例如 `docs/srs/auth.md`。

## 分阶段工作流

### 阶段 1：读取变更的 SRS

- **输入**：SRS 路径参数
- **处理**：
  1. 使用 Read 读取变更后的 SRS 文件
  2. 使用 Grep 提取 SRS 中引用的其他模块、ADR、用户故事
  3. 使用 Read 读取 `docs/systems-index.md` 获取模块依赖关系
- **输出**：变更 SRS 内容 + 依赖信息

### 阶段 2：构建影响依赖图

- **输入**：SRS 变更内容 + 依赖信息
- **处理**：
  1. 使用 Grep 在所有 ADR 中检索是否引用了该 SRS
  2. 使用 Grep 在所有 Epic 与 Story 中检索是否依赖该 SRS 的需求
  3. 使用 Glob 扫描 `docs/adr/*.md`、`docs/epics/*.md`、`docs/stories/*.md`
  4. 构建变更影响传播图（直接依赖 + 间接依赖）
- **输出**：影响依赖图

### 阶段 3：列出受影响文档

- **输入**：影响依赖图
- **处理**：
  1. 分类列出受影响文档：ADR、架构文档、Epic、Story、控制清单
  2. 为每个受影响文档标注影响程度（高/中/低）
  3. 标注具体受影响的章节或条目
  4. 通过 AskUserQuestion 与用户确认影响范围
- **输出**：受影响文档清单

### 阶段 4：生成变更传播报告

- **输入**：受影响文档清单
- **处理**：
  1. 使用 Write 生成 `docs/change-impact-report.md`
  2. 报告包含：变更摘要、影响图、受影响文档清单、建议操作
  3. 为每个受影响文档提供更新建议
- **输出**：变更传播报告文件

## 协作协议引用

- 遵循 `.claude/docs/templates/collaborative-protocols/design-agent-protocol.md` 变更管理规范
- 报告写入前需经用户确认影响范围
- 变更传播报告是后续更新操作的依据

## 推荐下一步

根据变更传播报告逐个更新受影响文档。更新 ADR 使用 `/architecture-decision`，更新 Story 使用 `/create-stories`。更新完成后使用 `/consistency-check` 验证一致性。