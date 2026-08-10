#!/bin/bash
# ============================================================
# validate-asset-change.sh
# PostToolUse(Write|Edit) 触发：检测资产文件变更
# 检查文件路径是否在 .studio/agents/、.studio/skills/、.studio/rules/ 目录下
# 根据变更类型输出对应提示建议
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

# 检查文件路径是否在 .studio/agents/ 或 .claude/agents/ 目录下
if echo "$FILE_PATH" | grep -E "(^|/)\.(studio|claude)/agents/" >/dev/null 2>&1; then
  echo "========================================" >&2
  echo "  [提示] 检测到 Agent 文件变更" >&2
  echo "========================================" >&2
  echo "  变更文件：$FILE_PATH" >&2
  echo "  检测到 Agent 文件变更，建议：" >&2
  echo "  1. 检查 _roster.md 是否已更新" >&2
  echo "  2. 运行 /asset-review 验证合规性" >&2
  echo "  3. 检查 asset-registry.yaml 是否已更新" >&2
  echo "========================================" >&2
fi

# 检查文件路径是否在 .studio/skills/ 或 .claude/skills/ 目录下
if echo "$FILE_PATH" | grep -E "(^|/)\.(studio|claude)/skills/" >/dev/null 2>&1; then
  echo "========================================" >&2
  echo "  [提示] 检测到技能文件变更" >&2
  echo "========================================" >&2
  echo "  变更文件：$FILE_PATH" >&2
  echo "  检测到技能文件变更，建议：" >&2
  echo "  1. 运行 /skill-test 验证合规性" >&2
  echo "  2. 检查 _catalog.yaml 是否已更新" >&2
  echo "  3. 如为新增技能，检查 asset-registry.yaml 是否已更新" >&2
  echo "========================================" >&2
fi

# 检查文件路径是否在 .studio/rules/ 或 .claude/rules/ 目录下
if echo "$FILE_PATH" | grep -E "(^|/)\.(studio|claude)/rules/" >/dev/null 2>&1; then
  echo "========================================" >&2
  echo "  [提示] 检测到规则文件变更" >&2
  echo "========================================" >&2
  echo "  变更文件：$FILE_PATH" >&2
  echo "  检测到规则文件变更，建议：" >&2
  echo "  1. 检查 rules-reference.md 是否已更新" >&2
  echo "  2. 如为新增规则，检查 asset-registry.yaml 是否已更新" >&2
  echo "========================================" >&2
fi

# registry 目录变更静默通过（注册表更新不需要提示）

# 始终通过
exit 0