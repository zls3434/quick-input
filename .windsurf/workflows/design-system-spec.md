# /design-system-spec — 编写设计系统规范，使用 design-system-spec.md 模板定义设计令牌、组件库、排版与色彩体系。

## 触发方式
在 Cascade 对话框中输入 /design-system-spec 触发此工作流。

## 工作流内容
# design-system-spec — 设计系统规范编写

## 技能目的

为指定产品编写设计系统规范，定义设计令牌（Design Tokens）、组件库规范、排版体系、色彩体系、间距体系与图标体系，确保 UI 一致性与可复用性。

## 参数说明

- `[产品名]`：目标产品名称，用于命名设计系统文档与令牌前缀。

## 分阶段工作流

### 阶段 1：加载上下文

- **输入**：产品名参数
- **处理**：
  1. 使用 Read 读取 `docs/product-concept.md` 获取品牌调性与目标用户
  2. 使用 Glob 查找 `.claude/docs/templates/design-system-spec.md` 模板
  3. 使用 Read 读取 `docs/technical-preferences.md` 获取前端框架偏好
  4. 使用 Grep 检查是否已存在设计系统文档
- **输出**：产品上下文 + 模板

### 阶段 2：定义设计令牌

- **输入**：产品上下文
- **处理**：
  1. 通过 AskUserQuestion 确认品牌色彩主色与辅助色
  2. 定义色彩令牌（primary、secondary、surface、error、warning、success）
  3. 定义排版令牌（字体族、字号、行高、字重）
  4. 定义间距令牌（4px 基准网格体系）
  5. 定义圆角、阴影、动效令牌
- **输出**：设计令牌定义表

### 阶段 3：组件库规范

- **输入**：设计令牌 + 前端框架
- **处理**：
  1. 列出基础组件清单（Button、Input、Card、Modal、Toast 等）
  2. 为每个组件定义属性、状态、变体
  3. 定义组件命名规范与文件组织结构
  4. 通过 AskUserQuestion 与用户确认组件优先级
- **输出**：组件库规范

### 阶段 4：编写与写入

- **输入**：全部规范内容
- **处理**：
  1. 基于模板填充各章节
  2. 向用户展示完整草稿
  3. 批准后使用 Write 写入 `docs/design/design-system-spec.md`
- **输出**：设计系统规范文件

## 协作协议引用

- 遵循 `.claude/docs/templates/collaborative-protocols/design-agent-protocol.md` 设计文档规范
- 参考 `.claude/docs/templates/design-system-spec.md` 模板
- 写入前需用户批准完整草稿

## 推荐下一步

使用 `/ux-design` 为具体界面编写 UX 规范，使用 `/ux-review` 审查无障碍性。开发阶段使用 `/test-helpers` 生成组件测试工厂。