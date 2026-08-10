<!-- Software Engineering Studios -->
# Agent 协调规则

本文件定义 Software Engineering Studios 中 40 个 Agent 之间的协作协议、模型分配和并行任务处理规则。所有 Agent 必须严格遵守本规则。

## 1. 五条核心规则

### 规则 1：纵向委派

上层 Agent 可向下层 Agent 委派任务，但必须遵守层级链，不得越级向下委派。

- **总监层 → 部门负责人层**：允许。例如 `product-director` 委派 `system-analyst` 进行需求拆解。
- **部门负责人层 → 专家层**：允许。例如 `lead-developer` 委派 `react` 专家开发组件。
- **总监层 → 专家层**：禁止越级。如需专家层服务，应通过对应部门负责人委派。
- **专家层之间**：平级，不可相互委派，只能横向协商。

委派时必须明确：
- 任务目标与验收标准
- 预期产出物及其存放路径
- 截止时间或优先级
- 相关上下文文件引用

### 规则 2：横向协商

同层 Agent 之间通过协商解决跨领域问题，任何一方无单方面决定权。

- 跨领域变更必须由涉及的所有 Agent 共同参与讨论
- 达成共识后由主责 Agent 执行，协作 Agent 审查
- 协商过程应记录在 `production/session-state/active.md` 中
- 若协商无法达成一致，提交至上一层级仲裁

示例：前端组件（`react` 专家）需要调用新 API（`api-designer`），两者需协商接口契约，任何一方不得单方面定义接口。

### 规则 3：冲突解决

当 Agent 之间出现意见冲突时，按以下层级仲裁：

1. **同层协商优先**：冲突双方首先尝试通过协商解决
2. **上级仲裁**：协商失败后提交至上一层级 Agent 仲裁
   - 专家层冲突 → 部门负责人层仲裁
   - 部门负责人层冲突 → 总监层仲裁
   - 总监层冲突 → 用户决策
3. **用户最终决策**：总监层无法解决的冲突提交给用户

冲突解决记录必须包含：
- 冲突描述与涉及 Agent
- 各方立场与理由
- 仲裁结果与依据
- 后续行动项

### 规则 4：变更传播

任何架构或设计变更必须通过 `/propagate-design-change` 传播到所有受影响的 Agent 和文档。

变更传播流程：
1. 发起变更的 Agent 创建变更提案（含变更内容、影响范围、受影响文件列表）
2. 通过 `/propagate-design-change` 命令启动传播
3. 所有受影响 Agent 收到通知并评估影响
4. 各 Agent 更新各自领域的文档和代码
5. 变更完成后更新 `production/session-state/active.md` 中的架构决策记录

**禁止**未经传播流程直接修改受影响文件。

**资产扩展传播**：新增 Agent/Skill/Rule 后，必须通过 `/sync-platforms` 传播到所有平台。新增资产的变更传播遵循与架构变更相同的流程。详见 `docs/extension-mechanism.md`。

### 规则 5：禁止单方面跨领域变更

任何 Agent 不得单方面修改不属于自己领域的文件或配置，必须通过协商和审批流程。

- 前端 Agent 不得直接修改后端代码或 API 定义
- 后端 Agent 不得直接修改前端组件或 UX 设计
- 任何 Agent 不得修改 `.claude/` 下的配置文件（由 `chief-architect` 审批）
- 跨领域变更必须遵循协作协议：**提问 → 选项 → 决策 → 草稿 → 审批**

## 2. 模型层级分配表

根据任务复杂度和风险级别分配不同的模型层级，以平衡成本与质量：

| 模型层级 | 适用场景 | 典型任务 | 风险等级 |
| --- | --- | --- | --- |
| **Haiku**（快速轻量） | 只读检查、状态查询、简单格式化 | `sprint-status`、`story-readiness`、`scope-check`、`changelog`、`patch-notes`、`onboard`、`project-stage-detect`、`help` | 低 |
| **Sonnet**（标准能力） | 实现与设计任务的默认选择 | 组件开发、API 实现、测试编写、文档撰写、大多数斜杠命令 | 中 |
| **Opus**（深度推理） | 多文档综合、高风险决策、复杂审查 | `gate-check`、`review-all-srs`、`architecture-review`、跨模块架构变更、安全审计综合报告 | 高 |

**分配原则：**
- 默认使用 Sonnet
- 仅在明确标注为高风险或需要跨多文档综合分析时升级至 Opus
- 简单只读或状态查询任务降级至 Haiku 以节省成本
- 模型分配在 `settings.json` 中配置，可在 `settings.local.json` 中覆盖

## 3. 子 Agent 与 Agent 团队说明

### 3.1 子 Agent（Subagent）

子 Agent 是由主 Agent 通过 `Task` 工具派发的独立执行单元，具有以下特征：

- 在独立的上下文窗口中运行
- 执行完毕后返回结果给主 Agent
- 无法访问主 Agent 的对话历史
- 适合执行边界清晰、可独立完成的单一任务

典型用法：
```
主 Agent → 派发子 Agent 执行单元测试 → 收集结果 → 继续后续工作
```

### 3.2 Agent 团队（Agent Team）

Agent 团队是通过团队编排命令（如 `/team-frontend`、`/team-backend`）协调的多 Agent 协作组，具有以下特征：

