<!-- Software Engineering Studios -->
# 系统前置条件

本文件列出 Software Engineering Studios 运行所需的系统环境和工具。分为必需项和推荐项。

## 1. 必需项

以下工具是系统运行的硬性要求，缺失时无法启动工作流。

### 1.1 Git 2.x+

- **用途**：版本控制、分支管理、提交规范验证
- **最低版本**：2.0
- **验证命令**：`git --version`
- **使用场景**：
  - 所有文件的版本控制
  - `validate-commit.sh` Hook 检查提交规范
  - `validate-push.sh` Hook 检查受保护分支
  - `/changelog` 和 `/patch-notes` 读取提交历史
- **安装指南**：
  - Windows：从 https://git-scm.com/download/win 下载安装
  - macOS：`brew install git`
  - Linux（Debian/Ubuntu）：`sudo apt-get install git`

### 1.2 Claude Code

- **用途**：Agent 运行时环境，提供 Agent 调度、工具使用和 Hook 执行能力
- **安装命令**：`npm install -g @anthropic-ai/claude-code`
- **验证命令**：`claude --version`
- **使用场景**：
  - 所有 40 个 Agent 的运行环境
  - 所有 74 个斜杠命令的执行入口
  - 所有 12 个 Hook 的触发和执行
  - 子 Agent 派发和协调
- **依赖**：Node.js（npm 的运行环境）
- **注意事项**：
  - 需要有效的 Anthropic API 密钥
  - 首次使用需运行 `claude` 命令完成初始化

## 2. 推荐项

以下工具非必需，但能增强系统功能。缺失时 Hook 将优雅降级，不影响核心功能。

### 2.1 jq

- **用途**：JSON 解析和验证
- **推荐原因**：多个 Hook 依赖 jq 解析 JSON 配置
- **验证命令**：`jq --version`
- **使用场景**：
  - `validate-commit.sh`：解析提交规范配置
  - `validate-assets.sh`：验证 JSON 文件格式
  - `validate-push.sh`：解析分支保护配置
- **降级行为**：缺失时使用基础正则匹配替代，验证精度降低
- **安装指南**：
  - Windows：`choco install jq` 或从 https://jqlang.github.io/jq/download/ 下载
  - macOS：`brew install jq`
  - Linux（Debian/Ubuntu）：`sudo apt-get install jq`

### 2.2 Python 3

- **用途**：JSON 验证、数据处理脚本
- **推荐原因**：作为 jq 的替代方案验证 JSON，同时支持部分数据处理脚本
- **最低版本**：Python 3.6
- **验证命令**：`python3 --version`
- **使用场景**：
  - `validate-assets.sh`：JSON 格式验证（jq 替代方案）
  - 数据迁移工具脚本
  - 测试数据生成
- **降级行为**：缺失时跳过 JSON 格式验证，仅检查文件命名
- **安装指南**：
  - Windows：从 https://www.python.org/downloads/ 下载安装
  - macOS：`brew install python3`
  - Linux（Debian/Ubuntu）：`sudo apt-get install python3`

### 2.3 Node.js

- **用途**：npm 包管理、前端构建工具、Node.js 后端运行时
- **推荐原因**：Claude Code 依赖 npm 安装；若选择 Node.js 后端则必需
- **最低版本**：Node.js 18 LTS
- **验证命令**：`node --version`
- **使用场景**：
  - Claude Code 安装（通过 npm）
  - 前端构建工具运行（Vite/Webpack 等）
  - Node.js 后端开发和运行
  - npm 测试脚本执行
- **降级行为**：缺失时无法安装 Claude Code（变为必需项）
- **安装指南**：
  - Windows：从 https://nodejs.org/ 下载安装
  - macOS：`brew install node`
  - Linux（Debian/Ubuntu）：通过 NodeSource 安装

### 2.4 Git Bash（Windows 平台）

- **用途**：Windows 平台上执行 Shell 脚本
- **推荐原因**：所有 Hook 脚本使用 Shell 语法编写，Windows 需要兼容环境
- **仅适用平台**：Windows
- **验证命令**：`bash --version`
- **使用场景**：
  - 执行所有 12 个 Hook 脚本（`.sh` 文件）
  - 执行 `tools/` 目录下的 Shell 脚本
- **降级行为**：缺失时 Hook 无法执行，功能受限
- **安装指南**：
  - 安装 Git for Windows 时自动包含 Git Bash
  - 下载地址：https://git-scm.com/download/win
- **注意事项**：
  - 确保 Git Bash 的 `bash` 命令在系统 PATH 中
  - 或在 `.claude/settings.json` 中配置 Shell 路径

## 3. 优雅降级机制

当推荐工具缺失时，系统遵循优雅降级原则：

