<!-- Software Engineering Studios -->
# 技术偏好模板

本文件为技术栈配置模板，初始状态所有技术选项标记为 `[TO BE CONFIGURED]`。运行 `/setup-stack` 命令后由 `chief-architect` 填充实际选择。

## 1. 技术栈选择

### 1.1 前端框架

```
前端框架: [TO BE CONFIGURED]
```

可选选项：
- React — 组件化、生态丰富、适合大型应用
- Vue — 渐进式框架、易上手、灵活
- Angular — 全功能框架、强类型、企业级

**配置说明：**
- 选择后由 `frontend-architect` 和对应技术栈专家（`react`/`vue`/`angular`）共同确认
- 影响目录结构、依赖管理、构建工具选择

### 1.2 后端语言

```
后端语言: [TO BE CONFIGURED]
```

可选选项：
- Node.js — 前后端统一语言、非阻塞 I/O、适合实时应用
- Python — 生态丰富、适合数据处理和 AI、快速开发
- Java — 企业级、强类型、生态成熟
- Go — 高性能、并发原生支持、适合微服务

**配置说明：**
- 选择后由 `backend-architect` 和对应技术栈专家（`node`/`python`/`java`/`go`）共同确认
- 影响项目结构、依赖管理、运行时环境

### 1.3 数据库

```
数据库: [TO BE CONFIGURED]
```

可选选项：
- PostgreSQL — 关系型、功能丰富、支持 JSON 和扩展
- MongoDB — 文档型、灵活模式、适合快速迭代
- Redis — 键值存储、高性能、适合缓存和会话

**配置说明：**
- 选择后由 `database-engineer` 确认
- 可能需要组合使用（如 PostgreSQL 主库 + Redis 缓存）
- 影响数据模型设计和迁移策略

### 1.4 部署平台

```
部署平台: [TO BE CONFIGURED]
```

可选选项：
- AWS — 服务全面、全球覆盖
- Azure — 企业集成、.NET 生态
- GCP — 数据与 AI 优势、Kubernetes 原生
- 自托管 — 完全控制、成本可控

**配置说明：**
- 选择后由 `devops-lead` 和 `cloud-architect` 共同确认
- 影响基础设施即代码方案、CI/CD 管线配置

### 1.5 构建系统

```
构建系统: [TO BE CONFIGURED]
```

**配置说明：**
- 前端构建：根据框架选择（Vite/Webpack/esbuild/Angular CLI）
- 后端构建：根据语言选择（npm/turbopack/pip/gradle/go build）
- 由 `devops-lead` 统一协调

### 1.6 版本控制

```
版本控制: Git，采用基于主干的开发模式
```

## 2. 命名约定

### 2.1 文件命名

```
文件命名约定: [TO BE CONFIGURED]
```

推荐方案：
- **前端组件**：PascalCase（如 `UserProfile.tsx`）
- **前端样式**：kebab-case（如 `user-profile.css`）
- **后端模块**：根据语言惯例（Python: snake_case，Java/Go: PascalCase/camelCase）
- **测试文件**：`[被测名].test.[ext]` 或 `[被测名].spec.[ext]`
- **文档文件**：kebab-case（如 `quick-start.md`）

### 2.2 代码命名

```
代码命名约定: [TO BE CONFIGURED]
```

推荐方案：
- **变量与函数**：camelCase（JavaScript/TypeScript/Java/Go）
- **变量与函数**：snake_case（Python）
- **类与接口**：PascalCase
- **常量**：UPPER_SNAKE_CASE
- **枚举**：PascalCase 类型名，UPPER_SNAKE_CASE 值

### 2.3 Git 分支命名

```
分支命名约定: [TO BE CONFIGURED]
```

推荐方案（基于主干的开发）：
- `main` — 主干
- `feature/[简短描述]` — 功能分支
- `fix/[简短描述]` — 修复分支
- `hotfix/[简短描述]` — 热修复分支
- `release/vX.Y.Z` — 发布分支

## 3. 性能预算

### 3.1 前端性能

```
首屏加载时间: < 3 秒（3G 网络模拟）
首次内容绘制（FCP）: < 1.8 秒
最大内容绘制（LCP）: < 2.5 秒
累积布局偏移（CLS）: < 0.1
首次输入延迟（FID）: < 100 毫秒
```

### 3.2 API 性能

```
API 响应时间（P95）: < 200 毫秒
API 响应时间（P99）: < 500 毫秒
API 吞吐量目标: [TO BE CONFIGURED] 请求/秒
```

### 3.3 资源限制

```
内存上限: [TO BE CONFIGURED]
CPU 上限: [TO BE CONFIGURED]
数据库连接池: [TO BE CONFIGURED]
```

**性能预算由 `performance` 专家和 `devops-lead` 共同制定，由 `/perf-profile` 命令验证。**

## 4. 禁止模式

以下模式在所有代码中严格禁止：

### 4.1 通用禁止

- 禁止硬编码环境相关配置（数据库连接、API 密钥、端口等）
- 禁止使用 `any` 类型（TypeScript）或跳过类型检查
- 禁止在生产代码中使用 `console.log` 或等效调试输出
- 禁止忽略错误（空 catch 块）
- 禁止使用已弃用的 API 或依赖

### 4.2 前端禁止

- 禁止直接操作 DOM（框架提供抽象层时）
- 禁止在组件中直接调用全局状态（应通过依赖注入或 props）
- 禁止内联样式（应使用样式表或 CSS-in-JS 方案）

### 4.3 后端禁止

- 禁止在业务逻辑中直接拼接 SQL（应使用参数化查询或 ORM）
- 禁止在请求处理中执行阻塞 I/O（Node.js 场景）
- 禁止跨请求共享可变状态（除非通过显式共享存储）

### 4.4 数据库禁止

- 禁止在生产环境直接执行破坏性操作（DROP/DELETE/TRUNCATE）
- 禁止创建无索引的外键关联
- 禁止存储明文敏感数据（密码、令牌等）

## 5. 技术栈专家配置说明

选择技术栈后，对应的技术栈专家 Agent 将自动激活并提供以下支持：

| 技术栈选择 | 激活的专家 Agent | 提供支持 |
| --- | --- | --- |
| React | `react`、`frontend-architect` | 组件设计、状态管理、性能优化 |
| Vue | `vue`、`frontend-architect` | 组合式 API、响应式系统、组件设计 |
| Angular | `angular`、`frontend-architect` | 模块组织、依赖注入、RxJS |
| Node.js | `node`、`backend-architect` | 异步编程、中间件设计、API 实现 |
| Python | `python`、`backend-architect` | 异步框架、数据模型、包管理 |
| Java | `java`、`backend-architect` | Spring 生态、JVM 调优、企业模式 |
| Go | `go`、`backend-architect` | 并发模型、接口设计、性能优化 |
| PostgreSQL | `database-engineer` | 模式设计、索引优化、查询调优 |
| MongoDB | `database-engineer` | 文档建模、聚合管道、分片策略 |
| Redis | `database-engineer` | 数据结构选择、持久化策略、集群配置 |

**注意：** 本文件由 `/setup-stack` 命令自动填充。在运行该命令前，所有技术选项保持 `[TO BE CONFIGURED]` 状态。配置完成后，相关 Agent 将引用本文件作为技术决策的依据。