---
name: create-control-manifest
description: "从已接受的 ADR 中提取实施指南，编译为面向程序员的平面控制清单。"
version: 1.0.0
platforms: [macos, linux, windows]
metadata:
  hermes:
    tags: ["control", "manifest"]
    load_mode: on-demand
    model: sonnet
    argument-hint: ""
    user-invocable: true
    allowed-tools:
      - Read
      - Glob
      - Grep
      - Write
---

# create-control-manifest — 控制清单生成

## 技能目的

从所有处于 Accepted 状态的 ADR 中提取实施指南（Implementation Guidelines），编译为一份面向程序员的平面规则表 `docs/architecture/control-manifest.md`，作为开发阶段的统一约束参考。

## 参数说明

本技能无参数。自动扫描所有 ADR。

## 分阶段工作流

### 阶段 1：扫描所有 ADR

- **输入**：项目目录
- **处理**：
  1. 使用 Glob 扫描 `docs/adr/*.md`
  2. 使用 Read 读取每个 ADR
  3. 使用 Grep 过滤状态为 Accepted 的 ADR
  4. 跳过 Proposed、Rejected、Superseded 状态的 ADR
- **输出**：Accepted ADR 集合

### 阶段 2：提取实施指南

- **输入**：Accepted ADR 集合
- **处理**：
  1. 使用 Grep 在每个 ADR 中定位"Implementation Guidelines"或"实施指南"章节
  2. 使用 Read 读取该章节内容
  3. 为每条指南标注来源 ADR 编号
  4. 按领域分类（前端、后端、数据库、部署、安全等）
- **输出**：分类后的实施指南条目

### 阶段 3：编译控制清单

- **输入**：分类后的指南条目
- **处理**：
  1. 按领域组织为平面规则表
  2. 每条规则包含：规则编号、规则描述、来源 ADR、适用范围
  3. 去重合并重叠规则
  4. 标注强制性等级（MUST / SHOULD / MAY）
- **输出**：控制清单草稿

### 阶段 4：写入文件

- **输入**：控制清单草稿
- **处理**：
  1. 向用户展示清单摘要并请求确认
  2. 使用 Write 写入 `docs/architecture/control-manifest.md`
  3. 在文件头部记录生成时间与来源 ADR 列表
- **输出**：control-manifest.md 文件

## 协作协议引用

- 遵循 `.claude/docs/templates/collaborative-protocols/design-agent-protocol.md` 架构文档规范
- 写入前需向用户展示清单摘要并获得确认
- 控制清单是开发阶段的核心约束文档，不得与 ADR 冲突

## 推荐下一步

控制清单生成后，进入预开发阶段使用 `/create-epics` 创建 Epic，或使用 `/prototype` 验证关键技术点。开发阶段使用 `/dev-story` 时将加载此控制清单。