---
状态: 已批准
作者: tech-architect
更新日期: 2026-07-09
---

name: tech-reference
description: "技术参考文档管理技能。在项目设计与选型期间获取并管理框架、中间件、三方服务的官方文档、SDK、参考示例，支持网页文档本地缓存、PDF 下载、本地文档导入、地址索引生成和引用一致性验证，确保开发全程技术参考资料可追溯、可引用。"
license: MIT
metadata:
  model: sonnet
  argument-hint: "[操作类型: fetch|index|import|list|verify]"
  user-invocable: true
  allowed-tools:
    - Read
    - Glob
    - Grep
    - Write
    - Edit
    - WebSearch
    - WebFetch
    - AskUserQuestion
  platforms:
    claude-code: {enabled: true}
    cursor: {enabled: true}
    codex: {enabled: true}
    windsurf: {enabled: true, trigger: /tech-reference}
    trae: {enabled: true}
    hermes: {enabled: true, platforms: [macos, linux, windows]}
    workbuddy: {enabled: true}
---

# tech-reference — 技术参考文档管理

## 技能目的

在项目设计与选型期间，获取并保存框架、中间件、三方服务等的相关文档（官方文档、SDK、参考示例）到 `docs/tech-reference/` 目录，保证功能引用一致性。针对不同文档来源采用分层策略：网页文档优先缓存到本地，无法缓存则生成地址索引说明文档；PDF 或可下载文档直接下载保存；用户本地文档复制到相应目录。遵循 `tech-reference-management.md` 规范定义的所有管理策略和规则。

## 参数说明

- `[操作类型]`：指定要执行的管理操作，支持以下值：
  - `fetch`：获取在线文档（网页缓存或 PDF 下载），保存到 `docs/tech-reference/` 对应子目录
  - `index`：为无法本地缓存的网页文档生成地址索引说明文档（目录与章节映射）
  - `import`：将用户指定的本地文档复制到 `docs/tech-reference/` 对应子目录
  - `list`：列出当前已有的技术参考文档清单和索引状态
  - `verify`：验证项目文档（SRS、架构文档、技术偏好）中引用的技术参考是否存在且路径有效
- 若未指定操作类型，使用 AskUserQuestion 询问用户选择。

## 分阶段工作流

### 阶段 1：操作路由

- **输入**：用户提供的 `[操作类型]` 参数。
- **处理**：
  1. 解析参数，确定操作类型（fetch/index/import/list/verify）
  2. 若未指定参数，使用 AskUserQuestion 列出 5 个操作选项供用户选择
  3. 读取 `tech-reference-management.md` 加载对应操作的规范
  4. 使用 Glob 检查 `docs/tech-reference/` 目录是否存在，不存在则提示创建
- **输出**：确定的操作类型和对应规范摘要

### 阶段 2：fetch 子流程（在线文档获取）

- **输入**：操作路由结果，用户提供的文档 URL 或技术关键词。
- **处理**：
  1. 使用 AskUserQuestion 询问用户要获取的文档信息：
     - 文档 URL（或技术名称，通过 WebSearch 查找官方文档地址）
     - 文档类别（framework/middleware/service/sdk/other）
     - 关联技术栈（如 react、node、postgresql 等）
  2. 根据文档类型选择获取策略：
     - **PDF / 可下载文件**：使用 WebFetch 尝试下载，保存到 `docs/tech-reference/[技术栈]/[文档名].pdf`
     - **网页文档**：使用 WebFetch 抓取页面内容，尝试缓存为本地 Markdown
  3. 网页文档缓存策略：
     - 使用 WebFetch 获取页面 Markdown 内容
     - 若内容完整且可读：清理为标准 Markdown，保存到 `docs/tech-reference/[技术栈]/[文档名].md`
     - 若内容不完整（JS 渲染、反爬、分页过多）：标记为"需索引模式"，转入 index 子流程
  4. 为每个成功缓存的文档生成元数据头（来源 URL、获取日期、版本信息、技术栈标签）
  5. 更新 `docs/tech-reference/INDEX.md` 索引文件，新增条目
  6. 使用 AskUserQuestion 展示获取结果摘要，获用户确认
- **输出**：本地缓存的文档文件或标记为需索引的文档清单

### 阶段 3：index 子流程（地址索引生成）

