<!--
  资产提案文档
  提案编号: skill-002
  提案类型: skill
  文件命名: skill-002-tech-reference-proposal.md
-->

---
状态: 已批准
提案编号: skill-002
提案类型: skill
提案 Agent: tech-architect
所属层级: 部门负责人层
提案日期: 2026-07-09
---

# 技能提案：tech-reference

## 1. 需求背景

当前项目拥有 80 个技能，`docs/tech-reference/` 目录在 `directory-structure.md` 中已定义（技术选型参考、第三方依赖说明、性能基准报告），但实际尚未创建。`setup-stack` 技能阶段 4（填充技术参考）仅用 WebSearch 收集最佳实践后直接写入 `docs/tech-reference/`，存在以下能力缺口：

1. **无文档本地缓存能力**：`setup-stack` 阶段 4 仅通过 WebSearch 获取摘要后写入文档，无法将完整的官方文档、SDK 文档缓存到本地，开发时仍需联网查阅，效率低且不可追溯。

2. **无 PDF/可下载文档管理**：许多框架和中间件的官方文档提供 PDF 版本或可下载的离线文档包，当前无任何技能支持下载和归档。

3. **无地址索引机制**：部分网页文档因 JS 渲染、反爬、分页等原因无法直接缓存，当前无技能为其生成目录与章节的地址索引说明文档，导致开发时无法快速定位参考章节。

4. **无本地文档导入流程**：用户可能已有本地技术文档（如企业内部规范、已购买的技术书籍 PDF），当前无标准化流程将其纳入 `docs/tech-reference/` 管理。

5. **无引用一致性验证**：项目文档（SRS、架构文档、技术偏好）中引用技术参考时，无机制验证引用路径是否有效、是否存在孤立文档。

6. **`docs/tech-reference/` 目录结构未细化**：`directory-structure.md` 仅列出三条概述（技术选型参考、第三方依赖说明、性能基准报告），未定义按技术栈还是按文档类型组织，实际使用时会产生混乱。

## 2. 资产概述

| 字段 | 值 |
|------|-----|
| 资产名称 | tech-reference |
| 中文名称 | 技术参考文档管理 |
| 资产类型 | skill |
| 所属分类 | 需求阶段 |
| 模型层级 | sonnet |
| 所属部门 | tech-architect |

### 职责/目的描述

技术参考文档管理技能，提供在线文档获取与缓存、PDF 下载、地址索引生成、本地文档导入、文档清单查询和引用一致性验证的统一入口，确保项目设计与选型期间的技术参考资料可追溯、可引用、一致性可控。

## 3. 与现有资产的关系

### 职责不重叠分析

| 现有资产 | 现有职责 | 新资产职责 | 是否重叠 | 说明 |
|----------|----------|------------|----------|------|
| setup-stack | 配置技术栈，填充技术偏好文档，阶段 4 简略填充技术参考 | 深化执行技术参考文档的获取、缓存、索引、导入和验证 | 否 | setup-stack 是技术栈配置入口，tech-reference 是文档管理专项工具；setup-stack 阶段 4 将调用 tech-reference |
| reverse-document | 为现有代码逆向生成文档 | 获取和管理外部技术文档 | 否 | reverse-document 面向内部代码生成文档，tech-reference 面向外部文档获取和管理 |
| audit-docs | 审计项目文档完整性和一致性 | 验证技术参考引用一致性 | 部分 | audit-docs 是全量文档审计，tech-reference verify 是专项引用一致性验证，聚焦 tech-reference 路径 |
| content-audit | 审计文档和代码内容质量 | 管理技术参考文档资产 | 否 | content-audit 面向内容质量，tech-reference 面向文档获取和组织 |

### 协作/补充关系

