# 资产变更日志

> 本文件记录通过动态扩展机制产生的所有资产（Agent/Skill/Rule）变更事件。
> 变更类型：created（新增）、modified（修改）、deprecated（废弃）。
> 按时间倒序排列，最新变更在前。

---

<!-- 变更条目格式：
## [日期] 变更类型 — 资产类型/资产名称

- **变更类型**: created / modified / deprecated
- **资产类型**: agent / skill / rule
- **资产名称**: kebab-case 名称
- **提案 Agent**: 提案者
- **审核者**: 审核者
- **批准者**: 批准者
- **变更摘要**: 简要描述变更内容
- **关联文件**: 变更涉及的文件路径
- **关联提案编号**: agent-NNN / skill-NNN / rule-NNN
-->

<!-- 初始为空，首次动态新增资产后在此处添加条目 -->

## [2026-07-09] created — skill/tech-reference

- **变更类型**: created
- **资产类型**: skill
- **资产名称**: tech-reference
- **提案 Agent**: tech-architect
- **审核者**: chief-architect
- **批准者**: user
- **变更摘要**: 新增技术参考文档管理技能，支持在线文档缓存、PDF 下载、地址索引生成、本地文档导入、文档清单查询和引用一致性验证
- **关联文件**: .studio/skills/tech-reference/SKILL.md, .studio/docs/tech-reference-management.md, .studio/templates/tech-reference-index.md, docs/tech-reference/README.md, .studio/skills/setup-stack/SKILL.md, .studio/docs/directory-structure.md
- **关联提案编号**: skill-002

## [2026-07-08] created — skill/code-management

- **变更类型**: created
- **资产类型**: skill
- **资产名称**: code-management
- **提案 Agent**: lead-developer
- **审核者**: chief-architect
- **批准者**: user
- **变更摘要**: 新增代码管理工作流技能，支持 git worktree 并行、提交时机判断、推送/PR 准备、分支清理和 rebase 策略
- **关联文件**: .studio/skills/code-management/SKILL.md, .studio/docs/code-management-workflow.md, .studio/project/pr-guidelines.md, .studio/hooks/validate-push.sh
- **关联提案编号**: skill-001