<!-- Software Engineering Studios -->
# AGENTS.md

## 项目概述

Software Engineering Studios 通过 40 个协调的 AI Agent 管理软件工程项目开发。每个 Agent 负责一个特定领域，确保关注点分离和质量把控。包含 74 个技能、12 个 Hook、11 个路径规则和 35 个文档模板。

### 核心理念
**用户驱动的协作，而非自主执行。** 每个任务遵循：提问 → 选项 → 决策 → 草稿 → 审批。

## 技术栈

- **前端框架**：[选择：React / Vue / Angular]
- **后端语言**：[选择：Node.js / Python / Java / Go]
- **数据库**：[选择：PostgreSQL / MongoDB / Redis]
- **版本控制**：Git，采用基于主干的开发模式

## 环境搭建

- 安装依赖：根据技术栈执行 `npm install` / `pip install -r requirements.txt` / `go mod download`
- 开发服务器：根据技术栈执行 `npm run dev` / `python manage.py runserver` 等
- 运行测试：`npm test` / `pytest` / `go test ./...` / `mvn test`

## 代码风格

- 前端组件名 PascalCase，文件名 kebab-case
- 后端按语言惯例：Python snake_case，Java/Go camelCase/PascalCase
- 测试文件命名：`[被测名].test.[ext]` 或 `[被测名].spec.[ext]`
- 文档文件命名：kebab-case
- 禁止硬编码环境配置、禁止 `any` 类型、禁止空 catch 块
- 禁止在生产代码中使用 `console.log`
- 详细规范见 `.studio/project/code-style.md`

## 测试指令

- 测试结构遵循 AAA 模式（Arrange-Act-Assert）
- 每个测试独立运行，不依赖执行顺序
- 测试结果稳定可重复
- 逻辑故事单元测试覆盖率 >= 80%
- 详细规范见 `.studio/project/testing-instructions.md`

## Agent 角色与协作

40 个 Agent 分三层架构：

### 第一层：总监层（3 个）
- `product-director`（产品总监）：产品愿景、需求方向、用户体验目标
- `chief-architect`（首席架构师）：技术决策、架构选型、技术风险评估
- `project-manager`（项目经理）：Sprint 规划、里程碑追踪、风险管理

### 第二层：部门负责人层（7 个）
- `system-analyst`、`lead-developer`、`tech-architect`、`ux-design-lead`、`qa-lead`、`devops-lead`、`security-lead`

### 第三层：专家层（30 个）
涵盖技术栈专家（React/Vue/Angular/Node/Python/Java/Go 等）、工程专家和领域专家。

完整花名册见 `.studio/agents/_roster.md`

### 协调规则
1. 纵向委派：上层可向下层委派，但不可越级
2. 横向协商：同层通过协商解决跨领域问题
3. 冲突解决：由上一层级仲裁
4. 变更传播：架构变更须传播到所有受影响方
5. 禁止单方面跨领域变更

详细规则见 `.studio/docs/coordination-rules.md`

## 可用技能（74 个）

通过 `/` 命令调用，按开发阶段分类：

- **入门导航**：`/start`、`/help`、`/onboard`、`/project-stage-detect`、`/adopt`
- **需求阶段**：`/brainstorm`、`/setup-stack`、`/product-concept`、`/map-modules`、`/requirement-spec` 等
- **架构设计**：`/create-architecture`、`/architecture-decision`、`/architecture-review` 等
- **开发阶段**：`/dev-story`、`/code-review`、`/story-done`、`/sprint-status` 等
- **测试优化**：`/perf-profile`、`/security-audit`、`/smoke-check` 等
- **发布部署**：`/gate-check`、`/release-checklist`、`/launch-checklist` 等
- **平台工具**：`/sync-platforms`、`/platform-check`

完整目录见 `.studio/skills/_catalog.yaml`

## 提交规范

遵循 Conventional Commits 格式：
- `feat:` 新功能
- `fix:` 缺陷修复
- `docs:` 文档变更
- `refactor:` 代码重构
- `test:` 测试相关
- `chore:` 构建/工具变更

Subject 不超过 50 字。禁止向 main/master/release/* 分支直接推送。

详细规范见 `.studio/project/pr-guidelines.md`

## 安全注意事项

- 禁止硬编码密钥、令牌等敏感信息
- `.env` 文件不纳入版本控制
- 禁止 `rm -rf`、`git push --force` 等危险操作
- 禁止读取 `.secrets/`、`*.key`、`*.pem` 文件
- 数据库禁止在生产环境执行破坏性操作（DROP/DELETE/TRUNCATE）

详细规范见 `.studio/project/security.md`

## 上下文管理

- 关键信息持久化到文件系统，不依赖对话历史
- 会话状态文件：`production/session-state/active.md`
- 会话日志归档：`production/session-logs/`
- 上下文压缩前保存进度到 `active.md`
- 上下文压缩后从 `active.md` 恢复状态

详细策略见 `.studio/docs/context-management.md`

## 详细文档

- 目录结构：`.studio/docs/directory-structure.md`
- 协调规则：`.studio/docs/coordination-rules.md`
- 编码规范：`.studio/docs/coding-standards.md`
- 上下文管理：`.studio/docs/context-management.md`
- 技术偏好：`.studio/docs/technical-preferences.md`
- 审查工作流：`.studio/docs/review-workflow.md`
- Hook 参考：`.studio/docs/hooks-reference.md`
- 规则参考：`.studio/docs/rules-reference.md`
- 技能参考：`.studio/docs/skills-reference.md`
- 系统前置条件：`.studio/docs/setup-requirements.md`