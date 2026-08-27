# 贡献指南

感谢你对 QuickInput 的兴趣！本指南覆盖应用本体的开发流程；仓库内附带的 AI Agent 工程框架（`.studio/`）贡献方式见文末。

## 环境准备

- Node.js 18+
- Rust stable 工具链（`rustup` 安装）
- Windows 10/11（自带 WebView2 运行时）
- GitHub CLI（`gh`，发布流程需要）

```bash
git clone https://github.com/zls3434/quick-input.git
cd quick-input/src-tauri-app
npm install
```

## 开发与测试

```bash
# 开发模式（热重载）
cd src-tauri-app && npm run tauri:dev

# 配置库单元测试（模型校验、管理器、进程匹配）
cd src/backend && cargo test

# Tauri 应用 Rust 单元测试
cd src-tauri-app/src-tauri && cargo test

# 生产构建（安装包 + 绿色版，构建后自动重启根目录 QuickInput.exe 便于实测）
cd src-tauri-app && npm run tauri:build
```

## 提交代码

1. Fork 本仓库并创建特性分支（`git checkout -b feature/amazing-feature`）
2. 提交更改，遵循 [Conventional Commits](https://www.conventionalcommits.org/zh-hans/)：
   - `feat:` 新功能　`fix:` 缺陷修复　`docs:` 文档　`refactor:` 重构　`test:` 测试　`chore:` 构建/工具
   - Subject 不超过 50 字
3. 推送分支并创建 Pull Request

### 版本号规则（提交前必读）

语义化版本 `主版本.次版本.修订号`，三处同步维护、缺一不可：

- `src-tauri-app/package.json`
- `src-tauri-app/src-tauri/tauri.conf.json`
- `src-tauri-app/src-tauri/Cargo.toml`（`Cargo.lock` 构建时自动同步）

递增规则：

| 改动类型 | 版本变化 |
|---|---|
| 常规改动（修复、小调整、UI 微调） | 修订号 +1 |
| 成体系新功能（新注入模式、新配置页等） | 次版本号 +1，修订号归零 |
| 主版本号 | 仅维护者确认后递增 |

版本号变更纳入功能提交本身，不单独发版本号提交。

### 代码规范

- 前端组件 PascalCase，文件 kebab-case；Rust 按标准惯例
- 禁止硬编码配置、禁止 `any` 类型、禁止空 catch 块、禁止生产代码 `console.log`
- 代码注释与文档使用简体中文
- 测试遵循 AAA 模式（Arrange-Act-Assert），独立、确定、无外部依赖

## 发布流程（维护者）

完整步骤见 `.trae/rules/project_rules.md` 的「发布流程」章节，概要：

1. 三处版本号同步
2. `npm run tauri:build` 构建
3. 提交推送
4. `gh release create` 上传安装包 + 绿色版 `portable.exe`
5. `npx tauri signer sign -f <私钥路径>` 签名生成 `.sig`
6. `node scripts/make-update-json.mjs` 生成 `latest.json`
7. `gh release upload` 上传 `.sig` 与 `latest.json`
8. 旧版本实测「检查更新」链路

注意：

- 签名私钥存于 `%USERPROFILE%\.quickinput\`，**严禁提交到版本控制**
- 更新分发走镜像多端点（`tauri.conf.json` 的 `endpoints` 与 `make-update-json.mjs` 的 `DOWNLOAD_MIRROR` 两处联动），换镜像域名时两处同步改

## 报告问题

使用 [GitHub Issues](https://github.com/zls3434/quick-input/issues) 提交 Bug 或功能请求，提交前请先搜索是否已有类似 issue。 Bug 报告请附：系统版本、QuickInput 版本（关于页可查）、复现步骤、预期与实际行为。

## Agent 工程框架（.studio/）

本仓库附带多平台 AI Agent 配置（40 Agent / 74 技能 / 11 路径规则），来自独立的开源框架项目 [Software Engineering Studios](https://github.com/zls3434/Software-Engineering-Studios)：将一个 AI 编码会话转变为一间完整的软件工程开发工作室，通过 40 个协调的子 Agent 团队为 AI 辅助开发赋予真实开发团队的结构、流程和质量关卡，并支持多平台适配（Claude Code、Cursor、Trae、Windsurf 等）。

本仓库中规范源在 `.studio/`，各平台目录（`.claude/`、`.cursor/`、`.trae/` 等）为生成产物：

- 所有框架资产修改在 `.studio/` 进行，不要直接改平台输出目录
- 修改后运行 `bash tools/adapters/sync-all.sh` 同步各平台
- 新增 Agent / 技能 / 规则 / 模板均先落 `.studio/` 对应目录，再同步
- 框架本身的贡献、完整文档与版本发布见[框架仓库](https://github.com/zls3434/Software-Engineering-Studios)

## 许可证

提交的代码将受 [MIT 许可证](LICENSE)保护。