| 工具缺失 | 影响范围 | 降级行为 | 影响 |
| --- | --- | --- | --- |
| `jq` 缺失 | Hook JSON 解析 | 使用基础正则匹配 | 验证精度降低，核心功能可用 |
| `python3` 缺失 | JSON 验证 | 跳过 JSON 格式验证 | 仅命名检查，核心功能可用 |
| `node` 缺失 | Claude Code 安装 | 无法安装 Claude Code | 系统无法运行（升级为必需项） |
| Git Bash 缺失（Windows） | Hook 脚本执行 | Hook 跳过执行 | 自动化检查不可用，需手动检查 |

## 4. 环境检查清单

首次使用前建议运行以下检查：

- [ ] Git 已安装且版本 >= 2.0
- [ ] Claude Code 已安装（`npm install -g @anthropic-ai/claude-code`）
- [ ] Claude Code 初始化完成（`claude` 命令可运行）
- [ ] jq 已安装（推荐）
- [ ] Python 3 已安装（推荐）
- [ ] Node.js 已安装（推荐，Claude Code 依赖）
- [ ] Git Bash 已安装（Windows 平台推荐）
- [ ] Anthropic API 密钥已配置

## 5. 平台兼容性

| 平台 | 支持状态 | 注意事项 |
| --- | --- | --- |
| macOS | 完全支持 | 所有工具均可通过 Homebrew 安装 |
| Linux | 完全支持 | 所有工具均可通过包管理器安装 |
| Windows | 支持 | 需安装 Git Bash 执行 Shell 脚本 |

## 6. AI Agent 平台前置条件

Software Engineering Studios 支持多个 AI Agent 平台，各平台的前置条件如下：

| AI 平台 | 必需配置 | 推荐配置 | Hook 支持 |
| --- | --- | --- | --- |
| Claude Code | Claude Code CLI + Anthropic API 密钥 | jq, Python 3, Node.js 18+ | 原生支持（12 个） |
| Codex | AGENTS.md（项目根目录） | — | 不支持 |
| Cursor | Cursor IDE + AGENTS.md | `.cursor/rules/*.mdc` | 不支持 |
| Windsurf | Windsurf IDE + `.windsurfrules` | `.windsurf/workflows/*.md` | 不支持 |
| Trae IDE | Trae IDE + `.trae/rules/project_rules.md` | `.trae/skills/` | 不支持 |
| Hermes Agent | Hermes Agent + `.hermes/SOUL.md` | `.hermes/skills/` | 后台审视机制 |
| WorkBuddy | WorkBuddy + `.workbuddy/manifest.yaml` | `.workbuddy/mcp.json` | MCP 协议 |

### 各平台说明

#### Claude Code
- 所有 40 个 Agent 的完整运行环境
- 所有 74 个斜杠命令的执行入口
- 所有 12 个 Hook 的触发和执行
- 子 Agent 派发和协调
- 需要：Node.js 18+、Anthropic API 密钥

#### Codex
- 通过 AGENTS.md 章节描述 Agent 角色和技能
- 子目录 AGENTS.md 提供路径范围规则
- 不支持 Hook，以 AGENTS.md 文档指令替代

#### Cursor
- 通过 `.cursor/rules/*.mdc` 提供规则（3 Always + 11 Auto Attached + 40 Agent Requested）
- 兼容 Agent Skills 标准的技能目录
- 不支持 Hook，以 AGENTS.md 文档指令替代

#### Windsurf
- `.windsurfrules` 内联所有路径规则
- `.windsurf/workflows/` 工作流定义（74 技能 + 40 Agent）
- 不支持 Hook，以 AGENTS.md 文档指令替代

#### Trae IDE
- `.trae/rules/project_rules.md` 内联所有规则
- `.trae/agents/` Agent 定义文件
- 兼容 Agent Skills 标准的技能目录
- 不支持 Hook

#### Hermes Agent
- `.hermes/SOUL.md` 身份配置
- `.hermes/skills/` 技能定义（Hermes 扩展格式）
- `.hermes/skills.lock` 版本锁定
- 后台审视机制替代部分 Hook 功能

#### WorkBuddy
- `.workbuddy/manifest.yaml` 平台清单
- `.workbuddy/skills/` 技能定义（Agent Skills 兼容）
- `.workbuddy/mcp.json` MCP 协议配置
- 不支持 Hook，以 MCP 协议替代

## 7. 故障排除

### 常见问题

**问题**：Hook 脚本执行失败
- **原因**：Windows 平台未安装 Git Bash
- **解决**：安装 Git for Windows，确保 `bash` 在 PATH 中

**问题**：`validate-commit.sh` 误报提交信息不合法
- **原因**：jq 缺失，正则匹配不够精确
- **解决**：安装 jq，或手动确认提交信息格式

**问题**：Claude Code 安装失败
- **原因**：Node.js 版本过低
- **解决**：升级 Node.js 至 18 LTS 或更高版本

**问题**：JSON 文件验证跳过
- **原因**：jq 和 python3 均缺失
- **解决**：安装其中任一工具