- **输入**：fetch 子流程中标记为"需索引模式"的文档，或用户直接指定的 URL。
- **处理**：
  1. 使用 WebFetch 获取文档站点的目录结构（站点地图、侧边栏导航、目录页）
  2. 分析文档结构，提取以下信息：
     - 文档标题和根 URL
     - 一级章节名称与 URL
     - 二级章节名称与 URL（如存在）
     - 每个章节的内容摘要（1-2 句话）
  3. 使用 `tech-reference-index.md` 模板生成地址索引说明文档
  4. 保存到 `docs/tech-reference/[技术栈]/[文档名]-index.md`
  5. 索引文档包含：
     - 元数据（来源 URL、索引生成日期、文档版本）
     - 目录结构树（章节名 → URL 映射）
     - 各章节内容摘要
     - 本地引用说明（开发时需查阅哪个章节的哪个 URL）
  6. 更新 `docs/tech-reference/INDEX.md` 索引文件，新增条目并标注类型为"地址索引"
  7. 使用 AskUserQuestion 展示索引文档草案，获用户审批
- **输出**：地址索引说明文档

### 阶段 4：import 子流程（本地文档导入）

- **输入**：操作路由结果，用户提供的本地文档路径。
- **处理**：
  1. 使用 AskUserQuestion 询问用户：
     - 源文件路径（支持 Glob 模式，如 `e:\docs\my-framework-guide.pdf`）
     - 文档类别（framework/middleware/service/sdk/other）
     - 关联技术栈
  2. 验证源文件存在且可读（使用 Read 工具）
  3. 确定目标路径：`docs/tech-reference/[技术栈]/[文件名]`
  4. 检查目标路径是否已存在同名文件，存在则询问用户是否覆盖
  5. 读取源文件内容，使用 Write 写入目标路径
  6. 为导入的文档生成元数据说明（如源路径、导入日期、文档类型）
  7. 更新 `docs/tech-reference/INDEX.md` 索引文件，新增条目并标注类型为"本地导入"
  8. 使用 AskUserQuestion 展示导入结果，获用户确认
- **输出**：导入后的文档文件路径

### 阶段 5：list 子流程（文档清单）

- **输入**：操作路由结果。
- **处理**：
  1. 使用 Glob 扫描 `docs/tech-reference/` 下所有文件（`**/*.md`、`**/*.pdf`）
  2. 读取 `docs/tech-reference/INDEX.md` 获取索引信息
  3. 按技术栈分组汇总：
     - 技术栈名称
     - 文档数量
     - 文档类型分布（本地缓存 / 地址索引 / 本地导入 / PDF 下载）
     - 各文档的来源 URL 和获取日期
  4. 检查索引与实际文件的一致性：
     - 索引中有记录但文件缺失 → 标注"文件缺失"
     - 文件存在但索引无记录 → 标注"未索引"
  5. 生成清单报告并展示给用户
- **输出**：技术参考文档清单报告

### 阶段 6：verify 子流程（引用一致性验证）

- **输入**：操作路由结果，项目文档集合。
- **处理**：
  1. 使用 Glob 查找项目文档：
     - `docs/architecture/*.md`
     - `docs/technical-preferences.md`
     - `design/requirements/*.md`
  2. 使用 Grep 扫描文档中引用的技术参考路径（模式：`docs/tech-reference/` 开头的路径引用）
  3. 对每个引用路径：
     - 使用 Glob 检查文件是否存在
     - 不存在则记录为"引用断裂"
  4. 反向检查：`docs/tech-reference/` 中的文档是否被任何项目文档引用
     - 未被引用的标注为"孤立文档"
  5. 生成验证报告：
     - 引用断裂清单（引用位置 → 缺失路径）
     - 孤立文档清单
     - 一致性评分（有效引用数 / 总引用数）
  6. 对引用断裂项，建议用户运行 `/tech-reference fetch` 补全
  7. 对孤立文档，建议用户确认是否需要或清理
  8. 使用 AskUserQuestion 展示验证报告，询问是否需要自动修复
- **输出**：引用一致性验证报告

## 协作协议引用

- 遵循 `docs/examples/COLLABORATIVE-DESIGN-PRINCIPLE.md` 用户驱动协作协议
- 遵循 `tech-reference-management.md` 技术参考文档管理规范
- 遵循 `docs-standards.md` 文档规范（元数据、ATX 标题、标准 Markdown）
- 所有文档写入操作必须经用户明确审批后执行
- 网页缓存仅用于内部参考，不得再分发受版权保护的文档
- 技能变更触发 `validate-skill-change.sh` Hook

## 推荐下一步

- fetch/index 完成后：运行 `/tech-reference verify` 验证引用一致性
- import 完成后：运行 `/tech-reference list` 查看完整文档清单
- verify 发现断裂后：运行 `/tech-reference fetch [缺失文档信息]` 补全
- 文档集就绪后：运行 `/create-architecture` 基于技术参考编写架构文档
- 技术栈配置时：运行 `/setup-stack`，其阶段 4 会自动调用本技能