# 代码管理工作流实施计划

## 概述

为 Software Engineering Studios 项目增加完整的代码管理工作流，合理利用 git worktree 等工具进行多任务并行，在项目产出或变更时进行代码提交，阶段性地推送或 PR 代码，推送或 PR 前完成 review 或门禁审查，保证代码仓库各分支整洁清晰合理，必要时可 rebase 合并分支。

本计划遵循三层架构原则（`.studio/` 规范源层 → `tools/adapters/` 适配层 → 各平台输出层），所有新增资产通过动态扩展机制注册，最终通过 `sync-all.sh` 同步到各平台。

---

## 当前状态分析

### 现有相关资产

| 资产 | 路径 | 现状 |
|------|------|------|
| PR/提交规范 | `.studio/project/pr-guidelines.md` | 已有分支命名、Conventional Commits、审查流程、门禁要求，但无 worktree、提交时机、rebase 策略 |
| 提交验证 Hook | `.studio/hooks/validate-commit.sh` | PreToolUse Hook，检查硬编码/TODO/JSON/文档章节 |
| 推送验证 Hook | `.studio/hooks/validate-push.sh` | PreToolUse Hook，对受保护分支**仅警告不阻断**，未与 review/gate-check 联动 |
| 审查工作流 | `.studio/docs/review-workflow.md` | 定义 full/lean/solo 三种审查模式，7 阶段审查点，无推送/PR 门禁联动 |
| 协调规则 | `.studio/docs/coordination-rules.md` | 第 4 节定义并行任务协议，无 worktree 隔离执行规范 |
| 上下文管理 | `.studio/docs/context-management.md` | 会话状态文件 active.md 管理，无与提交时机的关联 |
| 技能目录 | `.studio/skills/_catalog.yaml` | 79 个技能，无 git-worktree 或代码管理相关技能 |
| 工作流目录 | `.studio/docs/workflow-catalog.yaml` | 7 阶段管线，无代码管理步骤 |
| dev-story 技能 | `.windsurf/workflows/dev-story.md` | 明确"未经用户指示不得执行 Git 提交"，但无定义何时应提交 |
| Agent | `.studio/agents/lead-developer.md` | 开发负责人，职责含"开发流程优化"，无代码管理工具关联 |

### 缺失分析

1. **无 git worktree 使用规范** — 无法支持多任务并行时的代码隔离
2. **提交时机未定义** — dev-story 仅说"不得自主提交"，但无明确何时应建议提交
3. **推送/PR 节奏未关联工作流阶段** — 无阶段与推送时机的映射
4. **无分支生命周期管理** — 缺少 stale 分支检测、清理规则
5. **无 rebase 策略** — 缺少何时 rebase、安全规则
6. **validate-push.sh 仅警告不阻断** — 未与 review/gate-check 联动，受保护分支保护力度不足
7. **无代码管理工作流技能** — 无执行入口

---

## 需求到资产的映射

| 用户需求 | 解决方案 |
|----------|----------|
| 1. git worktree 多任务并行 | 新增 `code-management-workflow.md` 文档 + `code-management` 技能 + 修改 `coordination-rules.md` |
| 2. 明确提交时机 | 新增 `code-management-workflow.md` 提交时机章节 + 修改 `pr-guidelines.md` + 修改 `dev-story` 技能 |
| 3. 阶段性推送或 PR | 修改 `workflow-catalog.yaml` + 新增 `code-management` 技能 push 子流程 |
| 4. 推送前完成 review/门禁 | 修改 `validate-push.sh`（阻断升级）+ 修改 `review-workflow.md`（门禁联动）|
| 5. 分支整洁清晰 | 新增 `code-management-workflow.md` 分支生命周期章节 + `code-management` 技能 cleanup 子流程 |
| 6. rebase 合并分支 | 新增 `code-management-workflow.md` rebase 策略章节 + `code-management` 技能 rebase 子流程 |

---

## 拟议变更

### 一、新增文件

#### 新增文件 1：`.studio/docs/code-management-workflow.md`

**用途：** 代码管理工作流的核心规范文档。

**内容要点（7 个章节）：**

1. **概述与背景** — 定义目标：在 AI Agent 多任务并行场景下保持代码仓库的整洁性、可追溯性和质量可控性
2. **Git Worktree 并行工作规范**
   - Worktree 目录约定：`../worktrees/[分支名]/`（与主仓库同级，避免嵌套）
   - 创建时机：`coordination-rules.md` 第 4 节"独立任务同时发起"场景触发时
   - 与 Agent 映射：每个并行子 Agent 在独立 worktree 中工作，与上下文隔离原则一致
   - 生命周期：创建 → 工作 → 提交 → 合并 → 清理
   - 清理规则：分支合并后立即移除对应 worktree
   - 命令示例：`git worktree add` / `git worktree remove`
