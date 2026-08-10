---
paths:
  - "**/config/**"
  - "**/*.env*"
platforms:
  claude-code: {enabled: true, path: .claude/rules/config-code.md}
  cursor: {enabled: true, type: auto-attached, globs: "**/config/**, **/*.env*"}
  codex: {enabled: true, section: agents.md}
  windsurf: {enabled: true, mode: append}
  trae: {enabled: true, mode: append}
---
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