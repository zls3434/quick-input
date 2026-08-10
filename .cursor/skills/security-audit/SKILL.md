---
name: security-audit
description: "审计安全漏洞，覆盖注入、XSS、CSRF、认证绕过、数据暴露等常见风险并生成含严重级别的修复报告。"
license: MIT
metadata:
  model: sonnet
  argument-hint: ""
  user-invocable: true
  allowed-tools:
    - Read
    - Glob
    - Grep
    - Write
    - Bash
    - AskUserQuestion
  platforms:
    claude-code: {enabled: true}
    cursor: {enabled: true}
    codex: {enabled: true}
    windsurf: {enabled: true, trigger: /security-audit}
    trae: {enabled: true}
    hermes: {enabled: true, platforms: [macos, linux, windows]}
    workbuddy: {enabled: true}
---

# security-audit — 安全审计

## 技能目的

对项目代码、依赖与配置进行系统性安全审计，识别注入、XSS、CSRF、认证绕过、数据暴露等常见漏洞，按严重级别排序并给出可执行的修复建议，确保发布前不存在已知高危风险。

## 参数说明

本技能无参数。自动扫描项目源码与依赖清单。

## 分阶段工作流

### 阶段 1：扫描代码

- **输入**：项目目录
- **处理**：
  1. 使用 Glob 查找源码文件（按技术栈扩展名过滤）
  2. 使用 Grep 搜索危险模式：`eval(`、`exec(`、`innerHTML`、`dangerouslySetInnerHTML`
  3. 使用 Grep 检查 SQL 拼接（字符串相加构造查询）
  4. 使用 Grep 检查命令注入风险（`child_process`、`os.system`、`subprocess`）
  5. 使用 Grep 检查硬编码密钥、Token、密码
- **输出**：代码级漏洞清单

### 阶段 2：检查依赖漏洞

- **输入**：代码漏洞清单
- **处理**：
  1. 使用 Glob 查找依赖清单（`package.json`、`requirements.txt`、`go.sum`、`pom.xml`）
  2. 使用 Bash 运行 `npm audit` / `pip-audit` / `govulncheck` / `mvn dependency-check`
  3. 解析漏洞公告（CVE）与受影响版本范围
  4. 标注可直接升级、需间接修复、暂无补丁三类
- **输出**：依赖漏洞清单

### 阶段 3：检查配置安全

- **输入**：依赖漏洞清单
- **处理**：
  1. 使用 Grep 检查 CORS 配置是否过于宽松（`*`）
  2. 使用 Grep 检查 HTTPS 强制策略（HSTS、重定向）
  3. 使用 Read 读取 `.env`、`config/` 检查敏感配置是否入库
  4. 检查 Cookie 安全属性（HttpOnly、Secure、SameSite）
  5. 检查 CSP（内容安全策略）头配置
- **输出**：配置安全清单

### 阶段 4：检查认证授权

- **输入**：配置安全清单
- **处理**：
  1. 使用 Grep 查找认证中间件与权限校验逻辑
  2. 检查密码存储是否使用安全哈希（bcrypt/argon2，非 MD5/SHA1）
  3. 检查 JWT 签发与校验（算法、过期、密钥轮换）
  4. 检查会话管理（固定会话、过期、重放防护）
  5. 检查越权风险（水平/垂直越权、IDOR）
- **输出**：认证授权漏洞清单

### 阶段 5：生成报告

- **输入**：全部漏洞清单
- **处理**：
  1. 按 CVSS 严重级别分类：严重 / 高 / 中 / 低 / 信息
  2. 为每条漏洞给出：位置、复现路径、影响、修复建议、参考链接
  3. 使用 AskUserQuestion 与用户确认修复优先级
  4. 使用 Write 生成报告 `docs/security/audit-report.md`
- **输出**：安全审计报告

## 协作协议引用

- 遵循 `.claude/docs/coding-standards.md` 编码规范
- 安全报告写入 `docs/security/` 目录
- 严重与高危漏洞必须修复后方可发布

## 推荐下一步

使用 `/gate-check` 进行阶段门禁评估。使用 `/release-checklist` 验证发布前检查清单。使用 `/tech-debt` 将低危漏洞纳入债务跟踪。