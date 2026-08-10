---
name: sprint-status
description: "Sprint 进度快照技能。生成不超过 30 行的 Sprint 状态摘要，含 Story 完成数、进行中、阻塞项与风险提示。"
version: 1.0.0
platforms: [macos, linux, windows]
metadata:
  hermes:
    tags: ["planning", "project"]
    load_mode: on-demand
    model: haiku
    argument-hint: "[sprint-id]"
    user-invocable: true
    allowed-tools:
      - Read
      - Glob
      - Grep
---

# sprint-status —— Sprint 进度快照技能

## 技能目的

为当前或指定 Sprint 生成一份精简的进度快照（不超过 30 行），
帮助用户快速掌握 Story 完成情况、进行中任务、阻塞项与潜在风险。
本技能为只读概览，不修改任何文件，适合在每日站会或周期回顾前快速调用。

## 参数说明

- `[sprint-id]`：可选的 Sprint 标识，例如 `Sprint-12`。
  若省略，则默认读取 `docs/sprints/` 下最新一个 Sprint 的状态文件。

## 分阶段工作流

### 阶段 1：定位 Sprint

- **输入**：用户提供的 `[sprint-id]` 或默认取最新 Sprint。
- **处理**：使用 Glob 在 `docs/sprints/` 下匹配对应 Sprint 文件；使用 Read 读取其内容。
- **输出**：Sprint 元数据：起止日期、目标、Story 列表。

### 阶段 2：汇总 Story 状态

- **输入**：Sprint 关联的 Story 列表。
- **处理**：使用 Grep 在 `docs/stories/` 中查询每个 Story 的当前状态（`todo` / `in-progress` / `implemented` / `done` / `blocked`）。
- **输出**：按状态分组的 Story 计数与清单。

### 阶段 3：识别阻塞与风险

- **输入**：阶段 2 的状态分组。
- **处理**：提取状态为 `blocked` 的 Story 及其阻塞原因；识别长期停留在 `in-progress` 的 Story 作为风险项。
- **输出**：阻塞项清单与风险提示。

### 阶段 4：生成快照

- **输入**：阶段 1-3 的汇总数据。
- **处理**：压缩为不超过 30 行的纯文本快照，包含：
  - Sprint 名称与时间窗
  - 完成数 / 进行中 / 待开始 / 阻塞数
  - 顶部 3 个风险或阻塞项
  - 一句话健康度评估
- **输出**：直接以文本形式输出给用户，不落盘。

## 协作协议引用

- 本技能为只读快照，不写入任何文件。
- 若用户要求持久化快照，应提示用户改用 `/story-done` 或手动保存，不得自主写入。
- 快照结论仅作参考，不替代正式的完成审查。

## 推荐下一步

- 若快照显示阻塞项较多，运行 `/bug-triage` 重新排序未解决 Bug。
- 接近 Sprint 结束时运行 `/retrospective` 进行阶段回顾。
- 个别 Story 需深入查看时运行 `/story-done [故事路径]`。