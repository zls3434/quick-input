---
name: project-stage-detect
description: "分析项目状态、检测当前阶段、识别各阶段差距，输出阶段诊断报告。"
version: 1.0.0
platforms: [macos, linux, windows]
metadata:
  hermes:
    tags: ["planning", "project"]
    load_mode: on-demand
    model: haiku
    argument-hint: ""
    user-invocable: true
    allowed-tools:
      - Read
      - Glob
      - Grep
---

# project-stage-detect — 项目阶段检测

## 技能目的

全面扫描项目目录，对照工作流目录检测当前所处阶段，识别各阶段产物是否存在及完整度，输出结构化阶段诊断报告。

## 参数说明

本技能无参数。

## 分阶段工作流

### 阶段 1：加载工作流定义

- **输入**：`.claude/workflow-catalog.yaml`
- **处理**：使用 Read 读取并解析所有阶段、必需产物、阶段依赖
- **输出**：工作流定义对象

### 阶段 2：扫描产物

- **输入**：工作流定义 + 项目根目录
- **处理**：
  1. 使用 Glob 按阶段扫描期望产物文件
  2. 对存在的文件使用 Grep 检查是否包含实质性内容（非空模板）
  3. 标记每个产物状态：存在且完整 / 存在但不完整 / 缺失
- **输出**：产物状态清单

### 阶段 3：识别差距

- **输入**：产物状态清单
- **处理**：对照各阶段必需产物列表，识别缺失或不完整的产物，计算阶段完成百分比
- **输出**：差距清单 + 完成度指标

### 阶段 4：输出阶段报告

- **输入**：差距清单 + 完成度
- **处理**：生成文本报告，包含：当前阶段判定、各阶段完成度条形图、关键缺失产物、推荐修复动作
- **输出**：阶段诊断报告（直接输出到对话）

## 协作协议引用

- 参考 `.claude/docs/templates/collaborative-protocols/design-agent-protocol.md` 中阶段判定规则

## 推荐下一步

根据报告中标识的最关键缺失产物，执行对应的技能补全。可使用 `/help` 获取技能推荐。