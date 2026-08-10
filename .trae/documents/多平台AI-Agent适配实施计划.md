# Software Engineering Studios 多 AI Agent 平台适配实施计划

## 摘要

本项目当前仅适配 Claude Code（`.claude/` 目录结构），需要扩展适配到 Codex、Cursor、Windsurf、Trae IDE、Trae Work、Hermes Agent、WorkBuddy 等通用 AI Agent 平台。各平台对专家（Agent）、技能（Skill/Command）、规则（Rules）等功能的实现方式各有不同，配置路径和格式也不一致。

本计划采用**三层架构**（规范源层 → 适配层 → 输出层）实现跨平台适配：以 `.studio/` 目录作为平台无关的单一真相源，以 `tools/adapters/` 同步脚本将规范源转换为各平台原生格式，以两大开放标准（AGENTS.md 和 Agent Skills）作为跨平台兼容基础。

---

## 当前状态分析

### 现有架构（仅 Claude Code）

| 组件 | 路径 | 格式 | 数量 |
|---|---|---|---|
| 主配置 | `CLAUDE.md` | Markdown + `@` 文件引用 | 1 |
| 设置 | `.claude/settings.json` | JSON（hooks、permissions、statusLine） | 1 |
| Agent 定义 | `.claude/agents/*.md` | YAML frontmatter + Markdown body | 40 |
| 技能定义 | `.claude/skills/*/SKILL.md` | YAML frontmatter + Markdown body | 65 |
| Hook 脚本 | `.claude/hooks/*.sh` | Bash，绑定到生命周期事件 | 12 |
| 路径规则 | `.claude/rules/*.md` | YAML frontmatter（paths）+ Markdown | 11 |
| 规范文档 | `.claude/docs/*.md` | 纯 Markdown，通过 `@` 引用加载 | 16 |
| 文档模板 | `.claude/docs/templates/*.md` | 纯 Markdown 模板 | 35 |

### 各目标平台配置方式（研究确认）

| 平台 | 主配置文件 | 规则系统 | 技能/命令系统 | Agent 定义 | Hook |
|---|---|---|---|---|---|
| Claude Code | `CLAUDE.md`（`@` 引用） | `.claude/rules/*.md`（YAML paths） | `.claude/skills/*/SKILL.md` | `.claude/agents/*.md` | 支持（12个） |
| Codex | `AGENTS.md`（层级继承） | 无独立规则系统 | 兼容 Agent Skills 标准 | 集中在 AGENTS.md | 不支持 |
| Cursor | `AGENTS.md` + `.cursor/rules/*.mdc` | `.mdc`（description, globs, alwaysApply） | `.cursor/skills/`（Agent Skills 兼容） | Agent Requested 规则 | 不支持 |
| Windsurf | `AGENTS.md` + `.windsurfrules` | `.windsurfrules`（纯文本） | `.windsurf/workflows/*.md`（`/` 触发） | 工作流代替 | 不支持 |
| Trae IDE | `AGENTS.md` + `.trae/rules/project_rules.md` | `.trae/rules/project_rules.md` | 兼容 Agent Skills 标准 | @Agent 机制 | 不支持 |
| Trae Work | 与 Trae IDE 部分共享 | 云端规则系统 | 云端技能市场 | 云端 Agent 定义 | 不支持 |
| Hermes Agent | `AGENTS.md` + `SOUL.md` | AGENTS.md 指令 | `~/.hermes/skills/*/SKILL.md`（Hermes 扩展） | SOUL.md 身份配置 | 后台审视机制 |
| WorkBuddy | `AGENTS.md` | AGENTS.md 指令 | SkillHub（Agent Skills 兼容） | CodeBuddy 架构 | MCP 协议 |

### 两大开放标准（关键发现）

**AGENTS.md 标准**：由 OpenAI、Google、Cursor 等联合发起，Linux Foundation 管理，60,000+ 开源项目使用。纯 Markdown 格式，支持目录层级继承，被 Codex/Cursor/Windsurf/Trae/Gemini CLI 等广泛支持。

