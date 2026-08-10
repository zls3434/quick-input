#!/bin/bash
# Software Engineering Studios — 状态行脚本
# 显示：上下文使用率%、模型、阶段、Epic 面包屑

# 上下文使用率（如果可获取）
# Claude Code 会在 stdin 传入 JSON，包含 context 使用信息
CONTEXT_PCT=""
if [ -n "$CLAUDE_CONTEXT_PCT" ]; then
    CONTEXT_PCT="${CLAUDE_CONTEXT_PCT}%"
fi

# 当前模型
MODEL="${CLAUDE_MODEL:-sonnet}"

# 当前阶段
STAGE="未检测"
if [ -f "production/stage.txt" ]; then
    STAGE=$(cat production/stage.txt 2>/dev/null | head -1)
fi

# Epic 面包屑（从 active.md 读取）
BREADCRUMB=""
if [ -f "production/session-state/active.md" ]; then
    STATUS_BLOCK=$(sed -n '/<!-- STATUS -->/,/<!-- \/STATUS -->/p' production/session-state/active.md 2>/dev/null)
    if [ -n "$STATUS_BLOCK" ]; then
        EPIC=$(echo "$STATUS_BLOCK" | grep "^Epic:" | sed 's/Epic: //')
        FEATURE=$(echo "$STATUS_BLOCK" | grep "^Feature:" | sed 's/Feature: //')
        TASK=$(echo "$STATUS_BLOCK" | grep "^Task:" | sed 's/Task: //')
        if [ -n "$EPIC" ]; then
            BREADCRUMB="$EPIC"
            if [ -n "$FEATURE" ]; then
                BREADCRUMB="$BREADCRUMB > $FEATURE"
            fi
            if [ -n "$TASK" ]; then
                BREADCRUMB="$BREADCRUMB > $TASK"
            fi
        fi
    fi
fi

# 输出状态行
OUTPUT=""
if [ -n "$CONTEXT_PCT" ]; then
    OUTPUT="ctx:${CONTEXT_PCT} | "
fi
OUTPUT="${OUTPUT}模型:${MODEL} | 阶段:${STAGE}"
if [ -n "$BREADCRUMB" ]; then
    OUTPUT="${OUTPUT} | ${BREADCRUMB}"
fi

echo "$OUTPUT"
exit 0