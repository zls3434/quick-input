<!--
  技能定义模板
  用途：通过 /create-skill 工作流创建新技能时使用此模板
  填写指南：替换所有 [占位符] 为实际内容
  规范参考：.studio/skills/_schema.yaml
  名称要求：1-64 字符，小写字母+数字+连字符，不与现有技能重复
-->

---
状态: 草稿
作者: [提案 Agent 名称]
更新日期: [YYYY-MM-DD]
---

name: [skill-kebab-case-name]
description: "[技能描述，1-1024 字符，说明技能的目的和触发场景]"
license: MIT
metadata:
  model: sonnet           # sonnet/haiku/opus，根据技能复杂度选择
  argument-hint: "[参数提示]"  # 如 "[技能名]" 或 "" 无参数时留空
  user-invocable: true    # 是否用户可调用
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
    windsurf: {enabled: true, trigger: /[skill-name]}
    trae: {enabled: true}
    hermes: {enabled: true, platforms: [macos, linux, windows]}
    workbuddy: {enabled: true}
---

# [skill-name] — [技能中文名]

## 技能目的

[一段话描述技能的核心目的：解决什么问题、在什么场景下使用、产出什么结果。]

## 参数说明

- `[参数名]`：[参数说明，包括格式要求和示例]

## 分阶段工作流

### 阶段 1：[阶段名称]

- **输入**：[此阶段的输入来源]
- **处理**：
  1. [处理步骤 1]
  2. [处理步骤 2]
  3. [处理步骤 3]
- **输出**：[此阶段的输出产物]

### 阶段 2：[阶段名称]

- **输入**：[此阶段的输入来源]
- **处理**：
  1. [处理步骤 1]
  2. [处理步骤 2]
- **输出**：[此阶段的输出产物]

### 阶段 3：[阶段名称]

- **输入**：[此阶段的输入来源]
- **处理**：
  1. [处理步骤 1]
  2. [处理步骤 2]
- **输出**：[此阶段的输出产物]

## 协作协议引用

- 遵循 `docs/examples/COLLABORATIVE-DESIGN-PRINCIPLE.md` 用户驱动协作协议
- [其他需要引用的规范文档]
- 技能变更触发 `validate-skill-change.sh` Hook

## 推荐下一步

[技能执行完成后推荐的后续操作，如运行其他技能命令、检查产物文件等。]