# /reverse-document — 从现有代码逆向生成设计文档与架构文档，分析代码结构推断设计意图。

## 触发方式
在 Cascade 对话框中输入 /reverse-document 触发此工作流。

## 工作流内容
# reverse-document — 逆向文档生成

## 技能目的

从现有代码库逆向生成设计文档与架构文档草稿，分析 `src/` 代码结构、推断设计意图、提取模块边界与依赖关系，生成 SRS 草稿和架构文档草稿，适用于遗留项目文档补全场景。

## 参数说明

本技能无参数。自动扫描 `src/` 目录。

## 分阶段工作流

### 阶段 1：扫描代码结构

- **输入**：项目目录
- **处理**：
  1. 使用 Glob 扫描 `src/**/*.{js,ts,jsx,tsx,py,java,go}` 等源文件
  2. 使用 Bash 运行目录结构命令获取模块划分
  3. 使用 Read 读取项目配置文件（package.json、pom.xml 等）
  4. 识别技术栈与框架
- **输出**：代码结构概览

### 阶段 2：分析模块与依赖

- **输入**：代码结构概览
- **处理**：
  1. 使用 Grep 提取各模块的 import/require 语句
  2. 构建模块依赖图
  3. 使用 Read 读取各模块入口文件，推断模块职责
  4. 使用 Grep 识别 API 端点、数据模型、服务层
- **输出**：模块依赖图 + 职责推断

### 阶段 3：推断设计意图

- **输入**：模块依赖图 + 职责推断
- **处理**：
  1. 分析架构模式（MVC、DDD、微服务等）
  2. 识别数据流与通信模式
  3. 推断安全模型与认证机制
  4. 识别已采用的技术决策（隐式 ADR）
- **输出**：设计意图分析报告

### 阶段 4：生成文档草稿

- **输入**：设计意图分析报告
- **处理**：
  1. 使用 Write 生成 `docs/srs/*.md` 各模块 SRS 草稿
  2. 使用 Write 生成 `docs/architecture-doc.md` 架构文档草稿
  3. 使用 Write 生成 `docs/adr/ADR-001-*.md` 隐式决策 ADR 草稿
  4. 所有文档标注"逆向生成草稿，需人工校验"
- **输出**：文档草稿集合

## 协作协议引用

- 逆向生成的文档均为草稿，需用户逐个校验
- 遵循 `.claude/docs/templates/collaborative-protocols/design-agent-protocol.md` SRS 编写规范
- 遵循 `.claude/docs/templates/collaborative-protocols/design-agent-protocol.md` 架构文档规范

## 推荐下一步

对逆向生成的草稿进行人工校验与补充。校验后使用 `/consistency-check` 验证一致性，使用 `/architecture-review` 审查架构完整性。如有变更需求使用 `/propagate-design-change` 分析影响。