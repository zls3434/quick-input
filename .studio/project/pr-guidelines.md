<!-- Software Engineering Studios -->
# PR/提交规范

本文件是平台无关的 PR 和提交规范源，定义提交信息格式、分支命名、代码审查流程和门禁检查要求。

## 1. Conventional Commits 格式

所有 Git 提交信息必须遵循 Conventional Commits 规范。

### 1.1 格式

```
<type>(<scope>): <subject>

<body>

<footer>
```

### 1.2 类型（type）

| 类型 | 说明 | 示例 |
|------|------|------|
| `feat` | 新功能 | `feat(auth): 添加 OAuth2 登录支持` |
| `fix` | 缺陷修复 | `fix(api): 修复分页参数 off-by-one 错误` |
| `docs` | 文档变更 | `docs(api): 更新用户接口文档` |
| `refactor` | 重构（不改变外部行为） | `refactor(auth): 提取令牌验证逻辑` |
| `test` | 测试相关 | `test(user): 添加用户注册单元测试` |
| `chore` | 构建/工具/杂项 | `chore(deps): 升级依赖至最新版本` |

### 1.3 规则

- **subject 使用简体中文**，不超过 50 字
- **subject 使用祈使语气**，首字母不大写，结尾不加句号
- **scope** 为可选项，表示影响的模块或领域
- **body** 说明变更原因和背景，每行不超过 72 字
- **footer** 标注 BREAKING CHANGE 或关联 issue（如 `Closes #123`）
- 提交前由 `validate-commit.sh` Hook 自动校验格式

### 1.4 示例

**标准提交：**

```
feat(auth): 添加 OAuth2 登录支持

集成 Google 和 GitHub OAuth2 提供商，
支持用户通过第三方账号登录。

Closes #42
```

**带 BREAKING CHANGE 的提交：**

```
feat(api): 重构用户接口返回结构

将用户信息从嵌套对象改为扁平结构，
提升前端消费便利性。

BREAKING CHANGE: /users 接口返回结构变更，
前端需同步更新数据解析逻辑
```

## 2. 分支命名

采用基于主干的开发模式。

| 分支类型 | 格式 | 用途 | 示例 |
|----------|------|------|------|
| 主干 | `main` | 生产分支，受保护 | `main` |
| 功能分支 | `feature/[简短描述]` | 新功能开发 | `feature/user-auth` |
| 修复分支 | `fix/[简短描述]` | 缺陷修复 | `fix/pagination-off-by-one` |
| 热修复分支 | `hotfix/[简短描述]` | 生产紧急修复 | `hotfix/login-crash` |
| 发布分支 | `release/vX.Y.Z` | 发布准备 | `release/v1.2.0` |

### 分支规则

- `main` 和 `release/*` 为受保护分支，禁止直接推送
- `validate-push.sh` Hook 阻断对受保护分支（main/master/develop）的直接推送
- 功能分支从 `main` 检出，开发完成后通过 PR 合并回 `main`
- 热修复分支从 `main` 检出，修复后同时合并到 `main` 和当前发布分支

### 2.1 分支生命周期

分支遵循完整的生命周期管理：创建 → 活跃 → 合并 → 清理。

| 生命周期阶段 | 状态定义 | 管理操作 |
|--------------|----------|----------|
| 创建 | 从 main 检出新分支 | 遵循命名规范 |
| 活跃 | 有持续提交（7 天内有活动） | 正常开发 |
| Stale | 超过 7 天无提交的非受保护分支 | 标注并在 3 天后建议删除 |
| 已合并 | 分支已通过 PR 合并回 main | 合并后立即删除 |
| 清理 | 分支和关联 worktree 被移除 | 清理残留引用 |

**Sprint 结束分支整理流程：**
1. 运行 `/code-management cleanup` 列出所有分支
2. 标注已合并分支和 stale 分支
3. 生成清理建议清单
4. 获用户审批后执行分支删除和 worktree 移除
5. 在 `active.md` 中记录清理结果

### 2.2 Git Worktree 使用规范

并行任务开发时使用 git worktree 提供代码隔离，详见 `docs/code-management-workflow.md` 第 2 节。

**目录约定：**
- Worktree 路径格式：`../worktrees/[分支名]`（分支名中 `/` 替换为 `-`）
- Worktree 目录与主仓库同级，不嵌套在主仓库内部
- Worktree 目录不纳入版本控制

