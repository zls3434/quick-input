# 软件工程开发工作流框架 —— 实现计划

## 摘要

参照 `Claude-Code-Game-Studios-zh` 项目的架构设计，在 `e:\0Projects\Software-Engineering-Studios-zh` 实现一个能力相似的**软件工程开发工作流**框架。该框架将游戏开发工作流映射为通用软件工程工作流，保持原项目的核心设计理念（协作式而非自主式、用户驱动决策、层级化 Agent 协调、基于文件的状态管理），同时将游戏开发领域的角色、文档、阶段重新定义为软件工程领域的对应概念。

---

## 当前状态分析

**目标工作区**：`e:\0Projects\Software-Engineering-Studios-zh` —— 当前为空目录。

**参考项目**：`E:\0Projects\Claude-Code-Game-Studios-zh` —— 完整的游戏开发工作流框架，包含：
- 49 个 Agent（3 层结构：导演层/部门负责人层/专家层）
- 73 个 Skill（斜杠命令工作流）
- 12 个 Hook（自动化验证脚本）
- 11 个 Rule（路径范围编码规范）
- 41 个 Template（文档模板）
- 7 阶段工作流管线（概念→系统设计→技术设置→预生产→生产→打磨→发布）

**核心架构模式**（已通过研究确认）：
1. `CLAUDE.md` 主配置文件，通过 `@` 引用 `.claude/docs/` 下的规范文件
2. `.claude/settings.json` 配置 hooks 和 permissions
3. `.claude/agents/*.md` Agent 定义（YAML frontmatter + markdown body）
4. `.claude/skills/*/SKILL.md` 技能定义（YAML frontmatter + 工作流 markdown）
5. `.claude/hooks/*.sh` Hook 脚本（bash，跨平台）
6. `.claude/rules/*.md` 路径规则（YAML frontmatter 指定 paths + markdown 规则）
7. `.claude/docs/templates/*.md` 文档模板

---

## 概念映射表

| 游戏开发概念 | 软件工程概念 |
|---|---|
| 游戏开发 | 软件工程开发 |
| 游戏引擎（Godot/Unity/Unreal） | 技术栈（React/Vue/Angular/Node/Python/Java/Go） |
| 引擎专家 | 技术栈专家 |
| 游戏设计师 | 系统分析师/产品经理 |
| 创意总监 | 产品总监 |
| 技术总监 | 首席架构师 |
| 制作人 | 项目经理 |
| GDD 游戏设计文档 | SRS 需求规格文档 |
| 美术圣经 | 设计系统规范 |
| 玩家体验 | 用户体验 |
| 核心循环 | 核心业务流程 |
| 数值平衡 | 业务规则/算法参数 |
| 原型 | POC 技术验证 |
| 垂直切片 | MVP 最小可行产品 |
| 试玩 | 用户验收测试/Beta 测试 |
| 关卡设计 | 模块/微服务划分 |
| 资源管线 | 构建管线/CI/CD |
| 发行 | 部署上线 |
| 经济设计 | 数据模型/业务逻辑 |
| 叙事设计 | 用户旅程/信息架构 |

### 阶段映射表

| 游戏开发阶段 | 软件工程阶段 |
|---|---|
| 概念阶段（Concept） | 需求阶段（Requirements） |
| 系统设计阶段（Systems Design） | 架构设计阶段（Architecture Design） |
| 技术设置阶段（Technical Setup） | 技术设计阶段（Technical Design） |
| 预生产阶段（Pre-production） | 预开发阶段（Pre-development） |
| 生产阶段（Production） | 开发阶段（Development） |
| 打磨阶段（Polish） | 测试优化阶段（Testing & Optimization） |
| 发布阶段（Release） | 发布部署阶段（Release & Deployment） |

---

## 完整目录结构

