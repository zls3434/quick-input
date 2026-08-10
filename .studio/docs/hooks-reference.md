<!-- Software Engineering Studios -->
# Hook 参考

本文件列出 Software Engineering Studios 中配置的 12 个 Hook 脚本，说明其触发时机、用途和配置参数。所有 Hook 脚本位于 `.claude/hooks/` 目录。

## 1. Hook 总览

| 序号 | 脚本名称 | 触发事件 | 用途 | 超时（秒） |
| --- | --- | --- | --- | --- |
| 1 | `session-start.sh` | SessionStart | 加载项目上下文 | 10 |
| 2 | `detect-gaps.sh` | SessionStart | 检测缺口 | 10 |
| 3 | `validate-commit.sh` | PreToolUse:Bash | 检查 git commit 规范 | 15 |
| 4 | `validate-push.sh` | PreToolUse:Bash | 受保护分支推送警告 | 10 |
| 5 | `validate-assets.sh` | PostToolUse:Write\|Edit | 验证命名和 JSON | 10 |
| 6 | `validate-skill-change.sh` | PostToolUse:Write\|Edit | 技能变更提示 | 5 |
| 7 | `notify.sh` | Notification | Toast 通知 | 10 |
| 8 | `pre-compact.sh` | PreCompact | 保存进度 | 10 |
| 9 | `post-compact.sh` | PostCompact | 恢复提醒 | 10 |
| 10 | `session-stop.sh` | Stop | 归档 active.md | 10 |
| 11 | `log-agent.sh` | SubagentStart | 审计记录 | 5 |
| 12 | `log-agent-stop.sh` | SubagentStop | 审计完成 | 5 |

## 2. 各 Hook 详细说明

### 2.1 session-start.sh

- **触发事件**：SessionStart
- **用途**：会话启动时加载项目上下文
- **超时**：10 秒
- **执行内容**：
  - 读取 `CLAUDE.md` 加载项目配置
  - 读取 `production/session-state/active.md` 恢复会话状态
  - 检测当前项目阶段
  - 输出上下文摘要供 Agent 参考
- **失败处理**：非阻塞，失败时继续启动会话并记录警告

### 2.2 detect-gaps.sh

- **触发事件**：SessionStart
- **用途**：会话启动时检测项目文档和配置缺口
- **超时**：10 秒
- **执行内容**：
  - 检查必需文件是否存在（`CLAUDE.md`、`.claude/settings.json` 等）
  - 检查 `technical-preferences.md` 是否已配置
  - 检查当前阶段所需产出物是否齐全
  - 输出缺口报告
- **失败处理**：非阻塞，缺口信息作为建议展示

### 2.3 validate-commit.sh

- **触发事件**：PreToolUse:Bash
- **用途**：检查 git commit 提交信息是否符合 Conventional Commits 规范
- **超时**：15 秒
- **执行内容**：
  - 拦截包含 `git commit` 的命令
  - 解析提交信息格式
  - 验证 type 字段是否为允许值（feat/fix/docs/refactor/test/chore）
  - 验证 subject 长度是否超过 50 字
  - 验证 scope 是否合法
  - 不通过时阻止命令执行
- **失败处理**：阻塞，不符合规范的提交被阻止

### 2.4 validate-push.sh

