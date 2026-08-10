---
paths:
  - "tools/**"
  - "**/Dockerfile*"
  - "**/*.yml"
  - "**/docker-compose*"
platforms:
  claude-code: {enabled: true, path: .claude/rules/infra-code.md}
  cursor: {enabled: true, type: auto-attached, globs: "tools/**, **/Dockerfile*, **/*.yml, **/docker-compose*"}
  codex: {enabled: true, section: agents.md}
  windsurf: {enabled: true, mode: append}
  trae: {enabled: true, mode: append}
---
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