```text
e:\0Projects\Software-Engineering-Studios-zh\
├── CLAUDE.md                         # 主配置文件
├── README.md                         # 项目说明
├── CONTRIBUTING.md                   # 贡献指南
├── LICENSE                           # MIT 许可证
├── .gitignore
├── .claude/
│   ├── settings.json                 # Hooks 配置和 permissions
│   ├── statusline.sh                 # 状态行脚本
│   ├── agents/                       # Agent 定义（40 个）
│   ├── skills/                       # 技能定义（65 个）
│   ├── hooks/                        # Hook 脚本（12 个）
│   ├── rules/                        # 路径规则（11 个）
│   └── docs/
│       ├── quick-start.md            # 快速上手指南
│       ├── coordination-rules.md     # Agent 协调规则
│       ├── coding-standards.md        # 编码规范
│       ├── context-management.md     # 上下文管理
│       ├── directory-structure.md    # 目录结构
│       ├── technical-preferences.md  # 技术偏好模板
│       ├── workflow-catalog.yaml     # 7 阶段管线定义
│       ├── director-gates.md         # 总监门禁定义
│       ├── review-workflow.md        # 审查工作流
│       ├── hooks-reference.md        # Hook 参考
│       ├── rules-reference.md        # 规则参考
│       ├── skills-reference.md       # 技能参考
│       ├── setup-requirements.md     # 系统前置条件
│       ├── settings-local-template.md
│       ├── CLAUDE-local-template.md
│       └── templates/                # 文档模板（约 35 个）
│           ├── collaborative-protocols/
│           │   ├── design-agent-protocol.md
│           │   ├── implementation-agent-protocol.md
│           │   └── leadership-agent-protocol.md
│           ├── product-concept.md
│           ├── requirement-spec.md
│           ├── architecture-decision-record.md
│           ├── architecture-doc.md
│           ├── architecture-traceability.md
│           ├── technical-design-document.md
│           ├── api-spec.md
│           ├── design-system-spec.md
│           ├── ux-spec.md
│           ├── accessibility-requirements.md
│           ├── sprint-plan.md
│           ├── epic.md
│           ├── user-story.md
│           ├── milestone-definition.md
│           ├── test-plan.md
│           ├── test-evidence.md
│           ├── bug-report.md
│           ├── risk-register-entry.md
│           ├── incident-response.md
│           ├── changelog-template.md
│           ├── release-notes.md
│           ├── release-checklist.md
│           ├── post-mortem.md
│           ├── retrospective.md
│           ├── prototype-report.md
│           ├── mvp-report.md
│           ├── user-journey.md
│           ├── interaction-pattern-library.md
│           ├── systems-index.md
│           ├── project-stage-report.md
│           ├── design-doc-from-implementation.md
│           ├── architecture-doc-from-code.md
│           ├── concept-doc-from-prototype.md
│           └── skill-test-spec.md
├── src/                              # 源代码
│   ├── frontend/
│   ├── backend/
│   ├── shared/
│   └── CLAUDE.md
├── docs/                             # 技术文档
│   ├── architecture/
│   ├── api/
│   ├── tech-reference/              # 技术栈参考文档
│   │   ├── react/
│   │   ├── vue/
│   │   ├── node/
│   │   ├── python/
│   │   ├── java/
│   │   ├── go/
│   │   └── README.md
│   ├── examples/
│   └── CLAUDE.md
├── design/                           # 产品设计文档
│   ├── requirements/                 # 需求文档（SRS）
│   ├── ux/                          # UX 规范
│   ├── design-system/              # 设计系统规范
│   └── CLAUDE.md
├── tests/                            # 测试套件
│   ├── unit/
│   ├── integration/
│   ├── e2e/
│   └── performance/
├── tools/                            # 构建和工具脚本
├── prototypes/                       # POC 原型
└── production/                       # 项目管理
    ├── session-state/
    ├── session-logs/
    ├── sprints/
    ├── milestones/
    ├── epics/
    └── releases/
```

---

## Agent 角色定义（40 个）

### 第一层：总监层（Opus，3 个）

