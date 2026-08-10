<!-- Software Engineering Studios -->
# 迁移指南：从 .claude/ 到 .studio/

本指南说明如何从仅支持 Claude Code 的架构迁移到多平台适配架构。

## 迁移概述

迁移将 `.claude/` 作为唯一配置源的模式，改为以 `.studio/` 为规范源、`.claude/` 为输出层的模式。

## 迁移步骤

### 步骤 1：确认规范源已创建

检查 `.studio/` 目录结构完整：
- `.studio/agents/` — 40 个 Agent 定义
- `.studio/skills/` — 72 个技能定义
- `.studio/rules/` — 11 个路径规则
- `.studio/hooks/` — 12 个 Hook 脚本
- `.studio/docs/` — 规范文档
- `.studio/templates/` — 文档模板
- `.studio/project/` — 项目概述文档
- `.studio/manifest.yaml` — 项目清单

### 步骤 2：确认各平台配置已生成

- `AGENTS.md` — 根目录跨平台主配置
- `.cursor/` — Cursor 配置
- `.windsurfrules` + `.windsurf/` — Windsurf 配置
- `.trae/` — Trae IDE 配置
- `.hermes/` — Hermes Agent 配置
- `.workbuddy/` — WorkBuddy 配置

### 步骤 3：验证 Claude Code 兼容性

确认 `.claude/` 目录内容与规范源一致：
```bash
# 检查 Agent 数量
ls .claude/agents/*.md | wc -l  # 应为 40

# 检查技能数量
ls -d .claude/skills/*/ | wc -l  # 应为 72

# 检查规则数量
ls .claude/rules/*.md | wc -l  # 应为 11
```

### 步骤 4：更新工作流

今后所有配置修改在 `.studio/` 中进行，修改后运行同步：
```bash
bash tools/adapters/sync-all.sh
```

## 回滚方案

如需回滚到仅 Claude Code 模式：
1. `.claude/` 目录保持不变，仍可正常工作
2. 删除 `.studio/`、`AGENTS.md`、各平台配置目录
3. 恢复原始的 CLAUDE.md（移除多平台适配章节）