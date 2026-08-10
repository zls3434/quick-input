<!-- Software Engineering Studios -->
# 规则参考

本文件列出 Software Engineering Studios 中配置的 11 个规则（Rule），说明其适用范围和核心要求。所有规则文件位于 `.claude/rules/` 目录。

## 1. 规则总览

| 序号 | 规则名称 | 适用范围（glob） | 核心要求 |
| --- | --- | --- | --- |
| 1 | `frontend-code` | `src/frontend/**` | 组件化、TypeScript、状态管理 |
| 2 | `backend-code` | `src/backend/**` | 线程安全、依赖注入、事务 |
| 3 | `api-code` | `src/api/**`、`docs/api/**` | RESTful、版本、幂等 |
| 4 | `database-code` | `src/database/**`、`**/migrations/**` | 迁移、索引、JSON |
| 5 | `test-standards` | `tests/**` | 命名、AAA、隔离、确定 |
| 6 | `ui-code` | `src/frontend/**/components/**` | 可访问性、响应式、复用 |
| 7 | `config-code` | `**/config/**`、`**/*.env*` | 环境变量、密钥 |
| 8 | `prototype-code` | `prototypes/**` | 宽松、README |
| 9 | `docs-standards` | `design/**`、`docs/**` | Markdown、必需章节 |
| 10 | `infra-code` | `tools/**`、`**/Dockerfile*`、`**/*.yml` | IaC、容器化 |
| 11 | `shared-code` | `src/shared/**` | 类型定义、接口契约 |

## 2. 各规则详细说明

### 2.1 frontend-code（前端代码规则）

- **适用范围**：`src/frontend/**`
- **核心要求**：

| 要求 | 说明 |
| --- | --- |
| 组件化 | 所有 UI 元素封装为可复用组件，单一职责 |
| TypeScript | 必须使用 TypeScript，禁止 `any` 类型 |
| 状态管理 | 使用框架推荐的状态管理方案，避免组件内直接管理全局状态 |
| 命名规范 | 组件名 PascalCase，文件名与组件名一致 |
| 样式隔离 | 组件样式使用 CSS Modules 或 CSS-in-JS，避免全局样式污染 |
| 响应式设计 | 组件必须支持响应式布局 |
| 可访问性 | 组件必须满足 WCAG AA 标准 |

### 2.2 backend-code（后端代码规则）

- **适用范围**：`src/backend/**`
- **核心要求**：

| 要求 | 说明 |
| --- | --- |
| 线程安全 | 共享状态访问必须线程安全，使用锁或无锁数据结构 |
| 依赖注入 | 优先依赖注入，避免单例（除非审批通过） |
| 事务管理 | 数据库操作必须在事务边界内，明确事务传播行为 |
| 错误处理 | 所有错误必须显式处理，禁止空 catch 块 |
| 日志记录 | 关键操作必须记录结构化日志 |
| API 文档 | 公共 API 必须有文档注释 |
| 配置外置 | 配置通过环境变量或配置文件注入，禁止硬编码 |

### 2.3 api-code（API 代码规则）

- **适用范围**：`src/api/**`、`docs/api/**`
- **核心要求**：

| 要求 | 说明 |
| --- | --- |
| RESTful | 遵循 REST 设计原则，资源命名使用名词复数 |
| 版本控制 | API 必须版本化（URI 版本或 Header 版本） |
| 幂等性 | GET/PUT/DELETE 操作必须幂等，POST 操作明确非幂等语义 |
| 状态码 | 使用标准 HTTP 状态码，不滥用 200 |
| 错误格式 | 错误响应使用统一格式（错误码、消息、详情） |
| 分页 | 列表接口必须支持分页和排序 |
| 限流 | 公开接口必须配置限流 |

### 2.4 database-code（数据库代码规则）

- **适用范围**：`src/database/**`、`**/migrations/**`
- **核心要求**：

| 要求 | 说明 |
| --- | --- |
| 迁移 | 所有 schema 变更通过迁移文件管理，禁止手动修改 |
| 迁移可逆 | 每个迁移必须有对应的回滚脚本 |
| 索引 | 外键和常用查询字段必须建索引 |
| JSON 字段 | JSON 字段需标注结构，避免无结构 JSON |
| 命名 | 表名 snake_case 复数，列名 snake_case |
| 约束 | 数据完整性通过数据库约束保证，不仅依赖应用层 |
| 软删除 | 根据业务需要选择软删除或硬删除，保持一致性 |

### 2.5 test-standards（测试标准规则）

- **适用范围**：`tests/**`
- **核心要求**：

| 要求 | 说明 |
| --- | --- |
| 命名 | 测试用例：`should [期望行为] when [前置条件]` |
| AAA | 测试结构：Arrange-Act-Assert，三段清晰分隔 |
| 隔离 | 每个测试独立运行，不依赖执行顺序 |
| 确定性 | 测试结果稳定可重复，不依赖不确定因素 |
| 不硬编码 | 禁止硬编码环境相关数据 |
| 独立性 | 测试不依赖外部服务，使用 mock/stub |
| 覆盖率 | 逻辑故事单元测试覆盖率达标（BLOCKING） |

### 2.6 ui-code（UI 代码规则）

- **适用范围**：`src/frontend/**/components/**`
- **核心要求**：

