---
状态: 已批准
作者: lead-developer
更新日期: 2026-07-08
---

name: code-management
description: "代码管理工作流技能。管理 git worktree 并行工作、提交时机判断、推送/PR 准备、分支生命周期管理和 rebase 策略执行，确保代码仓库整洁有序。"
license: MIT
metadata:
  model: sonnet
  argument-hint: "[操作类型: worktree|commit|push|cleanup|rebase]"
  user-invocable: true
  allowed-tools:
    - Read
    - Glob
    - Grep
    - Write
    - Edit
    - Bash
    - AskUserQuestion
  platforms:
    claude-code: {enabled: true}
    cursor: {enabled: true}
    codex: {enabled: true}
    windsurf: {enabled: true, trigger: /code-management}
    trae: {enabled: true}
    hermes: {enabled: true, platforms: [macos, linux, windows]}
    workbuddy: {enabled: true}
---

# code-management — 代码管理

## 技能目的

提供代码管理工作流的统一执行入口，支持 git worktree 并行工作、提交时机判断、推送/PR 准备、分支生命周期管理和 rebase 策略执行。在 AI Agent 多任务并行场景下，确保代码仓库保持整洁性、可追溯性和质量可控性。遵循 `code-management-workflow.md` 规范定义的所有策略和规则。

## 参数说明

- `[操作类型]`：指定要执行的代码管理操作，支持以下值：
  - `worktree`：创建或管理 git worktree，用于并行任务代码隔离
  - `commit`：判断提交时机并执行提交，遵循 Conventional Commits 规范
  - `push`：准备推送或创建 PR，检查审查前置条件
  - `cleanup`：清理已合并分支、stale 分支和残留 worktree
  - `rebase`：执行 rebase 操作，整理提交历史
- 若未指定操作类型，使用 AskUserQuestion 询问用户选择。

## 分阶段工作流

### 阶段 1：操作路由

- **输入**：用户提供的 `[操作类型]` 参数。
- **处理**：
  1. 解析参数，确定操作类型（worktree/commit/push/cleanup/rebase）
  2. 若未指定参数，使用 AskUserQuestion 列出 5 个操作选项供用户选择
  3. 读取 `code-management-workflow.md` 加载对应操作的规范
  4. 读取 `active.md` 获取当前会话状态和并行任务信息
- **输出**：确定的操作类型和对应规范摘要

### 阶段 2：worktree 子流程

