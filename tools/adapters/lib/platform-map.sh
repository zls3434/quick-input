#!/usr/bin/env bash
# 平台映射配置
# 定义各平台的输出路径、格式转换方式等

# 平台列表
PLATFORMS=(
  "claude-code"
  "codex"
  "cursor"
  "windsurf"
  "trae"
  "hermes"
  "workbuddy"
)

# 平台输出目录映射
declare -A PLATFORM_OUTPUT_DIR=(
  ["claude-code"]=".claude"
  ["codex"]=""
  ["cursor"]=".cursor"
  ["windsurf"]=".windsurf"
  ["trae"]=".trae"
  ["hermes"]=".hermes"
  ["workbuddy"]=".workbuddy"
)

# 平台主配置文件映射
declare -A PLATFORM_MAIN_CONFIG=(
  ["claude-code"]="CLAUDE.md"
  ["codex"]="AGENTS.md"
  ["cursor"]="AGENTS.md"
  ["windsurf"]=".windsurfrules"
  ["trae"]=".trae/rules/project_rules.md"
  ["hermes"]=".hermes/SOUL.md"
  ["workbuddy"]=".workbuddy/manifest.yaml"
)

# 平台技能格式映射
# direct-copy: 直接复制 .studio 格式
# metadata-promote: metadata 字段提升为顶层
# workflow: 转为工作流格式
# hermes-extend: 添加 Hermes 扩展字段
declare -A PLATFORM_SKILL_FORMAT=(
  ["claude-code"]="metadata-promote"
  ["codex"]="none"
  ["cursor"]="direct-copy"
  ["windsurf"]="workflow"
  ["trae"]="direct-copy"
  ["hermes"]="hermes-extend"
  ["workbuddy"]="direct-copy"
)

# 平台规则格式映射
# yaml-paths: YAML frontmatter 含 paths
# mdc: Cursor .mdc 格式
# inline: 内联到单一文件
# subdirectory: 拆分到子目录
declare -A PLATFORM_RULE_FORMAT=(
  ["claude-code"]="yaml-paths"
  ["codex"]="subdirectory"
  ["cursor"]="mdc"
  ["windsurf"]="inline"
  ["trae"]="inline"
  ["hermes"]="agents-md"
  ["workbuddy"]="agents-md"
)

# 平台 Hook 支持
declare -A PLATFORM_HOOKS=(
  ["claude-code"]="supported"
  ["codex"]="not-supported"
  ["cursor"]="not-supported"
  ["windsurf"]="not-supported"
  ["trae"]="not-supported"
  ["hermes"]="partial"
  ["workbuddy"]="not-supported"
)

# 获取平台技能输出目录
get_skill_output_dir() {
  local platform="$1"
  case "$platform" in
    claude-code) echo ".claude/skills" ;;
    cursor) echo ".cursor/skills" ;;
    windsurf) echo ".windsurf/workflows" ;;
    trae) echo ".trae/skills" ;;
    hermes) echo ".hermes/skills" ;;
    workbuddy) echo ".workbuddy/skills" ;;
    *) echo "" ;;
  esac
}

# 获取平台规则输出路径
get_rule_output_path() {
  local platform="$1"
  case "$platform" in
    claude-code) echo ".claude/rules" ;;
    cursor) echo ".cursor/rules" ;;
    windsurf) echo ".windsurfrules" ;;
    trae) echo ".trae/rules/project_rules.md" ;;
    *) echo "" ;;
  esac
}

# 获取平台 Agent 输出目录
get_agent_output_dir() {
  local platform="$1"
  case "$platform" in
    claude-code) echo ".claude/agents" ;;
    cursor) echo ".cursor/rules" ;;
    windsurf) echo ".windsurf/workflows" ;;
    trae) echo ".trae/agents" ;;
    *) echo "" ;;
  esac
}