---
name: milestone-review
description: "审查里程碑进度，检查已完成与未完成 Story，评估风险并生成进度报告。"
license: MIT
metadata:
  model: sonnet
  argument-hint: "[里程碑编号]"
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
    windsurf: {enabled: true, trigger: /milestone-review}
    trae: {enabled: true}
    hermes: {enabled: true, platforms: [macos, linux, windows]}
    workbuddy: {enabled: true}
---

# milestone-review — 里程碑进度审查

## 技能目的

审查里程碑的整体进度，对照里程碑定义检查已完成与未完成的 Story，评估延期与风险，生成进度报告，帮助团队与利益相关方掌握交付状态并做出调整决策。

## 参数说明

- `[里程碑编号]`：可选，指定要审查的里程碑（如 `M1`、`M2`）。省略时审查当前活跃里程碑。

## 分阶段工作流

### 阶段 1：读取里程碑定义

- **输入**：里程碑编号参数
- **处理**：
  1. 使用 Glob 查找 `docs/milestones/**/*.md` 里程碑定义文件
  2. 使用 Read 读取里程碑目标、范围、Story 清单、截止日期
  3. 若未指定编号，使用 Read 读取 `docs/milestones/index.md` 确认当前里程碑
  4. 使用 AskUserQuestion 与用户确认待审查里程碑
- **输出**：里程碑定义

### 阶段 2：检查已完成 Story

- **输入**：里程碑定义
- **处理**：
  1. 使用 Glob 查找 `docs/stories/**/*.md` 获取全部 Story
  2. 使用 Grep 检查每个 Story 的状态标记（`status: done`、`status: complete`）
  3. 比对已完成 Story 与里程碑范围内的预期 Story
  4. 统计已完成数量、完成率、实际耗时与估算偏差
- **输出**：已完成 Story 清单

### 阶段 3：检查未完成 Story

- **输入**：已完成 Story 清单
- **处理**：
  1. 识别处于 `in-progress`、`todo`、`blocked` 状态的 Story
  2. 使用 Read 读取未完成 Story 的剩余工作说明
  3. 评估每个未完成 Story 的预计完成时间
  4. 判断是否影响里程碑截止日期
  5. 对阻塞项识别阻塞原因与依赖
- **输出**：未完成 Story 清单

### 阶段 4：评估风险

- **输入**：未完成 Story 清单
- **处理**：
  1. 识别进度风险：延期 Story 数、关键路径上的阻塞
  2. 识别范围风险：范围蔓延、新增需求
  3. 识别资源风险：成员可用性、技能缺口
  4. 识别技术风险：未解决的架构问题、性能隐患
  5. 使用 AskUserQuestion 与用户确认风险缓解措施
- **输出**：风险登记表

### 阶段 5：生成进度报告

- **输入**：全部检查与评估结果
- **处理**：
  1. 使用 Write 生成报告 `docs/milestones/{里程碑}-review.md`
  2. 报告含：进度摘要（完成率）、Story 状态分布、风险清单、建议
  3. 给出里程碑整体评级：ON TRACK / AT RISK / OFF TRACK
  4. 给出调整建议：保持、缩减范围、调整日期、增补资源
- **输出**：里程碑进度报告

## 协作协议引用

- 遵循 `.claude/docs/coding-standards.md` 编码规范
- 进度报告写入 `docs/milestones/` 目录
- OFF TRACK 评级需触发里程碑范围调整讨论

## 推荐下一步

使用 `/gate-check` 验证里程碑内各阶段门禁。使用 `/sprint-plan` 调整后续 Sprint 以追赶进度。使用 `/release-checklist` 在里程碑交付前验证发布检查清单。