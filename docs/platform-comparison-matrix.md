<!-- Software Engineering Studios -->
# 平台功能对照矩阵

| 功能 | Claude Code | Codex | Cursor | Windsurf | Trae IDE | Trae Work | Hermes | WorkBuddy |
|---|---|---|---|---|---|---|---|---|
| 主配置文件 | CLAUDE.md | AGENTS.md | AGENTS.md + .mdc | AGENTS.md + .windsurfrules | AGENTS.md + project_rules.md | 云端 | AGENTS.md + SOUL.md | AGENTS.md |
| 目录继承 | 部分（@引用） | 支持 | 支持（globs） | 不支持 | 不支持 | 云端 | 不支持 | 不支持 |
| 独立 Agent 定义 | 支持（40个文件） | 不支持 | 支持（Agent Requested） | 不支持（工作流） | 支持（@Agent） | 云端 | 不支持（SOUL.md） | 不支持 |
| 技能系统 | SKILL.md | Agent Skills 兼容 | Agent Skills 兼容 | 工作流 .md | Agent Skills 兼容 | 云端技能市场 | Hermes 扩展 | Agent Skills 兼容 |
| 路径范围规则 | YAML paths | 子目录 AGENTS.md | globs 匹配 | 内联 | 内联 | 云端 | AGENTS.md 指令 | AGENTS.md 指令 |
| Hook 系统 | 支持（12个） | 不支持 | 不支持 | 不支持 | 不支持 | 不支持 | 后台审视 | MCP 协议 |
| 权限控制 | settings.json | 不支持 | 部分 | 不支持 | 不支持 | 云端 | 不支持 | 云端 |
| 模型层级分配 | YAML model | 不支持 | 不支持 | 不支持 | 不支持 | 云端 | 不支持 | 不支持 |
| 审计日志 | Hook 支持 | 不支持 | 不支持 | 不支持 | 不支持 | 云端 | 内建记忆 | 云端 |

## 功能降级说明

以下功能在非 Claude Code 平台上以降级形式提供：

| 功能 | 降级方式 |
|---|---|
| Hook 自动验证 | AGENTS.md 文档指令 |
| 模型层级分配 | 所有任务使用默认模型 |
| 权限控制 | 依赖平台原生安全机制 |
| 子 Agent 派发 | 通过技能工作流模拟 |
| 审计日志 | 依赖平台原生日志或手动记录 |