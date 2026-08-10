<!-- Software Engineering Studios -->
# 平台适配脚本

本目录包含将 `.studio/` 规范源同步到各平台配置的脚本。

## 使用方法

### 同步所有平台
```bash
bash tools/adapters/sync-all.sh
```

### 同步单个平台
```bash
bash tools/adapters/sync-claude-code.sh
bash tools/adapters/sync-codex.sh
bash tools/adapters/sync-cursor.sh
bash tools/adapters/sync-windsurf.sh
bash tools/adapters/sync-trae.sh
bash tools/adapters/sync-hermes.sh
bash tools/adapters/sync-workbuddy.sh
```

## 平台对照

| 脚本 | 目标平台 | 输出路径 |
|---|---|---|
| sync-claude-code.sh | Claude Code | CLAUDE.md, .claude/ |
| sync-codex.sh | Codex | AGENTS.md, 子目录AGENTS.md |
| sync-cursor.sh | Cursor | .cursor/, AGENTS.md |
| sync-windsurf.sh | Windsurf | .windsurfrules, .windsurf/ |
| sync-trae.sh | Trae IDE | .trae/ |
| sync-hermes.sh | Hermes Agent | .hermes/ |
| sync-workbuddy.sh | WorkBuddy | .workbuddy/ |

## 架构

脚本从 `.studio/`（规范源层）读取内容，转换为各平台格式后写入输出目录。修改项目配置时，始终修改 `.studio/` 中的源文件，然后运行同步脚本。

## 依赖

- PowerShell 5.1+（Windows）或 Bash（macOS/Linux）
- Python 3（可选，用于 YAML 验证）