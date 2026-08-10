#!/bin/bash
# ============================================================
# session-stop.sh
# 会话关闭：将 active.md 归档到 production/session-logs/
# 以时间戳命名，并记录 git 活动（最近 1 小时的提交）
# ============================================================

set -euo pipefail

# 路径定义
STATE_DIR="production/session-state"
STATE_FILE="$STATE_DIR/active.md"
LOG_DIR="production/session-logs"

# 创建日志目录（如果不存在）
if [ ! -d "$LOG_DIR" ]; then
  mkdir -p "$LOG_DIR"
fi

# 生成时间戳（用于归档文件名）
TIMESTAMP=$(date -u +"%Y%m%dT%H%M%SZ" 2>/dev/null || date +"%Y%m%dT%H%M%S" 2>/dev/null || echo "unknown")

# 归档文件路径
ARCHIVE_FILE="$LOG_DIR/session-${TIMESTAMP}.md"

# 如果 active.md 存在，执行归档
if [ -f "$STATE_FILE" ]; then
  # 追加会话关闭信息
  cat >> "$STATE_FILE" << EOF

---

## 会话关闭 - ${TIMESTAMP}

**会话结束时间**：${TIMESTAMP}

### Git 活动（最近 1 小时）
EOF

  # 记录最近 1 小时的 git 活动
  if git rev-parse --is-inside-work-tree >/dev/null 2>&1; then
    GIT_LOG=$(git log --oneline --since="1 hour ago" 2>/dev/null || echo "")
    if [ -n "$GIT_LOG" ]; then
      echo '```' >> "$STATE_FILE"
      echo "$GIT_LOG" >> "$STATE_FILE"
      echo '```' >> "$STATE_FILE"
    else
      echo "（最近 1 小时无提交活动）" >> "$STATE_FILE"
    fi

    # 追加当前工作区状态
    echo "" >> "$STATE_FILE"
    echo "### 工作区状态" >> "$STATE_FILE"
    echo '```' >> "$STATE_FILE"
    git status --short 2>/dev/null >> "$STATE_FILE" || echo "（无法获取状态）" >> "$STATE_FILE"
    echo '```' >> "$STATE_FILE"
  else
    echo "（不在 Git 仓库中，跳过 Git 活动记录）" >> "$STATE_FILE"
  fi

  # 复制到归档文件
  cp "$STATE_FILE" "$ARCHIVE_FILE"

  # 清空 active.md 以备下次会话
  cat > "$STATE_FILE" << 'EOF'
# 会话状态

（等待新会话初始化）
EOF

  echo "会话已归档到：$ARCHIVE_FILE" >&2
  echo "active.md 已重置，等待下次会话。" >&2
else
  echo "未找到会话状态文件：$STATE_FILE" >&2
  echo "无需归档，可能是首次会话。" >&2

  # 即使没有 active.md，也创建一个归档记录
  cat > "$ARCHIVE_FILE" << EOF
# 会话关闭记录 - ${TIMESTAMP}

（无 active.md 文件，可能是首次会话或已手动清理）

## Git 活动（最近 1 小时）
EOF

  if git rev-parse --is-inside-work-tree >/dev/null 2>&1; then
    GIT_LOG=$(git log --oneline --since="1 hour ago" 2>/dev/null || echo "")
    if [ -n "$GIT_LOG" ]; then
      echo '```' >> "$ARCHIVE_FILE"
      echo "$GIT_LOG" >> "$ARCHIVE_FILE"
      echo '```' >> "$ARCHIVE_FILE"
    else
      echo "（最近 1 小时无提交活动）" >> "$ARCHIVE_FILE"
    fi
  else
    echo "（不在 Git 仓库中）" >> "$ARCHIVE_FILE"
  fi

  echo "已创建归档记录：$ARCHIVE_FILE" >&2
fi

exit 0