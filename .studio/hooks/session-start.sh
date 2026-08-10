#!/bin/bash
# ============================================================
# session-start.sh
# 会话启动时加载项目上下文
# 输出当前分支、最近提交、活跃 Sprint、里程碑、Bug 计数、
# 代码健康检查及会话状态恢复信息
# ============================================================

set -euo pipefail

echo "========================================"
echo "  会话上下文加载"
echo "========================================"

# 1. 当前分支
echo ""
echo "[1] 当前分支："
if git rev-parse --is-inside-work-tree >/dev/null 2>&1; then
  BRANCH=$(git branch --show-current 2>/dev/null || echo "未知")
  echo "  当前分支：$BRANCH"
else
  echo "  当前目录不在 Git 仓库中，跳过分支检测。"
fi

# 2. 最近 5 条提交
echo ""
echo "[2] 最近 5 条提交："
if git rev-parse --is-inside-work-tree >/dev/null 2>&1; then
  git log --oneline -5 2>/dev/null || echo "  无提交记录"
else
  echo "  跳过提交记录。"
fi

# 3. 活跃 Sprint（查找 production/sprints/sprint-*.md）
echo ""
echo "[3] 活跃 Sprint："
SPRINT_DIR="production/sprints"
if [ -d "$SPRINT_DIR" ]; then
  SPRINTS=$(ls "$SPRINT_DIR"/sprint-*.md 2>/dev/null || true)
  if [ -n "$SPRINTS" ]; then
    for sprint in $SPRINTS; do
      echo "  - $(basename "$sprint")"
    done
  else
    echo "  未找到活跃 Sprint。"
  fi
else
  echo "  Sprint 目录不存在：$SPRINT_DIR"
fi

# 4. 活跃里程碑（查找 production/milestones/*.md）
echo ""
echo "[4] 活跃里程碑："
MILESTONE_DIR="production/milestones"
if [ -d "$MILESTONE_DIR" ]; then
  MILESTONES=$(ls "$MILESTONE_DIR"/*.md 2>/dev/null || true)
  if [ -n "$MILESTONES" ]; then
    for ms in $MILESTONES; do
      echo "  - $(basename "$ms")"
    done
  else
    echo "  未找到里程碑文档。"
  fi
else
  echo "  里程碑目录不存在：$MILESTONE_DIR"
fi

# 5. 未解决 Bug 计数（查找 BUG-*.md）
echo ""
echo "[5] 未解决 Bug 计数："
BUG_COUNT=0
if git rev-parse --is-inside-work-tree >/dev/null 2>&1; then
  # 在工作区内搜索 BUG-*.md 文件
  BUG_COUNT=$(find . -name "BUG-*.md" -not -path "*/node_modules/*" 2>/dev/null | wc -l | tr -d ' ')
else
  BUG_COUNT=$(find . -name "BUG-*.md" -not -path "*/node_modules/*" 2>/dev/null | wc -l | tr -d ' ')
fi
echo "  未解决 Bug 文档数：$BUG_COUNT"

# 6. 代码健康检查（src/ 中 TODO / FIXME 计数）
echo ""
echo "[6] 代码健康检查："
if [ -d "src" ]; then
  TODO_COUNT=$(grep -r "TODO" src/ 2>/dev/null | wc -l | tr -d ' ' || echo "0")
  FIXME_COUNT=$(grep -r "FIXME" src/ 2>/dev/null | wc -l | tr -d ' ' || echo "0")
  echo "  src/ 中 TODO 数量：$TODO_COUNT"
  echo "  src/ 中 FIXME 数量：$FIXME_COUNT"
else
  echo "  src/ 目录不存在，跳过代码健康检查。"
fi

# 7. 会话状态恢复（预览 active.md 最后 20 行）
echo ""
echo "[7] 会话状态恢复："
STATE_FILE="production/session-state/active.md"
if [ -f "$STATE_FILE" ]; then
  echo "  发现上次会话状态，预览最后 20 行："
  echo "  ----------------------------------------"
  tail -n 20 "$STATE_FILE" | sed 's/^/  /'
  echo "  ----------------------------------------"
else
  echo "  无历史会话状态文件：$STATE_FILE"
  echo "  这是首次会话或状态文件已归档。"
fi

echo ""
echo "========================================"
echo "  上下文加载完成"
echo "========================================"

exit 0