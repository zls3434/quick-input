# 贡献指南

感谢你对 Software Engineering Studios 项目的兴趣！以下是如何 contributing 的指南。

## 如何贡献

### 报告问题

- 使用 [GitHub Issues](../../issues) 提交 Bug 报告或功能请求
- 提交前请先搜索是否已有类似的 issue

### 提交代码

1. Fork 本仓库
2. 创建特性分支（`git checkout -b feature/amazing-feature`）
3. 提交更改（`git commit -m 'feat: 添加了某功能'`）
   - 遵循 Conventional Commits 格式（`feat:`、`fix:`、`docs:`、`refactor:`、`test:`、`chore:`）
4. 推送到分支（`git push origin feature/amazing-feature`）
5. 创建 Pull Request

### 添加新 Agent

1. 在 `.studio/agents/` 创建新的 `.md` 文件（规范源）
2. 包含完整的 YAML frontmatter（name、description、tools、model）
3. 编写详细的 markdown body（协作协议、职责、委托地图）
4. 更新 `.studio/docs/agent-roster.md`（如果存在）
5. 运行 `bash tools/adapters/sync-all.sh` 同步到各平台

### 添加新技能

1. 在 `.studio/skills/` 下创建新目录和 `SKILL.md`（规范源）
2. 包含完整的 YAML frontmatter（name、description、allowed-tools、model）
3. 编写详细的工作流说明
4. 运行 `/skill-test` 验证合规性
5. 运行 `bash tools/adapters/sync-all.sh` 同步到各平台

### 添加新规则

1. 在 `.studio/rules/` 创建新的 `.md` 文件（规范源）
2. 包含 YAML frontmatter 指定 `paths` 范围
3. 编写清晰的规则说明和代码示例
4. 运行 `bash tools/adapters/sync-all.sh` 同步到各平台

### 添加新模板

1. 在 `.studio/templates/` 创建新的 `.md` 文件（规范源）
2. 包含完整的元数据（状态、作者、更新日期）
3. 编写模板内容，使用占位符标注需填充部分
4. 运行 `bash tools/adapters/sync-all.sh` 同步到各平台

## 代码规范

- 所有文档使用简体中文
- 代码注释使用简体中文
- 遵循 `.studio/docs/coding-standards.md` 中的规范
- 公共 API 必须有文档注释

## 多平台开发指南

### 修改规则

所有配置修改必须在 `.studio/` 规范源中进行，**不要直接修改各平台输出目录**（`.claude/`、`.cursor/`、`.windsurf/` 等）。

### 修改流程

1. 修改 `.studio/` 中的源文件
2. 运行 `bash tools/adapters/sync-all.sh` 同步到各平台
3. 运行 `/platform-check` 验证一致性
4. 提交变更（包含源文件和生成的平台配置）

### 目录结构

| 目录 | 说明 | 可否直接修改 |
|---|---|---|
| `.studio/` | 规范源层 | ✅ 在此修改 |
| `tools/adapters/` | 同步脚本 | ✅ 可修改脚本 |
| `.claude/` | Claude Code 输出 | ❌ 自动生成 |
| `.cursor/` | Cursor 输出 | ❌ 自动生成 |
| `.windsurf/` | Windsurf 输出 | ❌ 自动生成 |
| `.trae/` | Trae 输出 | ❌ 自动生成 |
| `.hermes/` | Hermes 输出 | ❌ 自动生成 |
| `.workbuddy/` | WorkBuddy 输出 | ❌ 自动生成 |

## 许可证

提交的代码将受 MIT 许可证保护。