| # | Agent | 对应原角色 | 职责 |
|---|---|---|---|
| 1 | `product-director` | creative-director | 产品总监：产品愿景、需求方向、用户体验目标、跨部门创意冲突解决 |
| 2 | `chief-architect` | technical-director | 首席架构师：高层技术决策、架构选型、技术风险评估 |
| 3 | `project-manager` | producer | 项目经理：Sprint 规划、里程碑追踪、风险管理、跨部门协调 |

### 第二层：部门负责人层（Sonnet，7 个）

| # | Agent | 对应原角色 | 职责 |
|---|---|---|---|
| 4 | `system-analyst` | game-designer | 系统分析师：需求分析、业务规则设计、系统行为定义 |
| 5 | `lead-developer` | lead-programmer | 开发负责人：代码架构、代码审查标准、编码规范执行 |
| 6 | `tech-architect` | systems-designer | 技术架构师：详细系统设计、API 设计、数据模型设计 |
| 7 | `ux-design-lead` | art-director | UX 设计负责人：UX 策略、设计系统、交互规范 |
| 8 | `qa-lead` | qa-lead | QA 负责人：测试策略、Bug 分类、发布质量门禁 |
| 9 | `devops-lead` | release-manager | DevOps 负责人：CI/CD 策略、基础设施、部署流程 |
| 10 | `security-lead` | security-engineer | 安全负责人：安全策略、合规审查、安全架构 |

### 第三层：技术栈专家（Sonnet，12 个）

| # | Agent | 对应原角色 | 职责 |
|---|---|---|---|
| 11 | `react-specialist` | godot-gdscript-specialist | React 前端专家 |
| 12 | `vue-specialist` | godot-csharp-specialist | Vue 前端专家 |
| 13 | `angular-specialist` | 新增 | Angular 前端专家 |
| 14 | `node-specialist` | unity-specialist | Node.js 后端专家 |
| 15 | `python-specialist` | unreal-specialist | Python 后端专家 |
| 16 | `java-specialist` | godot-gdextension-specialist | Java 后端专家 |
| 17 | `go-specialist` | 新增 | Go 后端专家 |
| 18 | `database-engineer` | engine-programmer | 数据库工程师（SQL/NoSQL/缓存） |
| 19 | `api-designer` | network-programmer | API 设计师（REST/GraphQL/gRPC） |
| 20 | `mobile-specialist` | 新增 | 移动端专家（React Native/Flutter） |
| 21 | `frontend-architect` | gameplay-programmer | 前端架构专家（状态管理、性能） |
| 22 | `backend-architect` | ai-programmer | 后端架构专家（微服务、消息队列） |

### 第三层：工程专家（Sonnet/Haiku，11 个）

| # | Agent | 对应原角色 | 职责 |
|---|---|---|---|
| 23 | `devops-engineer` | devops-engineer | DevOps 工程师：CI/CD 实施、Docker/K8s |
| 24 | `security-engineer` | security-engineer | 安全工程师：安全实现、漏洞修复 |
| 25 | `performance-engineer` | performance-analyst | 性能工程师：性能分析、优化 |
| 26 | `test-engineer` | qa-tester | 测试工程师：自动化测试编写 |
| 27 | `technical-writer` | writer | 技术文档工程师：API 文档、用户文档 |
| 28 | `prototyper` | prototyper | 原型工程师：快速 POC 验证 |
| 29 | `code-reviewer` | 新增 | 代码审查专家 |
| 30 | `refactoring-engineer` | tools-programmer | 重构工程师：技术债务处理 |
| 31 | `data-engineer` | analytics-engineer | 数据工程师：数据管线、ETL |
| 32 | `integration-engineer` | 新增 | 集成工程师：第三方服务集成 |
| 33 | `accessibility-specialist` | accessibility-specialist | 无障碍专家：a11y 合规 |

### 第三层：领域专家（Sonnet，7 个）