**Agent Skills 开放标准**：Anthropic 于 2025 年发布。技能格式为目录含 `SKILL.md`（YAML frontmatter + Markdown），必填字段 `name` 和 `description`，被 Claude Code/Cursor/Codex/Trae 等广泛支持。

---

## 适配架构设计：三层架构

### 设计原则

1. **单一真相源**：所有内容在 `.studio/` 维护，平台特定文件通过转换生成
2. **开放标准优先**：以 AGENTS.md 和 Agent Skills 作为跨平台基础
3. **Claude Code 特性保留**：保留 hooks、permissions、subagent 作为增强层
4. **零信息丢失**：不支持的功能以文档指令形式降级，而非删除
5. **可维护性**：通过同步脚本实现自动化，避免手动维护多份配置

### 架构图

```
┌─────────────────────────────────────────────────────┐
│  第一层：规范源层 .studio/                           │
│  平台无关的单一真相源（兼容 Agent Skills 标准）        │
├─────────────────────────────────────────────────────┤
│  第二层：适配层 tools/adapters/                      │
│  同步脚本，将规范源转换为各平台格式                   │
├─────────────────────────────────────────────────────┤
│  第三层：输出层                                      │
│  .claude/ / AGENTS.md / .cursor/ / .windsurf/       │
│  .trae/ / .hermes/ / .workbuddy/                    │
└─────────────────────────────────────────────────────┘
```

---

## 提议变更

### 变更 1：创建 `.studio/` 规范源目录

**文件/目录**：`.studio/`（新建）

**内容**：平台无关的规范定义，作为所有平台配置的单一真相源。

```
.studio/
├── manifest.yaml                 # 项目清单：版本、平台列表、同步状态
├── project/
│   ├── overview.md               # 项目概述（技术栈、结构、协作协议）
│   ├── setup-commands.md         # 环境搭建命令
│   ├── code-style.md             # 代码风格指南
│   ├── testing-instructions.md   # 测试指令
│   ├── security.md               # 安全注意事项
│   └── pr-guidelines.md          # PR/提交规范
├── agents/                       # 40 个 Agent 规范定义
│   ├── _schema.yaml              # Agent 定义模式
│   ├── _roster.md                # Agent 花名册
│   └── *.md                      # 各 Agent 定义
├── skills/                       # 67 个技能规范定义（兼容 Agent Skills 标准）
│   ├── _schema.yaml
│   ├── _catalog.yaml             # 技能目录
│   ├── start/SKILL.md
│   ├── sync-platforms/SKILL.md   # 新增技能
│   ├── platform-check/SKILL.md   # 新增技能
│   └── .../
├── rules/                        # 11 个路径规则规范定义
│   ├── _schema.yaml
│   └── *.md
├── hooks/                        # 12 个 Hook 脚本源码
│   ├── _schema.yaml
│   ├── _manifest.md              # Hook 平台兼容性说明
│   └── *.sh
├── docs/                         # 规范文档（从 .claude/docs/ 迁移）
└── templates/                    # 文档模板（从 .claude/docs/templates/ 迁移）
```

**为何**：建立平台无关的单一真相源，避免在多个平台目录中重复维护相同内容。

**如何**：将现有 `.claude/` 下的内容迁移并规范化，格式转换为兼容 Agent Skills 标准的规范格式。

### 变更 2：规范源格式定义

#### Agent 规范格式

保留 YAML frontmatter，增加 `platforms` 字段标注平台兼容性：

```yaml
---
name: chief-architect
description: 首席架构师，负责系统架构决策、技术选型、跨系统技术冲突解决。
tools: [Read, Glob, Grep, Write, Edit, Bash, WebSearch]
model: opus
maxTurns: 30
skills: [architecture-decision, architecture-review, create-architecture]
platforms:
  claude-code: {enabled: true, path: .claude/agents/chief-architect.md}
  cursor: {enabled: true, type: agent-requested-rule}
  codex: {enabled: true, section: agents.md}
  windsurf: {enabled: true, type: workflow}
  trae: {enabled: true, type: agent}
  hermes: {enabled: false}
  workbuddy: {enabled: true}
---
```

