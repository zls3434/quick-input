---
name: create-skill
description: "创建新技能，通过提案-草稿-审核-批准-注册-同步六阶段工作流，确保新技能定义符合 Agent Skills 开放标准和项目模式规范。"
license: MIT
metadata:
  model: sonnet
  argument-hint: "[技能名称]"
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
    windsurf: {enabled: true, trigger: /create-skill}
    trae: {enabled: true}
    hermes: {enabled: true, platforms: [macos, linux, windows]}
    workbuddy: {enabled: true}
---

# create-skill — 创建新技能

## 技能目的

引导部门负责人层 Agent 创建新技能（Skill），通过六阶段工作流确保新技能定义符合 `_schema.yaml` 模式规范、名称合规、不与现有技能功能重复，并经对应部门负责人初审和 chief-architect 终审后由用户批准。

## 参数说明

- `[技能名称]`：待创建技能的 kebab-case 名称（1-64 字符，小写字母+数字+连字符）。如未提供，在提案阶段通过交互确定。

## 分阶段工作流

### 阶段 1：提案

- **输入**：`[技能名称]` 参数 + 用户需求描述
- **处理**：
  1. 使用 Read 读取 `.studio/skills/_schema.yaml`，确认技能定义模式
  2. 使用 Glob 列出 `.studio/skills/*/SKILL.md`，获取现有技能列表
  3. 使用 Read 读取 `.studio/skills/_catalog.yaml`，确认名称不重复
  4. 校验技能名称：kebab-case 格式、1-64 字符、不与现有技能重复
  5. 使用 Read 读取 `.studio/templates/asset-proposal.md` 模板
  6. 生成提案文档，包含：技能目的、触发场景、目标用户、所需工具、模型层级建议、所属工作流阶段分类
  7. 通过 AskUserQuestion 向用户展示提案，询问："我可以将提案写入 `.studio/registry/proposals/skill-NNN-[名称]-proposal.md` 吗？"
  8. 用户同意后，使用 Write 写入提案文档
  9. 读取并更新 `.studio/registry/asset-registry.yaml`，新增条目（status: proposed）
- **输出**：提案文档 + 注册表更新

### 阶段 2：草稿

- **输入**：阶段 1 的提案文档
- **处理**：
  1. 使用 Read 读取 `.studio/templates/skill-definition.md` 模板
  2. 使用 Read 读取 1-2 个现有技能（如 `skill-improve/SKILL.md`）作为格式参考
  3. 基于提案内容填充技能定义草稿：
     - YAML frontmatter：name、description、license、metadata（model/argument-hint/user-invocable/allowed-tools/platforms）
     - Markdown body 五章节：技能目的、参数说明、分阶段工作流（每阶段含输入/处理/输出）、协作协议引用、推荐下一步
  4. 通过 AskUserQuestion 向用户展示技能定义草稿，询问修改意见
  5. 根据用户反馈修改草稿
  6. 更新 `asset-registry.yaml`（status: draft）
- **输出**：技能定义草稿

### 阶段 3：审核

- **输入**：阶段 2 的技能定义草稿 + 提案文档
- **处理**：
  1. 提示用户调用 `/asset-review skill [技能名称]` 进行审核
  2. 审核由对应部门负责人初审 → chief-architect 终审
  3. 审核检查项：
     - 名称合规（kebab-case、1-64 字符、不与现有重复）
     - frontmatter 完整（name/description/model/allowed-tools）
     - Body 完整（技能目的/参数说明/分阶段工作流/协作协议引用/推荐下一步）
     - 模型层级合理
     - 不与现有技能功能重复
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

- **输入**：阶段 4 的批准决定 + 技能定义草稿
- **处理**：
  1. 通过 AskUserQuestion 向用户展示所有待写入文件列表，请求批量审批：
     - `.studio/skills/[名称]/SKILL.md`（新建技能定义）
     - `.studio/skills/_catalog.yaml`（更新技能目录）
     - `.studio/docs/skills-reference.md`（更新技能说明）
     - `.studio/registry/asset-registry.yaml`（更新状态）
     - `.studio/registry/asset-changelog.md`（追加变更记录）
  2. 用户批准后，使用 Write 写入技能定义文件到 `.studio/skills/[名称]/SKILL.md`
  3. 使用 Read + Write 更新 `_catalog.yaml`：在对应分类下新增条目、更新 total 数量
  4. 使用 Read + Write 更新 `skills-reference.md`：新增技能说明、更新合计数字
  5. 使用 Read + Write 更新 `asset-registry.yaml`：status → registered，填写 approval_date
  6. 使用 Read + Write 更新 `asset-changelog.md`：追加 created 条目
- **输出**：注册完成的技能文件 + 更新的索引文件

### 阶段 6：同步与验证

- **输入**：阶段 5 的注册结果
- **处理**：
  1. 提示用户运行 `/skill-test [技能名称]` 验证新技能合规性
  2. 提示用户运行 `/sync-platforms` 将新技能同步到所有平台
  3. 同步完成后提示运行 `/platform-check` 验证各平台配置一致性
- **输出**：验证与同步指引

## 协作协议引用

- 遵循 `docs/examples/COLLABORATIVE-DESIGN-PRINCIPLE.md` 用户驱动协作协议
- 在使用 Write 工具前必须询问用户："我可以将此写入 [文件路径] 吗？"
- 多文件修改需要针对完整变更集的明确审批
- 技能变更触发 `validate-skill-change.sh` Hook
- 技能变更需通过 chief-architect 审批
- 参考 `docs/extension-mechanism.md` 动态扩展机制规范
- 参考 `skills/_schema.yaml` 技能定义模式

## 推荐下一步

- 运行 `/asset-review skill [技能名称]` 审核新技能（阶段 3）
- 运行 `/skill-test [技能名称]` 验证技能合规性（阶段 6）
- 运行 `/sync-platforms` 将新技能同步到所有平台（阶段 6）
- 如发现问题，运行 `/skill-improve [技能名称]` 改进技能