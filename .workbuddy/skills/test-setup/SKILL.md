---
name: test-setup
description: "根据技术栈配置测试框架与CI管线，搭建单元/集成/E2E测试基础设施。"
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
  platforms:
    claude-code: {enabled: true}
    cursor: {enabled: true}
    codex: {enabled: true}
    windsurf: {enabled: true, trigger: /test-setup}
    trae: {enabled: true}
    hermes: {enabled: true, platforms: [macos, linux, windows]}
    workbuddy: {enabled: true}
---

# test-setup — 测试框架搭建

## 技能目的

根据项目技术栈配置测试框架与 CI 管线，搭建单元测试、集成测试、端到端测试基础设施，确保开发阶段有完整的测试支撑。

## 参数说明

本技能无参数。自动检测技术栈。

## 分阶段工作流

### 阶段 1：检测技术栈

- **输入**：项目目录
- **处理**：
  1. 使用 Read 读取 `docs/technical-preferences.md` 获取技术栈选择
  2. 使用 Glob 查找 `package.json`、`pom.xml`、`go.mod` 等项目配置文件
  3. 使用 Read 读取配置文件确认框架版本
  4. 使用 Grep 检查是否已有测试配置
- **输出**：技术栈检测结果

### 阶段 2：配置单元测试框架

- **输入**：技术栈检测结果
- **处理**：
  1. 根据前端框架选择测试框架（React→Jest+Testing Library，Vue→Vitest，Angular→Jasmine）
  2. 根据后端语言选择测试框架（Node→Jest，Python→pytest，Java→JUnit，Go→testing）
  3. 使用 Write 创建测试配置文件
  4. 使用 Bash 安装测试依赖
  5. 配置覆盖率收集工具
- **输出**：单元测试配置

### 阶段 3：配置集成测试框架

- **输入**：单元测试配置
- **处理**：
  1. 配置数据库测试环境（测试数据库、事务回滚）
  2. 配置 API 集成测试框架（Supertest、httpx、MockMvc）
  3. 使用 Write 创建集成测试目录结构与示例文件
  4. 配置测试夹具（Fixture）管理
- **输出**：集成测试配置

### 阶段 4：配置 E2E 测试框架

- **输入**：集成测试配置
- **处理**：
  1. 选择 E2E 框架（Playwright / Cypress / Selenium）
  2. 使用 Write 创建 E2E 测试配置文件
  3. 使用 Bash 安装 E2E 框架与浏览器
  4. 创建 E2E 测试目录结构与示例
- **输出**：E2E 测试配置

### 阶段 5：配置 CI 管线

- **输入**：全部测试配置
- **处理**：
  1. 使用 Write 创建 CI 配置文件（GitHub Actions / GitLab CI）
  2. 定义测试阶段：lint → unit → integration → e2e
  3. 配置覆盖率报告上传与阈值检查
  4. 使用 Bash 验证 CI 配置语法
- **输出**：CI 管线配置

## 协作协议引用

- 遵循 `.claude/docs/coding-standards.md` 编码规范
- 测试配置文件写入项目根目录
- CI 配置需与版本控制平台一致

## 推荐下一步

使用 `/test-helpers` 生成技术栈特定测试辅助库。使用 `/sprint-plan` 规划 Sprint。开发阶段使用 `/dev-story` 实现 Story 并运行测试。