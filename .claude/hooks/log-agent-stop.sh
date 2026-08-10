#!/bin/bash
# ============================================================
# log-agent-stop.sh
# SubagentStop 事件：记录子 Agent 完成到审计日志
# 记录：时间戳、Agent 名称、结果摘要
# 日志文件：production/session-logs/agent-audit.log
# ============================================================

set -euo pipefail

# 审计日志文件路径
LOG_DIR="production/session-logs"
LOG_FILE="$LOG_DIR/agent-audit.log"

# 创建日志目录（如果不存在）
if [ ! -d "$LOG_DIR" ]; then
  mkdir -p "$LOG_DIR"
fi

# 从 stdin 读取事件输入（Claude Code 传入的 JSON）
INPUT=$(cat)

# 时间戳（ISO 8601 格式）
TIMESTAMP=$(date -u +"%Y-%m-%dT%H:%M:%SZ" 2>/dev/null || date +"%Y-%m-%dT%H:%M:%S" 2>/dev/null || echo "未知时间")

# 提取 Agent 名称（尝试多种可能的字段名）
AGENT_NAME=$(echo "$INPUT" | grep -Eo '"agent_name"\s*:\s*"[^"]*"' | head -n 1 | sed -E 's/.*"agent_name"\s*:\s*"([^"]*)".*/\1/')
if [ -z "$AGENT_NAME" ]; then
  AGENT_NAME=$(echo "$INPUT" | grep -Eo '"name"\s*:\s*"[^"]*"' | head -n 1 | sed -E 's/.*"name"\s*:\s*"([^"]*)".*/\1/')
fi
if [ -z "$AGENT_NAME" ]; then
  AGENT_NAME=$(echo "$INPUT" | grep -Eo '"agent"\s*:\s*"[^"]*"' | head -n 1 | sed -E 's/.*"agent"\s*:\s*"([^"]*)".*/\1/')
fi
if [ -z "$AGENT_NAME" ]; then
  AGENT_NAME="unknown"
fi

# 提取结果摘要（尝试多种可能的字段名）
AGENT_RESULT=$(echo "$INPUT" | grep -Eo '"result"\s*:\s*"[^"]*"' | head -n 1 | sed -E 's/.*"result"\s*:\s*"([^"]*)".*/\1/')
if [ -z "$AGENT_RESULT" ]; then
  AGENT_RESULT=$(echo "$INPUT" | grep -Eo '"output"\s*:\s*"[^"]*"' | head -n 1 | sed -E 's/.*"output"\s*:\s*"([^"]*)".*/\1/')
fi
if [ -z "$AGENT_RESULT" ]; then
  AGENT_RESULT=$(echo "$INPUT" | grep -Eo '"response"\s*:\s*"[^"]*"' | head -n 1 | sed -E 's/.*"response"\s*:\s*"([^"]*)".*/\1/')
fi

# 截断过长的结果摘要
if [ ${#AGENT_RESULT} -gt 200 ]; then
  AGENT_RESULT="${AGENT_RESULT:0:200}..."
fi

if [ -z "$AGENT_RESULT" ]; then
  AGENT_RESULT="(无结果摘要)"
fi

# 写入审计日志（JSON 格式，便于后续解析）
echo "{\"event\":\"agent_stop\",\"timestamp\":\"${TIMESTAMP}\",\"agent\":\"${AGENT_NAME}\",\"result\":\"${AGENT_RESULT}\"}" >> "$LOG_FILE"

echo "已记录子 Agent 完成：${AGENT_NAME}" >&2

exit 0