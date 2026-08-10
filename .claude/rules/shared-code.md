---
paths:
  - "src/shared/**"
platforms:
  claude-code: {enabled: true, path: .claude/rules/shared-code.md}
  cursor: {enabled: true, type: auto-attached, globs: "src/shared/**"}
  codex: {enabled: true, section: agents.md}
  windsurf: {enabled: true, mode: append}
  trae: {enabled: true, mode: append}
---
# 共享代码规范

- 类型定义统一导出
  - 所有公共类型从 `src/shared/types/` 导出
  - 使用 `index.ts` 聚合导出，便于引用
- 接口契约必须文档化
  - 每个公共接口附带 JSDoc 注释
  - 说明参数、返回值、异常、副作用
- 跨端兼容
  - 共享代码不得引用前端特定模块（如 DOM、`window`）
  - 不得引用后端特定模块（如 `fs`、`process`）
  - 前端/后端专属逻辑各自留在对应目录
- 版本化 API 变更
  - 公共接口变更需评估影响范围
  - 破坏性变更需提升主版本号
- 向后兼容
  - 新增字段为可选，不破坏现有调用方
  - 废弃字段先标记 `@deprecated` 再移除
  - 移除前至少保留一个版本的过渡期

## 示例

**正确**：
```typescript
// src/shared/types/user.ts

/**
 * 用户实体类型
 * @interface User
 * @property {string} id - 用户唯一标识
 * @property {string} name - 显示名称
 * @property {string} email - 邮箱地址
 * @property {string} [avatar] - 头像 URL（可选）
 */
export interface User {
  id: string;
  name: string;
  email: string;
  avatar?: string;  // 可选，向后兼容
}

/**
 * @deprecated 请改用 UserV2，将在 v3 移除
 */
export interface LegacyUser {
  user_id: string;
  display_name: string;
}
```

**错误**：
```typescript
// src/shared/utils.ts
import { window } from '...';      // 引用前端特定对象
import { readFileSync } from 'fs';  // 引用后端特定模块

export interface User {
  id: string;
  name: string;
  avatar: string;  // 改为必填，破坏现有调用方
}
// 无 JSDoc、无版本管理
```