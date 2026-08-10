#!/usr/bin/env bash
# 同步 Cursor 配置
# 从 .studio/ 生成 .cursor/ 配置

set -euo pipefail

PROJECT_ROOT="${1:-$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)}"
STUDIO="$PROJECT_ROOT/.studio"
CURSOR="$PROJECT_ROOT/.cursor"

echo "同步 Cursor 配置..."

# 复制技能（Agent Skills 兼容格式，直接复制）
for skill_dir in "$STUDIO/skills"/*/; do
    skill_name=$(basename "$skill_dir")
    if [ -f "$skill_dir/SKILL.md" ]; then
        mkdir -p "$CURSOR/skills/$skill_name"
        cp "$skill_dir/SKILL.md" "$CURSOR/skills/$skill_name/SKILL.md"
    fi
done

echo "Cursor 配置同步完成（规则和Agent定义需要通过 PowerShell 脚本转换）"
echo "请运行: powershell -File gen-cursor.ps1"