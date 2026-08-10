---
paths:
  - "src/database/**"
  - "**/migrations/**"
platforms:
  claude-code: {enabled: true, path: .claude/rules/database-code.md}
  cursor: {enabled: true, type: auto-attached, globs: "src/database/**, **/migrations/**"}
  codex: {enabled: true, section: agents.md}
  windsurf: {enabled: true, mode: append}
  trae: {enabled: true, mode: append}
---
# 数据库规范

- 迁移脚本一经提交不可修改，只能新增
  - 修改已上线迁移会破坏环境一致性
  - 需要变更时编写新迁移文件
- 索引策略必须文档化
  - 每个索引说明查询场景、基数、写入开销
  - 禁止随意添加索引导致写入性能下降
- JSON 字段必须进行有效性验证
  - 使用 Schema 校验 JSON 字段内容
  - 禁止存储未验证的任意 JSON
- 外键约束必须显式命名
  - 命名规则：`fk_表名_引用表名`
  - 禁止依赖数据库自动生成的外键名
- 连接池配置必须外部化
  - 池大小、超时、最大连接数来自配置
  - 禁止在代码中硬编码连接池参数
- 查询禁止使用 `SELECT *`
  - 显式列出所需字段
  - 避免字段变更导致的隐式破坏

## 示例

**正确**：
```sql
-- 迁移：20260707_001_add_user_status.sql（只新增，不改旧的）
ALTER TABLE users ADD COLUMN status VARCHAR(20) DEFAULT 'active';
CREATE INDEX idx_users_status ON users(status);
-- 索引文档：用于按状态筛选用户列表，基数低（约5种状态），写入开销可接受

-- 查询显式字段
SELECT id, name, email, status FROM users WHERE status = 'active';
```

**错误**：
```sql
SELECT * FROM users;  -- 禁止 SELECT *

-- 修改已上线迁移
-- 文件：20260101_001_init.sql（已提交，不应改动）
```