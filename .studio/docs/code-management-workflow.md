<!-- Software Engineering Studios -->
---
状态: 已批准
作者: lead-developer
更新日期: 2026-07-08
---

# 代码管理工作流

本文件定义 Software Engineering Studios 中的代码管理工作流，包括 git worktree 并行工作规范、提交时机定义、推送与 PR 节奏、分支生命周期管理和 rebase 策略。确保在 AI Agent 多任务并行场景下，代码仓库保持整洁性、可追溯性和质量可控性。

## 1. 概述与背景

Software Engineering Studios 通过 40 个协调的 Agent 管理软件工程项目开发。`coordination-rules.md` 第 4 节定义了并行任务协议，支持最多 5 个子 Agent 同时执行独立任务。然而，多个并行子 Agent 在同一工作目录操作会产生文件冲突，且现有工作流未定义明确的提交时机、推送节奏和分支管理策略。

本规范旨在填补以下缺口：

- **代码隔离**：通过 git worktree 为每个并行子 Agent 提供独立工作目录
- **提交时机**：将"未经用户指示不得执行 Git 提交"细化为明确时机表
- **推送节奏**：将推送/PR 时机与工作流阶段关联
- **分支管理**：定义分支生命周期，包括 stale 分支检测和清理
- **Rebase 策略**：定义何时 rebase、安全规则和禁止场景

## 2. Git Worktree 并行工作规范

### 2.1 目录约定

Worktree 目录与主仓库同级，避免嵌套在主仓库内部：

```
项目根目录/
├── .studio/                     # 主仓库（规范源层）
├── src/                         # 主仓库源代码
├── ../worktrees/                # Worktree 根目录（与主仓库同级）
│   ├── feature-user-auth/       # feature/user-auth 分支的工作树
│   ├── fix-pagination/          # fix/pagination 分支的工作树
│   └── hotfix-login-crash/      # hotfix/login-crash 分支的工作树
```

**规则：**
- Worktree 路径格式：`../worktrees/[分支名]`（分支名中的 `/` 替换为 `-`）
- 每个 worktree 对应一个分支，一个分支最多一个 worktree
- Worktree 目录不纳入版本控制（添加到 `.gitignore`）

### 2.2 创建时机

当 `coordination-rules.md` 第 4 节定义的"独立任务同时发起"场景触发时，应为每个并行子 Agent 创建独立 worktree：

- **多模块并行开发**：不同模块的独立开发任务
- **多文件并行审查**：同时审查前端、后端、测试代码
- **开发与修复并行**：一个 Agent 开发新功能，另一个修复缺陷

**不适用 worktree 的场景：**
- 串行任务（有依赖关系的任务）
- 单一 Agent 的工作
- 只读分析任务（不产生代码变更）

### 2.3 与 Agent 映射

每个并行子 Agent 在独立 worktree 中工作，与 `context-management.md` 的上下文隔离原则一致：

| 并行场景 | 子 Agent | Worktree | 分支 |
|----------|----------|----------|------|
| 前端开发 | react-specialist | `../worktrees/feature-user-auth-ui` | `feature/user-auth-ui` |
| 后端开发 | node-specialist | `../worktrees/feature-user-auth-api` | `feature/user-auth-api` |
| 测试编写 | test-engineer | `../worktrees/feature-user-auth-test` | `feature/user-auth-test` |

### 2.4 生命周期

Worktree 的完整生命周期：

```
创建 → 工作 → 提交 → 合并 → 清理
```

1. **创建**：从 main（或指定基础分支）创建新分支和 worktree
2. **工作**：子 Agent 在 worktree 中独立工作，不与其他 worktree 产生冲突
3. **提交**：在 worktree 中完成逻辑单元后提交（遵循第 3 节提交时机）
4. **合并**：通过 PR 或 merge 将分支合并回 main
5. **清理**：合并后立即移除 worktree 和对应分支

### 2.5 清理规则

- **合并后立即清理**：分支合并回 main 后，立即执行 `git worktree remove` 和 `git branch -d`
- **Sprint 结束清理**：每个 Sprint 结束时，运行 `/code-management cleanup` 清理所有残留 worktree
- **异常清理**：会话崩溃或中断后，检查 `active.md` 中的 worktree 记录，清理无对应分支的 worktree

### 2.6 命令示例

```bash
# 创建 worktree（从 main 创建新分支）
git worktree add ../worktrees/feature-user-auth -b feature/user-auth main

# 在 worktree 中工作
cd ../worktrees/feature-user-auth
# ... 编写代码、运行测试 ...

# 提交（在 worktree 中）
git add .
git commit -m "feat(auth): 添加用户认证组件"

# 合并后清理
cd [主仓库目录]
git worktree remove ../worktrees/feature-user-auth
git branch -d feature/user-auth

# 查看所有 worktree
git worktree list

# 清理失效的 worktree 引用
git worktree prune
```

## 3. 提交时机定义

### 3.1 核心原则

