#!/bin/bash
# ============================================================
# notify.sh
# Notification 事件：显示 Windows Toast 通知
# 非Windows平台 exit 0
# Windows 平台使用 PowerShell 显示 BurntToast 通知
# 环境变量 $CLAUDE_NOTIFICATION 包含通知内容
# ============================================================

set -euo pipefail

# 通知内容（来自环境变量或参数）
NOTIFICATION="${CLAUDE_NOTIFICATION:-${1:-Claude Code 通知}}"

# 检测操作系统
OS_TYPE=$(uname -s 2>/dev/null || echo "")

# Windows 平台（包括 Git Bash / WSL 互操作）
# uname 在 Git Bash 下返回 MINGW* 或 MSYS*，在 Cygwin 下返回 CYGWIN*
# 在 WSL 下返回 Linux，但可通过检查 /mnt/c 判断
IS_WINDOWS=false

if echo "$OS_TYPE" | grep -E "^(MINGW|MSYS|CYGWIN)" >/dev/null 2>&1; then
  IS_WINDOWS=true
elif echo "$OS_TYPE" | grep -E "^Windows" >/dev/null 2>&1; then
  IS_WINDOWS=true
fi

# 非Windows平台直接退出
if [ "$IS_WINDOWS" = false ]; then
  # 在 Linux/macOS 上也可尝试替代通知方式，但按需求仅 Windows 支持
  exit 0
fi

# Windows 平台：使用 PowerShell 显示 Toast 通知
# 尝试使用 BurntToast 模块，若不可用则使用基本通知
# 转义通知内容中的单引号（PowerShell 字符串安全）
ESCAPED_NOTIFICATION=$(echo "$NOTIFICATION" | sed "s/'/''/g")

powershell.exe -NoProfile -Command "try { New-BurntToastNotification -Text 'Claude Code', '${ESCAPED_NOTIFICATION}' } catch { Write-Host '通知模块不可用: '$_ }" 2>/dev/null || {
  # BurntToast 不可用时回退到基本消息框
  powershell.exe -NoProfile -Command "Add-Type -AssemblyName System.Windows.Forms; [System.Windows.Forms.MessageBox]::Show('${ESCAPED_NOTIFICATION}', 'Claude Code', 'OK', 'Information')" 2>/dev/null || true
}

exit 0