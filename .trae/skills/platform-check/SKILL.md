---
name: platform-check
description: 检查各平台配置的一致性和完整性。对比 .studio/ 规范源与各平台输出层，检测过期、缺失或不一致的配置文件。
license: MIT
metadata:
  model: haiku
  user-invocable: true
  allowed-tools:
    - Read
    - Glob
    - Grep
  platforms:
    claude-code: {enabled: true}
    cursor: {enabled: true}
    codex: {enabled: true}
    windsurf: {enabled: true, trigger: /platform-check}
    trae: {enabled: true}
    hermes: {enabled: true, platforms: [macos, linux, windows]}
    workbuddy: {enabled: true}
---

# platform-check — 平台配置检查

## 技能目的

检查各平台配置与 `.studio/` 规范源的一致性，识别需要同步的文件。

## 检查项目

1. **规范源完整性**：.studio/ 下所有必需文件存在
2. **Agent 数量一致**：各平台 Agent 定义数量 = 40
3. **技能数量一致**：各平台技能数量 = 74
4. **规则数量一致**：各平台规则数量 = 11
5. **AGENTS.md 存在**：根目录和子目录 AGENTS.md 存在
6. **manifest.yaml 状态**：检查 last-sync 是否过期

## 输出

- 各平台配置状态报告
- 不一致项清单
- 建议操作（运行 /sync-platforms 修复）