#!/bin/bash
# ============================================================
# pre-compact.sh
# 压缩前：将当前会话进度备注追加到
# production/session-state/active.md
# 记录：当前任务、修改的文件、关键决策
# 如果目录不存在则创建
# ============================================================

set -euo pipefail

# 会话状态文件路径
STATE_DIR="production/session-state"
STATE_FILE="$STATE_DIR/active.md"

# 创建目录（如果不存在）
if [ ! -d "$STATE_DIR" ]; then
  mkdir -p "$STATE_DIR"
  echo "已创建会话状态目录：$STATE_DIR" >&2
fi

# 当前时间戳（ISO 8601 格式）
TIMESTAMP=$(date -u +"%Y-%m-%dT%H:%M:%SZ" 2>/dev/null || date +"%Y-%m-%dT%H:%M:%S" 2>/dev/null || echo "未知时间")

# 当前分支
CURRENT_BRANCH=$(git branch --show-current 2>/dev/null || echo "未知")

# 追加会话进度备注到 active.md
cat >> "$STATE_FILE" << EOF

---

## 会话压缩快照 - ${TIMESTAMP}

**当前分支**：${CURRENT_BRANCH}

**当前任务**：
（请在此处填写当前正在进行的任务）

**本次修改的文件**：
EOF

# 尝试列出本次会话修改的文件（通过 git status）
if git rev-parse --is-inside-work-tree >/dev/null 2>&1; then
  MODIFIED_FILES=$(git status --short 2>/dev/null || echo "")
  if [ -n "$MODIFIED_FILES" ]; then
    echo '```' >> "$STATE_FILE"
    echo "$MODIFIED_FILES" >> "$STATE_FILE"
    echo '```' >> "$STATE_FILE"
  else
    echo "（无未提交的文件变更）" >> "$STATE_FILE"
  fi
else
  echo "（不在 Git 仓库中，无法获取文件变更）" >> "$STATE_FILE"
fi

# 追加关键决策部分
cat >> "$STATE_FILE" << 'EOF'

**关键决策**：
（请在此处记录本次会话做出的关键决策）

**下次继续事项**：
（请在此处记录下次需要继续的工作）

---
EOF

echo "会话进度已保存到 $STATE_FILE" >&2
echo "压缩前快照已完成，可安全执行压缩操作。" >&2

exit 0