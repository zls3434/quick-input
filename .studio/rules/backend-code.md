---
paths:
  - "src/backend/**"
platforms:
  claude-code: {enabled: true, path: .claude/rules/backend-code.md}
  cursor: {enabled: true, type: auto-attached, globs: "src/backend/**"}
  codex: {enabled: true, section: agents.md}
  windsurf: {enabled: true, mode: append}
  trae: {enabled: true, mode: append}
---
# 后端代码规范

- 所有 API 端点必须线程安全
  - 共享状态必须使用锁或并发安全数据结构
  - 禁止在请求处理函数中使用可变全局变量
- 热路径（高频调用函数）零内存分配
  - 避免在循环中创建对象、拼接字符串
  - 使用对象池或预分配缓冲区
- 依赖方向严格分层，禁止跨层引用
  - 分层顺序：Controller → Service → Repository → Model
  - 禁止 Repository 直接调用 Controller
  - 禁止 Model 层反向依赖 Service
- 使用依赖注入（DI）管理依赖关系
  - 禁止在类内部直接 `new` 依赖对象
  - 通过构造函数注入或 DI 容器注入
- 所有配置必须来自环境变量或配置文件
  - 禁止在代码中硬编码数据库连接、端口、密钥
- 错误处理使用统一错误处理中间件
  - 禁止在每个端点中重复编写 try-catch 样板
  - 业务错误抛出统一异常类型，由中间件转换为响应
- 数据库操作必须使用事务
  - 涉及多表写入时必须开启事务
  - 事务范围尽可能小，避免长事务

## 示例

**正确**：
```typescript
// 依赖注入、统一错误处理、事务
@Injectable()
export class OrderService {
  constructor(
    private readonly orderRepo: OrderRepository,
    private readonly paymentRepo: PaymentRepository,
    @InjectConfig() private readonly config: AppConfig,
  ) {}

  async createOrder(input: CreateOrderInput): Promise<Order> {
    return this.orderRepo.withTransaction(async (tx) => {
      const order = await this.orderRepo.create(tx, input);
      await this.paymentRepo.record(tx, order.id);
      return order;
    });
  }
}
```

**错误**：
```typescript
// 硬编码配置、无事务、直接 new 依赖
export class OrderService {
  private db = new Database('localhost:5432'); // 硬编码

  async createOrder(input: any) {
    try {
      const order = await this.db.query('INSERT ...');
      await this.db.query('INSERT payment ...'); // 无事务
      return order;
    } catch (e) {
      return { error: e.message }; // 自行处理错误，未走中间件
    }
  }
}
```