| # | Agent | 对应原角色 | 职责 |
|---|---|---|---|
| 34 | `release-manager` | release-manager | 发布经理：版本管理、发布协调 |
| 35 | `i18n-specialist` | localization-lead | 国际化专家 |
| 36 | `api-documenter` | 新增 | API 文档专家（OpenAPI/Swagger） |
| 37 | `cloud-architect` | 新增 | 云架构师（AWS/Azure/GCP） |
| 38 | `ml-engineer` | 新增 | ML/AI 集成工程师 |
| 39 | `sre-engineer` | 新增 | 站点可靠性工程师：监控、告警 |
| 40 | `community-manager` | community-manager | 开源社区/用户反馈管理 |

### Agent 文件 YAML frontmatter 格式

```yaml
---
name: chief-architect
description: "首席架构师负责所有高层级技术决策，包括系统架构、技术选型、性能策略和技术风险管理。当需要架构级决策、技术评估、跨系统技术冲突，或技术选择将约束或开放产品可能性时，使用此 Agent。"
tools: Read, Glob, Grep, Write, Edit, Bash, WebSearch
model: opus
maxTurns: 30
memory: user
disallowedTools: 
skills: [architecture-decision, architecture-review, create-architecture]
---
```

每个 Agent 的 markdown body 包含：协作协议、关键职责、决策框架、委托地图、不得做的事情、关卡评审格式、输出格式。

---

## Skill 技能定义（72 个）

### 入门与导航（5 个）

| 技能 | 对应原技能 | 说明 |
|---|---|---|
| `/start` | start | 首次上手引导 |
| `/help` | help | 上下文感知的"下一步做什么" |
| `/onboard` | onboard | 为某角色生成入职文档 |
| `/project-stage-detect` | project-stage-detect | 分析项目状态、检测阶段 |
| `/adopt` | adopt | 对现有项目进行棕地审计 |

### 需求阶段（8 个）

| 技能 | 对应原技能 | 说明 |
|---|---|---|
| `/brainstorm` | brainstorm | 产品概念创意构思 |
| `/setup-stack` | setup-engine | 配置技术栈、框架版本 |
| `/product-concept` | brainstorm 产出 | 产品概念文档 |
| `/map-modules` | map-systems | 将需求分解为模块，映射依赖 |
| `/requirement-spec` | design-system | 编写逐模块需求规格（SRS） |
| `/quick-spec` | quick-design | 轻量级变更规范 |
| `/review-all-srs` | review-all-gdds | 跨 SRS 一致性检查 |
| `/consistency-check` | consistency-check | 扫描所有 SRS 的矛盾 |

### 架构设计阶段（7 个）

| 技能 | 对应原技能 | 说明 |
|---|---|---|
| `/design-review` | design-review | 审查设计文档 |
| `/create-architecture` | create-architecture | 编写主架构文档 |
| `/architecture-decision` | architecture-decision | 创建 ADR |
| `/architecture-review` | architecture-review | 验证架构完整性 |
| `/create-control-manifest` | create-control-manifest | 生成程序员规则表 |
| `/propagate-design-change` | propagate-design-change | 传播设计变更影响 |
| `/reverse-document` | reverse-document | 从代码生成设计文档 |

### 技术设计阶段（5 个）

| 技能 | 对应原技能 | 说明 |
|---|---|---|
| `/design-system-spec` | design-system | 设计系统规范编写 |
| `/ux-design` | ux-design | UX 规范编写 |
| `/ux-review` | ux-review | UX 规范审查 |
| `/api-spec` | asset-spec | API 规范编写 |
| `/accessibility-requirements` | accessibility template | 无障碍需求确认 |

### 预开发阶段（7 个）

| 技能 | 对应原技能 | 说明 |
|---|---|---|
| `/prototype` | prototype | 技术原型验证 |
| `/build-mvp` | vertical-slice | MVP 端到端构建 |
| `/create-epics` | create-epics | 创建 Epic |
| `/create-stories` | create-stories | 创建用户故事 |
| `/test-setup` | test-setup | 测试框架搭建 |
| `/sprint-plan` | sprint-plan | Sprint 规划 |
| `/estimate` | estimate | 工作量估算 |

### 开发阶段（12 个）

