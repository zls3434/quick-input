<!-- Software Engineering Studios -->
# 目录结构

本文件定义 Software Engineering Studios 的标准项目目录结构。所有项目应遵循此结构，确保 Agent 能预测性地定位文件。

## 1. 顶层结构

本项目采用三层架构：规范源层（`.studio/`）→ 适配层（`tools/adapters/`）→ 输出层（各平台配置目录）。

```
项目根目录/
├── .studio/                     # ★ 规范源层（平台无关的单一真相源）
│   ├── manifest.yaml            # 多平台清单（版本、平台列表、同步状态）
│   ├── project/                 # 项目概述文档
│   ├── agents/                  # 40 个 Agent 规范定义
│   ├── skills/                  # 74 个技能定义（Agent Skills 标准）
│   ├── rules/                   # 11 个路径规则
│   ├── hooks/                   # 12 个 Hook 脚本源码
│   ├── registry/                # 资产注册表与变更追踪（动态扩展机制）
│   │   ├── asset-registry.yaml  # 资产注册表
│   │   ├── asset-changelog.md   # 资产变更日志
│   │   ├── proposals/           # 资产提案文档
│   │   └── reviews/             # 资产审核报告
│   ├── docs/                    # 规范文档
│   └── templates/               # 文档模板
├── tools/adapters/              # 适配层（同步脚本）
│   ├── sync-all.sh              # 同步所有平台
│   ├── sync-claude-code.sh      # 同步 Claude Code
│   ├── sync-codex.sh            # 同步 Codex
│   ├── sync-cursor.sh           # 同步 Cursor
│   ├── sync-windsurf.sh         # 同步 Windsurf
│   ├── sync-trae.sh             # 同步 Trae IDE
│   ├── sync-hermes.sh           # 同步 Hermes Agent
│   └── sync-workbuddy.sh        # 同步 WorkBuddy
├── CLAUDE.md                    # Claude Code 主配置
├── AGENTS.md                    # 跨平台主配置（Codex/Cursor/Windsurf/Trae 等）
├── .claude/                     # Claude Code 输出层
│   ├── settings.json            # 全局设置（权限、Hook、模型分配）
│   ├── settings.local.json      # 个人本地设置（不纳入版本控制）
│   ├── agents/                  # Agent 定义文件
│   ├── skills/                  # 技能定义文件
│   ├── hooks/                   # Hook 脚本
│   ├── rules/                   # 规则文件
│   └── docs/                    # 系统文档与模板
│       └── templates/           # 文档模板
├── .cursor/                     # Cursor 输出层
│   ├── rules/                   # .mdc 规则文件
│   ├── skills/                  # 技能定义（Agent Skills 兼容）
│   └── settings.json            # Cursor 设置
├── .windsurf/                   # Windsurf 输出层
│   └── workflows/               # 工作流定义
├── .windsurfrules               # Windsurf 规则（内联）
├── .trae/                       # Trae IDE 输出层
│   ├── rules/                   # 规则文件
│   ├── agents/                  # Agent 定义
│   └── skills/                  # 技能定义
├── .hermes/                     # Hermes Agent 输出层
│   ├── SOUL.md                  # 身份配置
│   ├── skills/                  # 技能定义
│   └── skills.lock              # 技能版本锁
├── .workbuddy/                  # WorkBuddy 输出层
│   ├── skills/                  # 技能定义
│   ├── mcp.json                 # MCP 配置
│   └── manifest.yaml            # WorkBuddy 清单
├── src/                         # 源代码
│   ├── frontend/                # 前端代码
│   │   └── AGENTS.md            # 前端路径范围规则
│   ├── backend/                 # 后端代码
│   │   └── AGENTS.md            # 后端路径范围规则
│   └── shared/                  # 前后端共享代码（类型、工具、常量）
├── docs/                        # 项目文档
│   ├── architecture/            # 架构文档与 ADR
│   ├── api/                     # API 文档
│   ├── tech-reference/          # 技术参考文档
│   ├── examples/                # 示例与最佳实践
│   ├── platform-adaptation-guide.md  # 平台适配指南
│   ├── platform-comparison-matrix.md # 平台功能对照矩阵
│   └── migration-guide-claude-to-studio.md # 迁移指南
├── design/                      # 设计文档
│   ├── requirements/            # 需求文档
│   ├── ux/                      # 用户体验设计
│   └── design-system/           # 设计系统
├── tests/                       # 测试代码
│   ├── unit/                    # 单元测试
│   ├── integration/             # 集成测试
│   ├── e2e/                     # 端到端测试
│   ├── performance/             # 性能测试
│   └── AGENTS.md                # 测试路径范围规则
├── tools/                       # 工具与脚本
├── prototypes/                  # 原型代码
└── production/                  # 生产管理文档
    ├── sprints/                 # Sprint 记录
    ├── milestones/              # 里程碑记录
    ├── epics/                   # 史诗记录
    ├── releases/                # 发布记录
    ├── session-state/           # 会话状态
    │   └── active.md            # 活跃会话状态文件
    └── session-logs/            # 会话日志归档
```

