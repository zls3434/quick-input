# 动态扩展机制工作流 — 实现计划

> **状态**: 草稿
> **作者**: SOLO
> **更新日期**: 2026-07-08

## 摘要

为 Software Engineering Studios 项目增加一个动态扩展机制工作流，允许总监层和部门负责人层根据项目需求新增专家 Agent、技能（Skill）和路径规则（Rule），并由总监层（chief-architect）审核新增资产的合理性、必要性和规范性，最终由用户批准后注册并同步到所有平台。

该机制解决核心问题：软件开发涉及的技术栈和场景极多，预设 40 个 Agent、74 个技能、11 个规则无法覆盖所有可能性，需要一套可扩展的、受控的、可审计的资产新增流程。

---

## 现状分析

### 现有资产结构
- **Agent**：40 个，定义在 `.studio/agents/*.md`，有 `_roster.md`（花名册）和 `_schema.yaml`（模式）
- **Skill**：74 个，定义在 `.studio/skills/[name]/SKILL.md`，有 `_catalog.yaml`（目录）和 `_schema.yaml`
- **Rule**：11 个，定义在 `.studio/rules/*.md`，有 `_schema.yaml`
- **工作流目录**：`.studio/docs/workflow-catalog.yaml`（7 阶段管线）
- **模板**：35 个，位于 `.studio/templates/`
- **多平台同步**：`.studio/` 为规范源，通过 `tools/adapters/` 同步到 7 个平台

### 现有变更管理
- 有 `/skill-test`（验证技能合规性）和 `/skill-improve`（改进技能）
- 技能变更触发 `validate-skill-change.sh` Hook
- 技能变更需 chief-architect 审批
- **缺少**：专门的 create-agent/create-skill/create-rule 工作流技能、资产注册表、审核工作流、变更追踪机制

### 协作协议约束
- 用户驱动协作：提问 → 选项 → 决策 → 草稿 → 审批
- 写入前必须询问用户
- 纵向委派，不可越级
- 变更须传播

---

## 设计决策

### 技能方案：1 个统一入口 + 3 个专项创建技能 + 1 个审核技能

| 技能 | 命令 | 模型 | 用途 |
|------|------|------|------|
| `create-asset` | `/create-asset` | Sonnet | 统一入口，引导选择创建 Agent/Skill/Rule |
| `create-agent` | `/create-agent` | Sonnet | 创建新专家 Agent |
| `create-skill` | `/create-skill` | Sonnet | 创建新技能 |
| `create-rule` | `/create-rule` | Sonnet | 创建新路径规则 |
| `asset-review` | `/asset-review` | Opus | 总监层审核待审资产提案 |

理由：统一入口降低认知负担，专项技能支持直接调用，审核技能独立符合关注点分离。

### 六阶段生命周期

```
提案 → 草稿 → 审核 → 批准/拒绝 → 注册 → 同步
 ①       ②      ③        ④         ⑤       ⑥
```

### 权限矩阵

| 资产类型 | 提案权 | 审核权 | 批准权 |
|----------|--------|--------|--------|
| 专家 Agent | 总监层、部门负责人层 | chief-architect | chief-architect 建议 → **用户最终批准** |
| Skill | 部门负责人层（专家层经部门负责人授权） | 对应部门负责人初审 → chief-architect 终审 | chief-architect 建议 → **用户最终批准** |
| Rule | 部门负责人层（专家层经部门负责人授权） | tech-architect 初审 → chief-architect 终审 | chief-architect 建议 → **用户最终批准** |

关键约束：
- 专家层不得直接提案，须通过部门负责人代为提案（遵循纵向委派规则）
- 所有资产创建的最终批准权属于用户
- chief-architect 是技术资产合理性的最终审核者

---

## 新建文件清单（14 个）

### 1. 新技能定义文件（5 个）

#### `.studio/skills/create-asset/SKILL.md`
- **frontmatter**：name=create-asset, model=sonnet, argument-hint="[agent|skill|rule]", allowed-tools=[Read,Glob,Grep,AskUserQuestion], 7 平台启用
- **正文**：统一入口技能，读取资产注册表展示当前状态 → AskUserQuestion 选择资产类型 → 路由至对应专项命令