**与并行任务协议的关联：**
当 `coordination-rules.md` 第 4 节"独立任务同时发起"场景触发时，为每个并行子 Agent 创建独立 worktree。

**常用命令：**
```bash
# 创建 worktree
git worktree add ../worktrees/feature-user-auth -b feature/user-auth main

# 清理 worktree
git worktree remove ../worktrees/feature-user-auth
git branch -d feature/user-auth

# 查看所有 worktree
git worktree list

# 清理失效引用
git worktree prune
```

## 3. 代码审查流程

### 3.1 审查要求

- 所有代码变更必须通过 Pull Request（或等效流程）合并
- PR 必须至少获得一名审查者的 approval 才能合并
- 审查由 `code-reviewer` Agent 或人工审查者执行
- 涉及架构变更的 PR 需 `chief-architect` 额外审批

### 3.2 审查内容

审查者应关注以下方面：

| 审查维度 | 检查项 |
|----------|--------|
| 功能正确性 | 代码是否实现了预期功能 |
| 编码规范 | 是否符合命名约定和禁止模式 |
| 测试覆盖 | 是否包含充分的单元测试和集成测试 |
| 安全性 | 是否存在安全风险（硬编码密钥、SQL 注入等） |
| 性能 | 是否有明显性能问题 |
| 可维护性 | 代码是否清晰、可读、可扩展 |
| 文档 | 公共 API 是否有完整文档注释 |

### 3.3 审查流程

1. **提交 PR**：开发者提交 PR，填写描述和关联 issue
2. **自动检查**：CI 流水线自动运行 lint、测试、安全扫描
3. **人工审查**：审查者审查代码，提出修改建议或 approval
4. **修改迭代**：开发者根据反馈修改代码
5. **最终审批**：审查者给出最终 approval
6. **合并**：通过所有门禁后合并到主干

### 3.4 推送前置审查检查

推送或创建 PR 前，必须确认审查状态满足前置条件：

| 分支类型 | 前置审查 | 审查报告路径 | 检查方式 |
|----------|----------|-------------|----------|
| `feature/*` | code-review | `production/sprints/*-review.md` | `validate-push.sh` 检查 |
| `fix/*` | code-review | `production/sprints/*-review.md` | `validate-push.sh` 检查 |
| `hotfix/*` | code-review（事后补完成） | `docs/hotfix/` | 提示补审查 |
| `release/*` | gate-check | `production/releases/gate-check.md` | `validate-push.sh` 检查 |

- `validate-push.sh` 对未找到审查报告的 feature/fix 分支输出警告（不阻断，提示运行 `/code-review`）
- 受保护分支（main/master/develop）直接推送被阻断，必须通过 PR 合并
- 详见 `docs/code-management-workflow.md` 第 4 节推送与 PR 节奏

## 4. 门禁检查要求

### 4.1 合并前必须通过的门禁

| 门禁 | 检查项 | 负责人 | 强制级别 |
|------|--------|--------|----------|
| 代码审查 | 至少一名审查者 approval | `code-reviewer` | BLOCKING |
| 代码静态检查 | Lint 无错误 | CI 自动 | BLOCKING |
| 单元测试 | 全部通过，覆盖率达标 | `qa-lead` | BLOCKING |
| 集成测试 | 关键路径全部通过 | `qa-lead` | BLOCKING |
| 安全扫描 | 无严重漏洞 | `security-lead` | BLOCKING |
| 构建验证 | 构建成功，产物完整 | `devops-lead` | BLOCKING |

### 4.2 CI 流水线阶段

```
提交 → Lint → 单元测试 → 集成测试 → 构建验证 → 安全扫描 → 合并
```

- 每次提交自动触发 CI 流水线
- 任一阶段失败禁止合并到主干
- CI 配置文件纳入版本控制

### 4.3 发布门禁

生产部署需通过额外的发布门禁：

- 运行 `/gate-check` 命令进行发布前检查
- `release-manager` 确认发布清单完整
- `security-lead` 确认安全审计通过
- `devops-lead` 确认部署策略（蓝绿部署或滚动部署）

## 5. 版本管理

### 5.1 语义化版本

采用语义化版本号：`MAJOR.MINOR.PATCH`