| 技能 | 对应原技能 | 说明 |
|---|---|---|
| `/dev-story` | dev-story | 实现用户故事 |
| `/code-review` | code-review | 代码审查 |
| `/story-readiness` | story-readiness | 故事就绪度检查 |
| `/story-done` | story-done | 故事完成审查 |
| `/sprint-status` | sprint-status | Sprint 状态快照 |
| `/scope-check` | scope-check | 范围检查 |
| `/bug-report` | bug-report | Bug 报告 |
| `/bug-triage` | bug-triage | Bug 分类 |
| `/hotfix` | hotfix | 紧急修复 |
| `/qa-plan` | qa-plan | QA 测试计划 |
| `/test-helpers` | test-helpers | 测试辅助库生成 |
| `/retrospective` | retrospective | Sprint 回顾 |

### 测试优化阶段（7 个）

| 技能 | 对应原技能 | 说明 |
|---|---|---|
| `/perf-profile` | perf-profile | 性能分析 |
| `/security-audit` | security-audit | 安全审计 |
| `/tech-debt` | tech-debt | 技术债务扫描 |
| `/smoke-check` | smoke-check | 冒烟测试 |
| `/soak-test` | soak-test | 压力测试协议 |
| `/regression-suite` | regression-suite | 回归测试套件 |
| `/test-evidence-review` | test-evidence-review | 测试证据审查 |

### 发布部署阶段（8 个）

| 技能 | 对应原技能 | 说明 |
|---|---|---|
| `/gate-check` | gate-check | 阶段门禁评估 |
| `/milestone-review` | milestone-review | 里程碑审查 |
| `/release-checklist` | release-checklist | 发布检查清单 |
| `/launch-checklist` | launch-checklist | 上线就绪度验证 |
| `/changelog` | changelog | 变更日志生成 |
| `/patch-notes` | patch-notes | 面向用户的更新说明 |
| `/day-one-patch` | day-one-patch | 首日补丁准备 |
| `/localize` | localize | 国际化扫描提取 |

### 团队编排（7 个）

| 技能 | 对应原技能 | 说明 |
|---|---|---|
| `/team-frontend` | team-ui | 编排前端团队管线 |
| `/team-backend` | team-combat | 编排后端团队管线 |
| `/team-qa` | team-qa | 编排 QA 团队周期 |
| `/team-release` | team-release | 编排发布团队管线 |
| `/team-polish` | team-polish | 编排优化打磨团队 |
| `/team-security` | 新增 | 编排安全审查团队 |
| `/team-devops` | 新增 | 编排 DevOps 团队管线 |

### 工具类（6 个）

| 技能 | 对应原技能 | 说明 |
|---|---|---|
| `/content-audit` | content-audit | 内容审计（需求 vs 实现） |
| `/test-flakiness` | test-flakiness | 不稳定测试检测 |
| `/skill-test` | skill-test | 验证技能文件合规性 |
| `/skill-improve` | skill-improve | 改进技能 |
| `/user-test-report` | playtest-report | 用户测试反馈报告 |
| `/audit-docs` | asset-audit | 文档/代码合规审计 |

### Skill 文件 YAML frontmatter 格式

```yaml
---
name: brainstorm
description: "引导式产品概念创意构思——从零想法到结构化的产品概念文档。使用产品管理框架、用户研究方法和结构化创意探索。"
argument-hint: "[产品类型或领域提示，或 'open'] [--review full|lean|solo]"
user-invocable: true
allowed-tools: Read, Glob, Grep, Write, WebSearch, Task, AskUserQuestion
model: sonnet
---
```

---

## Rules 规则定义（11 个）

