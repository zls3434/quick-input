---
name: skill-test
description: "验证技能文件的合规性和正确性，检查 YAML frontmatter 格式与 markdown body 完整性，生成合规报告。"
version: 1.0.0
platforms: [macos, linux, windows]
metadata:
  hermes:
    tags: ["maintenance", "quality"]
    load_mode: on-demand
    model: sonnet
    argument-hint: "[技能名]"
    user-invocable: true
    allowed-tools:
      - Read
      - Glob
      - Grep
      - Write
---

# skill-test — 技能文件合规性验证

## 技能目的

验证 `.claude/skills/` 目录下的技能定义文件（SKILL.md）是否满足合规要求与正确性约束。检查 YAML frontmatter 格式是否包含必需字段，检查 markdown body 是否具备完整的工作流阶段、协作协议引用与推荐下一步章节，生成合规报告供改进参考。

## 参数说明

- `[技能名]`：待验证的技能名称，对应 `.claude/skills/[技能名]/SKILL.md`。如未提供则扫描全部技能。

## 分阶段工作流

### 阶段 1：读取 SKILL.md

- **输入**：`[技能名]` 参数
- **处理**：
  1. 使用 Glob 查找 `.claude/skills/[技能名]/SKILL.md`，如未指定技能名则扫描全部 `SKILL.md` 文件
  2. 使用 Read 读取目标 SKILL.md 文件内容
  3. 使用 Grep 分离 YAML frontmatter 与 markdown body
- **输出**：SKILL.md 原始内容

### 阶段 2：检查 YAML frontmatter 格式

- **输入**：YAML frontmatter 部分
- **处理**：
  1. 验证 `name` 字段存在且与目录名一致
  2. 验证 `description` 字段存在且非空
  3. 验证 `allowed-tools` 字段存在且为合法工具列表（Read / Glob / Grep / Write / Bash / Task / AskUserQuestion 等）
  4. 验证 `model` 字段存在且取值为合法模型层级（haiku / sonnet / opus）
  5. 检查可选字段（argument-hint、user-invocable）格式是否正确
- **输出**：frontmatter 合规检查结果

### 阶段 3：检查 markdown body 完整性

- **输入**：markdown body 部分
- **处理**：
  1. 检查是否存在"工作流"相关章节（分阶段工作流或执行流程）
  2. 检查是否存在"协作协议引用"章节
  3. 检查是否存在"推荐下一步"章节
  4. 检查 body 行数是否满足最低要求（至少 20 行）
  5. 检查工作流阶段是否清晰定义输入、处理与输出
- **输出**：body 完整性检查结果

### 阶段 4：生成合规报告

- **输入**：全部检查结果
- **处理**：
  1. 汇总 frontmatter 与 body 的全部检查项
  2. 按严重度分类问题：错误（缺失必需字段）、警告（格式不规范）、提示（可改进项）
  3. 给出整体合规评分与通过/不通过结论
  4. 使用 Write 写入报告至 `docs/reports/skill-test-[技能名].md`
- **输出**：技能合规报告

## 协作协议引用

- 遵循 `docs/examples/COLLABORATIVE-DESIGN-PRINCIPLE.md` 用户驱动协作协议
- 参考 `docs/skills-reference.md` 技能定义格式规范
- 参考 `docs/templates/skill-test-spec.md` 技能测试规格模板
- 验证为只读分析，不修改 SKILL.md 文件

## 推荐下一步

合规报告显示问题时，使用 `/skill-improve` 改进技能定义。修复后重新运行本技能验证。新技能创建后应始终运行本技能确认合规性。