- 多个 Agent 围绕共同目标协同工作
- 共享部分上下文（通过文件系统）
- 由一个牵头 Agent 负责协调
- 适合处理需要多领域协作的复杂任务

典型用法：
```
/team-frontend → ux-design-lead 牵头 → react/vue/angular 专家协作 → 前端架构师审查
```

### 3.3 选择指南

| 场景 | 选择 | 原因 |
| --- | --- | --- |
| 单一明确任务 | 子 Agent | 上下文隔离，执行效率高 |
| 多领域协作 | Agent 团队 | 共享上下文，协调便利 |
| 需要深度审查 | 子 Agent（Opus） | 独立上下文避免偏见 |
 | 并行独立任务 | 多个子 Agent | 可同时执行 |

## 4. 并行任务协议

### 4.1 独立任务同时发起

当多个任务之间无依赖关系时，主 Agent 应同时发起多个子 Agent 调用，而非串行执行。

- 在单个消息中发起多个独立的 `Task` 调用（最多 5 个并行）
- 每个子 Agent 在独立上下文中执行
- 适用于：不同模块的独立开发、多文件并行审查、多维度独立测试

示例场景：
- 同时审查前端代码、后端代码、测试代码（3 个子 Agent 并行）
- 同时编写 3 个独立模块的需求规格

### 4.2 收集所有结果再进入依赖阶段

所有并行任务必须全部完成后，才能进入依赖阶段。

- 主 Agent 等待所有子 Agent 返回结果
- 收集所有产出物并验证完整性
- 若任一子 Agent 失败，评估是否影响依赖阶段
- 所有结果就绪后，整合并传入依赖阶段

**禁止**在并行任务部分完成时就启动依赖阶段（除非遵循 4.4 的部分报告规则）。

### 4.3 BLOCKED 立即上报

当子 Agent 遇到无法自行解决的阻塞时，必须立即上报而非等待超时。

- 子 Agent 在输出中标注 `BLOCKED: [原因]`
- 主 Agent 收到 BLOCKED 后：
  1. 评估阻塞是否可由主 Agent 解决
  2. 若不可解决，上报至更上层或用户
  3. 决定是否继续其他并行任务
- 阻塞记录写入 `production/session-state/active.md`

### 4.4 产出部分报告

当并行任务中部分完成、部分阻塞时，可产出部分报告以推进工作。

- 已完成的任务结果整合为部分报告
- 阻塞任务标注为 `PENDING: [原因]`
- 部分报告可用于：
  - 推进不依赖阻塞结果的后续工作
  - 向用户展示进展并请求决策
  - 识别是否需要调整计划
- 部分报告必须明确标注覆盖范围与缺失部分

### 4.5 Worktree 隔离执行

当 4.1 定义的"独立任务同时发起"场景触发时，应使用 git worktree 为每个并行子 Agent 创建独立工作树，实现代码隔离。

- **隔离原则**：每个并行子 Agent 在独立 worktree 中工作，与上下文隔离原则一致（子 Agent 在独立上下文窗口中运行）
- **Worktree 创建**：运行 `/code-management worktree` 创建工作树，路径格式 `../worktrees/[分支名]`
- **Worktree 映射**：在 `active.md` 中记录 worktree 路径 → 分支名 → 任务名的映射关系
- **Worktree 清理**：与 4.2"收集所有结果再进入依赖阶段"关联，所有 worktree 合并后再进入依赖阶段，合并后立即清理 worktree
- **完整规范**：详见 `docs/code-management-workflow.md` 第 2 节

## 5. 协作协议

所有 Agent 协作遵循用户驱动的协作协议：

**提问 → 选项 → 决策 → 草稿 → 审批**

- Agent 在使用 Write/Edit 工具前必须询问："我可以将此写入 [文件路径] 吗？"
- Agent 在请求审批前必须展示草稿或摘要
- 多文件修改需要针对完整变更集的明确审批
- 未经用户指示不得进行提交

### 5.1 动态扩展协议

Software Engineering Studios 支持根据项目需求动态新增专家 Agent、技能和路径规则。扩展流程遵循以下协议：

- **新增专家 Agent**：总监层或部门负责人层可提案，chief-architect 审核，用户最终批准。详见 `/create-agent` 工作流。
- **新增 Skill**：部门负责人层可提案（专家层经部门负责人授权），对应部门负责人初审 + chief-architect 终审，用户最终批准。详见 `/create-skill` 工作流。
- **新增 Rule**：部门负责人层可提案（专家层经部门负责人授权），tech-architect 初审 + chief-architect 终审，用户最终批准。详见 `/create-rule` 工作流。
- **注册与追踪**：所有新增资产必须注册到 `asset-registry.yaml` 并记录到 `asset-changelog.md`。
- **审核门禁**：新增资产须通过 CA-ASSET 门禁（由 chief-architect 执行）。
- **变更传播**：注册后必须通过 `/sync-platforms` 传播到所有平台。

完整规范详见 `docs/extension-mechanism.md`。

## 6. 审计与记录

所有 Agent 调用通过 Hook 自动记录审计日志：

- `log-agent.sh`（SubagentStart）— 记录子 Agent 启动
- `log-agent-stop.sh`（SubagentStop）— 记录子 Agent 完成
- 审计日志存放于 `production/session-logs/` 目录

审计记录包含：调用时间、Agent 名称、任务描述、模型层级、执行状态、产出物路径。