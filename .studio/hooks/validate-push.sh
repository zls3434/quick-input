#!/bin/bash
# ============================================================
# validate-push.sh
# PreToolUse(Bash) 触发：验证 git push 命令
# 检查项：
#   1. 受保护分支（main/master/develop）直接推送 → 阻断（exit 2）
#   2. feature/fix 分支推送时检查审查报告是否存在 → 警告（exit 0）
#   3. 分支名规范检查 → 警告（exit 0）
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

# 受保护分支列表（直接推送会被阻断）
PROTECTED_BRANCHES="main master develop"

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

# ============================================================
# 检查 1：受保护分支直接推送 → 阻断
# ============================================================
IS_PROTECTED=false
for protected in $PROTECTED_BRANCHES; do
  if [ "$TARGET_BRANCH" = "$protected" ]; then
    IS_PROTECTED=true
    break
  fi
done

if [ "$IS_PROTECTED" = true ]; then
  echo "========================================" >&2
  echo "  [阻断] 受保护分支禁止直接推送" >&2
  echo "========================================" >&2
  echo "  目标分支：$TARGET_BRANCH" >&2
  echo "  该分支为受保护分支，禁止直接推送。" >&2
  echo "" >&2
  echo "  请通过以下方式合并代码：" >&2
  echo "    1. 在功能分支上完成开发" >&2
  echo "    2. 运行 /code-review 完成代码审查" >&2
  echo "    3. 通过 Pull Request 合并到 $TARGET_BRANCH" >&2
  echo "" >&2
  echo "  相关规范：" >&2
  echo "    - docs/code-management-workflow.md 第 4 节" >&2
  echo "    - .studio/project/pr-guidelines.md 第 3.4 节" >&2
  echo "========================================" >&2
  exit 2
fi

# ============================================================
# 检查 2：feature/fix 分支推送时检查审查报告
# ============================================================
WARNINGS=""

# 检查是否为 feature 或 fix 分支
if echo "$TARGET_BRANCH" | grep -E "^feature/" >/dev/null 2>&1; then
  # 查找审查报告
  REVIEW_REPORTS=$(find production/sprints/ -name "*-review.md" 2>/dev/null || echo "")
  if [ -z "$REVIEW_REPORTS" ]; then
    WARNINGS="${WARNINGS}\n[审查未完成] 未找到代码审查报告。"
    WARNINGS="${WARNINGS}\n  建议在推送前运行 /code-review 完成代码审查。"
    WARNINGS="${WARNINGS}\n  审查报告应存放在 production/sprints/*-review.md"
  fi
elif echo "$TARGET_BRANCH" | grep -E "^fix/" >/dev/null 2>&1; then
  # 查找审查报告
  REVIEW_REPORTS=$(find production/sprints/ -name "*-review.md" 2>/dev/null || echo "")
  if [ -z "$REVIEW_REPORTS" ]; then
    WARNINGS="${WARNINGS}\n[审查未完成] 未找到代码审查报告。"
    WARNINGS="${WARNINGS}\n  建议在推送前运行 /code-review 完成代码审查。"
    WARNINGS="${WARNINGS}\n  审查报告应存放在 production/sprints/*-review.md"
  fi
elif echo "$TARGET_BRANCH" | grep -E "^hotfix/" >/dev/null 2>&1; then
  # hotfix 可事后补审查，仅提示
  WARNINGS="${WARNINGS}\n[提示] hotfix 分支可在推送后补完成代码审查。"
  WARNINGS="${WARNINGS}\n  请在推送后尽快运行 /code-review 完成审查。"
elif echo "$TARGET_BRANCH" | grep -E "^release/" >/dev/null 2>&1; then
  # 检查 gate-check 报告
  GATE_REPORT="production/releases/gate-check.md"
  if [ ! -f "$GATE_REPORT" ]; then
    WARNINGS="${WARNINGS}\n[门禁未通过] 未找到门禁检查报告。"
    WARNINGS="${WARNINGS}\n  建议在推送前运行 /gate-check 完成门禁检查。"
    WARNINGS="${WARNINGS}\n  门禁报告应存放在 $GATE_REPORT"
  fi
fi

# ============================================================
# 检查 3：分支名规范检查
# ============================================================
# 合法分支名前缀模式
VALID_BRANCH_PATTERN="^(feature|fix|hotfix|release)/[a-zA-Z0-9_-]+$|^(main|master|develop)$"

# 检查分支名是否符合规范（跳过受保护分支，已在上面处理）
if ! echo "$TARGET_BRANCH" | grep -E "$VALID_BRANCH_PATTERN" >/dev/null 2>&1; then
  # 检查是否为已存在的远程分支（对已有分支仅警告不严格阻断）
  REMOTE_EXISTS=$(git ls-remote --heads origin "$TARGET_BRANCH" 2>/dev/null | head -n 1)
  if [ -z "$REMOTE_EXISTS" ]; then
    WARNINGS="${WARNINGS}\n[分支命名] 分支名 '$TARGET_BRANCH' 不符合命名规范。"
    WARNINGS="${WARNINGS}\n  规范格式：feature/[描述]、fix/[描述]、hotfix/[描述]、release/vX.Y.Z"
    WARNINGS="${WARNINGS}\n  详见 .studio/project/pr-guidelines.md 第 2 节"
  fi
fi

# ============================================================
# 输出警告（不阻断）
# ============================================================
if [ -n "$WARNINGS" ]; then
  echo "========================================" >&2
  echo "  [警告] 推送前检查提醒" >&2
  echo "========================================" >&2
  echo "  目标分支：$TARGET_BRANCH" >&2
  echo -e "$WARNINGS" >&2
  echo "========================================" >&2
fi

# 非受保护分支推送通过（仅有警告）
exit 0