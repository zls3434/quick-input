---
name: asset-review
description: "总监层审核待审的资产提案（Agent/Skill/Rule），对照审核标准逐项检查，生成审核报告，支持 APPROVE/CONCERNS/REJECT 三级判定。"
license: MIT
metadata:
  model: opus
  argument-hint: "[资产类型] [资产名称]"
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
    windsurf: {enabled: true, trigger: /asset-review}
    trae: {enabled: true}
    hermes: {enabled: true, platforms: [macos, linux, windows]}
    workbuddy: {enabled: true}
---

# asset-review — 资产审核

## 技能目的

为 chief-architect（首席架构师）提供资产审核工作流，对待审的新增资产提案（Agent/Skill/Rule）对照审核标准逐项检查，生成结构化审核报告，给出 APPROVE/CONCERNS/REJECT 三级判定和修复建议。审核结果作为用户最终批准的依据。

## 参数说明

- `[资产类型]`：待审核资产类型（agent / skill / rule）
- `[资产名称]`：待审核资产的 kebab-case 名称

## 分阶段工作流

### 阶段 1：读取提案和草稿

- **输入**：`[资产类型]` 和 `[资产名称]` 参数
- **处理**：
  1. 使用 Read 读取 `.studio/registry/asset-registry.yaml`，查找匹配的条目（status 应为 draft）
  2. 从注册表条目中获取提案文档路径和草稿文件路径
  3. 使用 Read 读取提案文档
  4. 使用 Read 读取资产定义草稿：
     - Agent → `.studio/agents/[名称].md`（如已创建临时文件）或从提案中提取草稿内容
     - Skill → `.studio/skills/[名称]/SKILL.md`（如已创建临时文件）或从提案中提取草稿内容
     - Rule → `.studio/rules/[名称].md`（如已创建临时文件）或从提案中提取草稿内容
  5. 获取现有资产列表用于重叠检查
- **输出**：提案文档 + 资产定义草稿 + 现有资产列表

### 阶段 2：对照审核标准检查

- **输入**：阶段 1 的提案和草稿
- **处理**：
  1. 根据资产类型执行对应检查清单：

  **Agent 审核检查清单**：
  - [ ] 职责不与现有 40 个 Agent 重叠（逐个比对职责描述）
  - [ ] 模型层级合理（专家层 → sonnet/haiku，非 opus）
  - [ ] 委托图完整（有上级部门负责人、有协作对象）
  - [ ] 协作协议明确（引用用户驱动协作模式）
  - [ ] Body 六章节完整（角色描述/技术专长领域/编码规范要点/关键职责/决策框架/协作协议/委托地图/不得做的事情）
  - [ ] frontmatter 必需字段完整（name/description/model）
  - [ ] platforms 配置正确（7 平台启用状态合理）

  **Skill 审核检查清单**：
  - [ ] 名称合规（kebab-case、1-64 字符、不与现有 74 个技能重复）
  - [ ] frontmatter 完整（name/description/license/metadata）
  - [ ] metadata 完整（model/argument-hint/user-invocable/allowed-tools/platforms）
  - [ ] Body 完整（技能目的/参数说明/分阶段工作流/协作协议引用/推荐下一步）
  - [ ] 模型层级合理（复杂技能 sonnet/opus，简单技能 haiku）
  - [ ] 不与现有技能功能重复

  **Rule 审核检查清单**：
  - [ ] 路径范围不与现有 11 个规则冲突（比对 paths glob 模式）
  - [ ] 规范条目清晰可执行（用"必须/禁止/应该"约束语言）
  - [ ] 示例完整（正确示例 + 错误示例，代码块标注语言）
  - [ ] frontmatter 必需字段完整（paths）
  - [ ] platforms 配置正确

  2. 记录每项检查结果：PASS / FAIL / WARNING
  3. 汇总 BLOCKING 问题（FAIL）和 ADVISORY 问题（WARNING）
- **输出**：检查结果清单

### 阶段 3：生成审核报告

- **输入**：阶段 2 的检查结果清单
- **处理**：
  1. 按以下结构生成审核报告：

  ```markdown
  ---
  状态: 已完成
  审核者: chief-architect
  更新日期: [YYYY-MM-DD]
  ---

  # 资产审核报告：[资产类型]/[资产名称]

  ## 审核对象
  | 字段 | 值 |
  |------|-----|
  | 资产类型 | [agent/skill/rule] |
  | 资产名称 | [名称] |
  | 提案编号 | [agent-NNN/skill-NNN/rule-NNN] |
  | 提案 Agent | [提案者] |
  | 审核日期 | [YYYY-MM-DD] |

  ## 审核标准与检查结果
  | 序号 | 检查项 | 结果 | 说明 |
  |------|--------|------|------|
  | 1 | [检查项] | PASS/FAIL/WARNING | [说明] |
  | ... | ... | ... | ... |

  ## 总体判定
  [APPROVE / CONCERNS / REJECT]

  ## 判定依据
  [一段话说明判定理由]

  ## 修复建议
  ### BLOCKING（必须修复）
  - [问题 1]: [修复建议]
  ### ADVISORY（建议修复）
  - [问题 1]: [修复建议]
  ```

  2. 判定规则：
     - **APPROVE**：全部检查项 PASS，或仅有 ADVISORY 问题
     - **CONCERNS**：存在 1-2 个 BLOCKING 问题，可通过修改草稿解决
     - **REJECT**：存在多个 BLOCKING 问题或根本性缺陷（如职责严重重叠、模式严重不合规）
- **输出**：审核报告文档

### 阶段 4：输出报告并更新注册表

- **输入**：阶段 3 的审核报告
- **处理**：
  1. 通过 AskUserQuestion 向用户展示审核报告摘要
  2. 通过 AskUserQuestion 询问用户："我可以将审核报告写入 `.studio/registry/reviews/[资产类型]-[资产名称]-review.md` 吗？"
  3. 用户同意后，使用 Write 写入审核报告
  4. 使用 Read + Write 更新 `asset-registry.yaml`：
     - status → reviewed
     - 填写 review_doc 路径
  5. 根据判定结果提示下一步：
     - APPROVE → 提示返回对应创建技能的阶段 4（批准/拒绝），请求用户最终批准
     - CONCERNS → 提示修改草稿后重新提交审核（返回创建技能的阶段 2）
     - REJECT → 提示流程终止，可发起新提案
- **输出**：审核报告文件 + 注册表更新 + 下一步指引

## 协作协议引用

- 遵循 `docs/examples/COLLABORATIVE-DESIGN-PRINCIPLE.md` 用户驱动协作协议
- 审核报告生成后通过 AskUserQuestion 向用户展示结果
- 在使用 Write 工具前必须询问用户
- 参考 `docs/extension-mechanism.md` 动态扩展机制规范
- 对应 CA-ASSET 门禁（见 `docs/director-gates.md`）
- 审核 Agent 为 chief-architect，Skill/Rule 审核含部门负责人初审环节

## 推荐下一步

- APPROVE → 返回 `/create-agent`、`/create-skill` 或 `/create-rule` 的阶段 4，请求用户最终批准
- CONCERNS → 修改草稿后重新调用 `/asset-review [资产类型] [资产名称]`
- REJECT → 流程终止，可发起新提案
- 批准后运行 `/sync-platforms` 同步到所有平台