#!/bin/bash
# ============================================================
# log-agent.sh
# SubagentStart 事件：记录子 Agent 调用到审计日志
# 记录：时间戳、Agent 名称、参数摘要
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

# 提取参数摘要（尝试多种可能的字段名）
AGENT_PARAMS=$(echo "$INPUT" | grep -Eo '"params"\s*:\s*"[^"]*"' | head -n 1 | sed -E 's/.*"params"\s*:\s*"([^"]*)".*/\1/')
if [ -z "$AGENT_PARAMS" ]; then
  AGENT_PARAMS=$(echo "$INPUT" | grep -Eo '"arguments"\s*:\s*"[^"]*"' | head -n 1 | sed -E 's/.*"arguments"\s*:\s*"([^"]*)".*/\1/')
fi
if [ -z "$AGENT_PARAMS" ]; then
  AGENT_PARAMS=$(echo "$INPUT" | grep -Eo '"input"\s*:\s*"[^"]*"' | head -n 1 | sed -E 's/.*"input"\s*:\s*"([^"]*)".*/\1/')
fi

# 截断过长的参数摘要
if [ ${#AGENT_PARAMS} -gt 200 ]; then
  AGENT_PARAMS="${AGENT_PARAMS:0:200}..."
fi

if [ -z "$AGENT_PARAMS" ]; then
  AGENT_PARAMS="(无参数)"
fi

# 写入审计日志（JSON 格式，便于后续解析）
echo "{\"event\":\"agent_start\",\"timestamp\":\"${TIMESTAMP}\",\"agent\":\"${AGENT_NAME}\",\"params\":\"${AGENT_PARAMS}\"}" >> "$LOG_FILE"

echo "已记录子 Agent 调用：${AGENT_NAME}" >&2

exit 0