#### `.studio/skills/create-agent/SKILL.md`
- **frontmatter**：name=create-agent, model=sonnet, argument-hint="[Agent名称]", allowed-tools=[Read,Glob,Grep,Write,AskUserQuestion], 7 平台启用
- **正文**：六阶段工作流
  - ① 提案：读取 `_schema.yaml` 和现有 Agent，用 `asset-proposal.md` 模板生成提案（需求背景、职责边界、与现有 Agent 协作关系、避免职责重叠分析），写入 `.studio/registry/proposals/`，更新注册表 status=proposed
  - ② 草稿：用 `agent-definition.md` 模板生成 Agent 定义草稿（YAML frontmatter + 六章节 body），向用户展示
  - ③ 审核：提示调用 `/asset-review`，chief-architect 检查职责不重叠、模型层级合理、委托图完整、协作协议明确
  - ④ 批准/拒绝：AskUserQuestion 展示审核报告，请求用户最终批准
  - ⑤ 注册：写入 `.studio/agents/[name].md`，更新 `_roster.md`、`manifest.yaml`、`asset-registry.yaml`、`asset-changelog.md`
  - ⑥ 同步：提示运行 `/sync-platforms`

#### `.studio/skills/create-skill/SKILL.md`
- **frontmatter**：name=create-skill, model=sonnet, argument-hint="[技能名称]", allowed-tools=[Read,Glob,Grep,Write,AskUserQuestion], 7 平台启用
- **正文**：六阶段工作流（同上结构），差异点：
  - 提案用 `asset-proposal.md`，含技能目的、触发场景、所需工具、模型层级建议、所属工作流阶段分类
  - 草稿用 `skill-definition.md` 模板
  - 审核检查：名称合规（kebab-case、1-64 字符、不重复）、frontmatter 完整、body 完整
  - 注册时更新 `_catalog.yaml`、`skills-reference.md`
  - 创建后建议运行 `/skill-test` 验证

#### `.studio/skills/create-rule/SKILL.md`
- **frontmatter**：name=create-rule, model=sonnet, argument-hint="[规则名称]", allowed-tools=[Read,Glob,Grep,Write,AskUserQuestion], 7 平台启用
- **正文**：六阶段工作流（同上结构），差异点：
  - 提案含规则目的、适用路径范围、与现有规则的关系（补充/独立/覆盖）
  - 草稿用 `rule-definition.md` 模板
  - 审核检查：路径不冲突、规范条目清晰、示例完整
  - 注册时更新 `rules-reference.md`

#### `.studio/skills/asset-review/SKILL.md`
- **frontmatter**：name=asset-review, model=opus, argument-hint="[资产类型] [资产名称]", allowed-tools=[Read,Glob,Grep,Write,AskUserQuestion], 7 平台启用
- **正文**：审核工作流
  - 读取 `asset-registry.yaml` 中 status=draft 的条目，读取提案和草稿
  - 按资产类型执行检查清单（Agent：职责不重叠/模型层级/委托图/协作协议/六章节完整；Skill：名称合规/frontmatter/body/模型层级/不重复；Rule：路径不冲突/规范清晰/示例完整/platforms 配置）
  - 生成审核报告（APPROVE/CONCERNS/REJECT 三级判定 + 修复建议）
  - 写入 `.studio/registry/reviews/`，更新注册表 status=reviewed

### 2. 新模板文件（4 个）

#### `.studio/templates/agent-definition.md`
- YAML frontmatter 占位符：name, description, tools, model(默认 sonnet), maxTurns(默认 20), skills, platforms(7 平台默认配置)
- Body 六章节模板：角色描述、协作协议、关键职责、决策框架、委托地图、不得做的事情
- 元数据表头 + 各占位符填写指南注释

#### `.studio/templates/skill-definition.md`
- YAML frontmatter 占位符：name, description, license(MIT), metadata(model/argument-hint/user-invocable/allowed-tools/platforms)
- Body 五章节模板：技能目的、参数说明、分阶段工作流（每阶段含输入/处理/输出）、协作协议引用、推荐下一步
- 字段合规要求注释（name 限 1-64 字符 kebab-case）

#### `.studio/templates/rule-definition.md`
- YAML frontmatter 占位符：paths(glob 模式列表), platforms(7 平台配置)
- Body 三章节模板：规则标题、规范条目（逐条，用"必须/禁止/应该"约束语言）、示例（正确+错误代码块）
- paths 字段 glob 语法说明

#### `.studio/templates/asset-proposal.md`
- 元数据表头：提案编号、提案类型、提案 Agent、所属层级、提案日期、状态
- 七章节：需求背景、资产概述、与现有资产的关系（不重叠分析）、影响范围评估、审核标准清单、风险与权衡、开放问题

### 3. 注册表与变更追踪文件（3 个）