- **触发事件**：PreToolUse:Bash
- **用途**：对受保护分支的推送操作发出警告
- **超时**：10 秒
- **执行内容**：
  - 拦截包含 `git push` 的命令
  - 检查目标分支是否为受保护分支（main/master/release/*）
  - 若为受保护分支，输出警告信息
  - 提示需要通过代码审查和门禁检查
- **失败处理**：非阻塞（仅警告），但建议用户确认后继续

### 2.5 validate-assets.sh

- **触发事件**：PostToolUse:Write|Edit
- **用途**：验证新创建或修改文件的命名规范和 JSON 格式
- **超时**：10 秒
- **执行内容**：
  - 检查文件命名是否符合命名约定（见 `technical-preferences.md`）
  - 若文件为 JSON 格式，验证 JSON 合法性
  - 若文件为 YAML 格式，验证 YAML 合法性
  - 检查文件是否包含必需的文件头注释
  - 输出验证结果
- **失败处理**：非阻塞，输出警告但不阻止写入

### 2.6 validate-skill-change.sh

- **触发事件**：PostToolUse:Write|Edit
- **用途**：检测技能文件的变更并发出提示
- **超时**：5 秒
- **执行内容**：
  - 检查被修改的文件是否位于 `.claude/skills/` 目录
  - 若是技能文件变更，输出提示信息
  - 提醒技能变更可能影响多个 Agent 行为
  - 建议运行 `/skill-test` 验证变更
- **失败处理**：非阻塞，仅输出提示信息

### 2.7 notify.sh

- **触发事件**：Notification
- **用途**：在关键事件发生时显示 Toast 通知
- **超时**：10 秒
- **执行内容**：
  - 接收通知事件类型
  - 根据事件类型生成通知消息
  - 通过系统通知机制展示
  - 通知类型包括：任务完成、阻塞、审查通过/失败、门禁结果
- **失败处理**：非阻塞，通知失败不影响主流程

### 2.8 pre-compact.sh

- **触发事件**：PreCompact
- **用途**：在上下文压缩前保存当前进度
- **超时**：10 秒
- **执行内容**：
  - 读取当前 `active.md` 状态
  - 保存当前进度快照至 `production/session-state/pre-compact-snapshot.md`
  - 记录压缩前上下文使用量
  - 确保所有未写入文件的草稿已保存
- **失败处理**：阻塞，失败时建议用户手动保存后重试压缩

### 2.9 post-compact.sh

- **触发事件**：PostCompact
- **用途**：上下文压缩后恢复提醒
- **超时**：10 秒
- **执行内容**：
  - 读取 `active.md` 恢复当前任务状态
  - 读取 `pre-compact-snapshot.md` 对比压缩前后状态
  - 输出恢复提醒，包含当前任务和进度
  - 提示用户确认状态正确后继续
- **失败处理**：非阻塞，失败时输出基本恢复信息

### 2.10 session-stop.sh

- **触发事件**：Stop
- **用途**：会话结束时归档 active.md
- **超时**：10 秒
- **执行内容**：
  - 读取当前 `active.md`
  - 在文件末尾添加会话结束时间戳
  - 将 `active.md` 复制至 `production/session-logs/YYYY-MM-DD-session.md`
  - 清空或保留 `active.md`（根据配置）
- **失败处理**：非阻塞，失败时记录错误但不阻止会话结束

### 2.11 log-agent.sh

- **触发事件**：SubagentStart
- **用途**：记录子 Agent 启动的审计日志
- **超时**：5 秒
- **执行内容**：
  - 记录子 Agent 名称、任务描述、模型层级
  - 记录启动时间戳
  - 写入 `production/session-logs/agent-audit.log`
  - 格式：`[时间戳] START [Agent名称] [模型] [任务描述]`
- **失败处理**：非阻塞，审计日志失败不影响子 Agent 执行

### 2.12 log-agent-stop.sh

- **触发事件**：SubagentStop
- **用途**：记录子 Agent 完成的审计日志
- **超时**：5 秒
- **执行内容**：
  - 记录子 Agent 名称、执行状态（成功/失败/阻塞）
  - 记录完成时间戳和执行时长
  - 记录产出物路径
  - 写入 `production/session-logs/agent-audit.log`
  - 格式：`[时间戳] STOP [Agent名称] [状态] [时长] [产出物路径]`
- **失败处理**：非阻塞，审计日志失败不影响主流程

## 3. 配置示例

Hook 在 `.claude/settings.json` 中的配置格式：

```json
{
  "hooks": {
    "SessionStart": [
      {
        "command": ".claude/hooks/session-start.sh",
        "timeout": 10
      },
      {
        "command": ".claude/hooks/detect-gaps.sh",
        "timeout": 10
      }
    ],
    "PreToolUse": [
      {
        "matcher": "Bash",
        "command": ".claude/hooks/validate-commit.sh",
        "timeout": 15
      },
      {
        "matcher": "Bash",
        "command": ".claude/hooks/validate-push.sh",
        "timeout": 10
      }
    ],
    "PostToolUse": [
      {
        "matcher": "Write|Edit",
        "command": ".claude/hooks/validate-assets.sh",
        "timeout": 10
      },
      {
        "matcher": "Write|Edit",
        "command": ".claude/hooks/validate-skill-change.sh",
        "timeout": 5
      }
    ],
    "Notification": [
      {
        "command": ".claude/hooks/notify.sh",
        "timeout": 10
      }
    ],
    "PreCompact": [
      {
        "command": ".claude/hooks/pre-compact.sh",
        "timeout": 10
      }
    ],
    "PostCompact": [
      {
        "command": ".claude/hooks/post-compact.sh",
        "timeout": 10
      }
    ],
    "Stop": [
      {
        "command": ".claude/hooks/session-stop.sh",
        "timeout": 10
      }
    ],
    "SubagentStart": [
      {
        "command": ".claude/hooks/log-agent.sh",
        "timeout": 5
      }
    ],
    "SubagentStop": [
      {
        "command": ".claude/hooks/log-agent-stop.sh",
        "timeout": 5
      }
    ]
  }
}
```

## 4. 依赖工具

部分 Hook 依赖以下工具，缺失时将优雅降级：

| Hook | 依赖工具 | 降级行为 |
| --- | --- | --- |
| `validate-assets.sh` | `jq`（JSON 验证）、`python3`（JSON 验证） | 跳过 JSON 验证，仅检查命名 |
| `notify.sh` | 系统通知工具 | 跳过通知，仅记录日志 |
| `validate-commit.sh` | `jq`（解析配置） | 使用基础正则匹配 |

详见 `setup-requirements.md` 了解工具安装说明。