**未经用户指示，Agent 不得自主执行 Git 提交。** 但 Agent 应在满足以下时机时，主动向用户建议执行提交。

### 3.2 提交时机表

| 时机类型 | 触发条件 | 关联工作流阶段 | 说明 |
|----------|----------|----------------|------|
| 阶段产出提交 | 每个工作流阶段完成必需产出物后 | 所有阶段 | 如 dev-story 阶段 6 完成、architecture-review 通过后 |
| 逻辑单元提交 | 一个完整的逻辑变更单元完成后 | 开发阶段 | 如一个验收标准实现 + 测试通过 |
| 检查点提交 | 通过 review 或 gate-check 后 | 测试优化、发布阶段 | 审查通过后的代码状态应被保存 |
| 会话保存提交 | 上下文压缩前、会话结束前 | 跨阶段 | 与 `context-management.md` 第 5 节压缩策略联动 |

### 3.3 禁止提交时机

以下情况禁止提交，Agent 应提示用户先修复：

- 代码未通过本地 lint 检查
- 测试失败（单元测试或集成测试）
- 包含 `console.log` 或调试输出
- 包含硬编码密钥或敏感信息（由 `validate-commit.sh` 拦截）
- 包含裸 TODO（应使用 `TODO:描述` 格式，由 `validate-commit.sh` 拦截）
- 包含无效 JSON 文件（由 `validate-commit.sh` 拦截）
- 需求文档缺少必需章节（由 `validate-commit.sh` 拦截）

### 3.4 提交粒度原则

- **一个提交对应一个逻辑变更**：禁止将多个不相关的变更混合在一个提交中
- **原子性提交**：每个提交应保持代码可编译、测试可通过
- **避免过大提交**：单个提交涉及文件不超过 20 个（除非是自动化生成文件）
- **避免过小提交**：如仅修改一个拼写错误，可合并到相关逻辑变更中

### 3.5 与 dev-story 阶段的关联

| dev-story 阶段 | 阶段产出 | 建议提交时机 |
|----------------|----------|-------------|
| 阶段 4：实现 | 源代码与配置文件 | 逻辑单元提交（每个验收标准实现+测试通过后） |
| 阶段 5：测试证据 | 测试通过证据 | 逻辑单元提交（与实现合并为一个提交） |
| 阶段 6：收集总结 | 实现总结写入 Story 文件 | 阶段产出提交（文档变更单独提交） |
| 阶段 7：更新会话状态 | active.md 更新 | 会话保存提交（如有变更） |

## 4. 推送与 PR 节奏

### 4.1 推送/PR 时机表

| 工作流阶段 | 推送/PR 时机 | PR 类型 | 前置审查 | 分支类型 |
|------------|-------------|---------|----------|----------|
| 开发阶段（dev-story） | 故事实现完成 | feature PR | code-review | `feature/*` |
| 开发阶段（hotfix） | 紧急修复完成 | hotfix PR | code-review（事后补完成） | `hotfix/*` |
| 测试优化阶段 | 全部测试通过后 | release PR | gate-check | `release/*` |
| 发布阶段 | 发布检查清单完成后 | release PR | launch-checklist | `release/*` |

### 4.2 推送前检查清单

推送或创建 PR 前，必须确认以下检查项（与 `pr-guidelines.md` 第 6 节集成）：

- [ ] 提交信息符合 Conventional Commits 格式
- [ ] subject 不超过 50 字
- [ ] 分支命名符合规范
- [ ] 代码通过本地 lint 检查
- [ ] 本地测试全部通过
- [ ] 无硬编码密钥或敏感信息
- [ ] 无 `console.log` 或调试输出
- [ ] 新功能包含对应的测试用例
- [ ] 公共 API 包含文档注释
- [ ] PR 描述清晰，关联相关 issue
- [ ] **对应审查已完成**（feature → code-review，release → gate-check）
- [ ] **受保护分支不直接推送**（main/master/develop 通过 PR 合并）

### 4.3 PR 描述模板

PR 描述应包含以下内容：

```markdown
## 变更说明

[简要描述本次变更的目的和内容]

## 关联 Story

- Story ID: [ID]
- Epic: [Epic 名称]

## 验收标准对照

| 验收标准 | 实现状态 | 说明 |
|----------|----------|------|
| [标准 1] | ✅ 已完成 | [说明] |
| [标准 2] | ✅ 已完成 | [说明] |

## 测试证据

- 单元测试: [通过/失败] ([数量] 个用例)
- 集成测试: [通过/失败] ([数量] 个用例)
- 测试覆盖率: [百分比]%

## 审查状态

- 代码审查: [通过/待审查] (审查报告: [路径])
- 门禁检查: [通过/待检查] (门禁报告: [路径])
```

## 5. 分支生命周期管理

### 5.1 分支创建规则

遵循 `pr-guidelines.md` 第 2 节的分支命名规范：