## 2. 各目录详细说明

### 2.1 CLAUDE.md（主配置文件）

项目根配置文件，定义：
- 技术栈选择
- 项目结构引用
- 协调规则引用
- 协作协议
- 编码规范引用
- 上下文管理引用

每个项目必须有一个 `CLAUDE.md`，可创建 `CLAUDE.local.md` 进行个人覆盖。

### 2.2 .claude/（Agent 系统配置）

#### settings.json

全局设置文件，包含：
- `permissions`：文件读写和命令执行权限
- `hooks`：Hook 脚本配置
- `model`：模型层级分配
- `env`：环境变量

#### agents/

Agent 定义文件，每个 Agent 一个 `.md` 文件，包含：
- Agent 名称与职责
- 系统提示词
- 可用工具
- 模型层级

#### skills/

技能定义文件，每个技能一个 `.md` 文件，定义斜杠命令的行为。

#### hooks/

Hook 脚本目录，存放 12 个 Hook 的 Shell 脚本。

#### rules/

规则文件目录，存放 11 个领域规则文件。

#### docs/

系统文档目录，包含所有规范文件（本文件所在目录）。

#### docs/templates/

文档模板，供 Agent 创建新文档时引用。

### 2.2b .studio/（规范源层）

平台无关的单一真相源，所有配置在此维护，通过同步脚本生成各平台配置。

#### manifest.yaml

多平台清单文件，包含：
- `version`：项目版本
- `platforms`：各平台启用状态和适配脚本路径

#### project/

项目概述文档，包含 overview.md、setup-commands.md、code-style.md、testing-instructions.md、security.md、pr-guidelines.md。

#### agents/

40 个 Agent 规范定义，每个 Agent 一个 `.md` 文件，YAML frontmatter 含 name、description、model、platforms 字段。

#### skills/

74 个技能定义，遵循 Agent Skills 开放标准，每个技能一个目录含 SKILL.md。

#### rules/

11 个路径规则定义，YAML frontmatter 含 paths 和 platforms 字段。

#### hooks/

12 个 Hook 脚本源码，附 `_manifest.md` 说明 Hook 平台兼容性。

#### registry/

资产注册表与变更追踪目录（动态扩展机制）：
- `asset-registry.yaml`：资产注册表，追踪所有通过动态扩展机制创建的资产
- `asset-changelog.md`：资产变更日志，按时间倒序记录变更事件
- `proposals/`：资产提案文档存放目录
- `reviews/`：资产审核报告存放目录

详见 `docs/extension-mechanism.md`。

### 2.2c 其他平台输出目录

#### .cursor/

Cursor 输出层，包含 `.mdc` 规则文件（3 Always + 11 Auto Attached + 40 Agent Requested）和技能目录。

#### .windsurf/ + .windsurfrules

Windsurf 输出层，`.windsurfrules` 为内联规则，`.windsurf/workflows/` 为工作流定义。

#### .trae/

Trae IDE 输出层，包含规则、Agent 定义和技能目录。

#### .hermes/

Hermes Agent 输出层，`SOUL.md` 为身份配置，`skills/` 为技能定义，`skills.lock` 为版本锁。

#### .workbuddy/

WorkBuddy 输出层，包含技能目录、MCP 配置和清单文件。

### 2.2d tools/adapters/（适配层）

同步脚本目录，将 `.studio/` 规范源转换为各平台配置格式。运行 `sync-all.sh` 同步所有平台。

### 2.3 src/（源代码）

#### frontend/

前端源代码，结构取决于选择的前端框架：
- React：`components/`、`hooks/`、`pages/`、`services/`、`store/`
- Vue：`components/`、`composables/`、`views/`、`stores/`、`api/`
- Angular：按模块组织（`modules/`、`core/`、`shared/`）

