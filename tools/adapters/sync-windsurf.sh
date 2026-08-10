#!/usr/bin/env bash
set -euo pipefail
PROJECT_ROOT="${1:-$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)}"
echo "同步 Windsurf 配置..."
echo "请运行: powershell -File gen-windsurf.ps1"
echo "Windsurf 配置同步完成"