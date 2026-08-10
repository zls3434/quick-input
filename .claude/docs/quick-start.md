<!-- Software Engineering Studios -->
# 快速上手指南

本指南帮助你在 5 分钟内理解 Software Engineering Studios（软件工程工作室）的 Agent 架构、命令体系和协作流程，并选择最合适的上手路径。

## 1. 三层 Agent 层级

Software Engineering Studios 通过 40 个协调的 Claude Code 子 Agent 管理软件工程项目开发。Agent 按职责分为三层：

### 1.1 第一层：总监层（3 个）

总监层负责全局决策、架构对齐和项目推进，是最高级别的协调者。

| Agent | 职责 |
| --- | --- |
| `product-director`（产品总监） | 验证产品支柱、产品概念对齐、把控产品方向，对最终交付物负责 |
| `chief-architect`（首席架构师） | 验证架构设计、评估技术可行性、审批 ADR（架构决策记录） |
| `project-manager`（项目经理） | 范围评估、Sprint 规划、资源分配、进度跟踪、风险预警 |

### 1.2 第二层：部门负责人层（7 个）

部门负责人层负责特定领域的横向协调和跨模块整合。

| Agent | 职责 |
| --- | --- |
| `system-analyst`（系统分析师） | 需求拆解、系统模块映射、跨模块依赖分析 |
| `lead-developer`（首席开发） | 开发规范制定、技术难点攻关、代码质量把关 |
| `tech-architect`（技术架构师） | 技术方案设计、技术选型、架构模式决策 |
| `ux-design-lead`（UX 设计负责人） | 用户体验方向把控、设计系统维护、UX 审查 |
| `qa-lead`（QA 负责人） | 测试策略制定、质量门禁、缺陷分级 |
| `devops-lead`（DevOps 负责人） | CI/CD 管线、部署策略、基础设施即代码 |
| `security-lead`（安全负责人） | 安全审计、威胁建模、合规检查 |

### 1.3 第三层：专家层（30 个）

专家层由技术栈专家、工程专家和领域专家组成，执行具体任务。详见 `agent-roster.md`。

- 技术栈专家：`react` / `vue` / `angular` / `node` / `python` / `java` / `go` / `database-engineer` / `api-designer` / `mobile` / `frontend-architect` / `backend-architect`
- 工程专家：`devops` / `security` / `performance` / `test` / `technical-writer` / `prototyper` / `code-reviewer` / `refactoring` / `data` / `integration` / `accessibility`
- 领域专家：`release-manager` / `i18n` / `api-documenter` / `cloud-architect` / `ml` / `sre` / `community-manager`

## 2. Agent 选择指南

根据任务类型快速定位负责的 Agent：

| 任务类型 | 主责 Agent | 协作 Agent |
| --- | --- | --- |
| 产品概念、用户故事、需求拆解 | `product-director` | `system-analyst` |
| 架构设计、技术选型、ADR | `chief-architect` | `tech-architect`、`backend-architect` |
| Sprint 规划、进度跟踪、范围管理 | `project-manager` | `lead-developer`、`qa-lead` |
| 前端组件开发 | `react` / `vue` / `angular` | `frontend-architect`、`ux-design-lead` |
| 后端服务开发 | `node` / `python` / `java` / `go` | `backend-architect`、`api-designer` |
| 数据库设计与迁移 | `database-engineer` | `data`、`backend-architect` |
| API 设计与文档 | `api-designer` | `api-documenter`、`backend-architect` |
| 测试编写与执行 | `test` | `qa-lead`、`lead-developer` |
| 性能优化 | `performance` | `devops`、`backend-architect` |
| 安全审计 | `security` | `security-lead`、`sre` |
| 部署与发布 | `release-manager` | `devops-lead`、`devops` |
| 代码审查 | `code-reviewer` | `lead-developer` |
| 国际化 | `i18n` | `technical-writer`、`frontend-architect` |
| 无障碍 | `accessibility` | `ux-design-lead` |
| 文档撰写 | `technical-writer` | `api-documenter` |
| 机器学习模型 | `ml` | `data`、`backend-architect` |
| 云架构 | `cloud-architect` | `devops-lead`、`sre` |
| 可观测性与运维 | `sre` | `devops`、`performance` |
| 社区与开源 | `community-manager` | `technical-writer` |
| 代码重构 | `refactoring` | `code-reviewer`、`lead-developer` |
| 集成对接 | `integration` | `api-designer`、`backend-architect` |
| 原型制作 | `prototyper` | `ux-design-lead`、`frontend-architect` |
| 移动端开发 | `mobile` | `frontend-architect`、`react`/`vue`/`angular` |

## 3. 斜杠命令按阶段分类

共 65 个斜杠命令，按软件开发生命周期阶段分类如下。

### 3.1 入门导航（5 个）

- `/start` — 引导式上手流程，检测项目阶段并推荐起点
- `/help` — 查看所有可用命令和 Agent
- `/onboard` — 新成员入职引导，展示项目结构和规范
- `/project-stage-detect` — 自动检测当前项目所处阶段
- `/adopt` — 将现有项目纳入工作室管理体系

### 3.2 需求阶段（8 个）

- `/brainstorm` — 产品头脑风暴（可选，适用于没有想法或模糊想法）
- `/setup-stack` — 技术栈配置（必需，生成 `technical-preferences.md`）
- `/product-concept` — 产品概念文档生成（必需）
- `/map-modules` — 系统模块映射（必需）
- `/requirement-spec` — 需求规格说明（必需，可重复执行）
- `/quick-spec` — 快速需求规格（轻量版）
- `/review-all-srs` — 全量需求规格审查
- `/consistency-check` — 需求一致性检查

### 3.3 架构设计阶段（7 个）

