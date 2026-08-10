#!/bin/bash
# ============================================================
# validate-skill-change.sh
# PostToolUse(Write|Edit) 触发：检测技能文件变更
# 检查文件路径是否在 .claude/skills/ 目录下
# 如果是，输出提示建议运行 /skill-test 验证合规性
# 始终 exit 0（仅提示，不阻断）
# ============================================================

set -euo pipefail

# 从 stdin 读取工具输入（Claude Code 传入的 JSON）
INPUT=$(cat)

# 提取文件路径字段
FILE_PATH=$(echo "$INPUT" | grep -Eo '"file_path"\s*:\s*"[^"]*"' | head -n 1 | sed -E 's/.*"file_path"\s*:\s*"([^"]*)".*/\1/')

# 如果无法从 JSON 提取，回退到第一个参数
if [ -z "$FILE_PATH" ]; then
  FILE_PATH="${1:-}"
fi

# 无文件路径则通过
if [ -z "$FILE_PATH" ]; then
  exit 0
fi

# 检查文件路径是否在 .claude/skills/ 目录下
if echo "$FILE_PATH" | grep -E "(^|/)\.claude/skills/" >/dev/null 2>&1; then
  echo "========================================" >&2
  echo "  [提示] 检测到技能文件变更" >&2
  echo "========================================" >&2
  echo "  变更文件：$FILE_PATH" >&2
  echo "  检测到技能文件变更，建议运行 /skill-test 验证合规性。" >&2
  echo "========================================" >&2
fi

# 始终通过
exit 0