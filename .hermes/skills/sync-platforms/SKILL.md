---
name: sync-platforms
description: "将 .studio/ 规范源同步到所有已启用平台的配置目录。运行各平台适配脚本，生成或更新 Claude Code、Codex、Cursor、Windsurf、Trae IDE、Hermes Agent、WorkBuddy 的配置文件。"
version: 1.0.0
platforms: [macos, linux, windows]
metadata:
  hermes:
    tags: ["platform-sync", "multi-platform", "configuration"]
    load_mode: on-demand
    model: sonnet
    user-invocable: true
    allowed-tools:
      - Bash
      - Read
      - Glob
---

# sync-platforms — 平台同步

## 技能目的

将 `.studio/` 规范源层的内容同步到所有已启用平台的配置目录，确保各平台配置与规范源保持一致。

## 执行流程

1. 读取 `.studio/manifest.yaml` 确认已启用的平台列表
2. 检查 `.studio/` 目录结构完整性
3. 依次运行各平台的适配脚本
4. 更新 `manifest.yaml` 中的 last-sync 时间戳
5. 输出同步报告

## 输出

- 各平台配置目录更新到最新状态
- manifest.yaml 时间戳更新
- 同步报告摘要