| 规则文件 | 对应原规则 | paths 范围 | 内容要点 |
|---|---|---|---|
| `frontend-code.md` | gameplay-code.md | `src/frontend/**` | 前端代码规范：组件化、状态管理、不硬编码、TypeScript |
| `backend-code.md` | engine-code.md | `src/backend/**` | 后端代码规范：API 线程安全、热路径零分配、依赖方向 |
| `api-code.md` | network-code.md | `src/api/**`, `docs/api/**` | API 规范：RESTful 标准、版本管理、错误处理、幂等性 |
| `database-code.md` | data-files.md | `src/database/**`, `**/migrations/**` | 数据库规范：迁移脚本、索引策略、JSON 有效性 |
| `test-standards.md` | test-standards.md | `tests/**` | 测试标准：命名、AAA 结构、隔离性、确定性 |
| `ui-code.md` | ui-code.md | `src/frontend/**/components/**` | UI 代码规范：可访问性、响应式、组件复用 |
| `config-code.md` | ai-code.md | `**/config/**`, `**/*.env*` | 配置代码规范：环境变量、密钥管理、不硬编码 |
| `prototype-code.md` | prototype-code.md | `prototypes/**` | 原型代码规范：快速验证、README 必需 |
| `docs-standards.md` | design-docs.md | `design/**`, `docs/**` | 文档规范：Markdown 格式、必需章节 |
| `infra-code.md` | shader-code.md | `tools/**`, `**/Dockerfile*`, `**/*.yml` | 基础设施代码规范：IaC、容器化、CI/CD |
| `shared-code.md` | narrative.md | `src/shared/**` | 共享代码规范：类型定义、接口契约、跨端兼容 |

---

## Hooks 脚本（12 个）

与参考项目保持相同的 12 个 hook 脚本，内容根据软件工程场景调整：

| Hook 脚本 | 事件 | 功能调整 |
|---|---|---|
| `session-start.sh` | SessionStart | 加载项目上下文：分支、提交、Sprint、里程碑、Bug、TODO/FIXME、会话状态 |
| `detect-gaps.sh` | SessionStart | 检测新项目（建议 `/start`）、代码存在但缺设计文档 |
| `validate-commit.sh` | PreToolUse(Bash) | 检查硬编码值、TODO 格式、JSON 有效性、文档章节 |
| `validate-push.sh` | PreToolUse(Bash) | 向受保护分支推送时警告 |
| `validate-assets.sh` | PostToolUse(Write\|Edit) | 验证命名规范和 JSON 结构 |
| `validate-skill-change.sh` | PostToolUse(Write\|Edit) | `.claude/skills/` 变更后建议运行 `/skill-test` |
| `notify.sh` | Notification | Windows Toast 通知 |
| `pre-compact.sh` | PreCompact | 保存会话进度备注 |
| `post-compact.sh` | PostCompact | 提醒从 `active.md` 恢复会话状态 |
| `session-stop.sh` | Stop | 归档 `active.md`、记录 git 活动 |
| `log-agent.sh` | SubagentStart | 审计跟踪：记录子 Agent 调用 |
| `log-agent-stop.sh` | SubagentStop | 审计跟踪：完成子 Agent 记录 |

---

## Templates 模板（38 个）

### 协作协议模板（3 个）
- `collaborative-protocols/design-agent-protocol.md` —— 设计 Agent 的"提问-选项-草稿-批准"周期
- `collaborative-protocols/implementation-agent-protocol.md` —— 编程 Agent 的"Story 领取到 /story-done"周期
- `collaborative-protocols/leadership-agent-protocol.md` —— 总监层 Agent 的跨部门委派和上报机制

### 产品设计模板（8 个）
- `product-concept.md` —— 产品概念文档（MDA→Kano 模型、用户画像、范围层级）
- `requirement-spec.md` —— 需求规格文档（8 必需章节）
- `systems-index.md` —— 系统分解和依赖映射
- `user-journey.md` —— 用户旅程地图
- `interaction-pattern-library.md` —— 交互模式库
- `design-system-spec.md` —— 设计系统规范
- `ux-spec.md` —— UX 规范
- `accessibility-requirements.md` —— 无障碍需求

### 架构模板（5 个）
- `architecture-decision-record.md` —— ADR
- `architecture-doc.md` —— 主架构文档
- `architecture-traceability.md` —— 需求可追溯矩阵
- `technical-design-document.md` —— 技术设计文档
- `api-spec.md` —— API 规范

