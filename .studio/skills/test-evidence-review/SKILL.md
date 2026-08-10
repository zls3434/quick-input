---
name: test-evidence-review
description: "审查测试文件与手动证据质量，按 ADEQUATE/INCOMPLETE/MISSING 评级并生成报告。"
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
  platforms:
    claude-code: {enabled: true}
    cursor: {enabled: true}
    codex: {enabled: true}
    windsurf: {enabled: true, trigger: /test-evidence-review}
    trae: {enabled: true}
    hermes: {enabled: true, platforms: [macos, linux, windows]}
    workbuddy: {enabled: true}
---

# test-evidence-review — 测试证据质量审查

## 技能目的

对测试文件与手动测试证据进行质量审查，按 ADEQUATE（充分）/ INCOMPLETE（不完整）/ MISSING（缺失）三级评定，生成审查报告，确保进入下一阶段前测试证据满足质量门槛。

## 参数说明

本技能无参数。自动收集并审查测试证据。

## 分阶段工作流

### 阶段 1：收集测试文件

- **输入**：项目目录
- **处理**：
  1. 使用 Glob 查找所有测试文件（`*.test.*`、`*.spec.*`、`tests/**`）
  2. 使用 Glob 查找手动证据目录（`docs/test/evidence/`、`docs/manual/`）
  3. 使用 Read 读取 SRS 中的验收标准清单
  4. 使用 Grep 将测试文件与需求编号关联
- **输出**：测试证据清单

### 阶段 2：检查覆盖率

- **输入**：测试证据清单
- **处理**：
  1. 比对 SRS 验收标准与已有测试证据
  2. 标注每条验收标准的证据状态：有自动化测试、有手动证据、无证据
  3. 统计覆盖率：自动化覆盖比例、手动覆盖比例、无证据比例
  4. 识别关键路径上的证据缺口
- **输出**：覆盖率评估表

### 阶段 3：检查测试质量

- **输入**：覆盖率评估表
- **处理**：
  1. 命名规范：用例名称是否表达意图（should-when 风格）
  2. 隔离性：是否依赖外部状态、是否共享可变夹具
  3. 确定性：是否依赖时间、随机数、网络、外部服务
  4. 断言充分性：是否仅检查无异常而非断言结果
  5. 可维护性：是否过度 Mock、是否脆性测试
- **输出**：测试质量评分表

### 阶段 4：评估手动证据

- **输入**：测试质量评分表
- **处理**：
  1. 使用 Read 读取手动测试记录（截图、录屏说明、检查表）
  2. 检查证据是否标注执行人、执行时间、环境版本
  3. 检查证据是否覆盖异常路径与边界值
  4. 检查证据是否可复现（含步骤与数据）
  5. 评估证据时效性：是否对应当前版本
- **输出**：手动证据评估表

### 阶段 5：生成评级报告

- **输入**：全部评估表
- **处理**：
  1. 对每条验收标准给出评级：ADEQUATE / INCOMPLETE / MISSING
  2. 汇总各评级数量与占比
  3. 对 INCOMPLETE 列出补全建议
  4. 对 MISSING 列出必需补齐的证据清单
  5. 使用 Write 生成报告 `docs/test/evidence-review.md`
- **输出**：测试证据审查报告

## 协作协议引用

- 遵循 `.claude/docs/coding-standards.md` 编码规范
- 审查报告写入 `docs/test/` 目录
- MISSING 证据阻塞阶段推进，需补齐后方可重新审查

## 推荐下一步

使用 `/regression-suite` 补齐缺失的自动化用例。使用 `/gate-check` 在证据补齐后重新评估。使用 `/smoke-check` 执行快速验证。