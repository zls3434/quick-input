#!/usr/bin/env bash
# 同步所有平台配置
# 从 .studio/ 规范源生成各平台配置

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

echo "=== Software Engineering Studios 平台同步 ==="
echo "规范源: $PROJECT_ROOT/.studio/"
echo ""

# 检查规范源存在
if [ ! -d "$PROJECT_ROOT/.studio" ]; then
    echo "错误: .studio/ 目录不存在"
    exit 1
fi

# 同步各平台
for platform in claude-code codex cursor windsurf trae hermes workbuddy; do
    echo "--- 同步 $platform ---"
    if [ -f "$SCRIPT_DIR/sync-$platform.sh" ]; then
        bash "$SCRIPT_DIR/sync-$platform.sh" "$PROJECT_ROOT" || echo "警告: $platform 同步失败"
    else
        echo "跳过: sync-$platform.sh 不存在"
    fi
    echo ""
done

# 更新 manifest.yaml 时间戳
if [ -f "$PROJECT_ROOT/.studio/manifest.yaml" ] && command -v python3 &>/dev/null; then
    python3 -c "
import datetime
print('manifest.yaml last-sync: ' + datetime.datetime.now().isoformat())
"
fi

echo "=== 同步完成 ==="