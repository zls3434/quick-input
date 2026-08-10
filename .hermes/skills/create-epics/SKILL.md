---
name: create-epics
description: "创建Epic，将SRS与ADR转化为分层Epic（foundation/core/feature）。"
version: 1.0.0
platforms: [macos, linux, windows]
metadata:
  hermes:
    tags: ["planning", "stories"]
    load_mode: on-demand
    model: sonnet
    argument-hint: "layer:[foundation/core/feature]"
    user-invocable: true
    allowed-tools:
      - Read
      - Glob
      - Grep
      - Write
      - AskUserQuestion
---

# create-epics — Epic 创建

## 技能目的

将 SRS 需求与 ADR 决策转化为分层 Epic，按 foundation（基础设施层）、core（核心层）、feature（功能层）三个层级组织，确保开发顺序合理且依赖关系清晰。

## 参数说明

- `layer:[foundation/core/feature]`：目标层级，可选基础层、核心层或功能层。

## 分阶段工作流

### 阶段 1：加载上下文

- **输入**：layer 参数
- **处理**：
  1. 使用 Read 读取 `docs/systems-index.md` 获取模块清单
  2. 使用 Glob 扫描 `docs/srs/*.md` 读取所有 SRS
  3. 使用 Glob 扫描 `docs/adr/*.md` 读取所有 Accepted ADR
  4. 使用 Read 读取 `docs/architecture-doc.md` 获取分层架构
  5. 使用 Glob 检查已存在的 Epic 文件
- **输出**：需求与架构上下文

### 阶段 2：识别该层 Epic 候选

- **输入**：上下文 + layer 参数
- **处理**：
  1. 根据 layer 过滤对应的 SRS 需求与 ADR 决策
  2. foundation 层：基础设施、数据库、认证、配置等
  3. core 层：核心业务逻辑、领域服务
  4. feature 层：用户可见功能、界面交互
  5. 为每个候选 Epic 定义范围与目标
- **输出**：Epic 候选清单

### 阶段 3：定义 Epic 详情

- **输入**：Epic 候选清单
- **处理**：
  1. 为每个 Epic 编写描述、业务价值、验收标准
  2. 定义 Epic 间的依赖关系
  3. 标注 Epic 关联的 SRS 编号与 ADR 编号
  4. 通过 AskUserQuestion 与用户确认 Epic 优先级
- **输出**：Epic 详情文档

### 阶段 4：写入文件

- **输入**：Epic 详情
- **处理**：
  1. 向用户展示 Epic 草稿
  2. 批准后使用 Write 写入 `docs/epics/{layer}/{epic-slug}.md`
  3. 更新 `docs/epics/index.md` 索引
- **输出**：Epic 文件 + 索引

## 协作协议引用

- 遵循 `.claude/docs/templates/collaborative-protocols/design-agent-protocol.md` Epic 编写规范
- Epic 必须关联 SRS 需求编号与 ADR 编号
- 写入前需用户批准

## 推荐下一步

对每个 Epic 使用 `/create-stories` 分解为用户故事。使用 `/estimate` 估算工作量。使用 `/sprint-plan` 将 Epic 纳入 Sprint 规划。