| 要求 | 说明 |
| --- | --- |
| 可访问性 | 组件满足 WCAG AA 标准，支持键盘导航和屏幕阅读器 |
| 响应式 | 组件支持响应式布局，适配不同屏幕尺寸 |
| 复用 | 组件设计为可复用，通过 props 配置行为 |
| Props 类型 | 所有 props 必须有明确的 TypeScript 类型定义 |
| 默认值 | 可选 props 必须有合理默认值 |
| 文档 | 复杂组件必须包含使用示例 |
| 测试 | 交互逻辑通过单元测试覆盖，视觉通过手动走查 |

### 2.7 config-code（配置代码规则）

- **适用范围**：`**/config/**`、`**/*.env*`
- **核心要求**：

| 要求 | 说明 |
| --- | --- |
| 环境变量 | 环境相关配置通过环境变量注入 |
| 密钥保护 | 密钥、令牌等敏感信息禁止提交到版本控制 |
| .env 模板 | 提供 `.env.example` 模板，列出所有必需变量 |
| 配置分层 | 开发/测试/预发布/生产配置分离 |
| 默认值 | 非敏感配置提供安全默认值 |
| 验证 | 应用启动时验证必需配置项是否存在 |

### 2.8 prototype-code（原型代码规则）

- **适用范围**：`prototypes/**`
- **核心要求**：

| 要求 | 说明 |
| --- | --- |
| 宽松规范 | 原型代码不强制遵循完整编码规范 |
| README | 每个原型必须包含 `README.md` 说明用途、限制和如何运行 |
| 隔离 | 原型代码与生产代码物理隔离，不混入 src/ |
| 标注 | 原型中使用的临时方案需标注 `// PROTOTYPE: 说明` |
| 不可生产 | 原型代码不得直接用于生产环境 |
| 保留期限 | 原型保留至验证完成，之后可清理 |

### 2.9 docs-standards（文档标准规则）

- **适用范围**：`design/**`、`docs/**`
- **核心要求**：

| 要求 | 说明 |
| --- | --- |
| Markdown | 所有文档使用 Markdown 格式 |
| 文件头 | 文档必须包含 `<!-- Software Engineering Studios -->` 文件头 |
| 必需章节 | 设计文档必须包含 8 个必需章节（见 `coding-standards.md`） |
| 命名 | 文件名 kebab-case |
| 链接 | 文档间引用使用相对路径 |
| 版本 | 架构文档标注版本和最后更新时间 |
| 图表 | 复杂流程使用图表辅助说明（Mermaid 或图片） |

### 2.10 infra-code（基础设施代码规则）

- **适用范围**：`tools/**`、`**/Dockerfile*`、`**/*.yml`
- **核心要求**：

| 要求 | 说明 |
| --- | --- |
| IaC | 基础设施通过代码定义，支持版本控制 |
| 容器化 | 应用通过 Dockerfile 容器化，遵循最佳实践 |
| 幂等部署 | 部署脚本必须幂等，可安全重复执行 |
| 镜像优化 | Docker 镜像最小化，使用多阶段构建 |
| 配置外置 | 基础设施配置通过变量注入，不硬编码 |
| 健康检查 | 容器配置健康检查端点 |
| 资源限制 | 容器设置 CPU 和内存限制 |

### 2.11 shared-code（共享代码规则）

- **适用范围**：`src/shared/**`
- **核心要求**：

| 要求 | 说明 |
| --- | --- |
| 类型定义 | 共享类型统一定义在 `src/shared/types/` |
| 接口契约 | 前后端接口契约定义在 `src/shared/contracts/` |
| 无副作用 | 共享代码不得包含副作用，保持纯函数 |
| 无框架依赖 | 共享代码不依赖特定框架（React/Vue/Angular） |
| 常量集中 | 共享常量定义在 `src/shared/constants/` |
| 工具函数 | 通用工具函数定义在 `src/shared/utils/` |
| 版本兼容 | 共享代码变更需考虑前后端兼容性 |

## 3. 规则配置示例

规则在 `.claude/settings.json` 中配置：

```json
{
  "rules": {
    "frontend-code": {
      "glob": "src/frontend/**",
      "file": ".claude/rules/frontend-code.md"
    },
    "backend-code": {
      "glob": "src/backend/**",
      "file": ".claude/rules/backend-code.md"
    },
    "api-code": {
      "glob": ["src/api/**", "docs/api/**"],
      "file": ".claude/rules/api-code.md"
    },
    "database-code": {
      "glob": ["src/database/**", "**/migrations/**"],
      "file": ".claude/rules/database-code.md"
    },
    "test-standards": {
      "glob": "tests/**",
      "file": ".claude/rules/test-standards.md"
    },
    "ui-code": {
      "glob": "src/frontend/**/components/**",
      "file": ".claude/rules/ui-code.md"
    },
    "config-code": {
      "glob": ["**/config/**", "**/*.env*"],
      "file": ".claude/rules/config-code.md"
    },
    "prototype-code": {
      "glob": "prototypes/**",
      "file": ".claude/rules/prototype-code.md"
    },
    "docs-standards": {
      "glob": ["design/**", "docs/**"],
      "file": ".claude/rules/docs-standards.md"
    },
    "infra-code": {
      "glob": ["tools/**", "**/Dockerfile*", "**/*.yml"],
      "file": ".claude/rules/infra-code.md"
    },
    "shared-code": {
      "glob": "src/shared/**",
      "file": ".claude/rules/shared-code.md"
    }
  }
}
```

## 4. 规则与编码规范的关系

- **编码规范**（`coding-standards.md`）：全局通用原则
- **规则**（本文件）：针对特定文件范围的细化要求
- 规则与编码规范可能重叠，规则提供更细粒度的约束
- 冲突时以规则为准（规则更具体）