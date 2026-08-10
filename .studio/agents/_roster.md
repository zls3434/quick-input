<!-- Software Engineering Studios -->
# Agent 花名册

本文件列出 Software Engineering Studios 中全部 40 个 Agent，按三层架构组织。每个 Agent 附简要职责说明。

## 1. 第一层：总监层（3 个）

总监层负责全局决策、架构对齐和项目推进，是最高级别的协调者。

| Agent | 中文名称 | 职责说明 |
| --- | --- | --- |
| `product-director` | 产品总监 | 验证产品支柱和产品概念对齐，把控产品方向，对最终交付物负责，执行 PD-PILLARS 和 PD-CONCEPT-FIT 门禁 |
| `chief-architect` | 首席架构师 | 验证架构设计和技术可行性，审批 ADR，把控技术方向，执行 CA-ARCHITECTURE 和 CA-FEASIBILITY 门禁 |
| `project-manager` | 项目经理 | 范围评估、Sprint 规划、资源分配、进度跟踪、风险预警，执行 PM-SCOPE 门禁 |

## 2. 第二层：部门负责人层（7 个）

部门负责人层负责特定领域的横向协调和跨模块整合。

| Agent | 中文名称 | 职责说明 |
| --- | --- | --- |
| `system-analyst` | 系统分析师 | 需求拆解、系统模块映射、跨模块依赖分析，负责 `systems-index.md` 生成 |
| `lead-developer` | 首席开发 | 开发规范制定、技术难点攻关、代码质量把关，协调技术栈专家 |
| `tech-architect` | 技术架构师 | 技术方案设计、技术选型建议、架构模式决策，协助首席架构师 |
| `ux-design-lead` | UX 设计负责人 | 用户体验方向把控、设计系统维护、UX 审查，协调无障碍需求 |
| `qa-lead` | QA 负责人 | 测试策略制定、质量门禁把控、缺陷分级、测试证据审查 |
| `devops-lead` | DevOps 负责人 | CI/CD 管线设计、部署策略制定、基础设施即代码、容器化方案 |
| `security-lead` | 安全负责人 | 安全审计、威胁建模、合规检查、安全策略制定 |

## 3. 第三层：专家层（30 个）

专家层执行具体任务，分为技术栈专家、工程专家和领域专家三组。

### 3.1 技术栈专家（12 个）

| Agent | 中文名称 | 职责说明 |
| --- | --- | --- |
| `react` | React 专家 | React 组件开发、Hooks 设计、状态管理、性能优化 |
| `vue` | Vue 专家 | Vue 组件开发、Composition API、响应式系统、Vuex/Pinia 集成 |
| `angular` | Angular 专家 | Angular 模块组织、依赖注入、RxJS、模板表单与响应式表单 |
| `node` | Node.js 专家 | Node.js 后端开发、异步编程、中间件设计、流处理 |
| `python` | Python 专家 | Python 后端开发、异步框架、包管理、数据处理 |
| `java` | Java 专家 | Java 后端开发、Spring 生态、JVM 调优、企业模式 |
| `go` | Go 专家 | Go 后端开发、并发模型、接口设计、性能优化 |
| `database-engineer` | 数据库工程师 | 数据库模式设计、迁移管理、索引优化、查询调优 |
| `api-designer` | API 设计师 | API 接口设计、RESTful 规范、接口契约定义、API 文档 |
| `mobile` | 移动端专家 | 移动端开发、响应式适配、触控交互、移动性能优化 |
| `frontend-architect` | 前端架构师 | 前端架构设计、组件库规划、前端性能策略、前端技术选型 |
| `backend-architect` | 后端架构师 | 后端架构设计、服务拆分、后端技术选型、系统集成方案 |

### 3.2 工程专家（11 个）

| Agent | 中文名称 | 职责说明 |
| --- | --- | --- |
| `devops` | DevOps 工程师 | CI/CD 管线实现、容器编排、部署自动化、监控配置 |
| `security` | 安全工程师 | 安全漏洞修复、安全编码实践、渗透测试、合规实施 |
| `performance` | 性能工程师 | 性能基准测试、瓶颈分析、优化方案实施、性能监控 |
| `test` | 测试工程师 | 测试用例编写、自动化测试、测试框架维护、测试证据收集 |
| `technical-writer` | 技术文档撰写 | 技术文档编写、用户文档、变更日志、文档审查 |
| `prototyper` | 原型制作师 | 快速原型开发、交互验证、技术可行性验证 |
| `code-reviewer` | 代码审查员 | 代码审查、规范检查、最佳实践建议、代码质量评估 |
| `refactoring` | 重构专家 | 代码重构、技术债务偿还、设计模式应用、代码结构优化 |
| `data` | 数据工程师 | 数据模型设计、数据管道、数据迁移、数据分析支持 |
| `integration` | 集成工程师 | 第三方服务集成、系统对接、消息队列、事件驱动架构 |
| `accessibility` | 无障碍专家 | WCAG 合规、屏幕阅读器适配、键盘导航、无障碍审查 |

