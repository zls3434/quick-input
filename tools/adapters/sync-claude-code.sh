#!/usr/bin/env bash
# 同步 Claude Code 配置
# 从 .studio/ 生成 .claude/ 配置

set -euo pipefail

PROJECT_ROOT="${1:-$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)}"
STUDIO="$PROJECT_ROOT/.studio"
CLAUDE="$PROJECT_ROOT/.claude"

echo "同步 Claude Code 配置..."

# 复制 agents
cp -r "$STUDIO/agents/"*.md "$CLAUDE/agents/" 2>/dev/null || true

# 复制 rules
cp -r "$STUDIO/rules/"*.md "$CLAUDE/rules/" 2>/dev/null || true

# 复制 hooks
cp "$STUDIO/hooks/"*.sh "$CLAUDE/hooks/" 2>/dev/null || true

# 复制 docs
cp "$STUDIO/docs/"*.md "$CLAUDE/docs/" 2>/dev/null || true

# 复制 templates
cp -r "$STUDIO/templates/"* "$CLAUDE/docs/templates/" 2>/dev/null || true

# 技能需要格式转换（metadata 字段提升为顶层）
# 此处简化处理，实际转换需要解析 YAML
for skill_dir in "$STUDIO/skills"/*/; do
    skill_name=$(basename "$skill_dir")
    if [ -f "$skill_dir/SKILL.md" ]; then
        mkdir -p "$CLAUDE/skills/$skill_name"
        cp "$skill_dir/SKILL.md" "$CLAUDE/skills/$skill_name/SKILL.md"
    fi
done

echo "Claude Code 配置同步完成"