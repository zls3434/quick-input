---
name: test-helpers
description: "测试辅助库与工厂函数生成技能。根据项目技术栈生成测试夹具、Mock 工厂、数据构造器与断言工具，降低测试编写成本并保持一致性。"
license: MIT
metadata:
  model: sonnet
  argument-hint: "[目标模块或技术栈]"
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
    windsurf: {enabled: true, trigger: /test-helpers}
    trae: {enabled: true}
    hermes: {enabled: true, platforms: [macos, linux, windows]}
    workbuddy: {enabled: true}
---

# test-helpers —— 测试辅助库生成技能

## 技能目的

根据项目技术栈与目标模块，生成可复用的测试夹具、Mock 工厂、数据构造器与断言工具，
降低测试用例编写成本，保证测试数据构造的一致性与可维护性，
避免每个测试文件重复造轮子。

## 参数说明

- `[目标模块或技术栈]`：
  - 可指定模块路径（如 `src/payment/`），技能将扫描该模块的测试需求。
  - 可指定技术栈（如 `React` / `Node.js` / `Python`），技能按对应测试框架生成辅助库。
  - 省略时默认从 `.claude/docs/technical-preferences.md` 推断技术栈。

## 分阶段工作流

### 阶段 1：探测技术栈与测试框架

- **输入**：用户参数或项目配置。
- **处理**：使用 Read 读取 `package.json` / `requirements.txt` / `go.mod` 等；使用 Glob 查找既有测试目录与框架配置；使用 Read 读取技术偏好文档。
- **输出**：技术栈与测试框架确认（如 Jest / Vitest / Pytest / JUnit）。

### 阶段 2：扫描测试需求

- **输入**：阶段 1 确定的目标模块。
- **处理**：使用 Grep 检索模块中的数据模型、外部依赖（API / DB / 缓存）、关键类型定义；识别需要构造的测试数据与 Mock 点。
- **输出**：测试需求清单，含待生成的夹具、Mock、工厂函数列表。

### 阶段 3：设计辅助库结构

- **输入**：阶段 2 的需求清单。
- **处理**：设计辅助库目录结构与命名规范，例如：
  - `test/helpers/fixtures/` —— 静态夹具
  - `test/helpers/factories/` —— 动态数据工厂
  - `test/helpers/mocks/` —— Mock 工厂
  - `test/helpers/assertions/` —— 自定义断言
- **输出**：辅助库结构设计草案。

### 阶段 4：生成代码

- **输入**：阶段 3 的结构设计。
- **处理**：使用 Write 生成各辅助文件代码，含 JSDoc/类型注解与使用示例；使用 Bash 运行既有测试确保辅助库可被正确导入。
- **输出**：生成的辅助库文件清单与导入验证结果。

### 阶段 5：文档与示例

- **输入**：阶段 4 的辅助库。
- **处理**：生成 `test/helpers/README.md`，含各工厂函数签名、参数说明与调用示例。
- **输出**：文档路径与一句话使用指引。

## 协作协议引用

- 写入辅助库代码前必须询问用户："我可以将测试辅助代码写入 [路径] 吗？"
- 生成的代码须符合项目编码规范（`.claude/docs/coding-standards.md`）。
- 不得自主修改业务源码；仅在测试目录内操作。

## 推荐下一步

- 辅助库就绪后运行 `/qa-plan [epic-slug]` 生成对应测试计划。
- 编写测试用例时可配合 `/dev-story` 在实现阶段同步产出测试。
- 若辅助库需要随技术栈演进，可在 `/retrospective` 中记录改进议题。