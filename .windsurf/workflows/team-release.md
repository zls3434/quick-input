# /team-release — 编排发布团队协作管线，协调发布经理、DevOps 负责人、QA 负责人与安全负责人按序完成发布计划、部署准备、质量验证、安全审查与签发。

## 触发方式
在 Cascade 对话框中输入 /team-release 触发此工作流。

## 工作流内容
# team-release — 发布团队编排

## 技能目的

编排完整的发布团队协作管线，按既定顺序协调发布相关 Agent 协同工作，将一个版本号从发布计划推进到正式签发的可发布状态。本技能负责调度各 Agent、传递中间产物、收集反馈并在阶段间执行发布门禁。

## 参数说明

- `[版本号]`：待发布的版本号，例如 `v1.2.0`，用于确定发布范围与关联变更集。

## 分阶段工作流

### 阶段 1：加载上下文与确认范围

- **输入**：`[版本号]` 参数
- **处理**：
  1. 使用 Read 读取 `docs/sprints/sprint-*.md` 获取已完成 Sprint 列表
  2. 使用 Glob 查找 `docs/epics/*.md` 获取本版本包含的 Epic
  3. 使用 Read 读取 `docs/technical-preferences.md` 确认部署平台（AWS / Azure / GCP / 自托管）
  4. 使用 Bash 执行 `git log` 获取自上一版本以来的提交记录
  5. 通过 AskUserQuestion 与用户确认发布范围、发布日期与回滚策略
- **输出**：发布上下文 + 编排计划

### 阶段 2：release-manager 发布计划

- **输入**：发布上下文
- **处理**：使用 Task 分派 release-manager Agent，制定发布计划、里程碑、发布内容清单与风险预案，产出发布计划文档
- **输出**：发布计划，写入 `docs/releases/release-plan-[版本号].md`

### 阶段 3：devops-lead 部署准备

- **输入**：发布计划
- **处理**：使用 Task 分派 devops-lead Agent，准备部署环境、配置变更、数据库迁移脚本与回滚方案
- **输出**：部署准备清单与环境配置

### 阶段 4：qa-lead 质量验证

- **输入**：部署准备 + 发布计划
- **处理**：使用 Task 分派 qa-lead Agent，执行回归测试、冒烟测试与验收测试，确认质量门禁达标
- **输出**：质量验证报告

### 阶段 5：security-lead 安全审查

- **输入**：质量验证报告 + 发布内容
- **处理**：使用 Task 分派 security-lead Agent，审查安全合规性、漏洞修复状态与依赖安全
- **输出**：安全审查报告

### 阶段 6：release-manager 签发

- **输入**：全部阶段产物
- **处理**：
  1. 使用 Task 分派 release-manager Agent 汇总各阶段结果
  2. 对照发布门禁评估是否可签发
  3. 生成发布说明与变更日志
  4. 通过 AskUserQuestion 向用户报告签发结论并获取批准
- **输出**：签发报告 + 发布说明

### 阶段 7：汇总与门禁

- **输入**：全部阶段产物
- **处理**：
  1. 汇总各阶段产出与反馈
  2. 检查所有阶段是否通过发布门禁
  3. 使用 Write 写入编排报告至 `docs/reports/team-release-[版本号].md`
- **输出**：发布团队编排报告

## 协作协议引用

- 遵循 `docs/examples/COLLABORATIVE-DESIGN-PRINCIPLE.md` 用户驱动协作协议
- 各 Agent 之间通过 Task 工具分派，中间产物以文件形式传递
- 使用 Bash 执行构建、部署脚本与版本管理命令
- 阶段间设置发布门禁，未通过则回退至上一阶段
- 签发阶段必须获得用户明确批准

## 推荐下一步

发布签发后，使用 `/team-devops` 持续监控部署环境。发布后紧急问题使用 `/day-one-patch` 处理。下一版本规划使用 `/sprint-plan` 启动新 Sprint。发布复盘使用 `/retrospective` 总结经验。