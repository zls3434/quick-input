# Software Engineering Studios — Trae 项目规则

## 项目概述

Software Engineering Studios 将一个 AI 编码会话转变为一间完整的软件工程开发工作室。通过 40 个协调的子 Agent 团队，为 AI 辅助开发赋予真实开发团队的结构、流程和质量关卡。

核心资产：40 个 Agent、72 个技能、12 个 Hooks、11 个路径范围编码规范、35 个文档模板。

三层 Agent 架构：
- 总监层（3）：product-director, chief-architect, project-manager（Opus 模型）
- 部门负责人层（7）：system-analyst, lead-developer, tech-architect, ux-design-lead, qa-lead, devops-lead, security-lead（Sonnet 模型）
- 专家层（30）：技术栈专家、工程专家、领域专家（Sonnet/Haiku 模型）

## 技术栈

- 前端框架：React / Vue / Angular（通过 /setup-stack 初始化）
- 后端语言：Node.js / Python / Java / Go
- 数据库：PostgreSQL / MongoDB / Redis
- 版本控制：Git，采用基于主干的开发模式
- 构建系统：根据技术栈选择（Vite/Webpack/esbuild/Angular CLI、npm/pip/gradle/go build）
- 部署平台：AWS / Azure / GCP / 自托管

> 存在针对 React、Vue、Angular、Node.js、Python、Java、Go 的技术栈专家 Agent。请使用与项目匹配的集合。

## 代码风格

- 组件名 PascalCase，文件名 kebab-case
- 禁止硬编码、禁止 any 类型、禁止空 catch 块
- 遵循 Conventional Commits
- 所有用户可见文本必须支持 i18n 国际化
- 所有配置必须通过环境变量提供，密钥禁止硬编码
- 测试遵循 AAA 模式（Arrange-Act-Assert），测试必须独立、确定、无外部依赖
- 使用标准 Markdown 格式，需求文档包含 8 个必需章节
- ADR 遵循标准格式，所有文档必须含元数据

## Agent 协调规则

1. 纵向委派，不可越级：上层可向下层委派，禁止越级向下委派
2. 横向协商，无单方面决定权：同层 Agent 通过协商解决跨领域问题
3. 冲突由上一层级仲裁：同层协商 → 上级仲裁 → 用户最终决策
4. 变更须传播：架构变更通过 /propagate-design-change 传播
5. 禁止单方面跨领域变更：不得修改不属于自己领域的文件

## 协作协议

用户驱动的协作，而非自主执行。每个任务遵循五步流程：
1. 提问 — Agent 在提出解决方案前先提问，理解用户意图
2. 呈现选项 — Agent 展示 2-4 个选项及其优缺点
3. 决策 — 用户始终掌握决策权
4. 草稿 — Agent 在最终确认前展示成果
5. 审批 — 未经用户签字确认，不会写入任何内容

关键约束：
- Agent 在使用 Write/Edit 工具前必须询问："我可以将此写入 [文件路径] 吗？"
- Agent 在请求审批前必须展示草稿或摘要
- 多文件修改需要针对完整变更集的明确审批
- 未经用户指示不得进行提交

## 路径范围规则

### api-code.md

# API 规范

- 遵循 RESTful 标准
  - 使用正确的 HTTP 方法：GET 查询、POST 创建、PUT 整体更新、PATCH 部分更新、DELETE 删除
  - 使用正确的状态码：200 成功、201 创建、204 无内容、400 参数错误、401 未认证、403 无权限、404 不存在、409 冲突、422 校验失败、500 服务器错误
  - 禁止用 GET 执行写操作，禁止用 POST 执行删除
- API 必须版本管理
  - 路径版本：`/api/v1/resources`
  - 新版本不破坏旧版本契约
- 统一错误响应格式
  - 所有错误返回统一 JSON 结构：`{ code, message, details, requestId }`
- 幂等性设计
  - 写操作应支持幂等键（`Idempotency-Key` 头）
  - 重复请求返回相同结果，不产生副作用
- 请求与响应 Schema 验证
  - 使用 JSON Schema 或 Zod / Joi 校验入参
  - 禁止未经验证的数据进入业务层
- 分页标准统一
  - 使用 `cursor` 游标分页或 `page/pageSize` 偏移分页
  - 响应包含分页元信息：`{ items, nextCursor, hasMore }`
- 速率限制
  - 公开端点必须配置速率限制
  - 响应头包含 `X-RateLimit-Limit` / `X-RateLimit-Remaining` / `X-RateLimit-Reset`

## 示例

**正确**：
```http
POST /api/v1/orders HTTP/1.1
Content-Type: application/json
Idempotency-Key: 9b1d2f3a-...

Response: 201 Created
{
  "data": { "id": "ord_123", "status": "pending" },
  "requestId": "req_abc"
}
```

