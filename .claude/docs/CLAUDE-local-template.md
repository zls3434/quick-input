<!-- Software Engineering Studios -->
# 个人 CLAUDE.md 模板

本文件提供个人项目配置覆盖模板。创建 `CLAUDE.local.md` 可在不修改团队共享 `CLAUDE.md` 的前提下，为个人开发环境定制配置。该文件不纳入版本控制。

## 1. 用途与位置

- **位置**：项目根目录 `CLAUDE.local.md`
- **用途**：个人项目特定配置覆盖
- **版本控制**：不纳入版本控制（应在 `.gitignore` 中添加 `CLAUDE.local.md`）
- **优先级**：高于 `CLAUDE.md`，低于命令行参数

## 2. 覆盖机制

`CLAUDE.local.md` 与 `CLAUDE.md` 按字段覆盖：
- 技术栈选择：覆盖全局配置中的 `[TO BE CONFIGURED]`
- 命名约定：覆盖全局命名规范
- 团队规范：添加个人或小团队特定规范
- 性能预算：调整性能指标阈值
- 禁止模式：添加额外的禁止项

## 3. 模板内容

以下为个人 `CLAUDE.local.md` 的模板示例，根据实际需求修改后使用：

```markdown
<!-- Software Engineering Studios -->
# 个人项目配置覆盖

本文件覆盖团队共享的 CLAUDE.md 配置，仅适用于个人开发环境。

## 技术栈选择

### 前端框架
前端框架: React

- 使用 React 18+ 版本
- 状态管理使用 Zustand
- 路由使用 React Router v6
- 构建工具使用 Vite

### 后端语言
后端语言: Node.js

- 使用 Node.js 20 LTS
- 框架使用 Express.js
- ORM 使用 Prisma
- 运行时使用 PM2 管理

### 数据库
数据库: PostgreSQL

- 版本 15+
- 主库 + Redis 缓存
- 连接池大小：20

### 部署平台
部署平台: AWS

- EC2 + RDS + ElastiCache
- 区域：us-east-1
- 使用 Terraform 管理基础设施

## 命名约定

### 文件命名
- 前端组件：PascalCase（如 `UserProfile.tsx`）
- 前端样式：PascalCase.module.css（如 `UserProfile.module.css`）
- 后端模块：kebab-case（如 `user-service.js`）
- 测试文件：`[被测名].test.[ext]`（如 `UserService.test.js`）
- 文档文件：kebab-case（如 `api-guide.md`）

### 代码命名
- 变量与函数：camelCase
- 类与接口：PascalCase
- 常量：UPPER_SNAKE_CASE
- 枚举值：PascalCase

### Git 分支命名
- main — 主干
- feature/[简短描述] — 功能分支
- fix/[简短描述] — 修复分支
- hotfix/[简短描述] — 热修复分支

## 团队规范

### 提交规范
- 提交前必须通过本地测试：`npm test`
- 提交前必须通过 lint 检查：`npm run lint`
- 每次提交只包含一个逻辑变更
- 提交信息使用简体中文

### 代码审查
- 个人项目可跳过代码审查（solo 模式）
- 若有协作，至少一名审查者 approval
- 审查重点关注：安全性、性能、可维护性

### 工作流
- 审查模式：solo（个人开发）
- Sprint 周期：1 周
- 每个 Sprint 至少 1 次回顾

## 性能预算

### 前端性能
- 首屏加载时间：< 2 秒
- 首次内容绘制（FCP）：< 1.5 秒
- 最大内容绘制（LCP）：< 2 秒
- 累积布局偏移（CLS）：< 0.1

### API 性能
- API 响应时间（P95）：< 200 毫秒
- API 响应时间（P99）：< 500 毫秒
- API 吞吐量目标：1000 请求/秒

### 资源限制
- 内存上限：512 MB（单服务实例）
- CPU 上限：1 核（单服务实例）
- 数据库连接池：20

## 禁止模式

### 额外禁止项
- 禁止使用 `var` 声明变量（使用 `let` 或 `const`）
- 禁止使用 `==` 比较（使用 `===`）
- 禁止在循环中创建函数（使用高阶函数）
- 禁止直接修改 props
- 禁止在渲染函数中产生副作用

## 个人偏好

### 开发工具
- 编辑器：VS Code
- 代码格式化：Prettier
- Lint：ESLint
- 调试：Chrome DevTools

### 环境配置
- Node.js 版本：20 LTS（使用 nvm 管理）
- 包管理器：pnpm
- 本地端口范围：3000-3999（前端）、4000-4999（后端）
```

## 4. 创建步骤

1. 在项目根目录创建 `CLAUDE.local.md`：
   ```bash
   touch CLAUDE.local.md
   ```

2. 复制上方模板内容至文件中

3. 根据个人需求修改：
   - 填写实际技术栈选择
   - 调整命名约定
   - 设置团队规范
   - 调整性能预算
   - 添加额外禁止模式
   - 配置个人偏好

4. 确保文件不被提交到版本控制：
   ```bash
   echo "CLAUDE.local.md" >> .gitignore
   ```

5. 验证配置生效：
   - 运行 `claude` 命令
   - 检查 Agent 是否使用个人配置

## 5. 配置覆盖示例

### 5.1 仅覆盖技术栈

若只需覆盖技术栈选择，其余保持默认：

```markdown
<!-- Software Engineering Studios -->
# 个人项目配置覆盖

## 技术栈选择

### 前端框架
前端框架: Vue

- 使用 Vue 3 Composition API
- 状态管理使用 Pinia
- 构建工具使用 Vite

### 后端语言
后端语言: Python

- 使用 Python 3.12
- 框架使用 FastAPI
- ORM 使用 SQLAlchemy

### 数据库
数据库: PostgreSQL

### 部署平台
部署平台: GCP
```

### 5.2 仅覆盖审查模式

若只想调整审查模式：

```markdown
<!-- Software Engineering Studios -->
# 个人项目配置覆盖

## 团队规范

### 工作流
- 审查模式: lean
- 跳过可选审查点
- 保留必需审查点
```

### 5.3 多人协作小团队

小团队共享配置（非个人配置，但可作为参考）：

```markdown
<!-- Software Engineering Studios -->
# 小团队项目配置覆盖

## 技术栈选择
前端框架: Angular
后端语言: Java
数据库: PostgreSQL
部署平台: Azure

## 团队规范
- 审查模式: lean
- 代码审查: 至少一名团队成员 approval
- Sprint 周期: 2 周
- 分支策略: feature 分支开发，PR 合并至主干
```

## 6. 注意事项

- **不覆盖安全相关配置**：权限和 Hook 配置应在 `settings.local.json` 中管理
- **保持与团队配置兼容**：个人配置不应破坏共享工作流
- **技术栈选择一致性**：一旦在 `CLAUDE.local.md` 中确定技术栈，应与 `technical-preferences.md` 保持一致
- **定期审查**：定期检查个人配置是否与项目进展匹配
- **文档引用**：个人配置中可引用 `docs/` 下的规范文件作为补充说明