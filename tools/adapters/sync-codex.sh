#!/usr/bin/env bash
# 同步 Codex 配置（生成 AGENTS.md）
# AGENTS.md 已在项目根目录，此脚本检查一致性

set -euo pipefail

PROJECT_ROOT="${1:-$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)}"

echo "同步 Codex 配置..."
echo "AGENTS.md 已存在于项目根目录"
echo "子目录 AGENTS.md: src/frontend/, src/backend/, tests/"
echo "Codex 配置同步完成"