#### 技能规范格式

以 Agent Skills 开放标准为基础，Claude Code 特有字段放入 `metadata`：

```yaml
---
name: start
description: 首次上手引导技能。检测项目状态、询问用户所处阶段、路由到对应工作流。
license: MIT
metadata:
  model: sonnet
  argument-hint: ""
  user-invocable: true
  allowed-tools: [Read, Glob, Grep, Write, AskUserQuestion]
  platforms:
    claude-code: {enabled: true}
    cursor: {enabled: true}
    codex: {enabled: true}
    windsurf: {enabled: true, trigger: /start}
    trae: {enabled: true}
    hermes: {enabled: true, platforms: [macos, linux, windows]}
    workbuddy: {enabled: true}
---
```

#### 规则规范格式

保留 `paths` 字段，增加 `platforms` 映射：

```yaml
---
paths: ["src/frontend/**"]
platforms:
  claude-code: {enabled: true, path: .claude/rules/frontend-code.md}
  cursor: {enabled: true, type: auto-attached, globs: "src/frontend/**"}
  codex: {enabled: true, section: agents.md}
  windsurf: {enabled: true, mode: append}
  trae: {enabled: true, mode: append}
---
```

### 变更 3：生成 AGENTS.md 跨平台主配置

**文件**：`AGENTS.md`（根目录，新建）+ 子目录 AGENTS.md

**内容**：200 行以内的项目主配置，遵循"地图而非手册"原则：

```markdown
# AGENTS.md
## 项目概述
[从 .studio/project/overview.md 浓缩]
## 环境搭建
[关键命令]
## 代码风格
[从 code-style.md 浓缩为关键条目]
## 测试指令
[关键命令]
## Agent 角色与协作
40 个 Agent 分三层，完整花名册见 .studio/agents/_roster.md
## 可用技能（67 个）
通过 / 命令调用，完整目录见 .studio/skills/_catalog.yaml
## 提交规范
遵循 Conventional Commits
## 安全注意事项
[关键条目]
## 详细文档
- 协调规则：.studio/docs/coordination-rules.md
- 编码规范：.studio/docs/coding-standards.md
```

子目录 AGENTS.md（`src/frontend/AGENTS.md`、`src/backend/AGENTS.md`、`tests/AGENTS.md`）包含对应路径范围的规则。

### 变更 4：生成 Cursor 配置

**文件/目录**：`.cursor/`（新建）

| 文件 | 类型 | 说明 |
|---|---|---|
| `.cursor/rules/00-overview.mdc` | Always | 项目概述 |
| `.cursor/rules/01-coordination.mdc` | Always | Agent 协调规则 |
| `.cursor/rules/02-coding-standards.mdc` | Always | 编码规范 |
| `.cursor/rules/frontend-code.mdc` | Auto Attached | `globs: "src/frontend/**"` |
| `.cursor/rules/backend-code.mdc` | Auto Attached | `globs: "src/backend/**"` |
| `.cursor/rules/*.mdc`（11个规则） | Auto Attached | 各路径规则 |
| `.cursor/rules/agent-*.mdc`（40个） | Agent Requested | Agent 定义 |
| `.cursor/skills/` | 技能目录 | Agent Skills 兼容格式 |
| `.cursor/settings.json` | 设置 | userContext 指向 AGENTS.md |

`.mdc` 格式示例：
```yaml
---
description: "前端代码规范：组件化、TypeScript、状态管理"
globs: "src/frontend/**"
alwaysApply: false
---
# 前端代码规范
[规则内容]
```

### 变更 5：生成 Windsurf 配置

**文件**：`.windsurfrules`（新建）+ `.windsurf/workflows/`（新建）

- `.windsurfrules`：将 11 个规则 + 编码规范内联，以路径标题分隔
- `.windsurf/workflows/*.md`：67 个技能转为工作流 + 40 个 Agent 工作流，通过 `/` 命令触发

