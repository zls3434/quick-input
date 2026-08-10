<!-- Software Engineering Studios -->
# Hook 平台兼容性清单

本文件列出所有 Hook 脚本的平台兼容性和降级策略。

## 平台兼容性总览

| Hook | Claude Code | Codex | Cursor | Windsurf | Trae | Hermes | WorkBuddy |
|---|---|---|---|---|---|---|---|
| session-start.sh | ✅ 原生 | 📄 文档指令 | 📄 Always规则 | 📄 .windsurfrules | 📄 project_rules | 📄 SOUL.md | 📄 AGENTS.md |
| detect-gaps.sh | ✅ 原生 | 📄 文档指令 | 📄 Always规则 | 📄 .windsurfrules | 📄 project_rules | 📄 SOUL.md | 📄 AGENTS.md |
| validate-commit.sh | ✅ 原生 | 📄 文档指令 | 📄 Always规则 | 📄 .windsurfrules | 📄 project_rules | 📄 AGENTS.md | 📄 AGENTS.md |
| validate-push.sh | ✅ 原生 | 📄 文档指令 | 📄 Always规则 | 📄 .windsurfrules | 📄 project_rules | 📄 AGENTS.md | 📄 AGENTS.md |
| validate-assets.sh | ✅ 原生 | 📄 文档指令 | 📄 Always规则 | 📄 .windsurfrules | 📄 project_rules | 📄 AGENTS.md | 📄 AGENTS.md |
| validate-skill-change.sh | ✅ 原生 | 📄 文档指令 | 📄 Always规则 | 📄 .windsurfrules | 📄 project_rules | 📄 AGENTS.md | 📄 AGENTS.md |
| validate-asset-change.sh | ✅ 原生 | 📄 文档指令 | 📄 Always规则 | 📄 .windsurfrules | 📄 project_rules | 📄 AGENTS.md | 📄 AGENTS.md |
| notify.sh | ✅ 原生 | N/A | N/A | N/A | N/A | N/A | N/A |
| pre-compact.sh | ✅ 原生 | 📄 文档指令 | 📄 Always规则 | 📄 .windsurfrules | 📄 project_rules | 📄 SOUL.md | 📄 AGENTS.md |
| post-compact.sh | ✅ 原生 | 📄 文档指令 | 📄 Always规则 | 📄 .windsurfrules | 📄 project_rules | 📄 SOUL.md | 📄 AGENTS.md |
| session-stop.sh | ✅ 原生 | 📄 文档指令 | 📄 Always规则 | 📄 .windsurfrules | 📄 project_rules | 📄 AGENTS.md | 📄 AGENTS.md |
| log-agent.sh | ✅ 原生 | N/A | N/A | N/A | N/A | ✅ 内建 | N/A |
| log-agent-stop.sh | ✅ 原生 | N/A | N/A | N/A | N/A | ✅ 内建 | N/A |

图例：✅ 原生支持 | 📄 降级为文档指令 | N/A 不适用

## 降级策略

### Codex / Cursor / Windsurf / Trae / WorkBuddy
Hook 降级为 AGENTS.md 或平台规则文件中的明确指令。例如：
- validate-commit.sh → "提交信息必须遵循 Conventional Commits 格式（feat/fix/docs/refactor/test/chore）"
- validate-push.sh → "禁止向 main/master/develop 分支直接推送（阻断）；feature/fix/hotfix 分支推送前建议完成代码审查（警告）"
- session-start.sh → "会话开始时先读取 production/session-state/active.md 了解当前状态"
- pre-compact.sh → "上下文压缩前将当前进度写入 production/session-state/active.md"
- post-compact.sh → "上下文压缩后从 active.md 恢复状态"

### Hermes Agent
Hermes 有内建的后台审视机制（background_review）和记忆系统，部分 Hook 功能由 Hermes 原生提供：
- log-agent.sh / log-agent-stop.sh → Hermes 内建审计和记忆
- 其他 Hook 降级为 SOUL.md 中的指令