### 项目管理模板（6 个）
- `sprint-plan.md` —— Sprint 计划
- `epic.md` —— Epic 定义
- `user-story.md` —— 用户故事
- `milestone-definition.md` —— 里程碑定义
- `risk-register-entry.md` —— 风险登记
- `estimate.md` —— 工作量估算

### 测试模板（3 个）
- `test-plan.md` —— 测试计划
- `test-evidence.md` —— 测试证据
- `bug-report.md` —— Bug 报告

### 发布模板（5 个）
- `release-checklist.md` —— 发布检查清单
- `release-notes.md` —— 发布说明
- `changelog-template.md` —— 变更日志
- `incident-response.md` —— 事故响应
- `post-mortem.md` —— 复盘

### 其他模板（8 个）
- `prototype-report.md` —— 原型报告
- `mvp-report.md` —— MVP 报告
- `project-stage-report.md` —— 项目阶段报告
- `design-doc-from-implementation.md` —— 从代码逆向设计文档
- `architecture-doc-from-code.md` —— 从代码逆向架构文档
- `concept-doc-from-prototype.md` —— 从原型逆向概念文档
- `retrospective.md` —— Sprint 回顾
- `skill-test-spec.md` —— 技能测试规范

---

## 核心配置文件

### CLAUDE.md（主配置文件）

```markdown
# Software Engineering Studios —— 软件工程工作室 Agent 架构

**语言要求：必须始终使用简体中文与用户对话，生成的文档与代码注释也必须使用简体中文编写。**

通过 40 个协调的 Claude Code 子 Agent 管理软件工程项目开发。
每个 Agent 负责一个特定领域，确保关注点分离和质量把控。

## 技术栈
- **前端框架**：[选择：React / Vue / Angular]
- **后端语言**：[选择：Node.js / Python / Java / Go]
- **数据库**：[选择：PostgreSQL / MongoDB / Redis]
- **部署平台**：[选择：AWS / Azure / GCP / 自托管]

## 项目结构
@.claude/docs/directory-structure.md

## 技术偏好
@.claude/docs/technical-preferences.md

## 协调规则
@.claude/docs/coordination-rules.md

## 协作协议
**用户驱动的协作，而非自主执行。**
每个任务都遵循：提问 → 选项 → 决策 → 草稿 → 审批

## 编码规范
@.claude/docs/coding-standards.md

## 上下文管理
@.claude/docs/context-management.md
```

### .claude/settings.json

与参考项目结构相同，包含：
- `statusLine` 配置
- `permissions.allow`（git status、git diff、git log、npm test、pytest 等安全操作）
- `permissions.deny`（rm -rf、git push --force、读取 .env 等）
- 12 个 hook 的事件配置

### .claude/docs/workflow-catalog.yaml

7 阶段管线定义：
1. **requirements**（需求阶段）—— brainstorm、setup-stack、product-concept、map-modules、requirement-spec
2. **architecture-design**（架构设计阶段）—— design-review、create-architecture、architecture-decision、architecture-review、create-control-manifest
3. **technical-design**（技术设计阶段）—— design-system-spec、ux-design、ux-review、api-spec、accessibility-requirements
4. **pre-development**（预开发阶段）—— prototype、create-epics、create-stories、test-setup、sprint-plan、build-mvp
5. **development**（开发阶段）—— dev-story、code-review、story-done、qa-plan、bug-report、retrospective
6. **testing**（测试优化阶段）—— perf-profile、security-audit、tech-debt、smoke-check、soak-test、regression-suite
7. **release**（发布部署阶段）—— gate-check、release-checklist、launch-checklist、changelog、patch-notes

---

## 实施步骤

### 步骤 1：创建目录结构
创建完整目录骨架，包括 `.claude/` 下所有子目录、`src/`、`docs/`、`design/`、`tests/`、`tools/`、`prototypes/`、`production/` 等。

