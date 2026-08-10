---
name: team-polish
description: "编排优化打磨团队管线，协调性能工程师、安全工程师、代码审查员与测试工程师按序完成性能优化、安全加固、代码质量与补充测试。"
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

# team-polish — 优化打磨团队编排

## 技能目的

编排优化打磨团队协作管线，按既定顺序协调优化相关 Agent 协同工作，将一个模块或系统从当前状态推进到性能、安全、代码质量与测试覆盖均达标的高质量状态。本技能负责调度各 Agent、传递中间产物、收集反馈并在阶段间执行质量门禁。

## 参数说明

- `[模块或系统名]`：待优化打磨的模块名称或系统名称，用于确定优化范围与关联代码。

## 分阶段工作流

### 阶段 1：加载上下文与确认范围

- **输入**：`[模块或系统名]` 参数
- **处理**：
  1. 使用 Glob 查找与模块名匹配的源码目录与文件
  2. 使用 Read 读取 `docs/design/architecture*.md` 获取模块架构说明
  3. 使用 Glob 扫描已有测试文件与测试报告
  4. 使用 Bash 执行性能基线测试获取当前性能指标
  5. 通过 AskUserQuestion 与用户确认优化目标、优先级与验收标准
- **输出**：优化上下文 + 编排计划

### 阶段 2：performance-engineer 性能优化

- **输入**：优化上下文 + 性能基线
- **处理**：使用 Task 分派 performance-engineer Agent，分析性能瓶颈、提出优化方案、实施优化并复测对比
- **输出**：性能优化报告 + 优化后代码

### 阶段 3：security-engineer 安全加固

- **输入**：优化后代码 + 性能优化报告
- **处理**：使用 Task 分派 security-engineer Agent，检查安全漏洞、加固认证授权、输入验证与依赖安全
- **输出**：安全加固报告 + 加固后代码

### 阶段 4：code-reviewer 代码质量

- **输入**：全部优化后代码
- **处理**：使用 Task 分派 code-reviewer Agent，审查代码可读性、可维护性、设计模式应用与编码规范一致性
- **输出**：代码质量审查报告 + 重构建议

### 阶段 5：test-engineer 补充测试

- **输入**：优化后代码 + 审查报告
- **处理**：使用 Task 分派 test-engineer Agent，补充缺失测试用例、提升覆盖率、执行回归测试确保优化未引入缺陷
- **输出**：补充测试用例 + 测试覆盖率报告

### 阶段 6：汇总与门禁

- **输入**：全部阶段产物
- **处理**：
  1. 汇总各阶段产出与反馈
  2. 检查所有阶段是否通过质量门禁
  3. 对比优化前后指标，确认提升幅度
  4. 通过 AskUserQuestion 向用户报告结果并确认返工需求
  5. 使用 Write 写入编排报告至 `docs/reports/team-polish-[模块].md`
- **输出**：优化打磨团队编排报告

## 协作协议引用

- 遵循 `docs/examples/COLLABORATIVE-DESIGN-PRINCIPLE.md` 用户驱动协作协议
- 各 Agent 之间通过 Task 工具分派，中间产物以文件形式传递
- 使用 Bash 执行性能测试、安全扫描与测试命令
- 阶段间设置质量门禁，未通过则回退至上一阶段
- 优化前后均需保留性能与覆盖率基线用于对比

## 推荐下一步

优化打磨完成后，使用 `/team-qa` 执行完整 QA 周期验证。安全深度审查使用 `/team-security`。发布优化成果使用 `/team-release`。技术债务评估使用 `/tech-debt` 制定长期偿还计划。