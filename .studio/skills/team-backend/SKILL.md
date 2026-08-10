---
name: team-backend
description: "编排后端团队协作管线，协调技术架构师、后端架构师、语言专家、数据库工程师、测试工程师与代码审查员按序交付后端功能。"
license: MIT
metadata:
  model: sonnet
  argument-hint: "[功能描述]"
  user-invocable: true
  allowed-tools:
    - Read
    - Glob
    - Grep
    - Write
    - Task
    - AskUserQuestion
  platforms:
    claude-code: {enabled: true}
    cursor: {enabled: true}
    codex: {enabled: true}
    windsurf: {enabled: true, trigger: /team-backend}
    trae: {enabled: true}
    hermes: {enabled: true, platforms: [macos, linux, windows]}
    workbuddy: {enabled: true}
---

# team-backend — 后端团队编排

## 技能目的

编排完整的后端团队协作管线，按既定顺序协调后端相关 Agent 协同工作，将一项功能描述从系统设计推进到通过代码审查与测试的可交付状态。本技能负责调度各 Agent、传递中间产物、收集反馈并在阶段间执行质量门禁。

## 参数说明

- `[功能描述]`：待实现的后端功能描述，可为用户故事标题、Epic 名称或自然语言功能说明。

## 分阶段工作流

### 阶段 1：加载上下文与确认范围

- **输入**：`[功能描述]` 参数
- **处理**：
  1. 使用 Read 读取 `docs/srs/*.md` 匹配功能描述对应的需求规格
  2. 使用 Glob 查找 `docs/design/architecture*.md` 获取已有架构文档
  3. 使用 Read 读取 `docs/technical-preferences.md` 确认后端语言（Node.js / Python / Java / Go）与数据库（PostgreSQL / MongoDB / Redis）
  4. 通过 AskUserQuestion 与用户确认功能边界、接口契约与验收标准
- **输出**：功能上下文 + 团队编排计划

### 阶段 2：tech-architect 系统设计

- **输入**：功能上下文
- **处理**：使用 Task 分派 tech-architect Agent，明确系统边界、外部依赖、服务划分与集成点
- **输出**：系统设计说明

### 阶段 3：backend-architect 后端架构

- **输入**：系统设计说明
- **处理**：使用 Task 分派 backend-architect Agent，设计分层架构、模块划分、接口协议与错误处理策略
- **输出**：后端架构方案

### 阶段 4：node/python/java/go-specialist 实现功能

- **输入**：后端架构方案
- **处理**：使用 Task 分派对应语言专家 Agent（依据技术栈选择），实现业务逻辑、接口与服务
- **输出**：后端代码实现

### 阶段 5：database-engineer 数据层

- **输入**：后端代码实现 + 架构方案
- **处理**：使用 Task 分派 database-engineer Agent，设计数据模型、迁移脚本与查询优化
- **输出**：数据层实现与迁移脚本

### 阶段 6：test-engineer 编写与执行测试

- **输入**：后端代码实现 + 数据层
- **处理**：使用 Task 分派 test-engineer Agent，编写单元测试、集成测试与接口测试，执行并收集结果
- **输出**：测试用例 + 测试执行报告

### 阶段 7：code-reviewer 代码审查

- **输入**：代码实现 + 测试报告
- **处理**：使用 Task 分派 code-reviewer Agent，审查代码质量、安全性、性能与规范一致性
- **输出**：代码审查报告

### 阶段 8：汇总与门禁

- **输入**：全部阶段产物
- **处理**：
  1. 汇总各阶段产出与反馈
  2. 检查所有阶段是否通过质量门禁
  3. 通过 AskUserQuestion 向用户报告结果并确认返工需求
  4. 使用 Write 写入编排报告至 `docs/reports/team-backend-[功能].md`
- **输出**：团队编排报告

## 协作协议引用

- 遵循 `docs/examples/COLLABORATIVE-DESIGN-PRINCIPLE.md` 用户驱动协作协议
- 各 Agent 之间通过 Task 工具分派，中间产物以文件形式传递
- 每个阶段切换前需向用户展示当前阶段产出并获取确认
- 阶段间设置质量门禁，未通过则回退至上一阶段

## 推荐下一步

后端功能交付后，使用 `/team-qa` 执行完整 QA 周期验证。性能问题使用 `/team-polish` 进行优化打磨。安全相关关注使用 `/team-security` 审查。发布阶段使用 `/team-release` 编排发布流程。