#### `.studio/registry/asset-registry.yaml`
```yaml
version: 1.0.0
last_updated: "2026-07-08"
assets:
  agents: []    # 条目含 id/name/chinese_name/layer/category/model/parent_department/status/proposal_date/approval_date/file_path/approved_by/proposal_doc/review_doc
  skills: []    # 同上 + phase(工作流阶段) + command(斜杠命令)
  rules: []     # 同上 + paths(适用路径范围)
statistics:
  total_agents: 40
  total_skills: 74
  total_rules: 11
  total_dynamic_additions: 0
```

#### `.studio/registry/asset-changelog.md`
- 文件头说明
- 变更条目格式：日期、变更类型(created/modified/deprecated)、资产类型、资产名称、提案 Agent、审核者、批准者、变更摘要、关联文件、关联提案编号
- 初始为空

#### `.studio/registry/proposals/.gitkeep`
- 提案文档目录占位符，提案文件命名为 `[资产类型]-[编号]-[名称]-proposal.md`

### 4. 扩展机制文档（1 个）

#### `.studio/docs/extension-mechanism.md`
- 元数据 + 九章节：概述与背景、目标与非目标、权限矩阵、六阶段工作流定义、资产类型与模板、审核标准、注册表与变更追踪、与现有协议的集成、开放问题（资产废弃流程等）

### 5. 新 Hook 脚本（1 个）

#### `.studio/hooks/validate-asset-change.sh`
- 触发条件：PostToolUse(Write|Edit)，路径在 `.studio/agents/`、`.studio/skills/`、`.studio/rules/`
- 逻辑：agents 变更 → 提示检查 _roster.md 并运行 /asset-review；skills 变更 → 沿用现有提示；rules 变更 → 提示检查 rules-reference.md；registry 变更 → 静默通过
- 始终 `exit 0`（仅提示不阻断）

---

## 修改现有文件清单（13 个）

### 索引与目录文件

| 文件 | 修改要点 |
|------|---------|
| `.studio/skills/_catalog.yaml` | total 74→79，工具类分类新增 5 个技能条目 |
| `.studio/agents/_roster.md` | 第 4 节新增动态扩展说明段落，第 7 节补充新增流程引用 |
| `.studio/docs/skills-reference.md` | 工具类数量 9→14，合计 74→79，新增 5 行技能说明，第 6 节新增动态扩展机制说明 |
| `.studio/docs/directory-structure.md` | 目录树新增 `registry/` 子目录及其说明 |

### 规则与协调文件

| 文件 | 修改要点 |
|------|---------|
| `.studio/docs/coordination-rules.md` | 规则 4 新增资产扩展传播子项，新增第 5.1 节"动态扩展协议" |
| `.studio/docs/workflow-catalog.yaml` | 新增阶段 8"动态扩展"（跨阶段按需触发），含 create-asset 和 asset-review 两个 step |
| `.studio/docs/director-gates.md` | 新增第 7.1 节"资产扩展门禁"，定义 CA-ASSET 门禁（APPROVE/CONCERNS/REJECT） |
| `.studio/docs/review-workflow.md` | 新增第 4.1 节"资产审查"，说明 /asset-review 与 CA-ASSET 门禁关系 |

### Agent 定义文件

| 文件 | 修改要点 |
|------|---------|
| `.studio/agents/chief-architect.md` | skills 列表新增 asset-review，关键职责新增"动态扩展审核"，委托地图新增资产初审路由 |
| `.studio/agents/lead-developer.md` | skills 列表新增 create-skill、create-rule，关键职责新增"技能与规则扩展" |
| `.studio/agents/tech-architect.md` | skills 列表新增 create-rule，关键职责新增规则扩展初审 |

### 其他文件

| 文件 | 修改要点 |
|------|---------|
| `.studio/hooks/_manifest.md` | 平台兼容性表新增 validate-asset-change.sh 行 |
| `.studio/manifest.yaml` | 版本号 1.0.0 → 1.1.0 |

---

## 审核标准清单

### Agent 审核标准
- [ ] 职责不与现有 40 个 Agent 重叠
- [ ] 模型层级合理（专家层 → sonnet/haiku）
- [ ] 委托图完整（有上级、有协作对象）
- [ ] 协作协议明确
- [ ] Body 六章节完整（角色描述/协作协议/关键职责/决策框架/委托地图/不得做的事情）
- [ ] platforms 配置正确

### Skill 审核标准
- [ ] 名称合规（kebab-case、1-64 字符、不与现有重复）
- [ ] frontmatter 完整（name/description/model/allowed-tools）
- [ ] Body 完整（技能目的/参数说明/分阶段工作流/协作协议引用/推荐下一步）
- [ ] 模型层级合理
- [ ] 不与现有技能功能重复