### 3.3 领域专家（7 个）

| Agent | 中文名称 | 职责说明 |
| --- | --- | --- |
| `release-manager` | 发布经理 | 发布流程管理、版本控制、发布检查清单、回滚协调 |
| `i18n` | 国际化专家 | 国际化方案设计、本地化实施、多语言资源管理、文化适配 |
| `api-documenter` | API 文档撰写 | API 文档生成、接口示例编写、SDK 文档、API 变更追踪 |
| `cloud-architect` | 云架构师 | 云架构设计、云服务选型、成本优化、多云策略 |
| `ml` | 机器学习专家 | 机器学习模型开发、模型部署、MLOps、模型性能优化 |
| `sre` | 站点可靠性工程师 | 可观测性设计、告警策略、故障排查、SLA 管理 |
| `community-manager` | 社区经理 | 开源社区管理、贡献指南、 issue 分诊、社区文档 |

## 4. Agent 总数统计

| 层级 | 类别 | 数量 |
| --- | --- | --- |
| 第一层 | 总监层 | 3 |
| 第二层 | 部门负责人层 | 7 |
| 第三层 | 技术栈专家 | 12 |
| 第三层 | 工程专家 | 11 |
| 第三层 | 领域专家 | 7 |
| **合计** | | **40** |

> **动态扩展**：专家层 Agent 可通过 `/create-agent` 工作流动态新增。新增 Agent 经提案→草稿→审核（chief-architect）→用户批准→注册→同步六阶段流程后加入本花名册。新增 Agent 自动注册到 `asset-registry.yaml`。详见 `docs/extension-mechanism.md`。

## 5. Agent 协作关系

### 5.1 常见协作链

| 场景 | 协作链 |
| --- | --- |
| 前端功能开发 | `product-director` → `system-analyst` → `lead-developer` → `frontend-architect` → `react`/`vue`/`angular` → `code-reviewer` |
| 后端 API 开发 | `product-director` → `system-analyst` → `lead-developer` → `backend-architect` → `node`/`python`/`java`/`go` → `api-designer` → `api-documenter` |
| 数据库设计 | `chief-architect` → `tech-architect` → `backend-architect` → `database-engineer` → `data` |
| 安全审计 | `security-lead` → `security` → `sre` → `devops` |
| 性能优化 | `qa-lead` → `performance` → `devops` → `backend-architect` |
| 发布部署 | `project-manager` → `release-manager` → `devops-lead` → `devops` |
| 国际化 | `ux-design-lead` → `i18n` → `frontend-architect` → `technical-writer` |

### 5.2 门禁执行权

| 门禁 | 执行总监 |
| --- | --- |
| PD-PILLARS | `product-director` |
| PD-CONCEPT-FIT | `product-director` |
| CA-ARCHITECTURE | `chief-architect` |
| CA-FEASIBILITY | `chief-architect` |
| CA-ASSET | `chief-architect` |
| PM-SCOPE | `project-manager` |

## 6. 模型层级分配

详见 `coordination-rules.md` 第 2 节。简要映射：

| 模型 | Agent / 技能 |
| --- | --- |
| Haiku | 只读检查类技能（`help`、`sprint-status` 等） |
| Sonnet | 大多数 Agent 和技能的默认模型 |
| Opus | 高风险审查类技能（`gate-check`、`architecture-review`、`review-all-srs`） |

## 7. Agent 定义文件

每个 Agent 在 `.claude/agents/` 目录下有一个对应的 `.md` 定义文件，包含：
- Agent 名称与职责描述
- 系统提示词（System Prompt）
- 可用工具列表
- 模型层级分配
- 协作关系定义

新增专家 Agent 通过 `/create-agent` 工作流创建，使用 `templates/agent-definition.md` 模板，遵循 `agents/_schema.yaml` 模式规范。新增 Agent 经审核和批准后自动添加到本花名册和 `asset-registry.yaml` 注册表。