| 分支类型 | 格式 | 用途 | 示例 |
|----------|------|------|------|
| 主干 | `main` | 生产分支，受保护 | `main` |
| 功能分支 | `feature/[简短描述]` | 新功能开发 | `feature/user-auth` |
| 修复分支 | `fix/[简短描述]` | 缺陷修复 | `fix/pagination-off-by-one` |
| 热修复分支 | `hotfix/[简短描述]` | 生产紧急修复 | `hotfix/login-crash` |
| 发布分支 | `release/vX.Y.Z` | 发布准备 | `release/v1.2.0` |

### 5.2 Stale 分支检测

- **Stale 定义**：超过 7 天无提交的非受保护分支
- **检测时机**：每个 Sprint 结束时运行 `/code-management cleanup` 检测
- **标注方式**：在清理建议中标注为 `stale`
- **处理规则**：stale 分支标注后 3 天无活动，建议删除

### 5.3 清理规则

| 分支状态 | 清理操作 | 执行时机 |
|----------|----------|----------|
| 已合并分支 | 合并后立即删除 | PR 合并后自动/手动 |
| Stale 分支 | 标注后 3 天无活动建议删除 | Sprint 结束时 |
| 受保护分支 | 永不自动删除 | 不适用 |
| Worktree 关联分支 | 先移除 worktree 再删除分支 | worktree 清理时 |

### 5.4 定期分支整理

每个 Sprint 结束时执行分支清理流程：

1. 运行 `/code-management cleanup` 列出所有分支
2. 标注已合并分支和 stale 分支
3. 生成清理建议清单
4. 获用户审批后执行分支删除和 worktree 移除
5. 在 `active.md` 中记录清理结果

## 6. Rebase 策略

### 6.1 触发条件

- feature 分支落后于 main 超过 5 个提交时，建议 rebase
- PR 审查过程中发现与 main 有冲突时，必须 rebase
- 发布前整理提交历史时，可使用交互式 rebase

### 6.2 安全规则

- **仅对未推送的本地提交执行 rebase**：已推送到远程的提交禁止 rebase（除非确认为个人分支且已通知所有协作者）
- **Rebase 前确保工作区干净**：使用 `git stash` 或先提交所有变更
- **Rebase 后必须运行测试**：确保 rebase 未引入回归
- **禁止 rebase 受保护分支**：main、master、release/* 分支不可 rebase

### 6.3 Rebase 后处理

```
1. 解决冲突 → 2. 运行测试 → 3. 强制推送（仅个人分支）
```

1. **解决冲突**：逐个提交解决冲突，保持提交原子性
2. **运行测试**：执行完整测试套件确认无回归
3. **强制推送**：仅对个人分支执行 `git push --force-with-lease`（比 `--force` 更安全）

### 6.4 交互式 Rebase

交互式 rebase 用于提交历史整理：

- **合并 WIP 提交**：将多个工作进度提交合并为有意义的提交
- **修正提交信息**：修正不符合 Conventional Commits 的提交信息
- **拆分过大提交**：将过大提交拆分为多个逻辑单元

```bash
# 交互式 rebase 最近 5 个提交
git rebase -i HEAD~5

# 在编辑器中将需要合并的提交标记为 squash 或 fixup
```

## 7. 与现有工作流的集成

### 7.1 与并行任务协议的集成

与 `coordination-rules.md` 第 4 节并行任务协议的集成：

| 协议条款 | 集成方式 |
|----------|----------|
| 4.1 独立任务同时发起 | 为每个并行子 Agent 创建独立 worktree |
| 4.2 收集所有结果再进入依赖阶段 | 所有 worktree 合并后再进入依赖阶段 |
| 4.3 BLOCKED 立即上报 | 在 active.md 中记录 worktree 状态 |
| 4.4 产出部分报告 | 部分完成的 worktree 标注为 PENDING |

### 7.2 与审查模式的集成

与 `review-workflow.md` 审查模式的集成：

| 审查模式 | Worktree 使用 | 提交时机 | 推送前置 |
|----------|-------------|----------|----------|
| full | 并行任务必须使用 worktree | 所有阶段产出提交 | code-review 必须 |
| lean | 并行任务建议使用 worktree | 关键阶段产出提交 | code-review 必须 |
| solo | 可选使用 worktree | 逻辑单元提交 | code-review 可选 |

### 7.3 与上下文管理的集成

与 `context-management.md` 会话状态管理的集成：

- `active.md` 新增"Worktree 映射"字段，记录当前会话创建的所有 worktree 及其分支
- 上下文压缩前，检查 worktree 中是否有未提交的变更，提示用户提交
- 会话恢复时，检查 worktree 状态，恢复中断的工作

### 7.4 与 PR 规范的集成

与 `pr-guidelines.md` PR 规范的集成：

- 分支命名遵循第 2 节规范
- 提交信息遵循第 1 节 Conventional Commits 格式
- 代码审查遵循第 3 节审查流程
- 门禁检查遵循第 4 节门禁要求
- 推送前检查清单遵循第 6 节检查项