**错误**：
```http
POST /api/createOrder  (非 RESTful、无版本)
Response: 200 OK        (创建应返回 201)
{ "error": "something wrong" }  (非标准错误格式)
```

---

### backend-code.md

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

---

### config-code.md

# 配置规范

- 所有配置必须通过环境变量提供
  - 禁止在代码中硬编码配置值
  - 使用配置加载器读取环境变量
- 密钥禁止硬编码
  - 密钥、令牌、密码必须来自环境变量或密钥管理服务
  - 禁止将密钥写入配置文件提交到版本控制
- 配置分层管理
  - `default` 基础配置
  - `development` 开发环境覆盖
  - `production` 生产环境覆盖
  - 环境配置覆盖默认配置，不替换
- 敏感信息禁止提交到版本控制
  - `.env` 文件加入 `.gitignore`
  - 仅提交 `.env.example` 作为模板
  - 密钥示例值使用占位符（如 `your-api-key-here`）
- 配置验证在启动时执行
  - 应用启动时校验所有必需配置项
  - 缺少必需配置时立即失败并报错

## 示例

**正确**：
```typescript
// config/loader.ts
import { z } from 'zod';

const schema = z.object({
  PORT: z.coerce.number().default(3000),
  DATABASE_URL: z.string().url(),
  API_KEY: z.string().min(20),
  NODE_ENV: z.enum(['development', 'production']),
});

const config = schema.parse(process.env);
export default config;

// .env.example（可提交）
// DATABASE_URL=postgresql://user:pass@localhost:5432/db
// API_KEY=your-api-key-here
```

**错误**：
```typescript
// 硬编码密钥
const db = connect('mongodb://admin:password123@prod-db:27017'); // 泄露密钥
const apiKey = 'sk-abc123def456'; // 硬编码

// .env 被提交到 git（未加入 .gitignore）
```

---

### database-code.md

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

---

### docs-standards.md

# 文档规范

- 使用标准 Markdown 格式
  - 使用 ATX 标题（`#` 而非下划线）
  - 代码块标注语言
  - 表格使用标准 Markdown 表格语法
- 需求文档包含 8 个必需章节
  1. 概述与背景
  2. 目标与非目标
  3. 用户故事
  4. 功能需求
  5. 非功能需求
  6. 验收标准
  7. 依赖与约束
  8. 开放问题
- ADR（架构决策记录）遵循标准格式
  - 标题：`ADR-NNN: 决策标题`
  - 状态：提议 / 已接受 / 已废弃 / 已替代
  - 上下文
  - 决策
  - 结果影响
- 所有文档必须含元数据
  - 状态：草稿 / 评审中 / 已批准 / 已归档
  - 作者
  - 更新日期（ISO 8601 格式）

## 示例

**正确**：
```markdown
---
状态: 已批准
作者: 张三
更新日期: 2026-07-07
---

# ADR-001: 使用 PostgreSQL 作为主数据库

## 状态
已接受

## 上下文
系统需要支持事务、复杂查询、高并发读写...

## 决策
采用 PostgreSQL 16，理由如下...

## 结果影响
正面：ACID 事务、成熟生态
负面：需运维 PostgreSQL 集群
```

**错误**：
```markdown
# 数据库选型
用 PostgreSQL 吧。  (无元数据、无 ADR 结构、无章节)
```

---

### frontend-code.md

# 前端代码规范

- 所有组件必须使用 TypeScript 编写，禁止使用 `any` 类型
  - 需要明确类型时应使用 `unknown` 并进行类型收窄
  - 禁止使用 `// @ts-ignore` 或 `// @ts-nocheck` 绕过类型检查
- 状态管理统一使用 store（如 Zustand / Redux Toolkit / Pinia），禁止散落的 `useState` 导致状态难以追踪
  - 跨组件共享状态必须放入全局 store
  - 仅组件内部局部状态可使用 `useState`
- 所有 API 调用必须通过统一的 API 层（如 `src/frontend/api/`），禁止在组件中直接使用 `fetch` / `axios`
  - API 层负责错误处理、鉴权头注入、重试策略
- 所有用户可见文本必须支持 i18n 国际化
  - 禁止在 JSX 中硬编码中英文字符串
  - 使用 `t('key')` 形式引用翻译键
- 组件遵循单一职责原则，一个组件只做一件事
  - 超过 300 行的组件必须拆分
- 所有 props 必须定义 TypeScript 接口类型，禁止使用隐式 `any`
  - 使用 `interface` 或 `type` 显式声明 Props
- 禁止使用内联样式（`style={{...}}`），必须使用 CSS 模块或 Tailwind 等方案
  - 动态样式应通过 className 切换实现

## 示例

