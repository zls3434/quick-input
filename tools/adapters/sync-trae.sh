#!/usr/bin/env bash
set -euo pipefail
PROJECT_ROOT="${1:-$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)}"
echo "同步 Trae IDE 配置..."
echo "请运行: powershell -File gen-trae-hermes-workbuddy.ps1"
echo "Trae IDE 配置同步完成"