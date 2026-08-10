---
状态: 已批准
作者: chief-architect
更新日期: 2026-07-08
---

# 动态扩展机制规范

## 1. 概述与背景

Software Engineering Studios 预设了 40 个 Agent、74 个技能、11 个路径规则，覆盖软件开发的主要场景。然而，软件开发涉及的技术栈、工具链和领域场景极其多样，预设资产无法穷举所有可能性。动态扩展机制提供一套受控的、可审计的流程，允许在项目运行过程中根据需求新增专家 Agent、技能和规则。

### 适用场景

- 项目采用了预设未覆盖的技术栈（如 Rust、Swift、Kotlin）
- 项目出现预设未覆盖的领域场景（如合规审计、数据迁移）
- 现有技能无法满足特定工作流需求
- 现有规则未覆盖新的代码路径范围

## 2. 目标与非目标

### 目标
- 提供结构化的资产新增流程（提案→草稿→审核→批准→注册→同步）
- 确保新增资产符合现有模式规范（`_schema.yaml`）
- 确保新增资产不与现有资产职责重叠或路径冲突
- 保留总监层（chief-architect）对技术资产的审核权
- 保留用户对所有新增资产的最终批准权
- 追踪所有动态新增资产的完整审计轨迹

### 非目标
- 不提供资产废弃（Deprecation）流程（列为开放问题，后续扩展）
- 不提供资产版本管理（列为开放问题，后续扩展）
- 不允许新增总监层或部门负责人层 Agent（仅限专家层）
- 不允许通过动态扩展修改现有预设资产

## 3. 权限矩阵

| 资产类型 | 提案权 | 初审权 | 终审权 | 批准权 | 注册权 | 同步权 |
|----------|--------|--------|--------|--------|--------|--------|
| 专家 Agent | 总监层、部门负责人层 | — | chief-architect | chief-architect 建议 → **用户最终批准** | 提案 Agent（用户指示后） | 任意 Agent（用户指示后） |
| Skill | 部门负责人层（专家层经部门负责人授权） | 对应部门负责人 | chief-architect | chief-architect 建议 → **用户最终批准** | 提案 Agent（用户指示后） | 任意 Agent（用户指示后） |
| Rule | 部门负责人层（专家层经部门负责人授权） | tech-architect | chief-architect | chief-architect 建议 → **用户最终批准** | 提案 Agent（用户指示后） | 任意 Agent（用户指示后） |

### 关键约束

1. **专家层不得直接提案**：专家层 Agent 需要新 Skill/Rule 时，必须通过其部门负责人代为提案（遵循"纵向委派"协调规则）
2. **最终批准权属于用户**：所有资产创建须用户明确批准（遵循"用户驱动协作"协议）
3. **chief-architect 是技术资产最终审核者**：与现有"技能变更需 chief-architect 审批"约定一致
4. **仅限专家层 Agent 新增**：不允许新增总监层或部门负责人层 Agent

## 4. 六阶段工作流定义

```
提案 → 草稿 → 审核 → 批准/拒绝 → 注册 → 同步
 ①       ②      ③        ④         ⑤       ⑥
```

### 阶段 1：提案

- **执行者**：提案 Agent（总监层或部门负责人层）
- **输入**：用户需求描述
- **处理**：
  1. 读取对应 `_schema.yaml` 和现有资产列表
  2. 使用 `asset-proposal.md` 模板生成提案文档
  3. 分析新资产与现有资产的关系，确保不重叠
  4. 将提案写入 `.studio/registry/proposals/`
  5. 更新 `asset-registry.yaml`（status: proposed）
- **输出**：提案文档
- **协作协议**：向用户展示提案，询问写入许可

### 阶段 2：草稿

- **执行者**：提案 Agent
- **输入**：阶段 1 的提案文档
- **处理**：
  1. 读取对应模板（`agent-definition.md` / `skill-definition.md` / `rule-definition.md`）
  2. 填充 YAML frontmatter 和 markdown body
  3. 向用户展示草稿，收集修改意见
  4. 更新 `asset-registry.yaml`（status: draft）
- **输出**：资产定义草稿
- **协作协议**：向用户展示草稿，询问修改意见

### 阶段 3：审核

- **执行者**：chief-architect（Asset Review 技能）
- **输入**：阶段 2 的草稿 + 阶段 1 的提案
- **处理**：
  1. 读取 `asset-registry.yaml` 中 status=draft 的条目
  2. 按资产类型执行审核检查清单
  3. 生成审核报告（APPROVE/CONCERNS/REJECT）
  4. 将报告写入 `.studio/registry/reviews/`
  5. 更新 `asset-registry.yaml`（status: reviewed）
- **输出**：审核报告
- **门禁**：对应 CA-ASSET 门禁

### 阶段 4：批准/拒绝

- **执行者**：用户（最终决策权）
- **输入**：阶段 3 的审核报告
- **处理**：
  1. 向用户展示审核报告
  2. APPROVE → 询问用户最终批准
  3. CONCERNS → 建议修改草稿后重新审核（返回阶段 2）
  4. REJECT → 流程终止
  5. 更新 `asset-registry.yaml`（status: approved/rejected）
- **输出**：批准或拒绝决定
- **协作协议**：用户做出最终决策

### 阶段 5：注册

