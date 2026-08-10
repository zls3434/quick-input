---
paths:
  - "tests/**"
platforms:
  claude-code: {enabled: true, path: .claude/rules/test-standards.md}
  cursor: {enabled: true, type: auto-attached, globs: "tests/**"}
  codex: {enabled: true, section: agents.md}
  windsurf: {enabled: true, mode: append}
  trae: {enabled: true, mode: append}
---
# 测试标准

- 测试文件命名：`test_[module]_[feature].ts`
  - 示例：`test_auth_token-refresh.ts`
- 测试函数命名：`test_[scenario]_[expected]`
  - 示例：`test_expired-token_throws-error`
- 测试结构遵循 AAA 模式（Arrange-Act-Assert）
  - 分三个清晰阶段：准备、执行、断言
  - 每个阶段使用注释分隔
- 测试隔离性
  - 每个测试独立运行，不依赖其他测试
  - 不依赖测试执行顺序
  - 每个测试前后清理状态
- 测试确定性
  - 禁止使用随机种子、当前时间等非确定性输入
  - 需要时间时使用固定时间或注入时钟
  - 需要随机时使用固定种子
- 禁止硬编码测试数据，使用工厂函数生成
  - `createUserFixture()` 而非直接写死 `id: 123`
- 单元测试禁止调用外部 API
  - 外部依赖必须 Mock 或 Stub
  - 集成测试单独标记，不在单元测试范围内

## 示例

**正确**：
```typescript
// test_auth_token-refresh.ts
import { createTokenFixture } from './factories';

describe('Token 刷新', () => {
  it('test_expired-token_throws-error', async () => {
    // Arrange
    const token = createTokenFixture({ expiresAt: FIXED_TIME });

    // Act
    const result = () => refresh(token);

    // Assert
    expect(result).rejects.toThrow(TokenExpiredError);
  });
});
```

**错误**：
```typescript
// 依赖顺序、硬编码、随机时间
it('should work', async () => {
  const id = Math.random();          // 非确定性
  const token = { id: 123, ... };    // 硬编码
  // 无 Arrange/Act/Assert 分隔
});
```