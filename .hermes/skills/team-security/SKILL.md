---
name: team-security
description: "编排安全审查团队管线，协调安全负责人、安全工程师、性能工程师与代码审查员按序完成审计计划、漏洞扫描修复、性能影响评估与修复代码审查。"
version: 1.0.0
platforms: [macos, linux, windows]
metadata:
  hermes:
    tags: ["team", "coordination"]
    load_mode: on-demand
    model: sonnet
    argument-hint: "[模块或系统名]"
    user-invocable: true
    allowed-tools:
      - Read
      - Glob
      - Grep
      - Write
      - Bash
      - Task
      - AskUserQuestion
---

# team-security — 安全审查团队编排

## 技能目的

编排安全审查团队协作管线，按既定顺序协调安全相关 Agent 协同工作，将一个模块或系统从当前状态推进到安全漏洞已修复、性能影响已评估、修复代码已通过审查的可信状态。本技能负责调度各 Agent、传递中间产物、收集反馈并在阶段间执行安全门禁。

## 参数说明

- `[模块或系统名]`：待安全审查的模块名称或系统名称，用于确定审查范围与关联代码。

## 分阶段工作流

### 阶段 1：加载上下文与确认范围

- **输入**：`[模块或系统名]` 参数
- **处理**：
  1. 使用 Glob 查找与模块名匹配的源码目录与文件
  2. 使用 Read 读取 `docs/design/architecture*.md` 获取模块架构与数据流说明
  3. 使用 Glob 扫描已有安全文档与合规要求
  4. 使用 Bash 执行依赖安全扫描（如 npm audit / pip audit）获取当前漏洞清单
  5. 通过 AskUserQuestion 与用户确认审查范围、合规标准与修复优先级
- **输出**：安全上下文 + 编排计划

### 阶段 2：security-lead 制定审计计划

- **输入**：安全上下文
- **处理**：使用 Task 分派 security-lead Agent，制定审计范围、检查项清单、合规标准（OWASP / CIS / 等保）与风险评估框架
- **输出**：安全审计计划，写入 `docs/security/audit-plan-[模块].md`

### 阶段 3：security-engineer 漏洞扫描与修复

- **输入**：审计计划
- **处理**：使用 Task 分派 security-engineer Agent，执行静态代码分析、动态扫描、漏洞验证与修复实施
- **输出**：漏洞清单 + 修复代码 + 扫描报告

### 阶段 4：performance-engineer 性能影响评估

- **输入**：修复代码 + 扫描报告
- **处理**：使用 Task 分派 performance-engineer Agent，评估安全修复对性能的影响，执行基准测试对比修复前后指标
- **输出**：性能影响评估报告

### 阶段 5：code-reviewer 修复代码审查

- **输入**：修复代码 + 性能影响评估
- **处理**：使用 Task 分派 code-reviewer Agent，审查修复代码的质量、是否引入新缺陷、是否符合编码规范与安全编码指南
- **输出**：修复代码审查报告

### 阶段 6：汇总与门禁

- **输入**：全部阶段产物
- **处理**：
  1. 汇总各阶段产出与反馈
  2. 检查所有阶段是否通过安全门禁
  3. 对照审计计划核对所有检查项完成情况
  4. 通过 AskUserQuestion 向用户报告审查结论并确认返工需求
  5. 使用 Write 写入编排报告至 `docs/reports/team-security-[模块].md`
- **输出**：安全审查团队编排报告

## 协作协议引用

- 遵循 `docs/examples/COLLABORATIVE-DESIGN-PRINCIPLE.md` 用户驱动协作协议
- 各 Agent 之间通过 Task 工具分派，中间产物以文件形式传递
- 使用 Bash 执行安全扫描、依赖审计与基准测试命令
- 阶段间设置安全门禁，未通过则回退至上一阶段
- 漏洞修复不得引入性能回退超过约定阈值

## 推荐下一步

安全审查完成后，使用 `/team-qa` 执行完整回归验证。性能进一步优化使用 `/team-polish`。发布安全修复使用 `/team-release`。持续安全监控使用 `/team-devops` 配置告警。