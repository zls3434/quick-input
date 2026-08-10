---
name: create-stories
description: "创建用户故事，将Epic分解为可开发的Story，定义验收标准与任务清单。"
license: MIT
metadata:
  model: sonnet
  argument-hint: "[epic-slug]"
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
    windsurf: {enabled: true, trigger: /create-stories}
    trae: {enabled: true}
    hermes: {enabled: true, platforms: [macos, linux, windows]}
    workbuddy: {enabled: true}
---

# create-stories — 用户故事创建

## 技能目的

将指定 Epic 分解为可开发的用户故事（Story），每个 Story 包含描述、验收标准、任务清单、依赖关系与估算，确保开发粒度适中且可独立交付。

## 参数说明

- `[epic-slug]`：目标 Epic 的 slug 标识符，例如 `auth-foundation`。

## 分阶段工作流

### 阶段 1：读取 Epic

- **输入**：epic-slug 参数
- **处理**：
  1. 使用 Glob 查找 `docs/epics/**/{epic-slug}.md`
  2. 使用 Read 读取 Epic 文件获取范围与验收标准
  3. 使用 Read 读取关联的 SRS 获取详细需求
  4. 使用 Read 读取关联 ADR 获取技术约束
  5. 使用 Read 读取 `docs/architecture/control-manifest.md` 获取控制规则
- **输出**：Epic 上下文 + 约束

### 阶段 2：分解 Story

- **输入**：Epic 范围与验收标准
- **处理**：
  1. 将 Epic 范围按功能维度分解为 Story 候选
  2. 为每个 Story 定义用户故事格式（作为...我希望...以便...）
  3. 确保每个 Story 可在 1~3 天内完成
  4. 通过 AskUserQuestion 与用户确认分解粒度
- **输出**：Story 候选清单

### 阶段 3：定义 Story 详情

- **输入**：Story 候选清单
- **处理**：
  1. 为每个 Story 编写验收标准（Given-When-Then 格式）
  2. 定义任务清单（技术任务细分）
  3. 标注 Story 间的依赖关系
  4. 标注关联的 SRS 需求编号
  5. 定义 Story 的技术路由（路由到哪个程序员 Agent）
- **输出**：Story 详情文档

### 阶段 4：写入文件

- **输入**：Story 详情
- **处理**：
  1. 向用户展示 Story 草稿
  2. 批准后使用 Write 写入 `docs/stories/{epic-slug}/{story-slug}.md`
  3. 更新 Epic 文件中的 Story 列表
- **输出**：Story 文件 + 更新的 Epic

## 协作协议引用

- 遵循 `.claude/docs/templates/collaborative-protocols/design-agent-protocol.md` Story 编写规范
- Story 验收标准必须可测试
- 写入前需用户批准

## 推荐下一步

使用 `/estimate` 估算 Story 工作量。使用 `/story-readiness` 验证 Story 是否就绪可开发。使用 `/sprint-plan` 将 Story 纳入 Sprint。开发阶段使用 `/dev-story` 实现 Story。