- **setup-stack → tech-reference**：setup-stack 阶段 4 修改为"调用 `/tech-reference fetch` 获取技术栈相关文档"，setup-stack 负责技术偏好填充，tech-reference 负责参考文档获取
- **create-architecture → tech-reference**：架构设计时可通过 `/tech-reference verify` 确认架构文档引用的技术参考是否存在
- **tech-architect 委派 tech-reference**：技术架构师通过此技能管理技术选型参考资料
- **audit-docs ↔ tech-reference**：audit-docs 审计时可调用 `/tech-reference list` 获取技术参考文档清单

## 4. 影响范围评估

### 受影响的文件

| 文件 | 修改类型 | 说明 |
|------|----------|------|
| `.studio/skills/_catalog.yaml` | 更新 | 需求阶段新增 tech-reference 条目，total 从 80 改为 81 |
| `.studio/docs/skills-reference.md` | 更新 | 需求阶段新增技能引用条目，合计从 80 改为 81 |
| `.studio/registry/asset-registry.yaml` | 更新 | 新增 skill-002 注册条目，统计信息更新 |
| `.studio/registry/asset-changelog.md` | 更新 | 记录变更 |
| `.studio/manifest.yaml` | 更新 | 版本号升级 |
| `.studio/skills/setup-stack/SKILL.md` | 更新 | 阶段 4 修改为调用 `/tech-reference` |
| `.studio/docs/directory-structure.md` | 更新 | tech-reference/ 小节扩展子目录结构说明 |

### 新增文件

| 文件 | 用途 |
|------|------|
| `.studio/skills/tech-reference/SKILL.md` | 技能定义 |
| `.studio/docs/tech-reference-management.md` | 管理规范文档 |
| `.studio/templates/tech-reference-index.md` | 地址索引文档模板 |
| `docs/tech-reference/README.md` | 目录说明文件 |

### 受影响的 Agent/Skill/Rule

- **tech-architect**：新增 tech-reference 技能到其技能调用范围，扩展技术选型参考资料管理职责
- **setup-stack**：阶段 4 行为变更，从直接 WebSearch + Write 改为先调用 `/tech-reference fetch`

## 5. 审核标准清单

### Skill 审核标准

- [x] 名称合规（kebab-case、14 字符、不与现有 80 个技能重复）
- [x] frontmatter 完整（name/description/license/metadata）
- [x] Body 完整（技能目的/参数说明/分阶段工作流/协作协议引用/推荐下一步）
- [x] 模型层级合理（sonnet，需要文档分析、网页抓取决策、索引生成）
- [x] 不与现有技能功能重复（与 setup-stack/reverse-document/audit-docs 职责不重叠）
- [x] platforms 配置完整（7 个平台全部启用）

## 6. 风险与权衡

| 风险 | 影响 | 应对措施 |
|------|------|----------|
| 网页文档版权风险 | 缓存受版权保护的文档可能侵权 | 技能规范中明确"网页缓存仅用于内部参考，不得再分发"；对已知不允许缓存的站点（如付费文档）仅生成地址索引 |
| 网页抓取不完整 | JS 渲染、反爬导致缓存失败 | 分层策略：先尝试缓存，失败则转为地址索引模式，两种路径均有产出 |
| 文档版本过时 | 缓存的文档与最新版本不一致 | 元数据中记录获取日期和文档版本；verify 子流程可检测过期文档 |
| 目录结构按技术栈组织可能导致跨栈文档归属模糊 | 某些文档涉及多个技术栈 | INDEX.md 支持多标签，文档物理存放按主技术栈，索引中标注关联技术栈 |
| Windows 路径兼容性 | 文件操作可能因路径分隔符失败 | 技能中使用跨平台路径处理，检测操作系统并适配 |

## 7. 开放问题

- 是否需要新增 `validate-tech-reference.sh` Hook，在项目文档引用 `docs/tech-reference/` 路径时自动检查文件是否存在？（初期依赖 verify 子流程手动执行，验证可行后再实施）
- 是否需要支持文档自动更新检测（定期检查在线文档是否有新版本）？（列为后续增强，当前版本仅记录获取日期）