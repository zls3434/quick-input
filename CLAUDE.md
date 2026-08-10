# Software Engineering Studios —— 软件工程工作室 Agent 架构

**语言要求：必须始终使用简体中文与用户对话，生成的文档与代码注释也必须使用简体中文编写。**

通过 40 个协调的 Claude Code 子 Agent 管理软件工程项目开发。
每个 Agent 负责一个特定领域，确保关注点分离和质量把控。包含 74 个技能、12 个 Hook、11 个路径规则和 35 个文档模板。

## 技术栈

- **前端框架**：[选择：React / Vue / Angular]
- **后端语言**：[选择：Node.js / Python / Java / Go]
- **数据库**：[选择：PostgreSQL / MongoDB / Redis]
- **版本控制**：Git，采用基于主干的开发模式
- **构建系统**：[选择技术栈后指定]
- **部署平台**：[选择：AWS / Azure / GCP / 自托管]

> **注意**：存在针对 React、Vue、Angular、Node.js、Python、Java、Go 的技术栈专家 Agent。请使用与您项目匹配的集合。

## 项目结构

@.claude/docs/directory-structure.md

## 技术偏好

@.claude/docs/technical-preferences.md

## 协调规则

@.claude/docs/coordination-rules.md

## 协作协议

**用户驱动的协作，而非自主执行。**
每个任务都遵循：**提问 → 选项 → 决策 → 草稿 → 审批**

- Agent 在使用 Write/Edit 工具前必须询问："我可以将此写入 [文件路径] 吗？"
- Agent 在请求审批前必须展示草稿或摘要
- 多文件修改需要针对完整变更集的明确审批
- 未经用户指示不得进行提交

完整协议和示例请参见 `docs/examples/COLLABORATIVE-DESIGN-PRINCIPLE.md`。

> **首次使用？** 如果项目尚未配置技术栈且没有产品概念，
> 请运行 `/start` 开始引导式上手流程。

## 编码规范

@.claude/docs/coding-standards.md

## 上下文管理

@.claude/docs/context-management.md

## 多平台适配

本项目支持多个 AI Agent 平台，采用三层架构：

1. **规范源层**（`.studio/`）：平台无关的单一真相源，所有修改在此进行
2. **适配层**（`tools/adapters/`）：同步脚本，将规范源转换为各平台格式
3. **输出层**：各平台原生配置目录（`.claude/`、`.cursor/`、`.windsurf/` 等）

### 支持的平台

| 平台 | 配置目录 | 主配置文件 |
|---|---|---|
| Claude Code | `.claude/` | `CLAUDE.md` |
| Codex | - | `AGENTS.md` |
| Cursor | `.cursor/` | `AGENTS.md` + `.cursor/rules/` |
| Windsurf | `.windsurf/` | `.windsurfrules` |
| Trae IDE | `.trae/` | `.trae/rules/project_rules.md` |
| Hermes Agent | `.hermes/` | `.hermes/SOUL.md` |
| WorkBuddy | `.workbuddy/` | `AGENTS.md` |

### 同步操作

修改 `.studio/` 中的源文件后，运行同步脚本更新各平台配置：

```bash
bash tools/adapters/sync-all.sh
```

或运行 `/sync-platforms` 技能。

详见 `docs/platform-adaptation-guide.md`。