3. **提交时机定义** — 将"未经用户指示不得执行 Git 提交"细化为明确时机表
   - 阶段产出提交：每个工作流阶段完成必需产出物后
   - 逻辑单元提交：一个完整逻辑变更单元完成后（验收标准实现+测试通过）
   - 检查点提交：通过 review 或 gate-check 后
   - 会话保存提交：上下文压缩前、会话结束前
   - 禁止提交时机：代码未通过本地 lint、测试失败、含调试代码、含硬编码密钥
   - 提交粒度原则：一个提交对应一个逻辑变更
4. **推送与 PR 节奏** — 与工作流阶段关联的推送/PR 时机表
   - 阶段→推送时机→PR 类型→前置审查的映射表
   - 推送前检查清单（与 `pr-guidelines.md` 第 6 节集成）
   - PR 描述模板要求：关联 Story ID、验收标准对照、测试证据摘要
5. **分支生命周期管理**
   - 分支创建规则（与 `pr-guidelines.md` 第 2 节一致）
   - Stale 分支检测：超过 7 天无提交标注为 stale
   - 清理规则：已合并分支立即删除、stale 分支 3 天无活动建议删除、受保护分支永不删除
   - 定期整理：每个 Sprint 结束时执行分支清理
6. **Rebase 策略**
   - 触发条件：feature 分支落后于 main 超过 5 个提交时
   - 安全规则：仅对未推送的本地提交执行、已推送分支禁止 rebase（除非个人分支且已通知协作者）
   - Rebase 后处理：解决冲突 → 运行测试 → 强制推送（仅个人分支）
   - 禁止 rebase 受保护分支（main、release/*）
   - 交互式 rebase 用于提交历史整理
7. **与现有工作流的集成**
   - 与 `coordination-rules.md` 第 4 节并行任务协议的集成
   - 与 `review-workflow.md` 审查模式的集成
   - 与 `context-management.md` 会话状态管理的集成
   - 与 `pr-guidelines.md` PR 规范的集成

#### 新增文件 2：`.studio/skills/code-management/SKILL.md`

**用途：** 代码管理操作技能，提供 git worktree 设置、提交时机判断、推送/PR 准备、分支清理和 rebase 的执行入口。

**frontmatter：**
```yaml
name: code-management
description: "代码管理工作流技能。管理 git worktree 并行工作、提交时机判断、推送/PR 准备、分支生命周期管理和 rebase 策略执行，确保代码仓库整洁有序。"
license: MIT
metadata:
  model: sonnet
  argument-hint: "[操作类型: worktree|commit|push|cleanup|rebase]"
  user-invocable: true
  allowed-tools: [Read, Glob, Grep, Write, Bash, AskUserQuestion]
```

**5 个子流程（按操作类型路由）：**

- **worktree 子流程**：读取 active.md 确认并行任务数 → 生成 worktree 路径和分支名 → 执行 `git worktree add` → 记录映射到 active.md → 提示切换
- **commit 子流程**：读取提交时机表 → 检查是否满足时机 → 运行 validate-commit.sh 预检 → 检查暂存区 → 生成 Conventional Commits 提交信息草案 → 获用户审批后提交
- **push 子流程**：检查当前分支是否受保护 → 检查审查状态（code-review/gate-check）→ 检查 PR 前置门禁 → 生成 PR 描述草案 → 获用户审批后推送或创建 PR
- **cleanup 子流程**：列出所有分支 → 标注已合并和 stale 分支 → 生成清理建议 → 获用户审批后删除分支和移除 worktree
- **rebase 子流程**：检查与 main 的提交差距 → 判断 rebase 条件 → 检查未推送提交 → 执行 rebase 处理冲突 → 运行测试验证

#### 新增文件 3：`.studio/registry/proposals/skill-001-code-management-proposal.md`

**用途：** 通过动态扩展机制创建 `code-management` 技能的提案文档，遵循 `asset-proposal.md` 模板格式。

**内容要点：**
- 需求背景：当前无代码管理技能，dev-story 无提交时机定义
- 资产概述：技能名 `code-management`，分类"开发"，模型 sonnet
- 与现有资产关系：与 dev-story（补充提交时机）、code-review（推送前置）、gate-check（发布前置）协作
- 影响范围：需更新 `_catalog.yaml`、`skills-reference.md`、`workflow-catalog.yaml`
- 审核标准清单

#### 新增文件 4（可选）：`.studio/hooks/validate-worktree.sh`

**用途：** PreToolUse Hook，检测并行任务时是否使用 worktree 隔离，仅警告不阻断。

> 初期可先依赖文档规范和技能引导，验证可行后再实施。

---

### 二、修改现有文件

#### 修改 1：`.studio/project/pr-guidelines.md`

- **第 2 节后新增 2.1 节"分支生命周期"**：创建、活跃、合并、清理的完整生命周期；stale 分支检测和清理规则；Sprint 结束时分支整理流程
- **第 2 节后新增 2.2 节"Git Worktree 使用规范"**：目录约定、创建和清理命令、与并行任务协议关联
- **新增第 7 节"提交时机"**：阶段产出提交时机表、逻辑单元提交原则、禁止提交时机、提交粒度指南、与 dev-story 阶段关联
- **新增第 8 节"Rebase 策略"**：触发条件、安全规则、禁止场景、交互式 rebase 场景
- **第 3.3 节新增推送前置检查**：推送前必须确认审查状态、审查报告路径检查

#### 修改 2：`.studio/hooks/validate-push.sh`

- **受保护分支从"仅警告"升级为"阻断"**：对 `main`、`master`、`develop` 分支直接推送从 `exit 0` 改为 `exit 2`（阻断），输出明确阻断原因和替代方案
- **新增审查状态检查**：对 `feature/*`、`fix/*`、`hotfix/*` 分支推送时检查是否存在对应审查报告，未找到时输出警告（不阻断，提示运行 `/code-review`）
- **新增分支名规范检查**：检查推送分支名是否符合 `pr-guidelines.md` 第 2 节命名规范，不符合时输出警告
- **更新文件头注释**：说明从"仅警告"升级为"受保护分支阻断+其他分支警告"

#### 修改 3：`.studio/skills/dev-story/SKILL.md`（对应 `.windsurf/workflows/dev-story.md`）

- **阶段 6"收集总结"新增提交时机指引**：实现总结写入后，根据 `code-management-workflow.md` 提交时机表，可向用户建议执行提交（保持"不自主执行"原则）
- **阶段 7"更新会话状态"新增提交建议**：会话状态更新后提示当前满足"会话保存提交"时机，推荐运行 `/code-management commit`
- **推荐下一步新增**：实现完成后运行 `/code-management push`；并行开发场景运行 `/code-management worktree`

#### 修改 4：`.studio/skills/_catalog.yaml`

- `total` 字段从 `79` 改为 `80`
- `开发` 分类新增：`- {name: code-management, command: /code-management, model: sonnet, description: "代码管理工作流"}`

#### 修改 5：`.studio/docs/workflow-catalog.yaml`

- 阶段 5"开发阶段"在 `code-review` 之后、`story-done` 之前新增 `code-management` 步骤（可选、可重复）
- 阶段 5 在 `retrospective` 之后新增 `branch-cleanup` 步骤（可选，Sprint 结束时调用 `/code-management cleanup`）

#### 修改 6：`.studio/docs/coordination-rules.md`

- 第 4 节新增 4.5 节"Worktree 隔离执行"：独立任务同时发起时应使用 git worktree 为每个并行子 Agent 创建独立工作树；与子 Agent 上下文隔离的对应关系；Worktree 清理与"收集所有结果再进入依赖阶段"的关联；引用 `code-management-workflow.md` 第 2 节

#### 修改 7：`.studio/docs/review-workflow.md`

- 新增第 6 节"推送/PR 门禁联动"：推送前置审查要求表（分支类型→审查要求→审查模式映射）；feature 分支推送前必须完成 code-review；release 分支推送前必须完成 gate-check；hotfix 分支推送后必须补完成 code-review；与 validate-push.sh 的联动关系
- 第 2.5 节"开发阶段"审查点新增推送关联：审查通过后方可推送/创建 PR

#### 修改 8：`.studio/registry/asset-registry.yaml`

- `skills` 列表新增 `code-management` 条目（id: skill-001, status: registered）
- `statistics` 更新：`total_skills` 递增，`total_dynamic_additions` 从 0 改为 1

#### 修改 9：`.studio/registry/asset-changelog.md`

- 新增变更条目：`[2026-07-08] created — skill/code-management`，记录变更类型、资产类型、资产名称、提案 Agent、审核者、批准者、变更摘要、关联文件、关联提案编号

#### 修改 10：`.studio/hooks/_manifest.md`

- 更新 `validate-push.sh` 行的降级策略说明：从"禁止向 main/master/release/* 分支直接推送"改为"禁止向 main/master/develop 分支直接推送（阻断）；feature/fix/hotfix 分支推送前建议完成代码审查（警告）"

#### 修改 11：`.studio/docs/skills-reference.md`

- 新增 `code-management` 技能条目（开发阶段技能部分）

#### 修改 12：`.studio/agents/lead-developer.md`

- `skills` 列表新增 `code-management`
- 关键职责第 5 项"开发流程优化"新增：负责维护 `code-management-workflow.md` 规范，确保 git worktree、提交时机、分支清理、rebase 策略一致执行

---

## 资产注册安排

### 动态扩展六阶段工作流

| 阶段 | 执行者 | 产出 | 文件位置 |
|------|--------|------|----------|
| 1. 提案 | lead-developer | 提案文档 | `.studio/registry/proposals/skill-001-code-management-proposal.md` |
| 2. 草稿 | lead-developer | 技能定义草稿 | `.studio/skills/code-management/SKILL.md` |
| 3. 审核 | chief-architect | 审核报告 | `.studio/registry/reviews/skill-code-management-review.md` |
| 4. 批准 | 用户 | 批准决定 | 更新 asset-registry.yaml status → approved |
| 5. 注册 | lead-developer | 注册完成 | 更新 `_catalog.yaml`、`skills-reference.md`、`workflow-catalog.yaml`、`asset-registry.yaml`、`asset-changelog.md` |
| 6. 同步 | 用户指示后 | 平台同步 | 运行 `/sync-platforms` |

### 非动态扩展资产说明

以下文件为规范文档和规范修改，直接在 `.studio/` 对应目录维护：

- `.studio/docs/code-management-workflow.md`（规范文档，类似 `review-workflow.md`）
- `.studio/project/pr-guidelines.md`（规范修改）
- `.studio/hooks/validate-push.sh`（Hook 修改）
- `.studio/skills/dev-story/SKILL.md`（技能修改）
- `.studio/docs/coordination-rules.md`（规范修改）
- `.studio/docs/review-workflow.md`（规范修改）
- `.studio/docs/workflow-catalog.yaml`（工作流修改）
- `.studio/agents/lead-developer.md`（Agent 修改）

---

## 实施步骤顺序

### 步骤 1：创建提案文档（动态扩展阶段 1）
- 创建 `.studio/registry/proposals/skill-001-code-management-proposal.md`
- 更新 `asset-registry.yaml` 中 skills 列表新增条目（status: proposed）
- **审批点**：向用户展示提案，获得写入许可

### 步骤 2：创建核心规范文档
- 创建 `.studio/docs/code-management-workflow.md`（7 个章节）
- 包含标准元数据（状态/作者/更新日期）
- **审批点**：向用户展示文档草稿，获得写入许可

### 步骤 3：创建技能定义草稿（动态扩展阶段 2）
- 创建 `.studio/skills/code-management/SKILL.md`
- 定义 5 个子流程
- 更新 `asset-registry.yaml`（status: draft）
- **审批点**：向用户展示技能草稿

### 步骤 4：提交审核（动态扩展阶段 3）
- 由 chief-architect 执行 `/asset-review skill code-management`
- 生成审核报告至 `.studio/registry/reviews/skill-code-management-review.md`
- 更新 `asset-registry.yaml`（status: reviewed）
- **审批点**：审核报告展示给用户

### 步骤 5：用户批准（动态扩展阶段 4）
- 向用户展示审核报告，获得最终批准
- 更新 `asset-registry.yaml`（status: approved）

### 步骤 6：修改现有文件（动态扩展阶段 5 — 注册）
以下修改作为完整变更集向用户申请批量审批：
1. 修改 `pr-guidelines.md` — 新增分支生命周期、worktree、提交时机、rebase 章节
2. 修改 `validate-push.sh` — 升级受保护分支为阻断、新增审查检查
3. 修改 `dev-story/SKILL.md` — 新增提交时机指引
4. 修改 `_catalog.yaml` — 注册新技能、更新 total
5. 修改 `workflow-catalog.yaml` — 新增 code-management 步骤
6. 修改 `coordination-rules.md` — 新增 worktree 隔离执行章节
7. 修改 `review-workflow.md` — 新增推送/PR 门禁联动章节
8. 修改 `asset-registry.yaml` — 新增技能条目、更新统计
9. 修改 `asset-changelog.md` — 记录变更
10. 修改 `_manifest.md` — 更新 validate-push 说明
11. 修改 `skills-reference.md` — 新增技能引用
12. 修改 `lead-developer.md` — 新增技能关联

### 步骤 7：平台同步（动态扩展阶段 6）
- 用户指示后运行 `/sync-platforms`
- 运行 `/platform-check` 验证各平台配置一致性

### 步骤 8：（可选）新增 validate-worktree.sh Hook
- 如规范验证可行，创建 Hook 脚本
- 更新 `_manifest.md`
- 同步到各平台

---

## 假设与决策

### 假设

1. 项目运行在支持 git worktree 的环境（Git 2.6+）
2. Windows 环境下 worktree 路径兼容性通过跨平台路径约定解决
3. validate-push.sh 审查报告路径使用 Glob 模式匹配，不硬编码
4. dev-story "不得自主提交"原则保持不变，新工作流仅提供"建议提交"
5. 已有项目分支命名可能不符合规范，validate-push.sh 对已有分支仅警告不阻断

### 决策

1. **规范文档 vs 技能分离**：`code-management-workflow.md` 作为规范文档（类似 `review-workflow.md`），`code-management` 技能作为执行入口，职责分离
2. **受保护分支阻断升级**：`validate-push.sh` 从"仅警告"升级为"阻断"，增强保护力度
3. **审查状态检查不阻断**：feature/fix/hotfix 分支推送时审查报告检查为"警告不阻断"，避免误阻断合法推送
4. **可选 Hook 延后实施**：`validate-worktree.sh` 初期依赖文档规范，验证可行后实施
5. **技能模型选择 sonnet**：代码管理需要 Git 操作和上下文判断，属于标准能力范畴

---

## 验证步骤

### 验证 1：技能完整性验证
- 运行 `/skill-test code-management` 验证技能定义符合 schema
- 检查 frontmatter 字段完整性
- 检查 Body 五章节完整性

### 验证 2：资产注册一致性验证
- 检查 `asset-registry.yaml` 中条目状态为 `registered`
- 检查 `asset-changelog.md` 有对应变更记录
- 检查 `_catalog.yaml` 的 `total` 与实际技能数量一致（80）
- 检查 `skills-reference.md` 包含新技能条目

### 验证 3：规范文档完整性验证
- 检查 `code-management-workflow.md` 包含元数据和全部 7 个章节
- 检查 `pr-guidelines.md` 新增章节与现有章节不冲突
- 检查 `coordination-rules.md` 新增 4.5 节与第 4 节其他子节风格一致
- 检查 `review-workflow.md` 新增第 6 节与现有审查模式表一致

### 验证 4：Hook 行为验证
- 模拟向 `main` 分支推送，验证 `validate-push.sh` 返回 `exit 2`（阻断）
- 模拟向 `feature/test` 分支推送（无审查报告），验证输出警告且 `exit 0`
- 模拟向 `feature/test` 分支推送（有审查报告），验证无警告且 `exit 0`
- 模拟不合规分支名推送，验证输出命名规范警告

### 验证 5：工作流集成验证
- 检查 `workflow-catalog.yaml` 中开发阶段新增步骤的 artifact 定义
- 检查 `dev-story` 技能中提交时机指引与 `code-management-workflow.md` 第 3 节一致
- 检查 `review-workflow.md` 推送/PR 门禁联动表与 `validate-push.sh` 检查逻辑一致

### 验证 6：平台同步验证
- 运行 `bash tools/adapters/sync-all.sh`
- 运行 `/platform-check` 验证各平台配置一致性
- 检查 `.claude/skills/`、`.cursor/skills/`、`.trae/` 等目录包含新技能

### 验证 7：端到端场景验证
1. **并行场景**：active.md 记录 2 个并行任务 → 运行 `/code-management worktree` 创建 2 个工作树 → 子 Agent 在各自 worktree 工作 → 合并后清理
2. **提交场景**：dev-story 完成 → 提示提交时机 → 运行 `/code-management commit` → validate-commit.sh 验证通过 → 提交成功
3. **推送场景**：运行 `/code-management push` → 检查审查报告存在 → 生成 PR 描述 → validate-push.sh 验证通过 → 推送成功
4. **清理场景**：Sprint 结束 → 运行 `/code-management cleanup` → 列出 stale 分支 → 获批准后删除 → 清理 worktree
5. **Rebase 场景**：feature 分支落后 main 7 个提交 → 运行 `/code-management rebase` → 检查条件 → 执行 rebase → 运行测试 → 通过