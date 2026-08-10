---
name: create-agent
description: "创建新专家 Agent，通过提案-草稿-审核-批准-注册-同步六阶段工作流，确保新 Agent 定义符合模式规范并经总监层审核。"
license: MIT
metadata:
  model: sonnet
  argument-hint: "[Agent名称]"
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
    windsurf: {enabled: true, trigger: /create-agent}
    trae: {enabled: true}
    hermes: {enabled: true, platforms: [macos, linux, windows]}
    workbuddy: {enabled: true}
---

# create-agent — 创建新专家 Agent

## 技能目的

引导总监层或部门负责人层 Agent 创建新的专家层 Agent，通过六阶段工作流（提案→草稿→审核→批准→注册→同步）确保新 Agent 定义符合 `_schema.yaml` 模式规范、职责不与现有 Agent 重叠、并经 chief-architect 审核和用户最终批准。

## 参数说明

- `[Agent名称]`：待创建 Agent 的 kebab-case 名称（如 `rust-specialist`）。如未提供，在提案阶段通过交互确定。

## 分阶段工作流

### 阶段 1：提案

- **输入**：`[Agent名称]` 参数 + 用户需求描述
- **处理**：
  1. 使用 Read 读取 `.studio/agents/_schema.yaml`，确认 Agent 定义模式
  2. 使用 Glob 列出 `.studio/agents/*.md`，获取现有 Agent 列表
  3. 使用 Read 读取 2-3 个相关现有 Agent 定义，理解职责边界
  4. 使用 Read 读取 `.studio/templates/asset-proposal.md` 模板
  5. 生成提案文档，包含：需求背景、新 Agent 职责边界、与现有 Agent 的协作关系、职责不重叠分析、提案 Agent 身份、所属部门负责人
  6. 通过 AskUserQuestion 向用户展示提案，询问："我可以将提案写入 `.studio/registry/proposals/agent-NNN-[名称]-proposal.md` 吗？"
  7. 用户同意后，使用 Write 写入提案文档
  8. 读取并更新 `.studio/registry/asset-registry.yaml`，新增条目（status: proposed）
- **输出**：提案文档 + 注册表更新

### 阶段 2：草稿

- **输入**：阶段 1 的提案文档
- **处理**：
  1. 使用 Read 读取 `.studio/templates/agent-definition.md` 模板
  2. 基于提案内容填充 Agent 定义草稿：
     - YAML frontmatter：name、description、tools、model（专家层默认 sonnet）、maxTurns、skills、platforms
     - Markdown body 六章节：角色描述、技术专长领域、编码规范要点、关键职责、决策框架、协作协议、委托地图、不得做的事情
  3. 通过 AskUserQuestion 向用户展示 Agent 定义草稿，询问修改意见
  4. 根据用户反馈修改草稿
  5. 更新 `asset-registry.yaml`（status: draft）
- **输出**：Agent 定义草稿

### 阶段 3：审核

- **输入**：阶段 2 的 Agent 定义草稿 + 提案文档
- **处理**：
  1. 提示用户调用 `/asset-review agent [Agent名称]` 进行审核
  2. 等待审核完成，审核由 chief-architect 执行
  3. 审核检查项：
     - 职责不与现有 40 个 Agent 重叠
     - 模型层级合理（专家层 → sonnet/haiku）
     - 委托图完整（有上级、有协作对象）
     - 协作协议明确
     - Body 六章节完整
     - platforms 配置正确
- **输出**：审核报告（APPROVE/CONCERNS/REJECT）

### 阶段 4：批准/拒绝

- **输入**：阶段 3 的审核报告
- **处理**：
  1. 通过 AskUserQuestion 向用户展示审核报告
  2. 根据审核判定执行：
     - **APPROVE**：询问用户是否最终批准，用户批准 → 进入阶段 5；用户拒绝 → 记录拒绝原因，流程终止
     - **CONCERNS**：建议修改草稿后重新审核（返回阶段 2）
     - **REJECT**：记录拒绝原因，流程终止
  3. 更新 `asset-registry.yaml`（status: approved 或 rejected）
- **输出**：批准或拒绝决定

### 阶段 5：注册

- **输入**：阶段 4 的批准决定 + Agent 定义草稿
- **处理**：
  1. 通过 AskUserQuestion 向用户展示所有待写入文件列表，请求批量审批：
     - `.studio/agents/[名称].md`（新建 Agent 定义）
     - `.studio/agents/_roster.md`（更新花名册）
     - `.studio/manifest.yaml`（更新 Agent 数量）
     - `.studio/registry/asset-registry.yaml`（更新状态）
     - `.studio/registry/asset-changelog.md`（追加变更记录）
  2. 用户批准后，使用 Write 写入 Agent 定义文件到 `.studio/agents/[名称].md`
  3. 使用 Read + Write 更新 `_roster.md`：在专家层对应分类表格中新增行、更新统计数字
  4. 使用 Read + Write 更新 `manifest.yaml`：更新 Agent 数量
  5. 使用 Read + Write 更新 `asset-registry.yaml`：status → registered，填写 approval_date
  6. 使用 Read + Write 更新 `asset-changelog.md`：追加 created 条目
- **输出**：注册完成的 Agent 文件 + 更新的索引文件

### 阶段 6：同步

- **输入**：阶段 5 的注册结果
- **处理**：
  1. 提示用户运行 `/sync-platforms` 将新 Agent 同步到所有平台
  2. 同步完成后提示运行 `/platform-check` 验证各平台配置一致性
- **输出**：同步指引

## 协作协议引用

- 遵循 `docs/examples/COLLABORATIVE-DESIGN-PRINCIPLE.md` 用户驱动协作协议
- 在使用 Write 工具前必须询问用户："我可以将此写入 [文件路径] 吗？"
- 多文件修改需要针对完整变更集的明确审批
- 参考 `docs/extension-mechanism.md` 动态扩展机制规范
- 参考 `agents/_schema.yaml` Agent 定义模式

## 推荐下一步

- 运行 `/asset-review agent [Agent名称]` 审核新 Agent（阶段 3）
- 运行 `/sync-platforms` 将新 Agent 同步到所有平台（阶段 6）
- 运行 `/platform-check` 验证各平台配置一致性