- **MAJOR**：不兼容的 API 修改
- **MINOR**：向下兼容的新功能
- **PATCH**：向下兼容的缺陷修复

### 5.2 发布标签

- 发布标签格式：`vX.Y.Z`（如 `v1.2.0`）
- 变更日志由 `/changelog` 自动生成
- 补丁说明由 `/patch-notes` 生成

## 6. 提交前检查清单

提交 PR 前开发者应确认：

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

## 7. 提交时机

### 7.1 核心原则

**未经用户指示，Agent 不得自主执行 Git 提交。** 但 Agent 应在满足以下时机时，主动向用户建议执行提交。

### 7.2 提交时机表

| 时机类型 | 触发条件 | 关联工作流阶段 |
|----------|----------|----------------|
| 阶段产出提交 | 每个工作流阶段完成必需产出物后 | 所有阶段 |
| 逻辑单元提交 | 一个完整的逻辑变更单元完成后（验收标准实现+测试通过） | 开发阶段 |
| 检查点提交 | 通过 review 或 gate-check 后 | 测试优化、发布阶段 |
| 会话保存提交 | 上下文压缩前、会话结束前 | 跨阶段 |

### 7.3 禁止提交时机

- 代码未通过本地 lint 检查
- 测试失败（单元测试或集成测试）
- 包含 `console.log` 或调试输出
- 包含硬编码密钥或敏感信息（由 `validate-commit.sh` 拦截）
- 包含裸 TODO（应使用 `TODO:描述` 格式，由 `validate-commit.sh` 拦截）
- 包含无效 JSON 文件（由 `validate-commit.sh` 拦截）
- 需求文档缺少必需章节（由 `validate-commit.sh` 拦截）

### 7.4 提交粒度原则

- **一个提交对应一个逻辑变更**：禁止将多个不相关的变更混合在一个提交中
- **原子性提交**：每个提交应保持代码可编译、测试可通过
- **避免过大提交**：单个提交涉及文件不超过 20 个（除非是自动化生成文件）
- **避免过小提交**：如仅修改一个拼写错误，可合并到相关逻辑变更中

### 7.5 与 dev-story 阶段的关联

| dev-story 阶段 | 阶段产出 | 建议提交时机 |
|----------------|----------|-------------|
| 阶段 4：实现 | 源代码与配置文件 | 逻辑单元提交（每个验收标准实现+测试通过后） |
| 阶段 5：测试证据 | 测试通过证据 | 逻辑单元提交（与实现合并为一个提交） |
| 阶段 6：收集总结 | 实现总结写入 Story 文件 | 阶段产出提交（文档变更单独提交） |
| 阶段 7：更新会话状态 | active.md 更新 | 会话保存提交（如有变更） |

> 完整提交时机定义详见 `docs/code-management-workflow.md` 第 3 节。

## 8. Rebase 策略

### 8.1 触发条件

- feature 分支落后于 main 超过 5 个提交时，建议 rebase
- PR 审查过程中发现与 main 有冲突时，必须 rebase
- 发布前整理提交历史时，可使用交互式 rebase

### 8.2 安全规则

- **仅对未推送的本地提交执行 rebase**：已推送到远程的提交禁止 rebase（除非确认为个人分支且已通知所有协作者）
- **Rebase 前确保工作区干净**：使用 `git stash` 或先提交所有变更
- **Rebase 后必须运行测试**：确保 rebase 未引入回归
- **禁止 rebase 受保护分支**：main、master、release/* 分支不可 rebase

### 8.3 Rebase 后处理

```
1. 解决冲突 → 2. 运行测试 → 3. 强制推送（仅个人分支）
```

1. **解决冲突**：逐个提交解决冲突，保持提交原子性
2. **运行测试**：执行完整测试套件确认无回归
3. **强制推送**：仅对个人分支执行 `git push --force-with-lease`（比 `--force` 更安全）

### 8.4 交互式 Rebase

交互式 rebase 用于提交历史整理：

- **合并 WIP 提交**：将多个工作进度提交合并为有意义的提交
- **修正提交信息**：修正不符合 Conventional Commits 的提交信息
- **拆分过大提交**：将过大提交拆分为多个逻辑单元

```bash
# 交互式 rebase 最近 5 个提交
git rebase -i HEAD~5
```

> 完整 rebase 策略详见 `docs/code-management-workflow.md` 第 6 节。