### 变更 6：生成 Trae IDE 配置

**文件**：`.trae/rules/project_rules.md`（新建）+ `.trae/agents/` + `.trae/skills/`

- `.trae/rules/project_rules.md`：所有规则 + 编码规范内联
- `.trae/agents/`：40 个 Agent 定义文件（@Agent 机制）
- `.trae/skills/`：67 个技能（Agent Skills 兼容格式）

### 变更 7：生成 Hermes Agent 配置

**文件**：`.hermes/SOUL.md`（新建）+ `.hermes/skills/` + `.hermes/skills.lock`

- `.hermes/SOUL.md`：项目身份配置（从 overview.md + agent roster 生成）
- `.hermes/skills/*/SKILL.md`：技能转为 Hermes 格式（添加 `platforms`、`metadata.hermes` 字段）
- `.hermes/skills.lock`：版本锁定文件

Hermes 技能格式：
```yaml
---
name: start
description: 首次上手引导技能...
version: 1.0.0
platforms: [macos, linux, windows]
metadata:
  hermes:
    tags: [onboarding, navigation]
    load_mode: on-demand
---
```

### 变更 8：生成 WorkBuddy 配置

**文件**：`.workbuddy/skills/` + `.workbuddy/mcp.json` + `.workbuddy/manifest.yaml`

- `.workbuddy/skills/`：直接从 `.studio/skills/` 复制（Agent Skills 兼容）
- `.workbuddy/mcp.json`：MCP 协议配置
- `.workbuddy/manifest.yaml`：WorkBuddy 清单

### 变更 9：创建适配脚本

**目录**：`tools/adapters/`（新建）

```
tools/adapters/
├── sync-all.sh              # 同步所有平台
├── sync-claude-code.sh      # 同步 Claude Code
├── sync-codex.sh            # 同步 Codex（生成 AGENTS.md）
├── sync-cursor.sh           # 同步 Cursor
├── sync-windsurf.sh         # 同步 Windsurf
├── sync-trae.sh             # 同步 Trae IDE
├── sync-hermes.sh           # 同步 Hermes
├── sync-workbuddy.sh        # 同步 WorkBuddy
├── lib/
│   ├── yaml-utils.sh        # YAML 解析工具
│   ├── markdown-utils.sh   # Markdown 处理工具
│   └── platform-map.sh      # 平台映射配置
└── README.md                # 适配脚本说明
```

### 变更 10：格式转换策略

#### Agent 定义的跨平台转换

| 目标平台 | 转换策略 |
|---|---|
| Codex | 40 个 Agent 角色描述浓缩为 AGENTS.md 的一个章节 |
| Cursor | 每个 Agent 生成 `.cursor/rules/agent-{name}.mdc`（Agent Requested 类型） |
| Windsurf | 每个 Agent 生成 `.windsurf/workflows/agent-{name}.md` |
| Trae IDE | 生成 `.trae/agents/{name}.md`（@Agent 机制） |
| Hermes | Agent 体系写入 SOUL.md 身份配置 |
| WorkBuddy | 浓缩到 AGENTS.md |

#### 技能定义的跨平台转换

```
.studio/skills/*/SKILL.md（规范源，兼容 Agent Skills 标准）
    ├──→ .claude/skills/*/SKILL.md   （metadata 字段提升为顶层）
    ├──→ .cursor/skills/*/SKILL.md   （直接复制，已兼容）
    ├──→ .windsurf/workflows/*.md    （提取 body，添加触发说明）
    ├──→ .trae/skills/*/SKILL.md     （直接复制，已兼容）
    ├──→ .hermes/skills/*/SKILL.md   （添加 platforms、metadata.hermes）
    └──→ .workbuddy/skills/*/SKILL.md（直接复制，已兼容）
```

#### Hook 系统降级处理

12 个 Hook 仅 Claude Code 支持。其他平台降级为文档指令：