- `/design-review` — 设计审查
- `/create-architecture` — 创建架构文档（必需）
- `/architecture-decision` — 架构决策记录（必需，可重复，至少 3 条）
- `/architecture-review` — 架构审查（必需）
- `/create-control-manifest` — 创建控制清单（必需）
- `/propagate-design-change` — 传播设计变更
- `/reverse-document` — 逆向文档生成

### 3.4 技术设计阶段（5 个）

- `/design-system-spec` — 设计系统规格
- `/ux-design` — UX 设计（必需，可重复，至少 3 个）
- `/ux-review` — UX 审查（必需）
- `/api-spec` — API 规格说明
- `/accessibility-requirements` — 无障碍需求（必需）

### 3.5 预开发阶段（7 个）

- `/prototype` — 原型制作（可选）
- `/build-mvp` — MVP 构建（可选）
- `/create-epics` — 创建史诗（必需）
- `/create-stories` — 创建用户故事（必需，可重复）
- `/test-setup` — 测试环境搭建（可选）
- `/sprint-plan` — Sprint 规划（必需）
- `/estimate` — 工作量估算

### 3.6 开发阶段（11 个）

- `/story-readiness` — 故事就绪检查（可选）
- `/dev-story` — 开发故事（必需，可重复）
- `/code-review` — 代码审查（可选，可重复）
- `/story-done` — 故事完成确认（必需，可重复）
- `/sprint-status` — Sprint 状态报告
- `/scope-check` — 范围检查
- `/bug-report` — 缺陷报告
- `/bug-triage` — 缺陷分诊
- `/hotfix` — 热修复
- `/qa-plan` — QA 计划（可选）
- `/retrospective` — 回顾会议（可选）

### 3.7 测试优化阶段（7 个）

- `/perf-profile` — 性能分析（可选）
- `/security-audit` — 安全审计（可选）
- `/tech-debt` — 技术债务评估（可选）
- `/smoke-check` — 冒烟测试（必需）
- `/soak-test` — 耐久测试（可选）
- `/regression-suite` — 回归测试套件（可选）
- `/test-evidence-review` — 测试证据审查（可选）

### 3.8 发布部署阶段（6 个）

- `/gate-check` — 门禁检查（可选）
- `/release-checklist` — 发布检查清单（必需）
- `/launch-checklist` — 上线检查清单（必需）
- `/changelog` — 变更日志（可选）
- `/patch-notes` — 补丁说明（可选）
- `/day-one-patch` — 首日补丁

### 3.9 团队编排与工具类（9 个）

- `/team-frontend` / `/team-backend` / `/team-qa` / `/team-release` / `/team-polish` / `/team-security` / `/team-devops` — 团队编排命令
- `/content-audit` — 内容审计
- `/test-flakiness` — 测试抖动检测
- `/user-test-report` — 用户测试报告

## 4. 四条上手路径

根据当前项目状态选择合适的起点：

### 路径 A：没有想法

适用场景：想做一个项目但还没有任何具体想法。

```
/start → /brainstorm → /product-concept → /setup-stack → ...
```

1. 运行 `/start` 进入引导式上手流程
2. 运行 `/brainstorm` 进行产品头脑风暴，由 `product-director` 引导产生创意
3. 产出 `design/requirements/product-concept.md`
4. 运行 `/setup-stack` 配置技术栈
5. 继续进入需求阶段

### 路径 B：模糊想法

适用场景：有一些模糊的产品想法或方向，但尚未成型。

```
/brainstorm [hint] → /product-concept → /setup-stack → ...
```

1. 运行 `/brainstorm` 并附带提示词（hint），如 `/brainstorm 在线教育平台`
2. 由 `product-director` 引导细化想法
3. 产出 `design/requirements/product-concept.md`
4. 运行 `/setup-stack` 配置技术栈

### 路径 C：清晰概念

适用场景：已有清晰的产品概念，准备开始架构设计。

```
/setup-stack → /create-architecture → /architecture-decision → ...
```

1. 运行 `/setup-stack` 配置技术栈，生成 `technical-preferences.md`
2. 直接进入架构设计阶段
3. 运行 `/create-architecture` 创建架构文档
4. 运行 `/architecture-decision` 记录关键决策（至少 3 条）

### 路径 D：已有项目

适用场景：已有代码库或正在进行的项目，希望纳入工作室管理体系。

```
/project-stage-detect → /adopt → [根据检测结果进入对应阶段]
```

1. 运行 `/project-stage-detect` 自动检测项目所处阶段
2. 运行 `/adopt` 将现有项目纳入工作室管理体系
3. 根据检测结果进入对应阶段，可能需要运行 `/reverse-document` 逆向生成文档

## 5. 协调规则

Agent 之间的协作遵循 5 条核心协调规则（详见 `coordination-rules.md`）：

1. **纵向委派**：上层 Agent 可向下层 Agent 委派任务，但不可越级向下委派（如 `product-director` 不可直接调用 `react` 专家，应通过 `lead-developer`）。
2. **横向协商**：同层 Agent 之间通过协商解决跨领域问题，任何一方无单方面决定权。
3. **冲突解决**：当 Agent 之间出现意见冲突时，由上一层级 Agent 仲裁（如部门负责人层冲突由对应总监仲裁）。
4. **变更传播**：任何架构或设计变更必须通过 `/propagate-design-change` 传播到所有受影响的 Agent 和文档。
5. **禁止单方面跨领域变更**：任何 Agent 不得单方面修改不属于自己领域的文件或配置，必须通过协商和审批流程。

## 6. 下一步

- 查看完整 Agent 花名册：`agent-roster.md`
- 了解协调规则细节：`coordination-rules.md`
- 查看命令工作流：`workflow-catalog.yaml`
- 配置技术栈：运行 `/setup-stack`