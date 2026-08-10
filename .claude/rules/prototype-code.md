---
paths:
  - "prototypes/**"
platforms:
  claude-code: {enabled: true, path: .claude/rules/prototype-code.md}
  cursor: {enabled: true, type: auto-attached, globs: "prototypes/**"}
  codex: {enabled: true, section: agents.md}
  windsurf: {enabled: true, mode: append}
  trae: {enabled: true, mode: append}
---
# 原型规范

- 适用宽松标准
  - 允许硬编码值（数据、URL、配置）
  - 允许使用 TODO 标记待完善项
  - 不要求完整类型定义、测试覆盖、错误处理
- README 必需
  - 说明原型的假设和前提条件
  - 列明已知限制和不在范围内的功能
  - 说明运行方式和依赖
- 明确标注为一次性代码
  - 代码顶部注释标明：`// PROTOTYPE: 一次性代码，不可用于生产`
  - 原型不得进入 `src/` 目录
- 禁止与 `src/` 共享代码
  - 原型应自包含，不引用生产代码
  - 避免生产代码反向依赖原型

## 示例

**正确**：
```typescript
// PROTOTYPE: 一次性代码，不可用于生产
// 用途：验证实时协作编辑器的冲突解决算法
// 限制：仅支持 2 个客户端，未实现持久化

const users = ['alice', 'bob']; // 硬编码可接受

// README.md
// # 实时协作编辑器原型
// ## 假设
// - 同时编辑的用户不超过 2 人
// - 网络延迟低于 500ms
// ## 限制
// - 不实现断线重连
// - 不支持富文本，仅纯文本
// ## 运行
// npm install && npm run dev
```

**错误**：
```typescript
// 无 README、无标注、引用生产代码
import { productionLogger } from '../src/utils/logger'; // 禁止引用 src/

const data = fetch('/api/users'); // 硬编码 URL 且无标注
// 无任何说明直接提交
```