| Hook | 降级为 AGENTS.md 指令 |
|---|---|
| `validate-commit.sh` | "提交信息必须遵循 Conventional Commits 格式" |
| `validate-push.sh` | "禁止向 main/master 分支直接推送" |
| `session-start.sh` | "会话开始时先读取 active.md 了解当前状态" |
| `pre-compact.sh` | "上下文压缩前将进度写入 active.md" |
| ... | 其余 Hook 同理降级为明确指令 |

#### Rules 的跨平台转换

| 目标平台 | 转换方式 |
|---|---|
| Codex | 拆分到子目录 AGENTS.md（`src/frontend/AGENTS.md` 等） |
| Cursor | `.cursor/rules/*.mdc`，`paths` → `globs`，Auto Attached 类型 |
| Windsurf | 内联到 `.windsurfrules`，以路径标题分隔 |
| Trae IDE | 内联到 `.trae/rules/project_rules.md` |

### 变更 11：新增技能

| 技能 | 文件 | 说明 |
|---|---|---|
| `/sync-platforms` | `.studio/skills/sync-platforms/SKILL.md` | 运行平台同步脚本 |
| `/platform-check` | `.studio/skills/platform-check/SKILL.md` | 检查各平台配置一致性 |

### 变更 12：更新现有文档

| 文档 | 更新内容 |
|---|---|
| `CLAUDE.md` | 添加平台适配章节，说明 `.studio/` 为规范源 |
| `README.md` | 添加多平台支持说明，更新安装步骤 |
| `CONTRIBUTING.md` | 添加多平台贡献指南，修改应在 `.studio/` 中进行 |
| `.claude/docs/directory-structure.md` | 添加 `.studio/` 和各平台目录说明 |
| `.claude/docs/setup-requirements.md` | 添加各平台前置条件 |
| `.claude/docs/skills-reference.md` | 添加技能跨平台兼容性说明 |
| `.claude/docs/hooks-reference.md` | 添加 Hook 平台兼容性降级说明 |
| `.claude/docs/rules-reference.md` | 添加规则平台映射说明 |

### 变更 13：新增文档

| 文档 | 说明 |
|---|---|
| `docs/platform-adaptation-guide.md` | 平台适配指南（三层架构、同步机制、各平台差异） |
| `docs/platform-comparison-matrix.md` | 平台功能对照矩阵（7 个平台功能对比表） |
| `docs/migration-guide-claude-to-studio.md` | 迁移指南（从 .claude/ 迁移到 .studio/ 的步骤） |

---

## 假设与决策

### 假设

1. 用户希望保持与 Claude Code 的完整功能兼容（hooks、permissions、subagent）
2. 用户希望采用开放标准（AGENTS.md、Agent Skills）作为跨平台基础
3. 各目标平台的配置格式在可预见未来不会发生重大变化
4. 通过同步脚本自动化生成平台配置是可接受的维护方式
5. Trae Work 作为云端平台，通过 AGENTS.md 和可导入配置实现适配

### 决策

1. **使用 AGENTS.md 作为跨平台主配置**：60,000+ 项目验证，Linux Foundation 管理，多数目标平台原生支持
2. **使用 Agent Skills 开放标准作为技能格式基础**：Anthropic 主导，与现有 SKILL.md 格式高度兼容
3. **保留 `.claude/` 作为 Claude Code 输出层**：避免破坏现有用户体验，通过同步脚本自动生成
4. **Hook 功能降级为文档指令**：仅 Claude Code 支持 hooks，其他平台以明确指令形式表达
5. **Agent 定义转为规则或工作流**：非 Claude Code 平台无独立 Agent 概念，用最接近的机制承载
6. **`.studio/` 作为单一真相源**：所有修改在此进行，通过同步脚本生成各平台配置

---

## 验证步骤

### 1. 规范源完整性验证

- 检查 `.studio/` 目录结构完整，所有子目录和文件存在
- 验证 40 个 Agent 文件的 YAML frontmatter 格式正确（name、description、model 字段齐全）
- 验证 67 个技能的 SKILL.md 符合 Agent Skills 标准（name 1-64 字符、description 1-1024 字符）
- 验证 11 个规则文件的 paths 字段正确
- 检查所有 `@` 引用和文档链接无断链
- 验证 `manifest.yaml` 包含 7 个平台配置

