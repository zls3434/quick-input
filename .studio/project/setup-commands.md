<!-- Software Engineering Studios -->
# 环境搭建命令

本文件列出 Software Engineering Studios 在各平台上的环境搭建命令。所有命令均为平台无关的规范源，各平台配置文件可引用本文件。

## 1. 必需环境

### 1.1 Git 2.x+

版本控制基础环境，所有 Hook 和工作流的依赖。

**验证命令：**

```bash
git --version
```

**安装命令：**

| 平台 | 命令 |
|------|------|
| macOS | `brew install git` |
| Linux（Debian/Ubuntu） | `sudo apt-get install git` |
| Windows | 从 https://git-scm.com/download/win 下载安装 |

### 1.2 Claude Code

Agent 运行时环境，提供 Agent 调度、工具使用和 Hook 执行能力。

**安装命令：**

```bash
npm install -g @anthropic-ai/claude-code
```

**验证命令：**

```bash
claude --version
```

**初始化：**

```bash
claude
```

> 需要有效的 Anthropic API 密钥。首次使用需运行 `claude` 命令完成初始化。

## 2. 推荐工具

以下工具非必需，但能增强系统功能。缺失时 Hook 将优雅降级。

### 2.1 jq

JSON 解析和验证工具，多个 Hook 依赖它解析 JSON 配置。

**验证命令：**

```bash
jq --version
```

**安装命令：**

| 平台 | 命令 |
|------|------|
| macOS | `brew install jq` |
| Linux（Debian/Ubuntu） | `sudo apt-get install jq` |
| Windows | `choco install jq` 或从 https://jqlang.github.io/jq/download/ 下载 |

### 2.2 Python 3

JSON 验证、数据处理脚本的基础环境。

**最低版本**：Python 3.6

**验证命令：**

```bash
python3 --version
```

**安装命令：**

| 平台 | 命令 |
|------|------|
| macOS | `brew install python3` |
| Linux（Debian/Ubuntu） | `sudo apt-get install python3` |
| Windows | 从 https://www.python.org/downloads/ 下载安装 |

### 2.3 Node.js

npm 包管理、前端构建工具、Node.js 后端运行时。Claude Code 依赖 npm 安装。

**最低版本**：Node.js 18 LTS

**验证命令：**

```bash
node --version
```

**安装命令：**

| 平台 | 命令 |
|------|------|
| macOS | `brew install node` |
| Linux（Debian/Ubuntu） | 通过 NodeSource 安装 |
| Windows | 从 https://nodejs.org/ 下载安装 |

### 2.4 Git Bash（Windows 平台）

Windows 平台上执行 Shell 脚本的兼容环境。所有 Hook 脚本使用 Shell 语法编写。

**验证命令：**

```bash
bash --version
```

**安装方式：**

安装 Git for Windows 时自动包含 Git Bash。

下载地址：https://git-scm.com/download/win

> 确保 Git Bash 的 `bash` 命令在系统 PATH 中，或在配置文件中指定 Shell 路径。

## 3. 各平台安装命令

Software Engineering Studios 的 Agent 架构可适配多种 AI 编码平台。以下是各平台的安装命令。

### 3.1 Codex CLI

OpenAI 的命令行编码助手。

```bash
npm install -g @openai/codex
```

**验证：**

```bash
codex --version
```

### 3.2 Cursor

AI 驱动的代码编辑器。

- 从 https://cursor.sh/ 下载安装
- 安装后打开 Cursor，进入设置配置 API 密钥

### 3.3 Windsurf

Codeium 的 AI IDE。

- 从 https://windsurf.com/ 下载安装
- 安装后打开 Windsurf，完成账号登录

### 3.4 Trae IDE

字节跳动的 AI IDE。

- 从 https://www.trae.cn/ 下载安装
- 安装后打开 Trae，完成账号登录

### 3.5 Hermes Agent

AI Agent 框架。

```bash
# 具体安装方式取决于 Hermes Agent 的发布渠道
# 参考官方文档获取最新安装命令
```

### 3.6 WorkBuddy

AI 工作助手。

```bash
# 具体安装方式取决于 WorkBuddy 的发布渠道
# 参考官方文档获取最新安装命令
```

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
|------|----------|----------|
| macOS | 完全支持 | 所有工具均可通过 Homebrew 安装 |
| Linux | 完全支持 | 所有工具均可通过包管理器安装 |
| Windows | 支持 | 需安装 Git Bash 执行 Shell 脚本 |