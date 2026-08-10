<!-- Software Engineering Studios -->
# 项目概述

本文件是 Software Engineering Studios（软件工程工作室）的平台无关概述，供各平台配置文件引用。

## 1. 项目简介

Software Engineering Studios 将一个 AI 编码会话转变为一间完整的软件工程开发工作室。通过协调的子 Agent 团队，为 AI 辅助开发赋予真实开发团队的结构、流程和质量关卡。

### 核心资产

| 类别 | 数量 | 描述 |
|----------|-------|-------------|
| **Agent** | 40 | 涵盖产品、架构、前端、后端、QA、DevOps、安全和文档的子 Agent |
| **技能** | 67 | 覆盖每个工作流阶段的斜杠命令（`/start`、`/brainstorm`、`/create-epics`、`/dev-story` 等） |
| **Hooks** | 12 | 提交、推送、文件变更、会话生命周期、Agent 审计跟踪和缺口检测的自动化验证 |
| **规则** | 11 | 编辑前端、后端、API、数据库、测试等代码时强制执行的路径范围编码规范 |
| **模板** | 35 | SRS、ADR、Sprint 计划、UX 规格、无障碍要求等文档模板 |

## 2. 三层 Agent 架构

Agent 被组织成三个层级，与真实开发团队的运作方式一致：

### 第一层 — 总监层（3 个，Opus 模型）

负责战略方向、跨领域决策和用户对齐。

| Agent | 职责 |
|-------|------|
| `product-director` | 维护产品愿景，对齐用户需求 |
| `chief-architect` | 技术架构总负责，审批架构决策 |
| `project-manager` | 项目计划、进度跟踪、风险管控 |

### 第二层 — 部门负责人层（7 个，Sonnet 模型）

负责各自领域的整体质量和协调。

| Agent | 职责 |
|-------|------|
| `system-analyst` | 需求分析与系统拆解 |
| `lead-developer` | 开发协调与技术决策 |
| `ux-design-lead` | 用户体验设计统筹 |
| `qa-lead` | 质量保证与测试策略 |
| `devops-lead` | DevOps 与基础设施 |
| `security-lead` | 安全策略与审计 |
| `tech-architect` | 技术架构细化与落地 |

### 第三层 — 专家层（30 个，Sonnet/Haiku 模型）

执行具体开发、测试、文档和运维任务。

涵盖的专家包括：`react-specialist`、`vue-specialist`、`angular-specialist`、`node-specialist`、`python-specialist`、`java-specialist`、`go-specialist`、`database-engineer`、`api-designer`、`mobile-specialist`、`frontend-architect`、`backend-architect`、`devops-engineer`、`security-engineer`、`performance-engineer`、`test-engineer`、`technical-writer`、`prototyper`、`code-reviewer`、`refactoring-engineer`、`data-engineer`、`integration-engineer`、`accessibility-specialist`、`release-manager`、`i18n-specialist`、`api-documenter`、`cloud-architect`、`ml-engineer`、`sre-engineer`、`community-manager`。

### 协调规则

- **纵向委派**：上层可向下层委派，禁止越级向下委派
- **横向协商**：同层 Agent 通过协商解决跨领域问题
- **冲突解决**：同层协商 → 上级仲裁 → 用户最终决策
- **变更传播**：架构变更通过 `/propagate-design-change` 传播
- **禁止单方面跨领域变更**：不得修改不属于自己领域的文件

## 3. 协作协议

**用户驱动的协作，而非自主执行。**

每个任务遵循五步流程：

1. **提问** — Agent 在提出解决方案前先提问，理解用户意图
2. **呈现选项** — Agent 展示 2-4 个选项及其优缺点
3. **决策** — 用户始终掌握决策权
4. **草稿** — Agent 在最终确认前展示成果
5. **审批** — 未经用户签字确认，不会写入任何内容

### 关键约束

- Agent 在使用 Write/Edit 工具前必须询问："我可以将此写入 [文件路径] 吗？"
- Agent 在请求审批前必须展示草稿或摘要
- 多文件修改需要针对完整变更集的明确审批
- 未经用户指示不得进行提交

## 4. 技术栈选择

项目采用可配置的技术栈，通过 `/setup-stack` 命令初始化。

### 前端框架

- **React** — 组件化、生态丰富、适合大型应用
- **Vue** — 渐进式框架、易上手、灵活
- **Angular** — 全功能框架、强类型、企业级

### 后端语言

- **Node.js** — 前后端统一语言、非阻塞 I/O、适合实时应用
- **Python** — 生态丰富、适合数据处理和 AI、快速开发
- **Java** — 企业级、强类型、生态成熟
- **Go** — 高性能、并发原生支持、适合微服务

### 数据库

- **PostgreSQL** — 关系型、功能丰富、支持 JSON 和扩展
- **MongoDB** — 文档型、灵活模式、适合快速迭代
- **Redis** — 键值存储、高性能、适合缓存和会话

### 其他配置

- **版本控制**：Git，采用基于主干的开发模式
- **构建系统**：根据技术栈选择（Vite/Webpack/esbuild/Angular CLI、npm/pip/gradle/go build）
- **部署平台**：AWS / Azure / GCP / 自托管

> 存在针对 React、Vue、Angular、Node.js、Python、Java、Go 的技术栈专家 Agent。请使用与项目匹配的集合。

## 5. 项目结构

```
项目根目录/
├── CLAUDE.md                    # 主配置文件
├── .claude/ 或 .studio/         # Agent 系统配置
│   ├── settings.json            # 全局设置（权限、Hook、模型分配）
│   ├── agents/                  # 40 个 Agent 定义
│   ├── skills/                  # 67 个斜杠命令
│   ├── hooks/                   # 12 个 Hook 脚本
│   ├── rules/                   # 11 个路径范围编码规范
│   └── docs/                    # 规范文档和模板
│       └── templates/           # 35 个文档模板
├── src/                         # 源代码
│   ├── frontend/                # 前端代码
│   ├── backend/                 # 后端代码
│   └── shared/                  # 前后端共享代码
├── docs/                        # 项目文档与 ADR
├── design/                      # 需求文档和 UX 规范
├── tests/                       # 测试套件
├── tools/                       # 构建和管线工具
├── prototypes/                  # POC 原型
└── production/                  # Sprint、里程碑、发布跟踪
```

## 6. 相关文档引用

| 文档 | 说明 |
|------|------|
| `docs/coding-standards.md` | 编码规范全文 |
| `docs/technical-preferences.md` | 技术偏好模板 |
| `docs/coordination-rules.md` | Agent 协调规则 |
| `docs/directory-structure.md` | 目录结构详细说明 |
| `docs/setup-requirements.md` | 系统前置条件 |
| `docs/context-management.md` | 上下文管理规范 |