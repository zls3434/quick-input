<!-- Software Engineering Studios -->
# 测试指令

本文件是平台无关的测试规范源，定义测试命令、测试标准和覆盖率要求。

## 1. 测试命令

根据后端语言选择对应的测试命令。

### 1.1 Node.js

```bash
# 运行所有测试
npm test

# 运行单个测试文件
npm test -- auth.test.ts

# 运行并生成覆盖率报告
npm test -- --coverage

# 监听模式
npm run test:watch
```

### 1.2 Python

```bash
# 运行所有测试
pytest

# 运行指定模块测试
pytest tests/unit/test_auth.py

# 生成覆盖率报告
pytest --cov=src --cov-report=html

# 显示详细输出
pytest -v
```

### 1.3 Go

```bash
# 运行所有测试
go test ./...

# 运行并显示详细输出
go test -v ./...

# 生成覆盖率报告
go test -coverprofile=coverage.out ./...
go tool cover -html=coverage.out

# 运行基准测试
go test -bench=. ./...
```

### 1.4 Java（Maven）

```bash
# 运行所有测试
mvn test

# 运行指定测试类
mvn test -Dtest=UserServiceTest

# 生成覆盖率报告
mvn test jacoco:report

# 运行集成测试
mvn verify
```

### 1.5 前端（通用）

```bash
# 运行单元测试
npm run test:unit

# 运行端到端测试
npm run test:e2e

# 运行并生成覆盖率
npm run test:unit -- --coverage
```

## 2. 测试标准

### 2.1 AAA 结构

所有测试必须遵循 AAA（Arrange-Act-Assert）模式：

- **Arrange（准备）**：设置测试前置条件和数据
- **Act（执行）**：调用被测代码
- **Assert（断言）**：验证结果符合预期

每个阶段使用注释分隔，结构清晰。

**示例：**

```typescript
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

### 2.2 隔离性

- 每个测试独立运行，不依赖其他测试的执行顺序
- 测试之间不共享可变状态
- 每个测试前后清理状态（setUp/tearDown）
- 数据库测试使用事务回滚或独立数据库实例
- 单元测试禁止调用外部 API，外部依赖必须 Mock 或 Stub

### 2.3 确定性

- 测试结果必须稳定可重复
- 禁止依赖当前时间、随机数、网络状态等不确定因素
- 时间相关测试使用 mock 时钟或固定时间
- 随机数测试使用固定种子

### 2.4 测试命名规范

- 测试文件：`[被测模块名].test.[ext]` 或 `[被测模块名].spec.[ext]`
- 测试用例：`should [期望行为] when [前置条件]`
- 示例：`should throw InvalidUserIdError when userId is empty`

### 2.5 测试数据管理

- 禁止硬编码测试数据，使用工厂函数生成
- 示例：`createUserFixture()` 而非直接写死 `id: 123`
- 测试配置与生产配置分离

## 3. 测试覆盖率要求

### 3.1 按故事类型的证据要求

| 故事类型 | 测试方式 | 证据要求 | 强制级别 |
|----------|----------|----------|----------|
| **逻辑**（业务规则、算法、状态转换） | 单元测试 | 覆盖正常/异常/边界路径的单元测试报告 | BLOCKING（必须通过才能合并） |
| **集成**（跨模块、跨服务交互） | 集成测试 | 集成测试报告，覆盖接口契约 | BLOCKING（必须通过才能合并） |
| **UI**（界面展示、交互、视觉） | 手动走查 | 走查清单与截图记录 | ADVISORY（建议但不阻塞） |
| **配置**（环境配置、参数调整） | 冒烟测试 | 冒烟测试通过确认 | ADVISORY（建议但不阻塞） |

### 3.2 覆盖率目标

| 指标 | 目标 | 说明 |
|------|------|------|
| 行覆盖率 | >= 80% | 已执行代码行占总代码行的比例 |
| 分支覆盖率 | >= 70% | 已执行分支占总分支的比例 |
| 函数覆盖率 | >= 80% | 已调用函数占总函数的比例 |

> 覆盖率由 `qa-lead` 负责把关，未达标不予合并。

## 4. 测试目录结构

```
tests/
├── unit/          # 单元测试，与源代码结构镜像
├── integration/   # 集成测试，测试模块间交互和接口契约
├── e2e/           # 端到端测试，测试完整用户旅程
└── performance/   # 性能测试，包括负载测试、压力测试、耐久测试
```

## 5. CI 集成

### 5.1 CI 流水线必须包含的测试阶段

1. 代码静态检查（Lint）
2. 单元测试
3. 集成测试
4. 构建验证
5. 安全扫描

### 5.2 合并门禁

| 门禁 | 检查项 | 负责人 |
|------|--------|--------|
| 代码审查 | 至少一名审查者 approval | `code-reviewer` |
| 单元测试 | 覆盖率达标，全部通过 | `qa-lead` |
| 集成测试 | 关键路径全部通过 | `qa-lead` |
| 安全扫描 | 无严重漏洞 | `security-lead` |
| 构建验证 | 构建成功，产物完整 | `devops-lead` |

> CI 失败禁止合并到主干。