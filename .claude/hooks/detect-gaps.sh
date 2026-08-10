#!/bin/bash
# ============================================================
# detect-gaps.sh
# 会话启动时检测项目缺口
# 检测三种场景：
#   1. 全新项目（无 src/ 无 design/）
#   2. 有代码但缺需求文档（有 src/ 无 design/requirements/）
#   3. 有需求文档但缺架构文档（有 design/ 无 docs/architecture/）
# ============================================================

set -euo pipefail

echo "========================================"
echo "  项目缺口检测"
echo "========================================"

HAS_SRC=false
HAS_DESIGN=false
HAS_REQUIREMENTS=false
HAS_ARCHITECTURE=false

# 检测 src/ 目录
if [ -d "src" ]; then
  HAS_SRC=true
fi

# 检测 design/ 目录
if [ -d "design" ]; then
  HAS_DESIGN=true
fi

# 检测 design/requirements/ 目录
if [ -d "design/requirements" ]; then
  HAS_REQUIREMENTS=true
fi

# 检测 docs/architecture/ 目录
if [ -d "docs/architecture" ]; then
  HAS_ARCHITECTURE=true
fi

# 场景 1：全新项目
if [ "$HAS_SRC" = false ] && [ "$HAS_DESIGN" = false ]; then
  echo ""
  echo "[缺口] 检测到全新项目：无 src/ 目录、无 design/ 目录。"
  echo "  建议：运行 /start 初始化项目结构与开发流程。"
  echo "  后续步骤："
  echo "    1. 编写需求文档（design/requirements/）"
  echo "    2. 编写架构设计（docs/architecture/）"
  echo "    3. 创建源码骨架（src/）"
  exit 0
fi

# 场景 2：有代码但缺需求文档
if [ "$HAS_SRC" = true ] && [ "$HAS_REQUIREMENTS" = false ]; then
  echo ""
  echo "[缺口] 检测到代码已存在但缺少需求文档："
  echo "  - src/ 目录存在"
  echo "  - design/requirements/ 目录缺失"
  echo "  建议：补充需求文档，确保开发有据可依。"
  echo "  后续步骤："
  echo "    1. 在 design/requirements/ 下创建需求文档"
  echo "    2. 明确用户故事与验收标准"
fi

# 场景 3：有需求文档但缺架构文档
if [ "$HAS_REQUIREMENTS" = true ] && [ "$HAS_ARCHITECTURE" = false ]; then
  echo ""
  echo "[缺口] 检测到有需求文档但缺少架构文档："
  echo "  - design/ 目录存在"
  echo "  - docs/architecture/ 目录缺失"
  echo "  建议：补充架构设计文档，明确技术选型与模块划分。"
  echo "  后续步骤："
  echo "    1. 在 docs/architecture/ 下创建架构文档"
  echo "    2. 编写 ADR（架构决策记录）"
fi

# 无缺口
if [ "$HAS_SRC" = true ] && [ "$HAS_REQUIREMENTS" = true ] && [ "$HAS_ARCHITECTURE" = true ]; then
  echo ""
  echo "未检测到明显缺口，项目结构完整。"
fi

echo ""
echo "========================================"
echo "  缺口检测完成"
echo "========================================"

exit 0