- **输入**：操作路由结果，当前会话状态。
- **处理**：
  1. 读取 `active.md` 确认当前并行任务数量和名称
  2. 检查当前操作系统，适配路径分隔符（Windows 用 `\`，Unix 用 `/`）
  3. 根据任务名生成 worktree 路径：`../worktrees/[分支名]`（分支名中 `/` 替换为 `-`）
  4. 生成分支名：遵循 `pr-guidelines.md` 第 2 节命名规范（feature/fix/hotfix）
  5. 使用 AskUserQuestion 确认 worktree 路径和分支名
  6. 执行 `git worktree add ../worktrees/[分支名] -b [分支名] main`
  7. 在 `active.md` 中记录 worktree 映射（worktree 路径 → 分支名 → 任务名）
  8. 提示用户切换到 worktree 目录开始工作
- **输出**：worktree 创建结果和路径

### 阶段 3：commit 子流程

- **输入**：操作路由结果，当前工作区状态。
- **处理**：
  1. 读取 `code-management-workflow.md` 第 3 节提交时机表
  2. 检查当前是否满足提交时机条件（阶段产出/逻辑单元/检查点/会话保存）
  3. 运行 `git status` 检查工作区状态
  4. 检查暂存区文件，确认无硬编码密钥、调试代码、无效 JSON（与 validate-commit.sh 检查项一致）
  5. 运行本地 lint 检查（如项目已配置）
  6. 运行测试（如项目已配置测试命令）
  7. 分析暂存区变更，生成符合 Conventional Commits 的提交信息草案
  8. 使用 AskUserQuestion 展示提交信息草案，获用户审批
  9. 获批准后执行 `git commit`
- **输出**：提交结果和提交哈希

### 阶段 4：push 子流程

- **输入**：操作路由结果，当前分支状态。
- **处理**：
  1. 运行 `git branch --show-current` 获取当前分支名
  2. 检查当前分支是否为受保护分支（main/master/develop）
     - 若是受保护分支：输出阻断提示，建议通过 PR 合并，终止流程
  3. 检查分支名是否符合 `pr-guidelines.md` 第 2 节命名规范
     - 不符合时输出警告，询问用户是否继续
  4. 根据分支类型检查对应审查状态：
     - `feature/*`：使用 Glob 查找 `production/sprints/*-review.md` 审查报告
     - `fix/*`：使用 Glob 查找 `production/sprints/*-review.md` 审查报告
     - `hotfix/*`：提示 hotfix 可事后补审查
     - `release/*`：使用 Glob 查找 `production/releases/gate-check.md` 门禁报告
  5. 未找到审查报告时输出警告，提示运行 `/code-review` 或 `/gate-check`
  6. 生成 PR 描述草案（关联 Story ID、验收标准对照、测试证据摘要）
  7. 使用 AskUserQuestion 展示 PR 描述草案，获用户审批
  8. 获批准后执行推送 `git push origin [分支名]` 或创建 PR
- **输出**：推送结果或 PR 链接

### 阶段 5：cleanup 子流程

- **输入**：操作路由结果，所有分支和 worktree 状态。
- **处理**：
  1. 执行 `git branch -a` 列出所有本地和远程分支
  2. 执行 `git worktree list` 列出所有 worktree
  3. 对每个分支检查是否已合并到 main：`git branch --merged main`
  4. 对未合并的非受保护分支检查最后提交时间：`git log -1 --format=%ci [分支名]`
  5. 超过 7 天无提交的标注为 `stale`
  6. 检查是否有失效的 worktree 引用：`git worktree prune --dry-run`
  7. 生成清理建议清单，分三类：
     - 已合并分支（建议立即删除）
     - Stale 分支（标注天数，建议删除）
     - 残留 worktree（建议移除）
  8. 使用 AskUserQuestion 展示清理建议，获用户审批
  9. 获批准后逐个执行：
     - `git worktree remove [路径]` 移除 worktree
     - `git branch -d [分支名]` 删除已合并分支
     - `git push origin --delete [分支名]` 删除远程分支（如适用）
  10. 更新 `active.md` 记录清理结果
- **输出**：清理结果摘要

### 阶段 6：rebase 子流程

- **输入**：操作路由结果，当前分支与 main 的差距。
- **处理**：
  1. 执行 `git rev-list --count main..HEAD` 检查当前分支领先 main 的提交数
  2. 执行 `git rev-list --count HEAD..main` 检查当前分支落后 main 的提交数
  3. 判断是否满足 rebase 条件：
     - 落后 main 超过 5 个提交 → 建议执行 rebase
     - 落后少于 5 个提交 → 询问用户是否仍需 rebase
  4. 检查是否有未推送的提交：`git log origin/[分支名]..HEAD`（如远程分支存在）
     - 若有已推送的提交：输出警告，询问用户是否确认 rebase（已推送提交 rebase 需强制推送）
  5. 检查工作区是否干净：`git status --porcelain`
     - 不干净时提示用户先提交或 stash
  6. 使用 AskUserQuestion 确认 rebase 操作
  7. 执行 `git rebase main`（或交互式 rebase `git rebase -i main` 如用户选择整理历史）
  8. 如遇冲突，输出冲突文件列表，提示用户解决后运行 `git rebase --continue`
  9. Rebase 完成后运行测试验证无回归
  10. 提示用户如需推送执行 `git push --force-with-lease`（仅个人分支）
- **输出**：rebase 结果和测试状态

## 协作协议引用

- 遵循 `docs/examples/COLLABORATIVE-DESIGN-PRINCIPLE.md` 用户驱动协作协议
- 遵循 `code-management-workflow.md` 代码管理工作流规范
- 遵循 `pr-guidelines.md` PR 和提交规范
- 遵循 `coordination-rules.md` 第 4 节并行任务协议
- 所有操作（commit、push、cleanup、rebase）必须经用户明确审批后执行
- 不得自主执行 Git 提交或推送，仅提供建议和草案
- 技能变更触发 `validate-skill-change.sh` Hook

## 推荐下一步

- worktree 创建后：在 worktree 目录中运行 `/dev-story [故事路径]` 开始开发
- commit 完成后：运行 `/code-management push` 准备推送或创建 PR
- push 完成后：运行 `/code-review [文件路径]` 执行代码审查（如尚未完成）
- cleanup 完成后：运行 `/sprint-status` 查看 Sprint 进度
- rebase 完成后：运行 `/code-management push` 推送更新
- 并行任务全部完成后：运行 `/retrospective` 进行 Sprint 回顾