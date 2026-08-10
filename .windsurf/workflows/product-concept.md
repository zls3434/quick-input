# /product-concept — 产品概念文档编写，使用 product-concept.md 模板引导式逐节编写。

## 触发方式
在 Cascade 对话框中输入 /product-concept 触发此工作流。

## 工作流内容
# product-concept — 产品概念文档编写

## 技能目的

使用 `product-concept.md` 模板，通过引导式逐节对话编写完整的产品概念文档，确保覆盖产品愿景、目标用户、核心价值、范围边界等关键维度。

## 参数说明

本技能无参数。通过交互式问答逐步填充文档。

## 分阶段工作流

### 阶段 1：加载模板

- **输入**：模板路径
- **处理**：
  1. 使用 Glob 查找 `.claude/docs/templates/product-concept.md`
  2. 使用 Read 读取模板，解析各章节结构
- **输出**：模板章节结构清单

### 阶段 2：检查已有内容

- **输入**：模板结构
- **处理**：
  1. 使用 Glob 检查 `docs/product-concept.md` 是否已存在
  2. 若存在，使用 Read 读取已有内容，识别已填充与未填充章节
- **输出**：章节填充状态

### 阶段 3：引导式逐节编写

- **输入**：未填充章节列表
- **处理**：对每个未填充章节：
  1. 通过 AskUserQuestion 提出针对性问题
  2. 根据用户回答生成该章节内容
  3. 请用户确认或调整
- **输出**：逐节内容

### 阶段 4：写入文档

- **输入**：所有章节内容
- **处理**：使用 Write 将完整内容写入 `docs/product-concept.md`
- **输出**：product-concept.md 文件

## 协作协议引用

- 遵循 `.claude/docs/templates/collaborative-protocols/design-agent-protocol.md` 概念文档规范
- 参考 `.claude/docs/templates/product-concept.md` 模板

## 推荐下一步

使用 `/map-modules` 将概念分解为模块，或使用 `/setup-stack` 配置技术栈。