# /team-frontend — 编排前端团队协作管线，协调 UX 设计负责人、前端架构师、框架专家、测试工程师与代码审查员按序交付前端功能。

## 触发方式
在 Cascade 对话框中输入 /team-frontend 触发此工作流。

## 工作流内容
# team-frontend — 前端团队编排

## 技能目的

编排完整的前端团队协作管线，按既定顺序协调前端相关 Agent 协同工作，将一项功能描述从界面规范推进到通过代码审查的可交付状态。本技能负责调度各 Agent、传递中间产物、收集反馈并在阶段间执行质量门禁。

## 参数说明

- `[功能描述]`：待实现的前端功能描述，可为用户故事标题、Epic 名称或自然语言功能说明。

## 分阶段工作流

### 阶段 1：加载上下文与确认范围

- **输入**：`[功能描述]` 参数
- **处理**：
  1. 使用 Read 读取 `docs/srs/*.md` 匹配功能描述对应的需求规格
  2. 使用 Glob 查找 `docs/design/ux-spec*.md` 获取已有 UX 规范
  3. 使用 Read 读取 `docs/technical-preferences.md` 确认前端框架（React / Vue / Angular）
  4. 通过 AskUserQuestion 与用户确认功能边界与验收标准
- **输出**：功能上下文 + 团队编排计划

### 阶段 2：ux-design-lead 生成界面规范

- **输入**：功能上下文
- **处理**：使用 Task 分派 ux-design-lead Agent，产出界面规范、交互流程、状态定义与无障碍要求
- **输出**：界面规范文档，写入 `docs/design/ux-spec-[功能].md`

### 阶段 3：frontend-architect 设计前端架构

- **输入**：界面规范
- **处理**：使用 Task 分派 frontend-architect Agent，设计组件结构、状态管理、路由划分与数据流
- **输出**：前端架构方案

### 阶段 4：react/vue/angular-specialist 实现功能

- **输入**：前端架构方案
- **处理**：使用 Task 分派对应框架专家 Agent（依据技术栈选择），实现组件、页面与交互逻辑
- **输出**：前端代码实现

### 阶段 5：test-engineer 编写与执行测试

- **输入**：前端代码实现
- **处理**：使用 Task 分派 test-engineer Agent，编写单元测试、组件测试与端到端测试，执行并收集结果
- **输出**：测试用例 + 测试执行报告

### 阶段 6：code-reviewer 代码审查

- **输入**：代码实现 + 测试报告
- **处理**：使用 Task 分派 code-reviewer Agent，按编码规范审查代码质量、可维护性与规范一致性
- **输出**：代码审查报告

### 阶段 7：汇总与门禁

- **输入**：全部阶段产物
- **处理**：
  1. 汇总各阶段产出与反馈
  2. 检查是否所有阶段均通过质量门禁
  3. 通过 AskUserQuestion 向用户报告结果并确认是否需要返工
  4. 使用 Write 写入编排报告至 `docs/reports/team-frontend-[功能].md`
- **输出**：团队编排报告

## 协作协议引用

- 遵循 `docs/examples/COLLABORATIVE-DESIGN-PRINCIPLE.md` 用户驱动协作协议
- 各 Agent 之间通过 Task 工具分派，中间产物以文件形式传递
- 每个阶段切换前需向用户展示当前阶段产出并获取确认
- 阶段间设置质量门禁，未通过则回退至上一阶段

## 推荐下一步

前端功能交付后，使用 `/team-qa` 执行完整 QA 周期验证。性能问题使用 `/team-polish` 进行优化打磨。安全相关关注使用 `/team-security` 审查。发布阶段使用 `/team-release` 编排发布流程。