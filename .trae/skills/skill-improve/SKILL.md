---
name: skill-improve
description: "使用测试-修复-重测循环改进技能，运行 skill-test 识别问题，提出修复方案，用户批准后重写 SKILL.md 并重测验证。"
license: MIT
metadata:
  model: sonnet
  argument-hint: "[技能名]"
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
    windsurf: {enabled: true, trigger: /skill-improve}
    trae: {enabled: true}
    hermes: {enabled: true, platforms: [macos, linux, windows]}
    workbuddy: {enabled: true}
---

# skill-improve — 技能改进

## 技能目的

使用测试-修复-重测循环改进现有技能定义文件（SKILL.md）的质量。先运行 `/skill-test` 识别合规性与完整性问题，提出修复方案，经用户批准后重写 SKILL.md，再重新测试验证修复效果，确保技能定义持续改进。

## 参数说明

- `[技能名]`：待改进的技能名称，对应 `.claude/skills/[技能名]/SKILL.md`。

## 分阶段工作流

### 阶段 1：运行 skill-test 识别问题

- **输入**：`[技能名]` 参数
- **处理**：
  1. 使用 Glob 查找 `.claude/skills/[技能名]/SKILL.md`
  2. 使用 Read 读取当前 SKILL.md 内容
  3. 执行与 `/skill-test` 相同的检查逻辑：验证 YAML frontmatter 字段（name / description / allowed-tools / model）
  4. 验证 markdown body 完整性（工作流阶段 / 协作协议 / 推荐下一步 / 最少行数）
  5. 汇总全部问题清单
- **输出**：问题清单

### 阶段 2：提出修复方案

- **输入**：问题清单
- **处理**：
  1. 为每个问题设计具体修复方案
  2. 区分必需修复（缺失必需字段）与可选改进（格式优化、内容增强）
  3. 评估每项修复对现有技能行为的影响
  4. 整理修复方案清单
- **输出**：修复方案清单

### 阶段 3：用户批准

- **输入**：修复方案清单
- **处理**：
  1. 通过 AskUserQuestion 向用户展示全部修复方案
  2. 逐项确认用户是否同意修复
  3. 记录用户选择：采纳、跳过或修改方案
  4. 确认最终修复列表
- **输出**：经批准的修复列表

### 阶段 4：重写 SKILL.md

- **输入**：经批准的修复列表 + 当前 SKILL.md
- **处理**：
  1. 基于修复列表逐项修改 SKILL.md 内容
  2. 使用 Write 写入修改后的完整 SKILL.md
  3. 确保修改不破坏原有的正确部分
  4. 保留修改前后的差异记录
- **输出**：更新后的 SKILL.md

### 阶段 5：重测验证

- **输入**：更新后的 SKILL.md
- **处理**：
  1. 重新执行阶段 1 的全部检查逻辑
  2. 对比修复前后的问题清单
  3. 确认所有目标问题已解决
  4. 检查是否引入新问题
  5. 通过 AskUserQuestion 向用户报告验证结果
  6. 如仍有问题，询问是否继续下一轮改进
- **输出**：重测验证报告

## 协作协议引用

- 遵循 `docs/examples/COLLABORATIVE-DESIGN-PRINCIPLE.md` 用户驱动协作协议
- 修改 SKILL.md 前必须获得用户明确批准
- 参考 `docs/skills-reference.md` 技能定义格式规范
- 参考 `docs/templates/skill-test-spec.md` 技能测试规格模板
- 技能变更触发 `validate-skill-change.sh` Hook

## 推荐下一步

改进完成后，使用 `/skill-test` 再次验证合规性。技能变更需通过 chief-architect 审批。新技能创建可参考改进后的格式作为模板。