### Rule 审核标准
- [ ] 路径范围不与现有 11 个规则冲突
- [ ] 规范条目清晰可执行（用"必须/禁止/应该"约束语言）
- [ ] 示例完整（正确示例 + 错误示例）
- [ ] platforms 配置正确

---

## 与现有体系的集成

### 与协作协议的映射
| 工作流阶段 | 协作协议映射 |
|-----------|-------------|
| 提案 | 提问：Agent 询问用户需求背景 |
| 草稿 | 选项 + 草稿：Agent 展示草稿供用户审阅 |
| 审核 | 草稿：审核报告作为另一种草稿 |
| 批准/拒绝 | 决策 + 审批：用户做出最终决策 |
| 注册 | 审批：多文件变更集的批量审批 |
| 同步 | 审批：用户指示后执行同步 |

### 与协调规则的集成
- 纵向委派：专家层需通过部门负责人代为提案
- 横向协商：新 Agent 职责涉及多部门时需相关部门负责人协商
- 变更传播：注册后必须 `/sync-platforms` 传播
- 禁止跨领域变更：新 Agent 不得单方面定义属于其他领域的职责

### 与门禁机制的集成
新增 `CA-ASSET` 门禁，与现有 5 个门禁并列，由 chief-architect 执行。

### 与 skill-test/skill-improve 的集成
```
/create-skill → /asset-review → /skill-test → /skill-improve（如需）
     创建           审核            验证            改进
```

---

## 实施顺序

### 阶段 A：基础设施搭建
1. 创建 `.studio/registry/` 目录及 `asset-registry.yaml`、`asset-changelog.md`、`proposals/.gitkeep`、`reviews/.gitkeep`
2. 创建 4 个模板文件（agent-definition.md、skill-definition.md、rule-definition.md、asset-proposal.md）

### 阶段 B：技能定义
3. 创建 5 个技能文件（create-asset、create-agent、create-skill、create-rule、asset-review）
4. 对新技能运行 `/skill-test` 验证合规性

### 阶段 C：文档与规则更新
5. 创建 `extension-mechanism.md` 扩展机制文档
6. 修改 `coordination-rules.md`、`director-gates.md`、`review-workflow.md`
7. 修改 `workflow-catalog.yaml`、`directory-structure.md`

### 阶段 D：索引与 Agent 更新
8. 修改 `_catalog.yaml`、`_roster.md`、`skills-reference.md`
9. 修改 `chief-architect.md`、`lead-developer.md`、`tech-architect.md`

### 阶段 E：Hook 与清单
10. 创建 `validate-asset-change.sh`，修改 `_manifest.md`
11. 修改 `manifest.yaml` 版本号

### 阶段 F：同步与验证
12. 运行 `/sync-platforms` 将所有变更同步到 7 个平台
13. 运行 `/platform-check` 验证各平台配置一致性

---

## 验证步骤

1. **技能合规性验证**：对 5 个新技能运行 `/skill-test`，确认 frontmatter 和 body 符合 `_schema.yaml` 模式
2. **注册表完整性验证**：检查 `asset-registry.yaml` 中 statistics 数量与实际文件数一致
3. **索引一致性验证**：确认 `_catalog.yaml` total=79、`_roster.md` 统计数字正确、`skills-reference.md` 合计正确
4. **平台同步验证**：运行 `/sync-platforms` 后执行 `/platform-check`，确认 7 个平台配置一致
5. **工作流完整性验证**：确认 `workflow-catalog.yaml` 新增阶段 8 的两个 step 都有对应的技能文件
6. **Agent 权限验证**：确认 chief-architect 的 skills 列表包含 asset-review，lead-developer 包含 create-skill/create-rule
7. **Hook 验证**：手动触发 `.studio/agents/` 下文件变更，确认 `validate-asset-change.sh` 输出提示

---

## 假设与决策

1. **假设**：项目 `.studio/` 为唯一规范源，所有修改在此进行后通过同步脚本传播到各平台
2. **假设**：现有 `validate-skill-change.sh` 的逻辑可被 `validate-asset-change.sh` 扩展覆盖（skill 变更提示沿用）
3. **决策**：采用 5 个技能而非 1 个统一技能，以保持粒度适中并支持直接调用专项命令
4. **决策**：审核技能使用 Opus 模型，因审核涉及多文档综合分析和高风险决策
5. **决策**：资产废弃流程不在本次范围内，列为 `extension-mechanism.md` 的开放问题，后续可新增 `/deprecate-asset` 技能
6. **决策**：新增阶段 8"动态扩展"为跨阶段按需触发，不插入现有 7 阶段顺序中