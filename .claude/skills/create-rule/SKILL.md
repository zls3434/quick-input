---
name: create-rule
description: "创建新路径规则，通过提案-草稿-审核-批准-注册-同步六阶段工作流，确保新规则符合路径规则模式规范并不与现有规则冲突。"
license: MIT
metadata:
  model: sonnet
  argument-hint: "[规则名称]"
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
    windsurf: {enabled: true, trigger: /create-rule}
    trae: {enabled: true}
    hermes: {enabled: true, platforms: [macos, linux, windows]}
    workbuddy: {enabled: true}
---

# create-rule — 创建新路径规则

## 技能目的

引导部门负责人层 Agent 创建新路径规则（Rule），通过六阶段工作流确保新规则符合 `_schema.yaml` 模式规范、路径范围不与现有 11 个规则冲突、规范条目清晰可执行，并经 tech-architect 初审和 chief-architect 终审后由用户批准。

## 参数说明

- `[规则名称]`：待创建规则的 kebab-case 名称（如 `mobile-code`）。如未提供，在提案阶段通过交互确定。

## 分阶段工作流

### 阶段 1：提案

- **输入**：`[规则名称]` 参数 + 用户需求描述
- **处理**：
  1. 使用 Read 读取 `.studio/rules/_schema.yaml`，确认规则定义模式
  2. 使用 Glob 列出 `.studio/rules/*.md`，获取现有规则列表
  3. 使用 Read 读取 1-2 个现有规则（如 `api-code.md`）作为格式参考
  4. 确认新规则名称不与现有规则重复
  5. 使用 Read 读取 `.studio/templates/asset-proposal.md` 模板
  6. 生成提案文档，包含：规则目的、适用路径范围、与现有规则的关系（补充/独立/覆盖）、所属部门
  7. 通过 AskUserQuestion 向用户展示提案，询问："我可以将提案写入 `.studio/registry/proposals/rule-NNN-[名称]-proposal.md` 吗？"
  8. 用户同意后，使用 Write 写入提案文档
  9. 读取并更新 `.studio/registry/asset-registry.yaml`，新增条目（status: proposed）
- **输出**：提案文档 + 注册表更新

### 阶段 2：草稿

- **输入**：阶段 1 的提案文档
- **处理**：
  1. 使用 Read 读取 `.studio/templates/rule-definition.md` 模板
  2. 基于提案内容填充规则定义草稿：
     - YAML frontmatter：paths（glob 模式列表）、platforms（7 平台配置）
     - Markdown body 三章节：规则标题、规范条目（逐条列出，用"必须/禁止/应该"约束语言）、示例（正确+错误代码块）
  3. 通过 AskUserQuestion 向用户展示规则定义草稿，询问修改意见
  4. 根据用户反馈修改草稿
  5. 更新 `asset-registry.yaml`（status: draft）
- **输出**：规则定义草稿

### 阶段 3：审核

- **输入**：阶段 2 的规则定义草稿 + 提案文档
- **处理**：
  1. 提示用户调用 `/asset-review rule [规则名称]` 进行审核
  2. 审核由 tech-architect 初审 → chief-architect 终审
  3. 审核检查项：
     - 路径范围不与现有 11 个规则冲突
     - 规范条目清晰可执行（用"必须/禁止/应该"约束语言）
     - 示例完整（正确示例 + 错误示例）
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

- **输入**：阶段 4 的批准决定 + 规则定义草稿
- **处理**：
  1. 通过 AskUserQuestion 向用户展示所有待写入文件列表，请求批量审批：
     - `.studio/rules/[名称].md`（新建规则定义）
     - `.studio/docs/rules-reference.md`（更新规则说明，如存在）
     - `.studio/registry/asset-registry.yaml`（更新状态）
     - `.studio/registry/asset-changelog.md`（追加变更记录）
  2. 用户批准后，使用 Write 写入规则定义文件到 `.studio/rules/[名称].md`
  3. 使用 Read + Write 更新 `rules-reference.md`（如存在）：新增规则说明
  4. 使用 Read + Write 更新 `asset-registry.yaml`：status → registered，填写 approval_date
  5. 使用 Read + Write 更新 `asset-changelog.md`：追加 created 条目
- **输出**：注册完成的规则文件 + 更新的索引文件

### 阶段 6：同步

- **输入**：阶段 5 的注册结果
- **处理**：
  1. 提示用户运行 `/sync-platforms` 将新规则同步到所有平台
  2. 同步完成后提示运行 `/platform-check` 验证各平台配置一致性
- **输出**：同步指引

## 协作协议引用

- 遵循 `docs/examples/COLLABORATIVE-DESIGN-PRINCIPLE.md` 用户驱动协作协议
- 在使用 Write 工具前必须询问用户："我可以将此写入 [文件路径] 吗？"
- 多文件修改需要针对完整变更集的明确审批
- 规则变更需通过 chief-architect 审批
- 参考 `docs/extension-mechanism.md` 动态扩展机制规范
- 参考 `rules/_schema.yaml` 规则定义模式

## 推荐下一步

- 运行 `/asset-review rule [规则名称]` 审核新规则（阶段 3）
- 运行 `/sync-platforms` 将新规则同步到所有平台（阶段 6）
- 运行 `/platform-check` 验证各平台配置一致性