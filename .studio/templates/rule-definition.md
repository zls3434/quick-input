<!--
  路径规则定义模板
  用途：通过 /create-rule 工作流创建新路径规则时使用此模板
  填写指南：替换所有 [占位符] 为实际内容
  规范参考：.studio/rules/_schema.yaml
  paths 字段：使用 glob 模式指定规则适用的文件路径范围
    示例："src/api/**" 匹配 src/api 下所有文件
    示例："**/*.test.ts" 匹配所有 .test.ts 文件
-->

---
状态: 草稿
作者: [提案 Agent 名称]
更新日期: [YYYY-MM-DD]
---

paths:
  - "[路径 glob 模式 1]"
  - "[路径 glob 模式 2]"
platforms:
  claude-code: {enabled: true, path: .claude/rules/[rule-name].md}
  cursor: {enabled: true, type: auto-attached, globs: "[路径 glob 模式 1], [路径 glob 模式 2]"}
  codex: {enabled: true, section: agents.md}
  windsurf: {enabled: true, mode: append}
  trae: {enabled: true, mode: append}
  hermes: {enabled: false}
  workbuddy: {enabled: true}
---
# [规则名称]

- [规范条目 1]
  - [子条目：具体要求，用"必须/禁止/应该"开头]
  - [子条目：具体要求]
- [规范条目 2]
  - [子条目：具体要求]
  - [子条目：具体要求]
- [规范条目 3]
  - [子条目：具体要求]
  - [子条目：具体要求]

## 示例

**正确**：
```[语言]
[正确示例代码]
```

**错误**：
```[语言]
[错误示例代码]
```