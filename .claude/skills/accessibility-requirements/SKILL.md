---
name: accessibility-requirements
description: "确认项目无障碍需求，使用 accessibility-requirements.md 模板定义合规级别与具体要求。"
license: MIT
metadata:
  model: sonnet
  argument-hint: ""
  user-invocable: true
  allowed-tools:
    - Read
    - Glob
    - Grep
    - Write
    - AskUserQuestion
  platforms:
    claude-code: {enabled: true}
    cursor: {enabled: true}
    codex: {enabled: true}
    windsurf: {enabled: true, trigger: /accessibility-requirements}
    trae: {enabled: true}
    hermes: {enabled: true, platforms: [macos, linux, windows]}
    workbuddy: {enabled: true}
---

# accessibility-requirements — 无障碍需求确认

## 技能目的

确认项目的无障碍（Accessibility）合规级别与具体需求，使用模板生成无障碍需求文档，明确 WCAG 合规等级、目标用户群体、辅助技术支持范围与测试验收标准。

## 参数说明

本技能无参数。通过交互式问答确认需求。

## 分阶段工作流

### 阶段 1：加载上下文

- **输入**：项目目录
- **处理**：
  1. 使用 Read 读取 `docs/product-concept.md` 获取目标用户群体
  2. 使用 Glob 扫描 `docs/srs/*.md` 检查已有的无障碍相关需求
  3. 使用 Glob 查找 `.claude/docs/templates/accessibility-requirements.md` 模板
  4. 使用 Read 读取 `docs/technical-preferences.md` 获取前端框架
- **输出**：项目上下文 + 模板

### 阶段 2：确认合规级别

- **输入**：项目上下文
- **处理**：
  1. 通过 AskUserQuestion 确认 WCAG 合规等级（A / AA / AAA）
  2. 确认是否需要遵循 Section 508 或 EN 301 549 等地区标准
  3. 确认目标用户群体中的残障用户需求
  4. 确认法律合规要求（如 ADA、EU Web Accessibility Directive）
- **输出**：合规级别定义

### 阶段 3：定义具体要求

- **输入**：合规级别定义
- **处理**：
  1. 定义视觉无障碍要求（色彩对比、字体缩放、屏幕阅读器）
  2. 定义听觉无障碍要求（字幕、文字替代）
  3. 定义运动无障碍要求（键盘导航、语音控制兼容）
  4. 定义认知无障碍要求（简洁语言、一致性布局、错误提示）
  5. 确认支持的辅助技术清单
- **输出**：具体无障碍要求清单

### 阶段 4：定义测试验收标准

- **输入**：具体要求清单
- **处理**：
  1. 定义自动化测试工具与检查项（axe-core、Lighthouse）
  2. 定义手动测试检查清单
  3. 定义用户测试验收标准
  4. 通过 AskUserQuestion 确认验收门槛
- **输出**：测试验收标准

### 阶段 5：写入文件

- **输入**：全部需求内容
- **处理**：
  1. 基于模板填充各章节，向用户展示草稿
  2. 批准后使用 Write 写入 `docs/design/accessibility-requirements.md`
- **输出**：无障碍需求文件

## 协作协议引用

- 遵循 `.claude/docs/templates/collaborative-protocols/design-agent-protocol.md` 设计文档规范
- 参考 `.claude/docs/templates/accessibility-requirements.md` 模板
- 参考 WCAG 2.1 标准文档

## 推荐下一步

无障碍需求确认后，在编写 UX 规范时使用 `/ux-design` 引用此文档。审查 UX 规范时使用 `/ux-review` 验证无障碍合规度。测试阶段使用 `/qa-plan` 纳入无障碍测试用例。