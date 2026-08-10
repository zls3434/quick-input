---
状态: 草稿
提案编号: skill-001
提案类型: skill
提案 Agent: lead-developer
所属层级: 部门负责人层
提案日期: 2026-07-08
---

# 技能提案：code-management

## 1. 需求背景

当前项目拥有 79 个技能，覆盖从需求到发布的完整开发管线，但缺少代码管理工作流相关技能。具体能力缺口如下：

1. **无 git worktree 使用规范**：`coordination-rules.md` 第 4 节定义了并行任务协议（独立任务同时发起、最多 5 个并行），但未提供代码隔离机制。多个并行子 Agent 在同一工作目录操作会产生文件冲突。

2. **提交时机未定义**：`dev-story` 技能明确"未经用户指示不得执行 Git 提交"，但未定义何时应建议用户提交。`context-management.md` 定义了上下文压缩前保存进度，但未与 Git 提交关联。

3. **推送/PR 节奏未关联工作流阶段**：`pr-guidelines.md` 定义了分支命名和 PR 规范，`review-workflow.md` 定义了审查模式，但两者未与工作流阶段建立推送时机映射。

4. **分支生命周期管理缺失**：无 stale 分支检测、清理规则，长期积累导致分支混乱。

5. **无 rebase 策略**：缺少何时 rebase、安全规则、禁止场景的定义。

6. **`validate-push.sh` 保护力度不足**：当前对受保护分支仅警告不阻断，未与 review/gate-check 联动。

## 2. 资产概述

| 字段 | 值 |
|------|-----|
| 资产名称 | code-management |
| 中文名称 | 代码管理 |
| 资产类型 | skill |
| 所属分类 | 开发 |
| 模型层级 | sonnet |
| 所属部门 | lead-developer |

### 职责/目的描述

代码管理工作流技能，提供 git worktree 并行工作、提交时机判断、推送/PR 准备、分支生命周期管理和 rebase 策略执行的统一入口，确保在 AI Agent 多任务并行场景下代码仓库的整洁性、可追溯性和质量可控性。

## 3. 与现有资产的关系

### 职责不重叠分析

| 现有资产 | 现有职责 | 新资产职责 | 是否重叠 | 说明 |
|----------|----------|------------|----------|------|
| dev-story | 开发用户故事，实现功能并编写测试 | 管理代码提交、推送、分支、worktree | 否 | dev-story 负责实现，code-management 负责代码管理操作 |
| code-review | 对代码执行结构化审查 | 检查审查状态是否满足推送前置条件 | 否 | code-review 是审查执行者，code-management 是推送前审查状态检查者 |
| gate-check | 验证阶段就绪度门禁 | 检查门禁是否通过作为推送前置 | 否 | gate-check 是门禁执行者，code-management 是门禁状态消费者 |
| story-done | 验证故事完成并关闭 | 在故事完成后建议提交时机 | 否 | story-done 确认完成，code-management 执行提交操作 |

### 协作/补充关系

- **dev-story → code-management**：dev-story 完成实现后，提示用户运行 `/code-management commit` 执行提交
- **code-review → code-management**：code-review 完成审查后，code-management 检查审查报告是否存在作为推送前置
- **gate-check → code-management**：gate-check 通过后，code-management 确认门禁状态满足推送条件
- **coordination-rules 第 4 节 → code-management**：并行任务触发时，code-management 提供 worktree 创建能力
- **lead-developer 委派 code-management**：开发负责人通过此技能执行代码管理操作

## 4. 影响范围评估

### 受影响的文件

| 文件 | 修改类型 | 说明 |
|------|----------|------|
| `.studio/skills/_catalog.yaml` | 更新 | 新增技能条目，total 从 79 改为 80 |
| `.studio/docs/workflow-catalog.yaml` | 更新 | 开发阶段新增 code-management 步骤 |
| `.studio/docs/skills-reference.md` | 更新 | 新增技能引用条目 |
| `.studio/registry/asset-registry.yaml` | 更新 | 新增技能注册条目 |
| `.studio/registry/asset-changelog.md` | 更新 | 记录变更 |
| `.studio/agents/lead-developer.md` | 更新 | skills 列表新增 code-management |
| `.studio/project/pr-guidelines.md` | 更新 | 新增分支生命周期、worktree、提交时机、rebase 章节 |
| `.studio/hooks/validate-push.sh` | 更新 | 升级受保护分支为阻断、新增审查检查 |
| `.studio/docs/coordination-rules.md` | 更新 | 新增 worktree 隔离执行章节 |
| `.studio/docs/review-workflow.md` | 更新 | 新增推送/PR 门禁联动章节 |

### 受影响的 Agent/Skill/Rule

- **lead-developer**：新增 code-management 技能到其 skills 列表，扩展开发流程优化职责
- **dev-story**：新增提交时机指引，推荐下一步新增 `/code-management` 调用
- **validate-push.sh**：行为变更，受保护分支从警告升级为阻断

## 5. 审核标准清单

### Skill 审核标准

- [x] 名称合规（kebab-case、15 字符、不与现有 79 个技能重复）
- [x] frontmatter 完整（name/description/license/metadata）
- [x] Body 完整（技能目的/参数说明/分阶段工作流/协作协议引用/推荐下一步）
- [x] 模型层级合理（sonnet，标准能力，需要 Git 操作和上下文判断）
- [x] 不与现有技能功能重复（与 dev-story/code-review/gate-check 职责不重叠）

## 6. 风险与权衡

| 风险 | 影响 | 应对措施 |
|------|------|----------|
| Windows 环境下 git worktree 路径兼容性 | worktree 创建可能失败 | 规范中使用跨平台路径约定，技能中检测操作系统并适配路径分隔符 |
| dev-story "不得自主提交"与提交时机建议的边界 | 可能导致 Agent 自主提交 | 严格保持"建议提交"措辞，所有提交操作必须经用户明确指示 |
| validate-push.sh 审查报告路径的确定性 | 不同项目审查报告路径可能不同 | 使用 Glob 模式匹配而非硬编码路径，找不到时仅警告不阻断 |
| 已有项目分支命名可能不符合规范 | 推送时产生不必要警告 | validate-push.sh 对已有分支仅警告不阻断，新分支严格执行规范 |

## 7. 开放问题

- 是否需要新增 `validate-worktree.sh` Hook 自动检测并行任务时未使用 worktree 的情况？（初期依赖文档规范和技能引导，验证可行后再实施）