### 2. 各平台输出验证

| 平台 | 验证项 | 预期结果 |
|---|---|---|
| Claude Code | `.claude/` 目录完整 | 40 Agent + 67 技能 + 11 规则 + 12 Hook + 16 文档 |
| Claude Code | `CLAUDE.md` 的 `@` 引用 | 所有引用可加载 |
| Codex | `AGENTS.md` 行数 | < 200 行 |
| Codex | 子目录 AGENTS.md | `src/frontend/`、`src/backend/`、`tests/` 各有 |
| Cursor | `.cursor/rules/*.mdc` frontmatter | description、globs、alwaysApply 齐全 |
| Cursor | `.cursor/rules/` 文件数 | 3 Always + 11 Auto Attached + 40 Agent Requested = 54 |
| Cursor | `.cursor/skills/` | 67 个技能目录 |
| Windsurf | `.windsurfrules` | 11 个规则全部内联 |
| Windsurf | `.windsurf/workflows/` | 67 技能 + 40 Agent = 107 个 .md |
| Trae IDE | `.trae/rules/project_rules.md` | 所有规则内联 |
| Trae IDE | `.trae/skills/` | 67 个技能目录 |
| Hermes | `.hermes/SOUL.md` | YAML frontmatter + body 格式正确 |
| Hermes | `.hermes/skills/` | 67 个技能，Hermes 格式字段齐全 |
| WorkBuddy | `.workbuddy/skills/` | 67 个技能目录 |
| WorkBuddy | `.workbuddy/mcp.json` | JSON 合法 |

### 3. 跨平台一致性验证

- Agent 数量一致：所有平台均为 40 个 Agent 定义
- 技能数量一致：所有平台均为 67 个技能
- 规则数量一致：所有平台均为 11 个规则
- 核心规则文本完全一致
- 所有文档引用路径有效

### 4. 功能降级验证

- 12 个 Hook 功能在 AGENTS.md 中有对应文档指令
- "提问→选项→决策→草稿→审批"协议在各平台配置中存在
- 模型层级分配说明在各平台配置中存在
- 上下文管理策略在各平台配置中存在

### 5. 同步机制验证

- `sync-all.sh` 执行成功，无错误
- `manifest.yaml` 的 last-sync 时间戳更新
- 增量同步正确：修改一个 Agent 后仅受影响文件更新
- 幂等性：连续运行两次，第二次无文件变更

---

## 最终目录结构（适配后）

```
项目根目录/
├── AGENTS.md                         # 跨平台主配置（所有平台共享）
├── CLAUDE.md                         # Claude Code 主配置（从 .studio 生成）
├── .studio/                          # ★ 规范源层（平台无关的单一真相源）
│   ├── manifest.yaml
│   ├── project/                      # 项目概述文档
│   ├── agents/                       # 40 个 Agent 规范定义
│   ├── skills/                       # 67 个技能（Agent Skills 标准）
│   ├── rules/                        # 11 个路径规则
│   ├── hooks/                        # 12 个 Hook 脚本源码
│   ├── docs/                         # 规范文档
│   └── templates/                   # 文档模板
├── .claude/                          # Claude Code 输出层
├── .cursor/                          # Cursor 输出层
├── .windsurf/ + .windsurfrules       # Windsurf 输出层
├── .trae/                            # Trae IDE 输出层
├── .hermes/                          # Hermes Agent 输出层
├── .workbuddy/                       # WorkBuddy 输出层
├── tools/adapters/                   # 平台适配同步脚本
├── src/                              # 源代码（含子目录 AGENTS.md）
├── docs/                             # 技术文档（含新增平台文档）
├── design/                           # 设计文档
├── tests/                            # 测试代码（含 AGENTS.md）
├── production/                       # 生产管理
└── prototypes/                       # 原型
```