#!/bin/bash
# ============================================================
# validate-assets.sh
# PostToolUse(Write|Edit) 触发：验证文档与资源文件
# 检查项：
#   1. 文件是否在 docs/ 或 design/ 目录
#   2. JSON 文件有效性
#   3. YAML frontmatter 格式（.claude/ 下的 .md 文件）
#   4. 命名规范（小写、连字符分隔）
# exit 0 通过，exit 2 输出警告
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

# 1. 检查文件是否在 docs/ 或 design/ 目录
if ! echo "$FILE_PATH" | grep -E "(^|/)(docs|design)/" >/dev/null 2>&1; then
  # 不在文档目录，通过
  exit 0
fi

# 文件不存在则跳过
if [ ! -f "$FILE_PATH" ]; then
  exit 0
fi

WARNINGS=""

# 2. 检查 JSON 文件有效性
if echo "$FILE_PATH" | grep -E "\.json$" >/dev/null 2>&1; then
  if ! python -m json.tool "$FILE_PATH" >/dev/null 2>&1; then
    WARNINGS="${WARNINGS}\n[JSON 无效] $FILE_PATH 不是有效的 JSON 文件。"
  fi
fi

# 3. 检查 YAML frontmatter 格式（.claude/ 下的 .md 文件）
if echo "$FILE_PATH" | grep -E "\.claude/.*\.md$" >/dev/null 2>&1; then
  # 检查是否以 --- 开头的 frontmatter
  FIRST_LINE=$(head -n 1 "$FILE_PATH" 2>/dev/null || echo "")
  if [ "$FIRST_LINE" = "---" ]; then
    # 查找第二个 --- 结束 frontmatter
    SECOND_DELIM=$(head -n 20 "$FILE_PATH" | grep -n "^---$" | head -n 2 | tail -n 1 | cut -d: -f1)
    if [ -z "$SECOND_DELIM" ]; then
      WARNINGS="${WARNINGS}\n[Frontmatter 格式] $FILE_PATH 的 YAML frontmatter 未正确闭合（缺少第二个 ---）。"
    else
      # 检查 paths 字段是否存在
      FRONTMATTER=$(head -n "$((SECOND_DELIM - 1))" "$FILE_PATH" | tail -n "+2")
      if ! echo "$FRONTMATTER" | grep -E "^\s*paths\s*:" >/dev/null 2>&1; then
        WARNINGS="${WARNINGS}\n[Frontmatter 缺失] $FILE_PATH 的 YAML frontmatter 缺少 paths 字段。"
      fi
    fi
  else
    WARNINGS="${WARNINGS}\n[Frontmatter 缺失] $FILE_PATH 缺少 YAML frontmatter（应以 --- 开头）。"
  fi
fi

# 4. 检查命名规范（小写、连字符分隔）
BASENAME=$(basename "$FILE_PATH")
# 文档文件应使用小写和连字符，不使用下划线或空格
if echo "$BASENAME" | grep -E "[A-Z]" >/dev/null 2>&1; then
  # 跳过 README 等约定俗成的大写文件名
  if ! echo "$BASENAME" | grep -E "^(README|CHANGELOG|LICENSE|CONTRIBUTING)" >/dev/null 2>&1; then
    WARNINGS="${WARNINGS}\n[命名规范] $FILE_PATH 文件名包含大写字母，建议使用小写+连字符。"
  fi
fi

# 输出结果
if [ -n "$WARNINGS" ]; then
  echo "========================================" >&2
  echo "  文档验证警告：" >&2
  echo "========================================" >&2
  echo -e "$WARNINGS" >&2
  echo "" >&2
  echo "建议修复以上问题后重新保存。" >&2
  exit 2
fi

exit 0