---
name: test-flakiness
description: "从 CI 历史中检测不稳定测试，统计通过/失败率，标记高变异性测试，分析根本原因并建议修复或隔离。"
license: MIT
metadata:
  model: sonnet
  argument-hint: ""
  user-invocable: true
  allowed-tools:
    - Read
    - Glob
    - Grep
    - Write
    - Bash
  platforms:
    claude-code: {enabled: true}
    cursor: {enabled: true}
    codex: {enabled: true}
    windsurf: {enabled: true, trigger: /test-flakiness}
    trae: {enabled: true}
    hermes: {enabled: true, platforms: [macos, linux, windows]}
    workbuddy: {enabled: true}
---

# test-flakiness — 不稳定测试检测

## 技能目的

从持续集成（CI）历史记录中检测不稳定的测试用例（Flaky Tests），通过统计每个测试的通过/失败率、标记高变异性测试、分析根本原因，给出修复或隔离建议，提升测试套件的可靠性与可信度。

## 参数说明

本技能无参数。自动从 CI 系统获取测试历史数据进行分析。

## 分阶段工作流

### 阶段 1：读取 CI 测试历史

- **输入**：项目目录 + CI 配置
- **处理**：
  1. 使用 Glob 查找 CI 配置文件（`.github/workflows/*.yml`、`.gitlab-ci.yml`、`Jenkinsfile`）
  2. 使用 Read 读取 CI 配置确认测试任务名称与产物路径
  3. 使用 Bash 执行 CI 命令或 API 调用获取最近 N 次构建的测试结果
  4. 使用 Glob 查找本地测试报告产物（`*.xml`、`*.json`、`test-results/`）
  5. 使用 Read 读取测试报告解析每个测试的执行状态
- **输出**：测试历史数据集

### 阶段 2：统计每个测试的通过/失败率

- **输入**：测试历史数据集
- **处理**：
  1. 使用 Bash 脚本汇总每个测试用例在历史构建中的通过/失败次数
  2. 计算每个测试的通过率、失败率与变异性指标
  3. 识别同一测试在不同构建中结果不一致的情况
  4. 按变异性排序所有测试用例
- **输出**：测试通过/失败统计表

### 阶段 3：标记高变异性测试

- **输入**：统计表
- **处理**：
  1. 将变异系数超过阈值的测试标记为不稳定测试
  2. 将最近 N 次构建中结果不一致的测试标记为疑似不稳定
  3. 排除因真实缺陷导致的稳定失败测试
  4. 汇总不稳定测试清单及其波动模式
- **输出**：不稳定测试清单

### 阶段 4：分析根本原因

- **输入**：不稳定测试清单
- **处理**：
  1. 使用 Read 读取不稳定测试的源码与测试代码
  2. 使用 Grep 搜索常见不稳定模式（时间依赖、顺序依赖、共享状态、外部服务调用、随机数据）
  3. 使用 Bash 查看失败构建的日志定位错误信息
  4. 为每个不稳定测试标注可能的原因分类
- **输出**：根本原因分析报告

### 阶段 5：建议修复或隔离

- **输入**：根本原因分析
- **处理**：
  1. 针对每种原因类型给出修复建议（如增加等待、隔离状态、mock 外部服务）
  2. 对短期内难以修复的测试建议隔离或标记为非阻塞
  3. 使用 Write 写入报告至 `docs/reports/test-flakiness.md`
- **输出**：不稳定测试检测与修复建议报告

## 协作协议引用

- 遵循 `docs/examples/COLLABORATIVE-DESIGN-PRINCIPLE.md` 用户驱动协作协议
- 参考 `docs/rules/test-standards.md` 测试规范
- 使用 Bash 执行 CI 命令、脚本与日志查询
- 检测为只读分析，不直接修改测试代码

## 推荐下一步

根据修复建议修补不稳定测试，修复后使用 `/team-qa` 执行完整回归验证。测试基础设施改进使用 `/test-setup`。持续监控使用 `/team-devops` 配置 CI 告警。