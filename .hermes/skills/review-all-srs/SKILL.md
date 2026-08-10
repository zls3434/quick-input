---
name: review-all-srs
description: "跨SRS一致性检查。两阶段并行：一致性检查(交叉引用验证)与需求分析审查(用户故事完整性/边界情况覆盖)。"
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

# review-all-srs — 跨 SRS 一致性检查

## 技能目的

对项目中所有 SRS 文档进行两阶段并行审查：一致性检查（交叉引用验证、术语统一）与需求分析审查（用户故事完整性、边界情况覆盖），产出综合审查报告。

## 参数说明

本技能无参数。自动扫描所有 SRS 文件。

## 分阶段工作流

### 阶段 1：收集所有 SRS

- **输入**：项目目录
- **处理**：使用 Glob 扫描 `docs/srs/*.md`，使用 Read 读取所有 SRS 文件内容
- **输出**：SRS 文件集合

### 阶段 2：并行两阶段审查

- **输入**：SRS 文件集合
- **处理**：使用 Task 启动两个并行子任务：
  1. **一致性检查**：交叉引用验证（引用的模块是否存在）、术语统一性（同一概念是否使用不同名称）、接口定义一致性
  2. **需求分析审查**：用户故事完整性（是否有验收标准）、边界情况覆盖（异常路径、空状态、权限不足等）
- **输出**：两份审查结果

### 阶段 3：汇总报告

- **输入**：两份审查结果
- **处理**：合并为综合审查报告，按严重程度排序问题（阻断/重要/建议），使用 Write 写入 `docs/reviews/srs-review.md`
- **输出**：综合审查报告文件

## 协作协议引用

- 遵循 `.claude/docs/templates/collaborative-protocols/design-agent-protocol.md` 审查规范
- 参考 `.claude/docs/templates/collaborative-protocols/design-agent-protocol.md` SRS 规范

## 推荐下一步

根据审查报告修复阻断与重要问题，修复后使用 `/consistency-check` 再次验证。全部通过后进入架构设计阶段，使用 `/create-architecture`。