- **执行者**：提案 Agent
- **输入**：阶段 4 的批准决定 + 阶段 2 的草稿
- **处理**：
  1. 向用户展示所有待写入文件列表，请求批量审批
  2. 写入资产定义文件
  3. 更新索引文件（`_roster.md` / `_catalog.yaml` / `skills-reference.md` / `rules-reference.md`）
  4. 更新 `manifest.yaml`（资产数量）
  5. 更新 `asset-registry.yaml`（status: registered）
  6. 追加 `asset-changelog.md` 变更记录
- **输出**：注册完成的资产文件 + 更新的索引
- **协作协议**：多文件变更集的批量审批

### 阶段 6：同步

- **执行者**：任意 Agent（用户指示后）
- **输入**：阶段 5 的注册结果
- **处理**：
  1. 运行 `/sync-platforms` 将新资产同步到所有平台
  2. 运行 `/platform-check` 验证各平台配置一致性
- **输出**：同步完成确认
- **协作协议**：用户指示后执行

## 5. 资产类型与模板

| 资产类型 | 模板文件 | 定义目录 | 模式文件 |
|----------|----------|----------|----------|
| Agent | `templates/agent-definition.md` | `.studio/agents/` | `agents/_schema.yaml` |
| Skill | `templates/skill-definition.md` | `.studio/skills/[name]/SKILL.md` | `skills/_schema.yaml` |
| Rule | `templates/rule-definition.md` | `.studio/rules/` | `rules/_schema.yaml` |
| 提案 | `templates/asset-proposal.md` | `.studio/registry/proposals/` | — |

## 6. 审核标准

### Agent 审核标准
1. 职责不与现有 40 个 Agent 重叠
2. 模型层级合理（专家层 → sonnet/haiku）
3. 委托图完整（有上级、有协作对象）
4. 协作协议明确（引用用户驱动协作模式）
5. Body 章节完整（角色描述/技术专长领域/编码规范要点/关键职责/决策框架/协作协议/委托地图/不得做的事情）
6. platforms 配置正确

### Skill 审核标准
1. 名称合规（kebab-case、1-64 字符、不与现有重复）
2. frontmatter 完整（name/description/license/metadata）
3. Body 完整（技能目的/参数说明/分阶段工作流/协作协议引用/推荐下一步）
4. 模型层级合理
5. 不与现有技能功能重复

### Rule 审核标准
1. 路径范围不与现有 11 个规则冲突
2. 规范条目清晰可执行（用"必须/禁止/应该"约束语言）
3. 示例完整（正确示例 + 错误示例）
4. platforms 配置正确

## 7. 注册表与变更追踪

### 资产注册表（`asset-registry.yaml`）

位于 `.studio/registry/asset-registry.yaml`，追踪所有通过动态扩展机制创建的资产。

条目状态流转：`proposed → draft → reviewed → approved → registered`（正常路径）或 `→ rejected`（拒绝路径）。

每个条目包含：id、name、chinese_name、layer、category、model、parent_department、status、proposal_date、approval_date、file_path、approved_by、proposal_doc、review_doc。

### 资产变更日志（`asset-changelog.md`）

位于 `.studio/registry/asset-changelog.md`，按时间倒序记录所有资产变更事件。

变更类型：created（新增）、modified（修改）、deprecated（废弃）。

## 8. 与现有协议的集成

### 与协作协议的集成

| 工作流阶段 | 协作协议映射 |
|-----------|-------------|
| 提案 | 提问：Agent 询问用户需求背景 |
| 草稿 | 选项 + 草稿：Agent 展示草稿供用户审阅 |
| 审核 | 草稿：审核报告作为另一种草稿 |
| 批准/拒绝 | 决策 + 审批：用户做出最终决策 |
| 注册 | 审批：多文件变更集的批量审批 |
| 同步 | 审批：用户指示后执行同步 |

### 与协调规则的集成

- **纵向委派**：专家层需通过部门负责人代为提案（规则 1）
- **横向协商**：新 Agent 职责涉及多部门时需相关部门负责人协商（规则 2）
- **冲突解决**：审核中如对新资产有异议，按层级仲裁（规则 3）
- **变更传播**：注册后必须通过 `/sync-platforms` 传播（规则 4）
- **禁止跨领域变更**：新 Agent 不得单方面定义属于其他领域的职责（规则 5）

### 与门禁机制的集成

新增 `CA-ASSET` 门禁，与现有门禁并列，由 chief-architect 执行。判定标准：APPROVE / CONCERNS / REJECT。

### 与 skill-test/skill-improve 的集成

```
/create-skill → /asset-review → /skill-test → /skill-improve（如需）
     创建           审核            验证            改进
```

### 与 Hook 机制的集成

`validate-asset-change.sh` 在资产文件变更时提示运行审核和检查索引更新。

## 9. 开放问题

1. **资产废弃流程**：当前方案聚焦创建，废弃流程尚未定义。未来可新增 `/deprecate-asset` 技能，支持将不再需要的资产标记为废弃、从索引中移除、同步到各平台。
2. **资产版本管理**：当前方案不追踪资产定义的版本变更。未来可在 `asset-registry.yaml` 中增加版本字段和变更历史。
3. **资产导入机制**：是否支持从外部导入社区创建的 Agent/Skill/Rule 定义。
4. **批量创建**：是否支持一次提案创建多个关联资产（如同时创建一个 Agent 和其专用 Skill）。