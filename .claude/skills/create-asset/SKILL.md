---
name: create-asset
description: "动态扩展机制统一入口，引导用户选择创建新 Agent、新技能或新规则，路由至对应专项工作流。"
license: MIT
metadata:
  model: sonnet
  argument-hint: "[agent|skill|rule]"
  user-invocable: true
  allowed-tools:
    - Read
    - Glob
    - Grep
    - AskUserQuestion
  platforms:
    claude-code: {enabled: true}
    cursor: {enabled: true}
    codex: {enabled: true}
    windsurf: {enabled: true, trigger: /create-asset}
    trae: {enabled: true}
    hermes: {enabled: true, platforms: [macos, linux, windows]}
    workbuddy: {enabled: true}
---

# create-asset — 动态扩展统一入口

## 技能目的

作为 Software Engineering Studios 动态扩展机制的统一入口，根据项目需求引导用户选择创建新专家 Agent、新技能或新路径规则，并路由至对应的专项创建工作流（`/create-agent`、`/create-skill`、`/create-rule`）。通过统一入口降低认知负担，同时展示当前资产状态和最近变更。

## 参数说明

- `[agent|skill|rule]`：可选参数，直接指定要创建的资产类型。如未提供，通过交互方式询问用户选择。

## 分阶段工作流

### 阶段 1：读取资产注册表，展示当前状态

- **输入**：可选参数 `[agent|skill|rule]`
- **处理**：
  1. 使用 Read 读取 `.studio/registry/asset-registry.yaml`，获取当前资产统计信息
  2. 使用 Read 读取 `.studio/registry/asset-changelog.md`，获取最近变更记录
  3. 向用户展示当前资产数量（Agent/Skill/Rule）和最近变更摘要
- **输出**：当前资产状态摘要

### 阶段 2：确定资产类型

- **输入**：阶段 1 的状态摘要 + 可选参数
- **处理**：
  1. 如已提供参数 `[agent|skill|rule]`，直接确认资产类型
  2. 如未提供参数，通过 AskUserQuestion 询问用户要创建的资产类型：
     - **专家 Agent**：当项目需要新的技术栈专家或领域专家时选择，路由至 `/create-agent`
     - **技能（Skill）**：当项目需要新的工作流或操作能力时选择，路由至 `/create-skill`
     - **路径规则（Rule）**：当项目需要新的编码规范或路径约束时选择，路由至 `/create-rule`
  3. 确认用户选择后，提示对应的专项命令
- **输出**：确定的资产类型和对应的专项命令

### 阶段 3：路由至专项工作流

- **输入**：确定的资产类型
- **处理**：
  1. 根据资产类型提示用户调用对应的专项命令：
     - Agent → `/create-agent [Agent名称]`
     - Skill → `/create-skill [技能名称]`
     - Rule → `/create-rule [规则名称]`
  2. 提供简要的专项工作流说明（六阶段：提案→草稿→审核→批准→注册→同步）
- **输出**：路由指引

## 协作协议引用

- 遵循 `docs/examples/COLLABORATIVE-DESIGN-PRINCIPLE.md` 用户驱动协作协议
- 用户始终掌握决策权，本技能仅提供路由引导
- 参考 `docs/extension-mechanism.md` 动态扩展机制规范

## 推荐下一步

根据选择的资产类型，调用对应的专项创建命令：
- 创建 Agent → `/create-agent [Agent名称]`
- 创建 Skill → `/create-skill [技能名称]`
- 创建 Rule → `/create-rule [规则名称]`