**正确**：
```tsx
// 组件通过 API 层调用，状态来自 store，文本支持 i18n
interface UserCardProps {
  userId: string;
}

export function UserCard({ userId }: UserCardProps) {
  const user = useUserStore((s) => s.users[userId]);
  const { t } = useTranslation();

  return (
    <div className="rounded-lg bg-white p-4 shadow">
      <h3 className="text-lg font-bold">{t('user.profile')}</h3>
      <p>{user?.name}</p>
    </div>
  );
}
```

**错误**：
```tsx
// 直接 fetch、内联样式、硬编码文本、any 类型
export function UserCard(props: any) {
  const [user, setUser] = useState(null);

  useEffect(() => {
    fetch(`/api/users/${props.userId}`)
      .then((r) => r.json())
      .then(setUser);
  }, []);

  return (
    <div style={{ padding: '16px', background: 'white' }}>
      <h3>用户资料</h3>
      <p>{user?.name}</p>
    </div>
  );
}
```

---

### infra-code.md

# 基础设施规范

- 基础设施即代码（IaC）
  - 所有基础设施通过代码定义，禁止手动配置
  - 使用 Terraform / Pulumi / Helm 等工具
- 容器镜像使用特定版本 tag，禁止使用 `latest`
  - 使用 `node:20.11-alpine` 而非 `node:latest`
  - 基础镜像同样锁定版本
- CI/CD 管线配置纳入版本控制
  - 管线定义文件提交到仓库
  - 禁止在 CI 平台 UI 中手动修改管线
- 环境变量注入，禁止硬编码
  - 镜像构建时通过 ARG 注入
  - 运行时通过环境变量或 Secret 注入
- 健康检查端点必需
  - 容器配置 `HEALTHCHECK` 指令
  - 服务暴露 `/health` 或 `/readiness` 端点

## 示例

**正确**：
```dockerfile
# Dockerfile
FROM node:20.11-alpine AS builder
WORKDIR /app
COPY package*.json ./
RUN npm ci
COPY . .
RUN npm run build

FROM node:20.11-alpine
WORKDIR /app
COPY --from=builder /app/dist ./dist
EXPOSE 3000
HEALTHCHECK --interval=30s --timeout=3s \
  CMD wget -qO- http://localhost:3000/health || exit 1
CMD ["node", "dist/main.js"]
```

```yaml
# docker-compose.yml
services:
  api:
    image: myapp:1.2.3  # 特定版本，非 latest
    environment:
      - DATABASE_URL=${DATABASE_URL}  # 注入环境变量
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:3000/health"]
```

**错误**：
```dockerfile
FROM node:latest  # 禁止 latest
ENV API_KEY=sk-abc123  # 禁止硬编码密钥
# 无 HEALTHCHECK
```

---

### prototype-code.md

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

---

### shared-code.md

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

---

### test-standards.md

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

---

### ui-code.md

# UI 代码规范

- 组件可访问性达到 WCAG AA 级标准
  - 所有交互元素可通过键盘访问
  - 颜色对比度至少 4.5:1
  - 表单字段必须有关联的 label
- 响应式设计
  - 组件在移动端、平板、桌面均可用
  - 使用响应式断点，禁止固定像素宽度
- 组件复用，禁止重复造轮子
  - 先查找现有组件库再开发
  - 公共组件放入 `components/` 共享目录
- 语义化 HTML
  - 使用 `<nav>` `<main>` `<article>` `<section>` 等语义标签
  - 禁止滥用 `<div>` 替代语义标签
- ARIA 标签正确使用
  - 图标按钮必须有 `aria-label`
  - 动态内容使用 `aria-live` 通知
  - 禁止冗余 ARIA（如 `<button>` 上再加 `role="button"`）
- 键盘导航支持
  - 所有可交互元素支持 Tab 导航
  - 自定义组件实现 `tabindex` 和键盘事件
  - 焦点可见（focus 样式不可移除）

## 示例

**正确**：
```tsx
<button
  aria-label="关闭对话框"
  className="focus:ring-2 focus:ring-blue-500"
  onClick={onClose}
>
  <CloseIcon />
</button>

<nav aria-label="主导航">
  <ul className="flex flex-col md:flex-row">
    <li><a href="/home">首页</a></li>
  </ul>
</nav>
```

**错误**：
```tsx
<div onClick={onClose} style={{cursor:'pointer'}}>  {/* div 不可键盘访问 */}
  <CloseIcon />
</div>

<div className="navigation">  {/* 应使用 <nav> */}
  <div><div>首页</div></div>
</div>
```

## 可用工作流

通过技能系统调用，完整目录见 .trae/skills/。关键工作流包括：
- /start — 首次上手引导
- /brainstorm — 产品概念创意构思
- /product-concept — 产品概念文档生成
- /create-epics — 创建史诗
- /create-stories — 创建用户故事
- /dev-story — 开发故事
- /code-review — 代码审查
- /qa-plan — 质量保证计划
- /release-checklist — 发布检查清单
- /setup-stack — 技术栈初始化
- /adopt — 已有项目接入框架
