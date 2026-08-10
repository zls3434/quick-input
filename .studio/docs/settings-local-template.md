<!-- Software Engineering Studios -->
# 个人 settings.local.json 指南

本文件说明如何创建个人本地配置文件 `settings.local.json`，以覆盖全局设置而不影响团队其他成员。该文件不纳入版本控制。

## 1. 文件位置与用途

- **位置**：`.claude/settings.local.json`
- **用途**：个人本地配置覆盖，优先级高于 `.claude/settings.json`
- **版本控制**：不纳入版本控制（应在 `.gitignore` 中添加 `.claude/settings.local.json`）

## 2. 覆盖机制

`settings.local.json` 的配置与 `settings.json` 深度合并，后者覆盖前者同名配置：

- **permissions**：合并权限列表，个人可扩展额外权限
- **hooks**：扩展 Hook 配置，个人可添加额外 Hook
- **model**：覆盖模型层级分配，个人可调整 Agent 模型
- **env**：覆盖环境变量，个人可设置本地环境特定变量

**覆盖优先级（从低到高）：**
1. `.claude/settings.json`（全局默认）
2. `.claude/settings.local.json`（个人覆盖）

## 3. 配置示例

### 3.1 基础示例

```json
{
  "permissions": {
    "allow": [
      "Read(**)",
      "Write(src/**)",
      "Write(tests/**)",
      "Edit(src/**)"
    ],
    "deny": [
      "Write(.claude/**)",
      "Edit(.claude/**)",
      "Write(production/releases/**)"
    ]
  },
  "model": {
    "default": "sonnet",
    "overrides": {
      "gate-check": "opus",
      "architecture-review": "opus",
      "help": "haiku",
      "sprint-status": "haiku"
    }
  },
  "env": {
    "LOCAL_ENV": "development",
    "DEBUG": "true"
  }
}
```

### 3.2 扩展权限示例

若个人需要额外的文件操作权限：

```json
{
  "permissions": {
    "allow": [
      "Read(**)",
      "Write(src/**)",
      "Write(tests/**)",
      "Write(docs/**)",
      "Edit(src/**)",
      "Edit(docs/**)",
      "Bash(npm test*)",
      "Bash(npm run lint*)",
      "Bash(git status*)",
      "Bash(git diff*)",
      "Bash(git log*)"
    ],
    "deny": [
      "Write(.claude/settings.json)",
      "Write(production/releases/**)",
      "Bash(git push*)",
      "Bash(rm -rf*)"
    ]
  }
}
```

### 3.3 扩展 Hook 示例

若个人需要添加额外的 Hook：

```json
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Write|Edit",
        "command": ".claude/hooks/validate-assets.sh",
        "timeout": 10
      },
      {
        "matcher": "Write|Edit",
        "command": ".claude/hooks/personal-lint.sh",
        "timeout": 15
      }
    ],
    "Notification": [
      {
        "command": ".claude/hooks/notify.sh",
        "timeout": 10
      },
      {
        "command": ".claude/hooks/personal-notify.sh",
        "timeout": 5
      }
    ]
  }
}
```

### 3.4 模型覆盖示例

若个人希望调整模型分配以节省成本：

```json
{
  "model": {
    "default": "sonnet",
    "overrides": {
      "gate-check": "opus",
      "review-all-srs": "opus",
      "architecture-review": "opus",
      "help": "haiku",
      "sprint-status": "haiku",
      "story-readiness": "haiku",
      "scope-check": "haiku",
      "changelog": "haiku",
      "patch-notes": "haiku",
      "onboard": "haiku",
      "project-stage-detect": "haiku"
    }
  }
}
```

### 3.5 完整示例

```json
{
  "permissions": {
    "allow": [
      "Read(**)",
      "Write(src/**)",
      "Write(tests/**)",
      "Write(docs/**)",
      "Write(design/**)",
      "Write(production/sprints/**)",
      "Write(production/session-state/**)",
      "Edit(src/**)",
      "Edit(tests/**)",
      "Edit(docs/**)",
      "Bash(npm *)",
      "Bash(git status*)",
      "Bash(git diff*)",
      "Bash(git log*)",
      "Bash(git add*)",
      "Bash(git commit*)"
    ],
    "deny": [
      "Write(.claude/settings.json)",
      "Edit(.claude/settings.json)",
      "Write(.claude/agents/**)",
      "Edit(.claude/agents/**)",
      "Write(production/releases/**)",
      "Bash(git push*)",
      "Bash(rm -rf*)",
      "Bash(sudo *)"
    ]
  },
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Write|Edit",
        "command": ".claude/hooks/validate-assets.sh",
        "timeout": 10
      }
    ]
  },
  "model": {
    "default": "sonnet",
    "overrides": {
      "gate-check": "opus",
      "architecture-review": "opus",
      "review-all-srs": "opus",
      "help": "haiku",
      "sprint-status": "haiku",
      "story-readiness": "haiku",
      "scope-check": "haiku",
      "changelog": "haiku",
      "patch-notes": "haiku"
    }
  },
  "env": {
    "LOCAL_ENV": "development",
    "DEBUG": "true",
    "MAX_CONTEXT_TOKENS": "150000"
  }
}
```

## 4. 创建步骤

1. 复制全局设置作为基础：
   ```bash
   cp .claude/settings.json .claude/settings.local.json
   ```

2. 编辑 `.claude/settings.local.json`，根据个人需求调整：
   - 修改 `permissions` 中的 `allow` 和 `deny` 列表
   - 添加或覆盖 `hooks` 配置
   - 调整 `model` 中的 `overrides`
   - 设置 `env` 中的个人环境变量

3. 确保文件不被提交到版本控制：
   ```bash
   echo ".claude/settings.local.json" >> .gitignore
   ```

4. 验证配置生效：
   - 运行 `claude` 命令
   - 检查权限和模型分配是否符合预期

## 5. 注意事项

- **不要删除必需权限**：即使个人配置也应保留基本读写权限
- **不要禁用安全 Hook**：`validate-commit.sh` 和 `validate-push.sh` 应保留
- **模型降级谨慎**：将高风险任务降级至 Haiku 可能影响质量
- **团队同步**：个人配置变更不需同步，但应确保不影响共享工作流
- **定期审查**：定期检查个人配置是否过期或冗余

## 6. 常见配置场景

| 场景 | 配置内容 |
| --- | --- |
| 限制写入范围 | 在 `deny` 中添加不应修改的路径 |
| 添加个人工具命令 | 在 `allow` 的 `Bash` 中添加命令模式 |
| 节省成本 | 将更多技能降级至 Haiku |
| 提升审查质量 | 将审查类技能升级至 Opus |
| 本地调试 | 在 `env` 中设置 `DEBUG=true` |
| 添加个人 Hook | 在 `hooks` 中添加个人脚本 |