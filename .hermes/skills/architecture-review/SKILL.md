---
name: architecture-review
description: "验证架构完整性、依赖排序与技术栈兼容性，两阶段检查并引导生成需求可追溯矩阵。"
version: 1.0.0
platforms: [macos, linux, windows]
metadata:
  hermes:
    tags: ["review", "quality"]
    load_mode: on-demand
    model: opus
    argument-hint: ""
    user-invocable: true
    allowed-tools:
      - Read
      - Glob
      - Grep
      - Write
      - Task
---

# architecture-review — 架构完整性审查

## 技能目的

对架构文档与所有 ADR 进行全面审查，验证架构完整性、依赖排序正确性、技术栈兼容性，并引导生成技术需求（TR）可追溯矩阵，确保架构设计覆盖所有 SRS 需求。

## 参数说明

本技能无参数。自动读取架构文档与所有 ADR。

## 分阶段工作流

### 阶段 1：完整性检查

- **输入**：项目目录
- **处理**：
  1. 使用 Read 读取 `docs/architecture-doc.md`
  2. 使用 Glob 扫描 `docs/adr/*.md` 获取所有 ADR
  3. 使用 Read 读取每个 ADR 的状态与决策内容
  4. 使用 Grep 检查架构文档中引用的 ADR 是否全部存在
  5. 使用 Read 读取 `docs/adr-index.md` 对照必需 ADR 列表
  6. 验证所有 SRS 是否都有对应 ADR 覆盖
- **输出**：完整性检查报告（缺失 ADR、未覆盖 SRS）

### 阶段 2：依赖排序验证

- **输入**：架构文档 + ADR 集合
- **处理**：
  1. 使用 Grep 提取各 SRS 中的模块依赖关系
  2. 使用 Grep 提取架构文档中的容器/组件依赖
  3. 构建依赖图并检测循环依赖
  4. 验证依赖排序是否与 `docs/systems-index.md` 一致
  5. 检查分层架构中是否存在跨层引用违规
- **输出**：依赖排序验证报告

### 阶段 3：技术栈兼容性验证

- **输入**：架构文档 + ADR + 技术偏好
- **处理**：
  1. 使用 Read 读取 `docs/technical-preferences.md`
  2. 对照 ADR 中的技术选型与技术偏好是否一致
  3. 检查前后端技术栈、数据库、部署平台的兼容性
  4. 识别潜在技术冲突并标注风险
- **输出**：技术栈兼容性报告

### 阶段 4：需求可追溯矩阵引导

- **输入**：所有 SRS + 架构文档 + ADR
- **处理**：
  1. 使用 Task 分派子任务提取每条需求的技术实现方案
  2. 构建 SRS 需求 → ADR 决策 → 架构组件的追溯矩阵
  3. 标记未追溯到的需求
  4. 通过 Write 生成 `docs/architecture/traceability-matrix.md`
- **输出**：需求可追溯矩阵文件

### 阶段 5：综合审查结论

- **输入**：以上所有检查结果
- **处理**：
  1. 汇总完整性、依赖排序、兼容性、可追溯性四个维度结论
  2. 使用 Write 生成 `docs/architecture-review-report.md`
  3. 给出 PASS / CONCERNS / FAIL 结论
- **输出**：架构审查报告

## 协作协议引用

- 遵循 `.claude/docs/templates/collaborative-protocols/design-agent-protocol.md` 架构文档规范
- 遵循 `.claude/docs/templates/collaborative-protocols/design-agent-protocol.md` 审查规范
- 审查报告写入前需向用户展示摘要

## 推荐下一步

根据审查报告修复问题 ADR 或补充缺失 ADR。通过后使用 `/create-control-manifest` 生成控制清单，或进入技术设计阶段使用 `/design-system-spec`、`/api-spec` 等技能。