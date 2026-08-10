#!/bin/bash
# ============================================================
# validate-commit.sh
# PreToolUse(Bash) 触发：验证 git commit 命令
# 检查项：
#   1. 硬编码值（URL / IP / 端口 / 密钥）
#   2. TODO 格式（应为 TODO:描述 而非裸 TODO）
#   3. JSON 文件有效性
#   4. 设计文档必需章节存在
# exit 0 通过，exit 2 阻断并输出错误到 stderr
# ============================================================

set -euo pipefail

# 从 stdin 读取工具输入（Claude Code 传入的 JSON）
INPUT=$(cat)

# 提取命令字段（兼容不同 JSON 结构）
COMMAND=$(echo "$INPUT" | grep -Eo '"command"\s*:\s*"[^"]*"' | head -n 1 | sed -E 's/.*"command"\s*:\s*"([^"]*)".*/\1/')

# 如果无法从 JSON 提取，回退到第一个参数
if [ -z "$COMMAND" ]; then
  COMMAND="${1:-}"
fi

# 检查是否是 git commit 命令
if ! echo "$COMMAND" | grep -E "git\s+commit" >/dev/null 2>&1; then
  # 不是 git commit，直接通过
  exit 0
fi

echo "检测到 git commit 命令，开始验证..." >&2

ERRORS=""

# 获取暂存区文件列表
STAGED_FILES=$(git diff --cached --name-only 2>/dev/null || echo "")

if [ -z "$STAGED_FILES" ]; then
  # 无暂存文件，无法检查，放行
  exit 0
fi

# 1. 检查硬编码值（URL / IP / 端口 / 密钥）
for file in $STAGED_FILES; do
  if [ ! -f "$file" ]; then
    continue
  fi
  # 跳过文档和配置示例文件
  if echo "$file" | grep -E "\.env\.example$|\.md$|README|CHANGELOG" >/dev/null 2>&1; then
    continue
  fi
  # 检查硬编码密钥模式
  if grep -E "(api[_-]?key|secret|password|token)\s*[:=]\s*['\"][^'\"]{10,}" "$file" >/dev/null 2>&1; then
    ERRORS="${ERRORS}\n[硬编码密钥] $file 中疑似存在硬编码密钥/令牌。"
  fi
  # 检查硬编码内网 IP（排除测试与注释中的示例）
  if grep -E "(127\.0\.0\.1|10\.[0-9]+\.[0-9]+\.[0-9]+|192\.168\.[0-9]+\.[0-9]+|172\.(1[6-9]|2[0-9]|3[01])\.[0-9]+\.[0-9]+)" "$file" >/dev/null 2>&1; then
    # 排除 localhost 注释
    if grep -E "(127\.0\.0\.1|192\.168\.|10\.[0-9]|172\.(1[6-9]|2[0-9]|3[01]))" "$file" | grep -vE "^\s*//|^\s*#|example|sample|localhost" >/dev/null 2>&1; then
      ERRORS="${ERRORS}\n[硬编码 IP] $file 中疑似存在硬编码 IP 地址。"
    fi
  fi
done

# 2. 检查 TODO 格式（应为 TODO:描述 而非裸 TODO）
for file in $STAGED_FILES; do
  if [ ! -f "$file" ]; then
    continue
  fi
  # 查找裸 TODO（后不跟冒号或括号）
  if grep -E "TODO[^:([:space:]]" "$file" >/dev/null 2>&1; then
    ERRORS="${ERRORS}\n[TODO 格式] $file 中存在裸 TODO，应使用 TODO:描述 格式。"
  fi
done

# 3. 检查 JSON 文件有效性
for file in $STAGED_FILES; do
  if [ ! -f "$file" ]; then
    continue
  fi
  # 仅检查 .json 文件（跳过 package-lock 等大型锁文件）
  if echo "$file" | grep -E "\.json$" >/dev/null 2>&1; then
    if echo "$file" | grep -E "package-lock\.json$|yarn\.lock$|pnpm-lock\.yaml$" >/dev/null 2>&1; then
      continue
    fi
    if ! python -m json.tool "$file" >/dev/null 2>&1; then
      ERRORS="${ERRORS}\n[JSON 无效] $file 不是有效的 JSON 文件。"
    fi
  fi
done

# 4. 检查设计文档必需章节存在
REQUIRED_SECTIONS="概述与背景|目标与非目标|用户故事|功能需求|非功能需求|验收标准|依赖与约束|开放问题"
for file in $STAGED_FILES; do
  if [ ! -f "$file" ]; then
    continue
  fi
  # 仅检查 design/requirements/ 下的需求文档
  if echo "$file" | grep -E "^design/requirements/" >/dev/null 2>&1; then
    CONTENT=$(cat "$file")
    for section in $REQUIRED_SECTIONS; do
      if ! echo "$CONTENT" | grep -E "$section" >/dev/null 2>&1; then
        ERRORS="${ERRORS}\n[文档章节缺失] $file 缺少必需章节：$section"
      fi
    done
  fi
done

# 输出结果
if [ -n "$ERRORS" ]; then
  echo "========================================" >&2
  echo "  提交验证失败，请修复以下问题：" >&2
  echo "========================================" >&2
  echo -e "$ERRORS" >&2
  echo "" >&2
  echo "修复后请重新提交。" >&2
  exit 2
fi

echo "提交验证通过。" >&2
exit 0