### 步骤 2：编写核心配置文件
- `CLAUDE.md` 主配置
- `.claude/settings.json` hooks 和 permissions 配置
- `.claude/statusline.sh` 状态行脚本
- `README.md`、`CONTRIBUTING.md`、`LICENSE`、`.gitignore`

### 步骤 3：编写 docs 规范文件（16 个）
- `quick-start.md`、`coordination-rules.md`、`coding-standards.md`、`context-management.md`
- `directory-structure.md`、`technical-preferences.md`、`workflow-catalog.yaml`
- `director-gates.md`、`review-workflow.md`、`hooks-reference.md`、`rules-reference.md`
- `skills-reference.md`、`setup-requirements.md`、`settings-local-template.md`、`CLAUDE-local-template.md`

### 步骤 4：编写 Agent 定义（40 个）
按三层结构编写所有 Agent 的 `.md` 文件，每个包含 YAML frontmatter + 详细 markdown body。

### 步骤 5：编写 Skill 定义（65 个）
每个 skill 创建独立目录和 `SKILL.md`，包含 YAML frontmatter + 详细工作流 markdown。

### 步骤 6：编写 Rules 规则（11 个）
每个规则文件包含 YAML frontmatter（paths）+ markdown 规则内容。

### 步骤 7：编写 Hook 脚本（12 个）
基于参考项目的 hook 脚本，调整为软件工程场景。

### 步骤 8：编写 Templates 模板（38 个）
包括协作协议模板和各类文档模板。

### 步骤 9：创建占位目录和文件
- `src/CLAUDE.md`、`docs/CLAUDE.md`、`design/CLAUDE.md`
- `production/session-state/.gitkeep`
- `src/.gitkeep`（如果需要）
- `.gitignore`（包含 session-state、session-logs、node_modules 等）

### 步骤 10：验证和测试
- 检查所有文件结构完整性
- 验证 YAML frontmatter 格式正确
- 确认 @ 引用路径正确
- 验证 hooks 脚本可执行

---

## 假设与决策

### 假设
1. 用户希望保持与参考项目相同的架构模式（Agent/Skill/Hook/Rule/Template）
2. 用户希望框架面向通用软件工程项目，而非特定技术栈
3. 用户希望保留原项目的协作式工作流理念
4. 目标平台仍为 Claude Code（`.claude/` 目录结构）

### 决策
1. **Agent 数量**：40 个（原项目 49 个），移除游戏引擎特有专家，增加软件工程特有专家
2. **Skill 数量**：65 个（原项目 73 个），移除游戏特有技能，增加软件工程特有技能
3. **保持 7 阶段管线**：阶段名称映射为软件工程概念，数量和结构不变
4. **保持 12 个 Hook**：功能与参考项目一致，内容调整为软件工程场景
5. **保持 11 个 Rule**：路径范围调整为软件工程目录结构
6. **模板数量**：38 个（原项目 41 个），移除游戏特有模板，新增软件工程特有模板
7. **技术栈专家**：涵盖前端（React/Vue/Angular）、后端（Node/Python/Java/Go）、数据库、API、移动端、云架构
8. **中文优先**：所有内容使用简体中文，与参考项目保持一致

---

## 验证步骤

1. **文件结构验证**：使用 `LS` 检查所有目录和文件是否创建完整
2. **YAML 格式验证**：检查所有 `.md` 文件的 YAML frontmatter 格式正确
3. **引用路径验证**：检查 `CLAUDE.md` 中的 `@` 引用路径是否正确
4. **hooks 配置验证**：检查 `settings.json` 中所有 hook 脚本路径是否正确
5. **permissions 验证**：检查 allow/deny 列表是否合理
6. **workflow-catalog 验证**：检查 7 阶段定义是否完整，每个 step 的 artifact glob 是否合理
7. **Agent 委托地图验证**：检查每个 Agent 的委托关系是否一致
8. **Skill 路由验证**：检查 `/dev-story` 等编排类 skill 的 Agent 路由表是否正确
9. **模板完整性**：检查模板数量和内容是否覆盖所有工作流需求