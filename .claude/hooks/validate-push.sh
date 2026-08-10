#!/bin/bash
# ============================================================
# validate-push.sh
# PreToolUse(Bash) 触发：验证 git push 命令
# 检查目标分支是否是受保护分支（main / master / develop）
# 如果是受保护分支，输出警告到 stderr
# 始终 exit 0（仅警告，不阻断）
# ============================================================

set -euo pipefail

# 从 stdin 读取工具输入（Claude Code 传入的 JSON）
INPUT=$(cat)

# 提取命令字段
COMMAND=$(echo "$INPUT" | grep -Eo '"command"\s*:\s*"[^"]*"' | head -n 1 | sed -E 's/.*"command"\s*:\s*"([^"]*)".*/\1/')

# 如果无法从 JSON 提取，回退到第一个参数
if [ -z "$COMMAND" ]; then
  COMMAND="${1:-}"
fi

# 检查是否是 git push 命令
if ! echo "$COMMAND" | grep -E "git\s+push" >/dev/null 2>&1; then
  # 不是 git push，直接通过
  exit 0
fi

# 受保护分支列表
PROTECTED_BRANCHES="main|master|develop"

# 尝试从命令中提取目标分支
# 情况 1：git push origin <branch>
# 情况 2：git push（使用当前分支）
# 情况 3：git push origin HEAD:branch

TARGET_BRANCH=""

# 提取 git push 后的最后一个分支参数
PUSH_ARGS=$(echo "$COMMAND" | sed -E 's/.*git\s+push\s+(.*)/\1/')

# 尝试匹配 remote branch 形式
if echo "$PUSH_ARGS" | grep -E "HEAD:([a-zA-Z0-9/_-]+)" >/dev/null 2>&1; then
  TARGET_BRANCH=$(echo "$PUSH_ARGS" | sed -E 's/.*HEAD:([a-zA-Z0-9/_-]+).*/\1/')
elif echo "$PUSH_ARGS" | grep -E "^origin\s+([a-zA-Z0-9/_-]+)$" >/dev/null 2>&1; then
  TARGET_BRANCH=$(echo "$PUSH_ARGS" | sed -E 's/^origin\s+([a-zA-Z0-9/_-]+).*/\1/')
elif echo "$PUSH_ARGS" | grep -E "\s([a-zA-Z0-9/_-]+)$" >/dev/null 2>&1; then
  TARGET_BRANCH=$(echo "$PUSH_ARGS" | sed -E 's/.*\s([a-zA-Z0-9/_-]+)$/\1/')
fi

# 如果未从命令提取到分支，使用当前分支
if [ -z "$TARGET_BRANCH" ]; then
  TARGET_BRANCH=$(git branch --show-current 2>/dev/null || echo "")
fi

# 检查是否是受保护分支
IS_PROTECTED=false
for protected in $PROTECTED_BRANCHES; do
  if [ "$TARGET_BRANCH" = "$protected" ]; then
    IS_PROTECTED=true
    break
  fi
done

# 如果是受保护分支，输出警告
if [ "$IS_PROTECTED" = true ]; then
  echo "========================================" >&2
  echo "  [警告] 受保护分支推送提醒" >&2
  echo "========================================" >&2
  echo "  目标分支：$TARGET_BRANCH" >&2
  echo "  该分支为受保护分支，请确认：" >&2
  echo "    1. 是否已通过代码评审（Code Review）？" >&2
  echo "    2. 是否已通过 CI/CD 管线检查？" >&2
  echo "    3. 是否遵循分支保护策略（如需要 PR 合并）？" >&2
  echo "  建议通过 Pull Request 合并到 $TARGET_BRANCH。" >&2
  echo "========================================" >&2
fi

# 始终通过（仅警告，不阻断）
exit 0