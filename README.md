# Software Engineering Studios

<div align="center">

**将一个 AI 编码会话转变为一间完整的软件工程开发工作室**

40 个 Agent · 81 个技能 · 12 个 Hooks · 11 个路径规则 · 36 个文档模板 · 7 大平台适配

</div>

---

## 目录

- [项目简介](#项目简介)
- [为什么需要这个项目](#为什么需要这个项目)
- [核心特性](#核心特性)
- [包含内容](#包含内容)
- [团队层级](#团队层级)
- [系统前置条件](#系统前置条件)
- [安装步骤](#安装步骤)
- [快速上手](#快速上手)
- [工作原理](#工作原理)
- [斜杠命令目录](#斜杠命令目录)
- [项目结构](#项目结构)
- [多平台支持](#多平台支持)
- [动态扩展机制](#动态扩展机制)
- [代码管理工作流](#代码管理工作流)
- [技术参考文档管理](#技术参考文档管理)
- [协调规则](#协调规则)
- [协作协议](#协作协议)
- [贡献指南](#贡献指南)
- [灵感来源与致谢](#灵感来源与致谢)
- [许可证](#许可证)
- [版权信息](#版权信息)
- [免责声明](#免责声明)

---

## 项目简介

**Software Engineering Studios（软件工程工作室）** 是一个为 AI 辅助软件开发赋予真实开发团队结构的框架。它将单个 AI 编码会话（如 Claude Code、Cursor、Windsurf、Trae IDE 等）转变为一间拥有 40 个专业角色、完整工作流和质量关卡的软件工程工作室。

传统 AI 编码助手是"一个人干所有事"——既写代码又做测试，既定架构又做需求，没有分工、没有审查、没有流程。Software Engineering Studios 通过引入真实开发团队的组织架构和协作机制来解决这个问题：

- **三层 Agent 架构**：总监层 → 部门负责人层 → 专家层，职责分明、逐级委派
- **全生命周期覆盖**：从产品头脑风暴到上线发布，覆盖软件开发全流程
- **动态扩展机制**：通过受控的六阶段工作流，按需新增专家 Agent、技能和路径规则
- **代码管理工作流**：git worktree 并行隔离、提交时机定义、推送/PR 节奏、分支生命周期管理
- **质量关卡**：12 个自动化 Hook 在提交、推送、文件变更等关键节点执行验证
- **路径范围规则**：11 个编码规范在编辑特定类型代码时自动激活
- **文档模板**：36 个标准化模板确保文档质量和一致性
- **多平台适配**：一套规范源，自动同步到 7 大 AI Agent 平台

### 核心理念

> **用户驱动的协作，而非自主执行。**

这**不是**一个自动驾驶系统。每个 Agent 都严格遵循五步协作流程：**提问 → 呈现选项 → 用户决策 → 展示草稿 → 审批确认**。未经用户签字，不会写入任何内容。

---

## 为什么需要这个项目

用 AI 独自构建软件功能强大——但单个聊天会话缺乏结构。没人阻止你硬编码配置、跳过需求文档或写出难以维护的代码。没有代码审查、没有设计评审、没有人问"这真的符合产品的愿景吗？"

**Software Engineering Studios** 通过为 AI 会话赋予真实开发团队的结构来解决这个问题。你获得的不是一个通用助手，而是被组织成开发团队层级结构的 40 个专业 Agent——维护愿景的产品总监、主导各自领域的部门负责人、执行具体工作的专家。每个 Agent 都有明确的职责、升级路径和质量关卡。

### 与普通 AI 助手的区别

| 维度 | 普通 AI 助手 | Software Engineering Studios |
|------|-------------|------------------------------|
| 组织结构 | 单一助手，无分工 | 40 个 Agent，三层架构，职责分明 |
| 需求阶段 | 直接开始编码 | 头脑风暴 → 产品概念 → 需求规格 |
| 架构设计 | 隐式决策 | 架构文档 + ADR 决策记录 + 架构审查 |
| 代码质量 | 无统一标准 | 11 个路径范围编码规范自动执行 |
| 代码审查 | 无 | 专门的 code-reviewer Agent 执行审查 |
| 测试体系 | 临时编写 | 测试计划 → 测试执行 → 证据审查 |
| 发布流程 | 无 | 门禁检查 → 发布检查清单 → 上线检查清单 |
| 协作模式 | AI 自主执行 | 用户决策驱动，每步需审批 |
| 审计追踪 | 无 | 12 个 Hook 记录 Agent 审计日志 |
| 文档规范 | 无 | 36 个标准化模板，8 章节需求文档 |
| 扩展能力 | 固定功能，无法扩展 | 动态扩展机制，按需新增 Agent/技能/规则 |
| 代码管理 | 无 | git worktree 并行隔离、提交时机、PR 节奏、分支生命周期 |
| 技术参考 | 无 | 在线文档缓存、PDF 下载、地址索引、本地导入、引用一致性验证 |

---

## 核心特性

### 🏗️ 三层 Agent 架构

40 个 Agent 按职责分为三层，模拟真实开发团队的组织结构：

- **总监层（3 个）**：产品总监、首席架构师、项目经理——负责战略方向和全局决策
- **部门负责人层（7 个）**：系统分析师、首席开发、技术架构师、UX 设计负责人、QA 负责人、DevOps 负责人、安全负责人——负责领域协调
- **专家层（30 个）**：技术栈专家、工程专家、领域专家——执行具体任务

### 🔄 全生命周期工作流

75 个斜杠命令覆盖软件开发生命周期的每个阶段：

```
入门导航 → 需求阶段 → 架构设计 → 技术设计 → 预开发 → 开发阶段 → 测试优化 → 发布部署
                                                                 ↑
                                                          动态扩展（贯穿全程）
```

### 🛡️ 自动化质量关卡

12 个 Hook 在关键操作节点自动执行验证：

- 提交信息规范检查（Conventional Commits）
- 受保护分支推送警告
- 文件命名和 JSON/YAML 格式验证
- 会话上下文持久化和恢复
- Agent 审计日志记录

### 📏 路径范围编码规范

11 个编码规范在编辑特定类型代码时自动激活：

- `frontend-code.md` — 前端代码规范（TypeScript、状态管理、i18n、组件拆分）
- `backend-code.md` — 后端代码规范（线程安全、依赖注入、事务、分层）
- `api-code.md` — API 规范（RESTful、版本管理、幂等性、分页）
- `database-code.md` — 数据库规范（迁移、索引、外键、连接池）
- `test-standards.md` — 测试标准（AAA 模式、隔离性、确定性、工厂函数）
- `ui-code.md` — UI 代码规范（WCAG AA、响应式、语义化 HTML、ARIA）
- `config-code.md` — 配置规范（环境变量、密钥管理、分层配置）
- `infra-code.md` — 基础设施规范（IaC、容器版本、CI/CD、健康检查）
- `shared-code.md` — 共享代码规范（类型导出、接口文档、跨端兼容）
- `docs-standards.md` — 文档规范（Markdown 标准、8 章节需求、ADR 格式）
- `prototype-code.md` — 原型规范（宽松标准、README 必需、隔离生产代码）

### 📄 标准化文档模板

36 个文档模板确保产出物质量和一致性，涵盖：

- 需求类：产品概念、需求规格、快速规格、用户故事、用户旅程
- 架构类：架构文档、ADR、架构追溯、技术设计文档
- 计划类：Sprint 计划、里程碑定义、工作量估算、系统索引
- 测试类：测试计划、测试证据、QA 计划、技能测试规格
- 发布类：发布检查清单、上线检查清单、变更日志、补丁说明
- 运维类：事故响应、事后分析、风险登记、缺陷报告
- 设计类：设计系统规格、UX 规格、无障碍需求、交互模式库
- 技术参考类：技术参考地址索引（网页文档无法缓存时的目录与章节映射）

### 🧩 动态扩展机制

预设资产无法穷举所有技术栈和领域场景。动态扩展机制提供一套受控的、可审计的六阶段工作流，允许在项目运行过程中按需新增资产：

```
提案 → 草稿 → 审核 → 批准/拒绝 → 注册 → 同步
 ①      ②      ③        ④         ⑤       ⑥
```

- **三类资产可扩展**：专家 Agent、技能（Skill）、路径规则（Rule）
- **统一入口**：`/create-asset` 引导选择资产类型并路由至专项工作流
- **权限分级**：总监层/部门负责人层提案权 → chief-architect 审核权 → 用户最终批准权
- **完整审计**：资产注册表（`asset-registry.yaml`）追踪所有动态新增资产的状态流转
- **变更日志**：`asset-changelog.md` 记录所有创建、修改、废弃事件

### 🔧 代码管理工作流

在 AI Agent 多任务并行场景下，确保代码仓库保持整洁性、可追溯性和质量可控性：

- **Git Worktree 并行隔离**：为每个并行子 Agent 提供独立工作目录，消除文件冲突
- **提交时机定义**：将"未经用户指示不得提交"细化为阶段产出、逻辑单元、检查点、会话保存四类时机
- **推送/PR 节奏**：推送时机与工作流阶段关联，feature → code-review，release → gate-check
- **分支生命周期**：分支创建规范、Stale 检测（7 天无提交）、合并后清理
- **Rebase 策略**：落后 main 超过 5 个提交建议 rebase，安全规则与禁止场景明确

### 📚 技术参考文档管理

在项目设计与选型期间，系统化地获取并管理框架、中间件、三方服务的官方文档、SDK 和参考示例，确保开发全程技术参考资料可追溯、可引用：

- **分层获取策略**：网页文档优先缓存到本地 Markdown；无法缓存（JS 渲染、反爬、分页过多）则生成地址索引说明文档；PDF 或可下载文档直接下载保存；用户本地文档可导入
- **按技术栈组织**：`docs/tech-reference/` 目录按技术栈分子目录（如 `react/`、`node/`、`postgresql/`），辅以通用分类目录（`middleware/`、`services/`、`general/`）
- **全局索引**：`INDEX.md` 由技能自动维护，记录所有文档的来源、类型、日期和版本
- **引用一致性验证**：检测项目文档中引用 `docs/tech-reference/` 路径的有效性，发现引用断裂和孤立文档

---

## 包含内容

| 类别 | 数量 | 描述 |
|------|------|------|
| **Agent** | 40 | 涵盖产品、架构、前端、后端、QA、DevOps、安全和文档的子 Agent |
| **技能** | 81 | 覆盖每个工作流阶段的斜杠命令（`/start`、`/brainstorm`、`/create-asset`、`/code-management`、`/tech-reference`、`/dev-story` 等） |
| **Hooks** | 12 | 提交、推送、文件变更、会话生命周期、Agent 审计跟踪和缺口检测的自动化验证 |
| **规则** | 11 | 编辑前端、后端、API、数据库、测试等代码时强制执行的路径范围编码规范 |
| **模板** | 36 | SRS、ADR、Sprint 计划、UX 规格、无障碍要求、技术参考索引等文档模板 |

---

## 团队层级

Agent 被组织成三个层级，与真实开发团队的运作方式一致：

```
第一层 — 总监层 (Opus)
  product-director    chief-architect    project-manager

第二层 — 部门负责人层 (Sonnet)
  system-analyst       lead-developer      ux-design-lead
  qa-lead             devops-lead          security-lead

第三层 — 专家层 (Sonnet/Haiku)
  react-specialist    vue-specialist       angular-specialist
  node-specialist     python-specialist    java-specialist
  go-specialist       database-engineer    api-designer
  mobile-specialist   frontend-architect   backend-architect
  devops-engineer     security-engineer    performance-engineer
  test-engineer       technical-writer     prototyper
  code-reviewer       refactoring-engineer data-engineer
  integration-engineer  accessibility-specialist
  release-manager     i18n-specialist      api-documenter
  cloud-architect     ml-engineer          sre-engineer
  community-manager
```

### 各层职责

| 层级 | 角色 | 职责 |
|------|------|------|
| **总监层** | `product-director` | 维护产品愿景，对齐用户需求，对最终交付物负责 |
| | `chief-architect` | 技术架构总负责，审批架构决策（ADR） |
| | `project-manager` | 项目计划、Sprint 规划、进度跟踪、风险管控 |
| **部门负责人层** | `system-analyst` | 需求分析与系统拆解 |
| | `lead-developer` | 开发协调与技术决策 |
| | `tech-architect` | 技术架构细化与落地 |
| | `ux-design-lead` | 用户体验设计统筹 |
| | `qa-lead` | 质量保证与测试策略 |
| | `devops-lead` | DevOps 与基础设施 |
| | `security-lead` | 安全策略与审计 |
| **专家层** | 技术栈专家 | React/Vue/Angular/Node/Python/Java/Go 等专项开发 |
| | 工程专家 | DevOps/安全/性能/测试/重构/代码审查等 |
| | 领域专家 | 发布管理/i18n/API 文档/云架构/ML/SRE/社区等 |

---

## 系统前置条件

### 必需项

以下工具是系统运行的硬性要求，缺失时无法启动工作流。

#### 1. Git 2.x+

- **用途**：版本控制、分支管理、提交规范验证
- **最低版本**：2.0
- **验证命令**：`git --version`
- **安装方式**：

| 平台 | 命令 |
|------|------|
| macOS | `brew install git` |
| Linux（Debian/Ubuntu） | `sudo apt-get install git` |
| Windows | 从 [git-scm.com/download/win](https://git-scm.com/download/win) 下载安装 |

#### 2. Claude Code（主要运行平台）

- **用途**：Agent 运行时环境，提供 Agent 调度、工具使用和 Hook 执行能力
- **安装命令**：`npm install -g @anthropic-ai/claude-code`
- **验证命令**：`claude --version`
- **依赖**：Node.js 18 LTS 或更高版本
- **注意事项**：
  - 需要有效的 Anthropic API 密钥
  - 首次使用需运行 `claude` 命令完成初始化

### 推荐项

以下工具非必需，但能增强系统功能。缺失时 Hook 将优雅降级，不影响核心功能。

#### jq

- **用途**：JSON 解析和验证，多个 Hook 依赖它解析 JSON 配置
- **验证命令**：`jq --version`
- **安装方式**：

| 平台 | 命令 |
|------|------|
| macOS | `brew install jq` |
| Linux（Debian/Ubuntu） | `sudo apt-get install jq` |
| Windows | `choco install jq` 或从 [jqlang.github.io/jq/download](https://jqlang.github.io/jq/download/) 下载 |

#### Python 3

- **用途**：JSON 验证、数据处理脚本
- **最低版本**：Python 3.6
- **验证命令**：`python3 --version`
- **安装方式**：

| 平台 | 命令 |
|------|------|
| macOS | `brew install python3` |
| Linux（Debian/Ubuntu） | `sudo apt-get install python3` |
| Windows | 从 [python.org/downloads](https://www.python.org/downloads/) 下载安装 |

#### Node.js

- **用途**：npm 包管理、前端构建工具、Node.js 后端运行时
- **最低版本**：Node.js 18 LTS
- **验证命令**：`node --version`
- **安装方式**：

| 平台 | 命令 |
|------|------|
| macOS | `brew install node` |
| Linux（Debian/Ubuntu） | 通过 [NodeSource](https://github.com/nodesource/distributions) 安装 |
| Windows | 从 [nodejs.org](https://nodejs.org/) 下载安装 |

#### Git Bash（仅 Windows 平台）

- **用途**：Windows 平台上执行 Shell 脚本（所有 Hook 脚本使用 Shell 语法编写）
- **验证命令**：`bash --version`
- **安装方式**：安装 Git for Windows 时自动包含 Git Bash
- **注意**：确保 Git Bash 的 `bash` 命令在系统 PATH 中

### 优雅降级机制

当推荐工具缺失时，系统遵循优雅降级原则：

| 工具缺失 | 影响范围 | 降级行为 | 影响 |
|----------|----------|----------|------|
| `jq` 缺失 | Hook JSON 解析 | 使用基础正则匹配 | 验证精度降低，核心功能可用 |
| `python3` 缺失 | JSON 验证 | 跳过 JSON 格式验证 | 仅命名检查，核心功能可用 |
| `node` 缺失 | Claude Code 安装 | 无法安装 Claude Code | 系统无法运行（升级为必需项） |
| Git Bash 缺失（Windows） | Hook 脚本执行 | Hook 跳过执行 | 自动化检查不可用，需手动检查 |

### 环境检查清单

首次使用前建议逐项确认：

- [ ] Git 已安装且版本 >= 2.0
- [ ] Claude Code 已安装（`npm install -g @anthropic-ai/claude-code`）
- [ ] Claude Code 初始化完成（`claude` 命令可运行）
- [ ] Anthropic API 密钥已配置
- [ ] jq 已安装（推荐）
- [ ] Python 3 已安装（推荐）
- [ ] Node.js 18 LTS 已安装（推荐，Claude Code 依赖）
- [ ] Git Bash 已安装（Windows 平台推荐）

### 平台兼容性

| 操作系统 | 支持状态 | 注意事项 |
|----------|----------|----------|
| macOS | 完全支持 | 所有工具均可通过 Homebrew 安装 |
| Linux | 完全支持 | 所有工具均可通过包管理器安装 |
| Windows | 支持 | 需安装 Git Bash 执行 Shell 脚本 |

---

## 安装步骤

### 方式一：全新项目（推荐）

适用于从零开始的新项目。

```bash
# 1. 克隆仓库
git clone <your-repo-url> my-project
cd my-project

# 2. （可选）删除原有 Git 历史，初始化为自己的仓库
rm -rf .git
git init
git add .
git commit -m "chore: 初始化项目基于 Software Engineering Studios"

# 3. 安装 Claude Code（如尚未安装）
npm install -g @anthropic-ai/claude-code

# 4. 配置 Anthropic API 密钥
#    按照 Claude Code 文档设置 API 密钥环境变量

# 5. 启动 Claude Code 会话
claude

# 6. 运行引导式上手流程
#    在 Claude Code 会话中输入：
/start
```

### 方式二：用作 GitHub 模板

适用于通过 GitHub 模板创建新仓库。

1. 在 GitHub 上将本仓库设为 Template（或点击 "Use this template"）
2. 基于模板创建新仓库并克隆到本地
3. 进入项目目录，启动 Claude Code 会话
4. 运行 `/start` 开始引导式上手流程

### 方式三：接入已有项目

适用于已有代码库，希望纳入工作室管理体系。

```bash
# 1. 进入已有项目目录
cd my-existing-project

# 2. 将 Software Engineering Studios 的配置文件复制到项目中
#    或将本仓库作为 Git 子模块添加
git submodule add <your-repo-url> .studio-framework
# 然后将所需的配置目录（.claude/、.studio/ 等）复制到项目根目录

# 3. 安装 Claude Code
npm install -g @anthropic-ai/claude-code

# 4. 启动 Claude Code 会话
claude

# 5. 检测项目当前阶段并接入
/project-stage-detect

# 6. 纳入工作室管理体系
/adopt
```

### 安装后验证

安装完成后，运行以下命令验证环境：

```bash
# 验证 Git
git --version

# 验证 Claude Code
claude --version

# 验证 jq（推荐）
jq --version

# 验证 Node.js（推荐）
node --version

# 验证 Python 3（推荐）
python3 --version
```

然后在 Claude Code 会话中运行：

```
/help
```

查看所有可用命令和 Agent 列表，确认系统已正确加载。

---

## 快速上手

### 四条上手路径

根据当前项目状态选择合适的起点：

#### 路径 A：没有想法

适用场景：想做一个项目但还没有任何具体想法。

```
/start → /brainstorm → /product-concept → /setup-stack → ...
```

1. 运行 `/start` 进入引导式上手流程
2. 运行 `/brainstorm` 进行产品头脑风暴，由 `product-director` 引导产生创意
3. 产出 `design/requirements/product-concept.md`
4. 运行 `/setup-stack` 配置技术栈
5. 继续进入需求阶段

#### 路径 B：模糊想法

适用场景：有一些模糊的产品想法或方向，但尚未成型。

```
/brainstorm [hint] → /product-concept → /setup-stack → ...
```

1. 运行 `/brainstorm` 并附带提示词，如 `/brainstorm 在线教育平台`
2. 由 `product-director` 引导细化想法
3. 产出 `design/requirements/product-concept.md`
4. 运行 `/setup-stack` 配置技术栈

#### 路径 C：清晰概念

适用场景：已有清晰的产品概念，准备开始架构设计。

```
/setup-stack → /create-architecture → /architecture-decision → ...
```

1. 运行 `/setup-stack` 配置技术栈，生成 `technical-preferences.md`
2. 直接进入架构设计阶段
3. 运行 `/create-architecture` 创建架构文档
4. 运行 `/architecture-decision` 记录关键决策（至少 3 条）

#### 路径 D：已有项目

适用场景：已有代码库或正在进行的项目，希望纳入工作室管理体系。

```
/project-stage-detect → /adopt → [根据检测结果进入对应阶段]
```

1. 运行 `/project-stage-detect` 自动检测项目所处阶段
2. 运行 `/adopt` 将现有项目纳入工作室管理体系
3. 根据检测结果进入对应阶段，可能需要运行 `/reverse-document` 逆向生成文档

### 典型工作流示例

以下是一个从零到上线的典型工作流：

```
# 1. 入门导航
/start                          # 引导式上手

# 2. 需求阶段
/brainstorm                     # 产品头脑风暴
/product-concept                # 产品概念文档
/setup-stack                    # 技术栈配置
/tech-reference fetch           # 获取技术参考文档
/map-modules                    # 系统模块映射
/requirement-spec               # 需求规格说明（可重复）
/review-all-srs                 # 需求审查

# 3. 架构设计阶段
/create-architecture            # 创建架构文档
/architecture-decision          # 架构决策记录（至少 3 条）
/architecture-review            # 架构审查
/create-control-manifest        # 创建控制清单

# 4. 技术设计阶段
/design-system-spec             # 设计系统规格
/ux-design                      # UX 设计（至少 3 个）
/ux-review                      # UX 审查
/api-spec                       # API 规格说明
/accessibility-requirements     # 无障碍需求

# 5. 预开发阶段
/create-epics                   # 创建史诗
/create-stories                 # 创建用户故事（可重复）
/sprint-plan                    # Sprint 规划

# 6. 开发阶段（可重复）
/dev-story                      # 开发单个故事
/code-review                    # 代码审查
/story-done                     # 故事完成确认
/sprint-status                  # Sprint 状态报告

# 7. 测试优化阶段
/smoke-check                    # 冒烟测试
/perf-profile                   # 性能分析（可选）
/security-audit                 # 安全审计（可选）
/test-evidence-review           # 测试证据审查

# 8. 发布部署阶段
/release-checklist              # 发布检查清单
/launch-checklist               # 上线检查清单
/changelog                      # 变更日志
```

---

## 工作原理

### 协作式，非自主式

这**不是**一个自动驾驶系统。每个 Agent 都遵循严格的协作协议：

1. **提问** — Agent 在提出解决方案前先提问，理解用户意图
2. **呈现选项** — Agent 展示 2-4 个选项及其优缺点
3. **你来决定** — 用户始终掌握决策权
4. **草稿** — Agent 在最终确认前展示成果
5. **审批** — 未经用户签字确认，不会写入任何内容

### Agent 任务路由

根据任务类型，系统自动定位负责的 Agent：

| 任务类型 | 主责 Agent | 协作 Agent |
|----------|-----------|-----------|
| 产品概念、用户故事 | `product-director` | `system-analyst` |
| 架构设计、技术选型 | `chief-architect` | `tech-architect`、`backend-architect` |
| Sprint 规划、进度跟踪 | `project-manager` | `lead-developer`、`qa-lead` |
| 前端组件开发 | `react`/`vue`/`angular` | `frontend-architect`、`ux-design-lead` |
| 后端服务开发 | `node`/`python`/`java`/`go` | `backend-architect`、`api-designer` |
| 数据库设计 | `database-engineer` | `data-engineer`、`backend-architect` |
| API 设计与文档 | `api-designer` | `api-documenter`、`backend-architect` |
| 测试编写与执行 | `test-engineer` | `qa-lead`、`lead-developer` |
| 性能优化 | `performance-engineer` | `devops-engineer`、`backend-architect` |
| 安全审计 | `security-engineer` | `security-lead`、`sre-engineer` |
| 部署与发布 | `release-manager` | `devops-lead`、`devops-engineer` |
| 代码审查 | `code-reviewer` | `lead-developer` |
| 国际化 | `i18n-specialist` | `technical-writer`、`frontend-architect` |
| 无障碍 | `accessibility-specialist` | `ux-design-lead` |

---

## 斜杠命令目录

共 81 个斜杠命令，按软件开发生命周期阶段分类：

### 入门导航（5 个）

| 命令 | 说明 |
|------|------|
| `/start` | 引导式上手流程，检测项目阶段并推荐起点 |
| `/help` | 查看所有可用命令和 Agent 列表 |
| `/onboard` | 新成员入职引导，展示项目结构和规范 |
| `/project-stage-detect` | 自动检测当前项目所处阶段 |
| `/adopt` | 将现有项目纳入工作室管理体系 |

### 需求阶段（9 个）

| 命令 | 说明 |
|------|------|
| `/brainstorm` | 产品头脑风暴，由产品总监引导产生创意 |
| `/setup-stack` | 技术栈配置，生成 `technical-preferences.md` |
| `/product-concept` | 生成产品概念文档 |
| `/map-modules` | 将产品概念拆解为系统模块 |
| `/requirement-spec` | 为各模块编写详细需求规格说明（可重复） |
| `/quick-spec` | 快速需求规格（轻量版） |
| `/review-all-srs` | 全量需求规格审查 |
| `/consistency-check` | 需求一致性检查 |
| `/tech-reference` | 技术参考文档管理（在线缓存、PDF 下载、地址索引、本地导入、引用验证） |

### 架构设计阶段（7 个）

| 命令 | 说明 |
|------|------|
| `/design-review` | 设计审查 |
| `/create-architecture` | 创建架构文档 |
| `/architecture-decision` | 记录架构决策（ADR），至少 3 条 |
| `/architecture-review` | 架构审查 |
| `/create-control-manifest` | 创建控制清单 |
| `/propagate-design-change` | 传播设计变更 |
| `/reverse-document` | 逆向文档生成 |

### 技术设计阶段（5 个）

| 命令 | 说明 |
|------|------|
| `/design-system-spec` | 设计系统规格 |
| `/ux-design` | UX 设计（至少 3 个） |
| `/ux-review` | UX 审查 |
| `/api-spec` | API 规格说明 |
| `/accessibility-requirements` | 无障碍需求 |

### 预开发阶段（7 个）

| 命令 | 说明 |
|------|------|
| `/prototype` | 原型制作（可选） |
| `/build-mvp` | MVP 构建（可选） |
| `/create-epics` | 创建史诗 |
| `/create-stories` | 创建用户故事（可重复） |
| `/test-setup` | 测试环境搭建 |
| `/sprint-plan` | Sprint 规划 |
| `/estimate` | 工作量估算 |

### 开发阶段（12 个）

| 命令 | 说明 |
|------|------|
| `/story-readiness` | 故事就绪检查 |
| `/dev-story` | 开发故事（可重复） |
| `/code-management` | 代码管理工作流（worktree/commit/push/cleanup/rebase） |
| `/code-review` | 代码审查（可重复） |
| `/story-done` | 故事完成确认（可重复） |
| `/sprint-status` | Sprint 状态报告 |
| `/scope-check` | 范围检查 |
| `/bug-report` | 缺陷报告 |
| `/bug-triage` | 缺陷分诊 |
| `/hotfix` | 热修复 |
| `/qa-plan` | QA 计划 |
| `/retrospective` | 回顾会议 |

### 测试优化阶段（7 个）

| 命令 | 说明 |
|------|------|
| `/perf-profile` | 性能分析 |
| `/security-audit` | 安全审计 |
| `/tech-debt` | 技术债务评估 |
| `/smoke-check` | 冒烟测试 |
| `/soak-test` | 耐久测试 |
| `/regression-suite` | 回归测试套件 |
| `/test-evidence-review` | 测试证据审查 |

### 发布部署阶段（6 个）

| 命令 | 说明 |
|------|------|
| `/gate-check` | 门禁检查 |
| `/release-checklist` | 发布检查清单 |
| `/launch-checklist` | 上线检查清单 |
| `/changelog` | 变更日志 |
| `/patch-notes` | 补丁说明 |
| `/day-one-patch` | 首日补丁 |

### 团队编排（7 个）

| 命令 | 说明 |
|------|------|
| `/team-frontend` | 前端团队编排 |
| `/team-backend` | 后端团队编排 |
| `/team-qa` | QA 团队编排 |
| `/team-release` | 发布团队编排 |
| `/team-polish` | 打磨团队编排 |
| `/team-security` | 安全团队编排 |
| `/team-devops` | DevOps 团队编排 |

### 动态扩展（5 个）

| 命令 | 说明 |
|------|------|
| `/create-asset` | 动态扩展统一入口，路由至对应创建工作流 |
| `/create-agent` | 创建新专家 Agent（六阶段工作流） |
| `/create-skill` | 创建新技能（六阶段工作流） |
| `/create-rule` | 创建新路径规则（六阶段工作流） |
| `/asset-review` | 资产审核（由 chief-architect 执行） |

### 工具类（9 个）

| 命令 | 说明 |
|------|------|
| `/content-audit` | 内容审计 |
| `/test-flakiness` | 测试抖动检测 |
| `/skill-test` | 技能测试 |
| `/skill-improve` | 技能改进 |
| `/user-test-report` | 用户测试报告 |
| `/audit-docs` | 文档审计 |
| `/test-helpers` | 测试辅助库生成 |
| `/localize` | 国际化扫描与提取 |
| `/milestone-review` | 里程碑审查 |

### 平台工具（2 个）

| 命令 | 说明 |
|------|------|
| `/sync-platforms` | 将 `.studio/` 规范源同步到所有平台 |
| `/platform-check` | 检查各平台配置一致性 |

---

## 项目结构

```
项目根目录/
├── CLAUDE.md                           # Claude Code 主配置文件
├── AGENTS.md                           # Codex/Cursor/WorkBuddy 主配置
├── README.md                           # 本文件
├── CONTRIBUTING.md                     # 贡献指南
├── LICENSE                             # MIT 许可证
├── .gitignore                          # Git 忽略规则
├── .windsurfrules                      # Windsurf 规则文件
│
├── .studio/                            # ★ 规范源层（所有修改在此进行）
│   ├── agents/                         # 40 个 Agent 定义（源文件）
│   ├── skills/                         # 81 个技能定义（源文件）
│   ├── hooks/                          # 12 个 Hook 脚本（源文件）
│   ├── rules/                          # 11 个路径范围编码规范（源文件）
│   ├── templates/                      # 36 个文档模板（源文件）
│   ├── registry/                      # 动态扩展资产注册表和变更日志
│   │   ├── asset-registry.yaml         # 资产注册表（追踪所有动态新增资产）
│   │   ├── asset-changelog.md          # 资产变更日志
│   │   ├── proposals/                  # 资产提案文档
│   │   └── reviews/                    # 资产审核报告
│   ├── docs/                           # 规范文档
│   ├── project/                        # 项目配置
│   └── manifest.yaml                   # 工作室清单
│
├── .claude/                            # Claude Code 输出层（自动生成）
│   ├── settings.json                   # Hooks、权限、安全规则
│   ├── agents/                         # 40 个 Agent 定义
│   ├── hooks/                          # 12 个 Hook 脚本
│   ├── rules/                          # 11 个路径范围编码规范
│   └── docs/                           # 规范文档和模板
│       └── templates/                  # 36 个文档模板
│
├── .cursor/                            # Cursor 输出层（自动生成）
├── .windsurf/                          # Windsurf 输出层（自动生成）
├── .trae/                              # Trae IDE 输出层（自动生成）
├── .hermes/                            # Hermes Agent 输出层（自动生成）
├── .workbuddy/                         # WorkBuddy 输出层（自动生成）
│
├── src/                                # 源代码
│   ├── frontend/                       # 前端代码
│   ├── backend/                        # 后端代码
│   └── shared/                         # 前后端共享代码
│
├── docs/                               # 技术文档和 ADR
│   ├── tech-reference/                 # 技术参考文档（按技术栈分目录）
│   └── examples/                       # 示例文档
│
├── design/                             # 需求文档和 UX 规范
├── tests/                              # 测试套件
│   └── unit/                           # 单元测试
│
├── tools/                              # 构建和管线工具
│   └── adapters/                       # 多平台同步脚本
│       ├── sync-all.sh                 # 同步所有平台
│       ├── sync-claude-code.sh         # 同步到 Claude Code
│       ├── sync-codex.sh               # 同步到 Codex
│       ├── sync-cursor.sh              # 同步到 Cursor
│       ├── sync-windsurf.sh            # 同步到 Windsurf
│       ├── sync-trae.sh                # 同步到 Trae IDE
│       ├── sync-hermes.sh              # 同步到 Hermes Agent
│       └── sync-workbuddy.sh           # 同步到 WorkBuddy
│
└── production/                         # Sprint、里程碑、发布跟踪
    ├── session-state/                  # 会话状态
    └── session-logs/                   # 会话日志
```

> **重要**：所有配置修改必须在 `.studio/` 规范源中进行，**不要直接修改各平台输出目录**（`.claude/`、`.cursor/`、`.windsurf/` 等），它们是自动生成的。

---

## 多平台支持

Software Engineering Studios 不仅支持 Claude Code，还适配以下 AI Agent 平台：

| 平台 | 配置方式 | 技能系统 | Hook 支持 |
|------|----------|----------|-----------|
| Claude Code | `.claude/` 目录 | 原生 SKILL.md | 原生支持（12 个） |
| Codex | `AGENTS.md` | Agent Skills 兼容 | 不支持 |
| Cursor | `.cursor/rules/*.mdc` | Agent Skills 兼容 | 不支持 |
| Windsurf | `.windsurfrules` + `.windsurf/workflows/` | 工作流 .md | 不支持 |
| Trae IDE | `.trae/rules/project_rules.md` | Agent Skills 兼容 | 不支持 |
| Hermes Agent | `.hermes/SOUL.md` + `.hermes/skills/` | Hermes 扩展 SKILL.md | 后台审视机制 |
| WorkBuddy | `.workbuddy/` + SkillHub | Agent Skills 兼容 | MCP 协议 |

### 三层适配架构

所有平台配置从 `.studio/` 规范源自动生成，确保一致性：

```
.studio/（规范源）→ tools/adapters/（同步脚本）→ 各平台配置目录（输出层）
```

1. **规范源层**（`.studio/`）：平台无关的单一真相源，所有修改在此进行
2. **适配层**（`tools/adapters/`）：同步脚本，将规范源转换为各平台格式
3. **输出层**：各平台原生配置目录（`.claude/`、`.cursor/`、`.windsurf/` 等）

### 同步操作

修改 `.studio/` 中的源文件后，运行同步脚本更新各平台配置：

```bash
# 同步所有平台
bash tools/adapters/sync-all.sh

# 同步单个平台
bash tools/adapters/sync-claude-code.sh
bash tools/adapters/sync-cursor.sh
bash tools/adapters/sync-windsurf.sh
bash tools/adapters/sync-trae.sh

# 检查配置一致性
# 在 AI 会话中运行：
/platform-check
```

### 各 AI 平台安装方式

| 平台 | 安装方式 |
|------|----------|
| Claude Code | `npm install -g @anthropic-ai/claude-code` |
| Codex CLI | `npm install -g @openai/codex` |
| Cursor | 从 [cursor.sh](https://cursor.sh/) 下载安装 |
| Windsurf | 从 [windsurf.com](https://windsurf.com/) 下载安装 |
| Trae IDE | 从 [trae.cn](https://www.trae.cn/) 下载安装 |
| Hermes Agent | 参考官方文档获取安装命令 |
| WorkBuddy | 参考官方文档获取安装命令 |

---

## 动态扩展机制

预设的 40 个 Agent、75 个技能和 11 个路径规则覆盖了软件开发的主要场景，但技术栈、工具链和领域场景极其多样，预设资产无法穷举所有可能性。动态扩展机制提供一套**受控的、可审计的**流程，允许在项目运行过程中根据需求新增资产。

### 适用场景

- 项目采用了预设未覆盖的技术栈（如 Rust、Swift、Kotlin）
- 项目出现预设未覆盖的领域场景（如合规审计、数据迁移）
- 现有技能无法满足特定工作流需求
- 现有规则未覆盖新的代码路径范围

### 三类可扩展资产

| 资产类型 | 创建命令 | 定义目录 | 审核者 |
|----------|----------|----------|--------|
| 专家 Agent | `/create-agent` | `.studio/agents/` | chief-architect |
| 技能（Skill） | `/create-skill` | `.studio/skills/[name]/` | chief-architect |
| 路径规则（Rule） | `/create-rule` | `.studio/rules/` | chief-architect |

> **统一入口**：运行 `/create-asset` 可引导选择资产类型并自动路由至对应工作流。

### 六阶段工作流

所有资产创建均遵循统一的六阶段工作流，确保新增资产符合规范、不与现有资产冲突，并经审核和用户批准：

```
提案 → 草稿 → 审核 → 批准/拒绝 → 注册 → 同步
 ①      ②      ③        ④         ⑤       ⑥
```

| 阶段 | 执行者 | 产出 | 协作协议 |
|------|--------|------|----------|
| ① 提案 | 总监层/部门负责人层 | 提案文档 | Agent 询问用户需求背景 |
| ② 草稿 | 提案 Agent | 资产定义草稿 | Agent 展示草稿供用户审阅 |
| ③ 审核 | chief-architect | 审核报告（APPROVE/CONCERNS/REJECT） | 审核报告作为另一种草稿 |
| ④ 批准/拒绝 | **用户** | 批准或拒绝决定 | 用户做出最终决策 |
| ⑤ 注册 | 提案 Agent | 资产文件 + 更新索引 | 多文件变更集的批量审批 |
| ⑥ 同步 | 任意 Agent | 各平台配置更新 | 用户指示后执行 |

### 权限矩阵

| 资产类型 | 提案权 | 审核权 | 批准权 |
|----------|--------|--------|--------|
| 专家 Agent | 总监层、部门负责人层 | chief-architect | **用户最终批准** |
| Skill | 部门负责人层 | chief-architect | **用户最终批准** |
| Rule | 部门负责人层 | chief-architect | **用户最终批准** |

**关键约束**：
- 专家层不得直接提案，需通过部门负责人代为提案
- 最终批准权始终属于用户
- 仅限专家层 Agent 新增，不允许新增总监层或部门负责人层 Agent

### 审核标准

**Agent 审核**：职责不重叠、模型层级合理、委托图完整、协作协议明确、Body 章节完整、platforms 配置正确

**Skill 审核**：名称合规（kebab-case）、frontmatter 完整、Body 完整、模型层级合理、不与现有技能重复

**Rule 审核**：路径范围不冲突、规范条目清晰可执行、示例完整（正确 + 错误）、platforms 配置正确

### 资产注册表与变更追踪

- **资产注册表**（`.studio/registry/asset-registry.yaml`）：追踪所有动态新增资产的状态流转
  - 状态流：`proposed → draft → reviewed → approved → registered`（正常）或 `→ rejected`（拒绝）
- **资产变更日志**（`.studio/registry/asset-changelog.md`）：按时间倒序记录所有创建、修改、废弃事件

### 典型扩展示例

```
# 场景：项目需要 Rust 技术栈支持

# 1. 通过统一入口启动
/create-asset agent

# 2. 或直接调用专项命令
/create-agent rust-specialist

# 3. 审核阶段
/asset-review agent rust-specialist

# 4. 批准后同步到所有平台
/sync-platforms
/platform-check
```

---

## 代码管理工作流

当多个 AI Agent 并行执行独立任务时，同一工作目录会产生文件冲突。代码管理工作流通过 **git worktree 并行隔离**、**提交时机定义**、**推送/PR 节奏**和**分支生命周期管理**，确保代码仓库保持整洁性、可追溯性和质量可控性。

### 统一入口

```
/code-management [操作类型]
```

支持 5 个操作子命令：

| 操作 | 命令 | 用途 |
|------|------|------|
| worktree | `/code-management worktree` | 创建或管理 git worktree，用于并行任务代码隔离 |
| commit | `/code-management commit` | 判断提交时机并执行提交，遵循 Conventional Commits |
| push | `/code-management push` | 准备推送或创建 PR，检查审查前置条件 |
| cleanup | `/code-management cleanup` | 清理已合并分支、stale 分支和残留 worktree |
| rebase | `/code-management rebase` | 执行 rebase 操作，整理提交历史 |

### Git Worktree 并行隔离

为每个并行子 Agent 创建独立工作目录，消除文件冲突：

```
项目根目录/
├── src/                         # 主仓库
├── ../worktrees/                # Worktree 根目录（与主仓库同级）
│   ├── feature-user-auth-ui/    # 前端 Agent 工作树
│   ├── feature-user-auth-api/   # 后端 Agent 工作树
│   └── feature-user-auth-test/  # 测试 Agent 工作树
```

**生命周期**：创建 → 工作 → 提交 → 合并 → 清理

| 并行场景 | 子 Agent | Worktree | 分支 |
|----------|----------|----------|------|
| 前端开发 | react-specialist | `feature-user-auth-ui` | `feature/user-auth-ui` |
| 后端开发 | node-specialist | `feature-user-auth-api` | `feature/user-auth-api` |
| 测试编写 | test-engineer | `feature-user-auth-test` | `feature/user-auth-test` |

### 提交时机定义

**核心原则**：未经用户指示，Agent 不得自主执行 Git 提交。但 Agent 应在满足以下时机时主动建议提交。

| 时机类型 | 触发条件 | 说明 |
|----------|----------|------|
| 阶段产出提交 | 工作流阶段完成必需产出物后 | 如 dev-story 完成、架构审查通过 |
| 逻辑单元提交 | 一个完整逻辑变更单元完成后 | 如验收标准实现 + 测试通过 |
| 检查点提交 | 通过 review 或 gate-check 后 | 审查通过的代码状态应被保存 |
| 会话保存提交 | 上下文压缩前、会话结束前 | 与上下文管理策略联动 |

**禁止提交时机**：lint 未通过、测试失败、含 `console.log`、含硬编码密钥、含裸 TODO、含无效 JSON、需求文档缺章节。

**提交粒度**：一个提交对应一个逻辑变更，保持原子性，单提交不超过 20 个文件。

### 推送与 PR 节奏

| 工作流阶段 | PR 类型 | 前置审查 | 分支类型 |
|------------|---------|----------|----------|
| 开发阶段（dev-story） | feature PR | code-review | `feature/*` |
| 开发阶段（hotfix） | hotfix PR | code-review（事后补） | `hotfix/*` |
| 测试优化阶段 | release PR | gate-check | `release/*` |
| 发布阶段 | release PR | launch-checklist | `release/*` |

**受保护分支不直接推送**：main/master/develop 必须通过 PR 合并。

### 分支生命周期

| 分支类型 | 格式 | 用途 | 示例 |
|----------|------|------|------|
| 主干 | `main` | 生产分支，受保护 | `main` |
| 功能分支 | `feature/[描述]` | 新功能开发 | `feature/user-auth` |
| 修复分支 | `fix/[描述]` | 缺陷修复 | `fix/pagination` |
| 热修复分支 | `hotfix/[描述]` | 生产紧急修复 | `hotfix/login-crash` |
| 发布分支 | `release/vX.Y.Z` | 发布准备 | `release/v1.2.0` |

**Stale 检测**：超过 7 天无提交的非受保护分支标注为 stale，标注后 3 天无活动建议删除。

**定期清理**：每个 Sprint 结束时运行 `/code-management cleanup` 执行分支整理流程。

### Rebase 策略

- **触发条件**：feature 分支落后 main 超过 5 个提交，或 PR 审查发现冲突
- **安全规则**：仅对未推送的本地提交 rebase；rebase 前确保工作区干净；rebase 后必须运行测试
- **禁止场景**：禁止 rebase 受保护分支（main/master/release/*）；禁止对已推送提交 rebase（除非个人分支且已通知协作者）
- **交互式 rebase**：用于合并 WIP 提交、修正提交信息、拆分过大提交

---

## 技术参考文档管理

在项目设计与选型期间，需要获取并保存框架、中间件、三方服务等的相关文档（官方文档、SDK、参考示例），以保证功能引用一致性。技术参考文档管理技能 `/tech-reference` 提供系统化的文档获取、缓存、索引和验证能力。

### 统一入口

```
/tech-reference [操作类型]
```

支持 5 个操作子命令：

| 操作 | 命令 | 用途 |
|------|------|------|
| fetch | `/tech-reference fetch` | 获取在线文档（网页缓存或 PDF 下载），保存到 `docs/tech-reference/` |
| index | `/tech-reference index` | 为无法本地缓存的网页文档生成地址索引说明文档 |
| import | `/tech-reference import` | 将用户本地文档复制到 `docs/tech-reference/` |
| list | `/tech-reference list` | 列出已有技术参考文档清单和索引状态 |
| verify | `/tech-reference verify` | 验证项目文档中引用的技术参考路径是否有效 |

### 分层获取策略

针对不同文档来源采用分层策略，确保两种路径均有产出：

| 文档类型 | 获取方式 | 保存格式 |
|----------|----------|----------|
| 网页文档（可缓存） | WebFetch 抓取 → 格式清理 → 元数据注入 | `docs/tech-reference/[技术栈]/[文档名].md` |
| 网页文档（无法缓存） | WebFetch 获取目录结构 → 生成地址索引 | `docs/tech-reference/[技术栈]/[文档名]-index.md` |
| PDF / 可下载文档 | 下载保存 | `docs/tech-reference/[技术栈]/[文档名].pdf` |
| 用户本地文档 | 读取源文件 → 复制到目标路径 | `docs/tech-reference/[技术栈]/[文件名]` |

**无法缓存的场景**：页面依赖 JavaScript 渲染、网站反爬机制、文档分页过多（超过 20 页）、付费内容或明确禁止缓存。

### 目录组织结构

`docs/tech-reference/` 按技术栈分子目录组织：

```
docs/tech-reference/
├── README.md                # 目录说明
├── INDEX.md                 # 全局索引（技能自动维护）
├── react/                   # 技术栈子目录
├── node/
├── postgresql/
├── middleware/              # 跨技术栈中间件文档
├── services/                # 三方服务文档
└── general/                 # 通用参考文档
```

### 引用一致性验证

`/tech-reference verify` 检查以下一致性：

| 检查项 | 规则 | 处理 |
|--------|------|------|
| 引用路径存在 | 项目文档引用的 `docs/tech-reference/` 路径须对应实际文件 | 不存在标注"引用断裂" |
| 索引完整性 | `INDEX.md` 中每个条目须有对应文件 | 缺失标注"索引过期" |
| 文件可索引 | 目录中每个文件应在 `INDEX.md` 有记录 | 未记录标注"未索引" |
| 孤立文档 | 存在但未被任何项目文档引用的参考文档 | 标注"孤立文档"（提示而非错误） |

### 与 setup-stack 的关系

`/setup-stack` 技术栈配置技能的阶段 4（填充技术参考）已修改为委派调用 `/tech-reference`：setup-stack 负责技术偏好填充和知识缺口检测，tech-reference 负责参考文档的获取、缓存、索引和验证。

---

## 协调规则

Agent 之间的协作遵循 5 条核心协调规则：

1. **纵向委派**：上层 Agent 可向下层 Agent 委派任务，但不可越级向下委派（如 `product-director` 不可直接调用 `react-specialist`，应通过 `lead-developer`）
2. **横向协商**：同层 Agent 之间通过协商解决跨领域问题，任何一方无单方面决定权
3. **冲突解决**：当 Agent 之间出现意见冲突时，由上一层级 Agent 仲裁（如部门负责人层冲突由对应总监仲裁）
4. **变更传播**：任何架构或设计变更必须通过 `/propagate-design-change` 传播到所有受影响的 Agent 和文档
5. **禁止单方面跨领域变更**：任何 Agent 不得单方面修改不属于自己领域的文件或配置，必须通过协商和审批流程

---

## 协作协议

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

---

## 贡献指南

感谢你对 Software Engineering Studios 项目的兴趣！详见 [CONTRIBUTING.md](CONTRIBUTING.md) 了解完整的贡献指南。

### 快速贡献流程

1. **Fork** 本仓库
2. 创建特性分支：`git checkout -b feature/amazing-feature`
3. 提交更改，遵循 Conventional Commits 格式：`git commit -m 'feat: 添加了某功能'`
4. 推送到分支：`git push origin feature/amazing-feature`
5. 创建 Pull Request

### 修改规则

所有配置修改必须在 `.studio/` 规范源中进行：

1. 修改 `.studio/` 中的源文件
2. 运行 `bash tools/adapters/sync-all.sh` 同步到各平台
3. 运行 `/platform-check` 验证一致性
4. 提交变更（包含源文件和生成的平台配置）

### 提交规范

遵循 Conventional Commits 格式：

| 类型 | 说明 |
|------|------|
| `feat:` | 新功能 |
| `fix:` | 缺陷修复 |
| `docs:` | 文档变更 |
| `refactor:` | 代码重构 |
| `test:` | 测试相关 |
| `chore:` | 构建/工具变更 |

Subject 不超过 50 字。禁止向 `main`/`master`/`release/*` 分支直接推送。

---

## 灵感来源与致谢

本项目的架构理念和组织模式灵感来源于开源项目 [Claude Code Game Studios](https://github.com/Donchitos/Claude-Code-Game-Studios)（作者：[Donchitos](https://github.com/Donchitos)）。该项目将 AI 编码会话转变为一间游戏开发工作室，通过多层 Agent 协作、自动化质量关卡和路径范围编码规范，为 AI 辅助开发赋予了真实团队的结构与流程。

Software Engineering Studios 在此基础上做了如下演进：

- **领域适配**：将游戏开发场景调整为通用软件工程领域，覆盖前端、后端、数据库、API、基础设施等通用技术栈
- **角色重构**：从游戏开发角色（创意总监、技术总监、制作人等）重构为软件工程角色（产品总监、首席架构师、项目经理等）
- **工作流泛化**：从游戏开发生命周期（GDD、垂直切片、试玩等）泛化为软件开发生命周期（需求规格、架构设计、Sprint、发布等）
- **多平台扩展**：从单一 Claude Code 平台扩展到 7 大 AI Agent 平台（Claude Code、Codex、Cursor、Windsurf、Trae IDE、Hermes Agent、WorkBuddy）

在此向 [Claude Code Game Studios](https://github.com/Donchitos/Claude-Code-Game-Studios) 项目及其作者致以诚挚感谢，感谢其开创性的工作为本项目提供了宝贵的架构思路。

---

## 许可证

本项目基于 **MIT 许可证** 开源。详见 [LICENSE](LICENSE) 文件。

```
MIT License

Copyright (c) 2026 zls3434

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

---

## 版权信息

© 2026 zls3434. 保留所有权利。

本项目由 GitHub 用户 `zls3434` 开发和维护。

- **项目名称**：Software Engineering Studios（软件工程工作室）
- **版权所有者**：zls3434（GitHub 用户名）
- **版权年份**：2026
- **开源许可证**：MIT License
- **版权验证**：如需主张版权或验证所有权，请通过 GitHub 账号 [github.com/zls3434](https://github.com/zls3434) 联系验证
- **适用司法管辖区**：无特定限制（MIT 许可证为全球性许可）

### 第三方提及

本项目可能提及或引用以下第三方产品和服务，这些提及仅用于说明兼容性，不构成 endorsement：

- **Claude Code** — Anthropic, PBC 的产品
- **Cursor** — Anysphere 的产品
- **Windsurf** — Codeium 的产品
- **Trae IDE** — 字节跳动的产品
- **Codex** — OpenAI 的产品
- **Git** — 自由软件，遵循 GPL 许可证

所有第三方商标、产品名称和品牌名称均属于其各自所有者。使用这些名称不表示任何 endorsement 或从属关系。

---

## 免责声明

### 软件按"现状"提供

本软件按"现状"（AS IS）基础提供，不附带任何明示或暗示的保证，包括但不限于：

- **适销性保证**（MERCHANTABILITY）
- **特定用途适用性保证**（FITNESS FOR A PARTICULAR PURPOSE）
- **非侵权保证**（NON-INFRINGEMENT）

作者或版权持有人不对软件的性能、可靠性、准确性或完整性作任何承诺或保证。

### AI 生成内容风险

本项目涉及 AI 辅助软件开发。使用者应知悉并理解以下风险：

- **AI 生成的代码可能包含错误**：AI 模型可能生成包含逻辑错误、安全漏洞或不符合最佳实践的代码。使用者必须对 AI 生成的所有代码进行人工审查和测试后方可用于生产环境。
- **AI 生成的内容可能不准确**：AI 模型可能生成不准确、过时或存在偏见的信息。使用者应独立验证所有 AI 生成的内容。
- **不可替代专业判断**：本工具不替代专业软件工程师的判断。涉及安全关键、金融关键或其他高风险场景的决策应由具备相应资质的专业人员做出。
- **模型行为可能变化**：AI 模型的行为可能随时间推移而变化，使用者应定期验证系统输出的一致性。

### 使用者责任

使用者在使用本软件时承担全部责任，包括但不限于：

- **代码审查**：使用者有责任对所有 AI 生成的代码进行全面审查，确保其安全性、正确性和适用性
- **测试验证**：使用者有责任进行充分的测试，包括单元测试、集成测试、端到端测试和安全测试
- **安全合规**：使用者有责任确保生成的代码符合适用的安全标准和法规要求
- **许可证合规**：使用者有责任确保使用的第三方组件和库符合其许可证条款
- **数据保护**：使用者有责任保护用户数据，遵守适用的数据保护法规（如 GDPR、CCPA 等）

### 不承担损害赔偿责任

在任何情况下，作者或版权持有人均不对因使用本软件或与本软件相关的任何直接、间接、附带、特殊、惩罚性或后果性损害（包括但不限于数据丢失、利润损失、业务中断）承担责任，无论此类损害是基于合同、侵权、过失还是其他法律理论产生，即使已被告知此类损害的可能性。

### 无专业建议

本项目提供的内容不构成任何形式的专业建议，包括但不限于法律、财务、工程或安全建议。在做出重要决策前，请咨询具备相应资质的专业人士。

### 持续变更

本软件可能随时更新、修改或停止提供，不另行通知。版权持有人不保证软件的持续可用性或兼容性。

### 反馈与报告

如发现 Bug、安全漏洞或有改进建议，欢迎通过 [GitHub Issues](../../issues) 提交反馈。提交前请先搜索是否已有类似 issue。

---

<div align="center">

**Software Engineering Studios** — 为 AI 辅助开发赋予真实开发团队的结构

[报告问题](../../issues) · [查看贡献指南](CONTRIBUTING.md) · [查看许可证](LICENSE)

</div>