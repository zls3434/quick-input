---
name: consistency-check
description: "扫描所有 SRS 的矛盾、未定义引用、规则冲突，输出一致性问题清单。"
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
  platforms:
    claude-code: {enabled: true}
    cursor: {enabled: true}
    codex: {enabled: true}
    windsurf: {enabled: true, trigger: /consistency-check}
    trae: {enabled: true}
    hermes: {enabled: true, platforms: [macos, linux, windows]}
    workbuddy: {enabled: true}
---

# consistency-check — SRS 一致性扫描

## 技能目的

扫描所有 SRS 文档，检测矛盾、未定义引用、规则冲突等问题，输出结构化的一致性问题清单，供后续修复。

## 参数说明

本技能无参数。自动扫描所有 SRS。

## 分阶段工作流

### 阶段 1：收集 SRS 与引用

- **输入**：项目目录
- **处理**：
  1. 使用 Glob 扫描 `docs/srs/*.md`
  2. 使用 Read 读取所有 SRS
  3. 使用 Grep 提取所有交叉引用（模块引用、接口引用、术语引用）
- **输出**：SRS 集合 + 引用表

### 阶段 2：检测矛盾

- **输入**：SRS 集合
- **处理**：对比不同 SRS 中对同一概念、同一接口、同一数据模型的描述，识别矛盾
- **输出**：矛盾清单

### 阶段 3：检测未定义引用

- **输入**：引用表
- **处理**：验证每个引用目标是否存在，标记未定义引用
- **输出**：未定义引用清单

### 阶段 4：检测规则冲突

- **输入**：SRS 集合
- **处理**：检查业务规则、约束条件、权限定义之间是否存在冲突
- **输出**：规则冲突清单

### 阶段 5：输出报告

- **输入**：所有问题清单
- **处理**：使用 Write 写入 `docs/reviews/consistency-report.md`
- **输出**：一致性报告文件

## 协作协议引用

- 遵循 `.claude/docs/templates/collaborative-protocols/design-agent-protocol.md` 审查规范

## 推荐下一步

修复报告中的问题，然后使用 `/create-architecture` 进入架构设计阶段。