#### backend/

后端源代码，结构取决于选择的后端语言：
- Node.js：`routes/`、`controllers/`、`services/`、`models/`、`middleware/`
- Python：`app/`、`models/`、`services/`、`api/`、`core/`
- Java：按包结构组织（`com.project.module/`）
- Go：`cmd/`、`internal/`、`pkg/`、`api/`

#### shared/

前后端共享代码：
- `types/`：类型定义
- `constants/`：常量
- `utils/`：工具函数
- `contracts/`：接口契约

### 2.4 docs/（项目文档）

#### architecture/

- `overview.md`：架构概述
- `decisions/`：ADR 目录，文件命名 `ADR-NNN-标题.md`
- `diagrams/`：架构图
- `control-manifest.md`：控制清单

#### api/

- API 规格说明
- 接口文档（由 `/api-spec` 生成）
- 接口契约定义

#### tech-reference/

- 技术选型参考
- 第三方依赖说明
- 性能基准报告
- 按技术栈分子目录组织（如 `react/`、`node/`、`postgresql/`）
- 通用分类目录：`middleware/`、`services/`、`general/`
- `README.md`：目录说明
- `INDEX.md`：全局索引文件（由 `/tech-reference` 技能自动维护）
- 管理规范详见 `.studio/docs/tech-reference-management.md`
- 通过 `/tech-reference` 技能管理文档获取、缓存、索引和验证

#### examples/

- 示例代码
- 最佳实践指南
- 协作设计原则示例

### 2.5 design/（设计文档）

#### requirements/

- `product-concept.md`：产品概念文档（由 `/product-concept` 生成）
- `systems-index.md`：系统模块索引（由 `/map-modules` 生成）
- 各模块需求规格说明（由 `/requirement-spec` 生成）

#### ux/

- 用户旅程图
- 线框图
- 交互原型描述
- UX 审查报告

#### design-system/

- 设计令牌（颜色、字体、间距）
- 组件库规格
- 无障碍规格
- 响应式断点定义

### 2.6 tests/（测试代码）

#### unit/

单元测试，与源代码结构镜像。每个源文件对应一个测试文件。

#### integration/

集成测试，测试模块间交互和接口契约。

#### e2e/

端到端测试，测试完整用户旅程。

#### performance/

性能测试，包括负载测试、压力测试、耐久测试。

### 2.7 tools/（工具与脚本）

- 构建脚本
- 部署脚本
- 数据迁移工具
- 开发辅助工具

### 2.8 prototypes/（原型代码）

- 快速验证想法的原型
- 不遵循完整编码规范
- 每个原型必须包含 `README.md` 说明用途和限制

### 2.9 production/（生产管理文档）

#### sprints/

Sprint 计划和记录，每个 Sprint 一个目录或文件。

#### milestones/

里程碑定义和达成记录。

#### epics/

史诗描述和跟踪。

#### releases/

发布记录，包括发布说明、回滚方案。

#### session-state/

会话状态目录：
- `active.md`：当前活跃会话状态（详见 `context-management.md`）

#### session-logs/

已归档的会话日志，按日期组织。

## 3. 文件命名约定

| 目录 | 命名规则 | 示例 |
| --- | --- | --- |
| `docs/architecture/decisions/` | `ADR-NNN-标题.md` | `ADR-001-选择-PostgreSQL.md` |
| `design/requirements/` | `[系统名]-requirement.md` | `user-auth-requirement.md` |
| `production/sprints/` | `sprint-NN.md` | `sprint-01.md` |
| `production/epics/` | `epic-NN-标题.md` | `epic-01-用户认证.md` |
| `production/releases/` | `release-vX.Y.Z.md` | `release-v1.0.0.md` |
| `production/session-logs/` | `YYYY-MM-DD-session.md` | `2026-07-07-session.md` |

## 4. 配置覆盖

配置按以下优先级覆盖（从低到高）：

1. `.claude/settings.json` — 全局默认
2. `.claude/settings.local.json` — 个人覆盖（不纳入版本控制）
3. `CLAUDE.md` — 项目级配置
4. `CLAUDE.local.md` — 个人项目配置（不纳入版本控制）

详见 `settings-local-template.md` 和 `CLAUDE-local-template.md`。