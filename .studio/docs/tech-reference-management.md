<!-- Software Engineering Studios -->
---
状态: 已批准
作者: tech-architect
更新日期: 2026-07-09
---

# 技术参考文档管理规范

本文件定义 Software Engineering Studios 中的技术参考文档管理规范，包括文档获取策略、目录组织结构、索引文件格式、元数据要求和引用一致性规则。确保项目设计与选型期间的技术参考资料可追溯、可引用、一致性可控。

## 1. 概述与背景

Software Engineering Studios 通过 `docs/tech-reference/` 目录集中管理技术参考文档。`directory-structure.md` 定义了该目录的用途（技术选型参考、第三方依赖说明、性能基准报告），但未细化管理策略。`setup-stack` 技能阶段 4 仅简略地通过 WebSearch 收集最佳实践后写入该目录，缺乏系统化的文档获取、缓存、索引和验证能力。

本规范旨在填补以下缺口：

- **文档获取策略**：针对网页文档、PDF 文档、本地文档分别定义获取和保存流程
- **目录组织结构**：按技术栈分子目录，避免文档混杂
- **索引文件格式**：统一定义 `INDEX.md` 的结构和字段
- **元数据要求**：每个文档必须包含来源、日期、版本等元数据
- **引用一致性规则**：定义项目文档引用技术参考时的路径规范和验证要求

## 2. 目录组织结构

`docs/tech-reference/` 按技术栈分子目录组织：

```
docs/tech-reference/
├── README.md                    # 目录说明（本目录用途和使用方式）
├── INDEX.md                     # 全局索引文件（所有文档的统一索引）
├── react/                       # React 技术栈参考文档
│   ├── official-docs.md         # 缓存的官方文档（或 PDF）
│   ├── state-management.md      # 状态管理最佳实践
│   └── ...
├── node/                        # Node.js 技术栈参考文档
│   ├── express-guide.pdf        # 下载的 PDF 文档
│   ├── nestjs-docs-index.md     # 地址索引说明文档（无法缓存的网页）
│   └── ...
├── postgresql/                  # PostgreSQL 参考文档
│   ├── performance-tuning.md
│   └── ...
├── middleware/                  # 中间件参考文档（跨技术栈）
│   ├── redis-best-practices.md
│   └── ...
├── services/                    # 三方服务参考文档
│   ├── aws-s3-sdk.md
│   └── ...
└── general/                     # 通用参考文档（不属于特定技术栈）
    ├── api-design-guide.pdf
    └── ...
```

### 目录命名规则

- 技术栈子目录名：kebab-case，与技术栈名称一致（如 `react`、`node`、`postgresql`）
- 跨技术栈文档使用通用分类目录：`middleware/`、`services/`、`general/`
- 文件名：kebab-case，含 `.md`（Markdown 缓存）或 `.pdf`（PDF 下载）后缀
- 地址索引文件：`[文档名]-index.md`，以 `-index` 后缀区分

## 3. 文档获取策略

### 3.1 网页文档（优先缓存）

| 步骤 | 操作 | 说明 |
|------|------|------|
| 1 | WebFetch 抓取 | 获取页面 Markdown 内容 |
| 2 | 完整性判断 | 内容完整且可读 → 缓存；不完整 → 转地址索引 |
| 3 | 格式清理 | 去除导航、广告等无关内容，保留正文 |
| 4 | 元数据注入 | 在文件头部添加元数据块 |
| 5 | 保存 | 写入 `docs/tech-reference/[技术栈]/[文档名].md` |
| 6 | 索引更新 | 更新 `INDEX.md` |

**无法缓存的场景**：
- 页面依赖 JavaScript 渲染（WebFetch 获取的内容为空或仅有框架）
- 网站反爬机制导致内容不完整
- 文档分页过多（超过 20 页），逐一抓取成本过高
- 文档为付费内容或明确禁止缓存

### 3.2 PDF / 可下载文档

| 步骤 | 操作 | 说明 |
|------|------|------|
| 1 | 确认下载 URL | 通过 WebSearch 或用户提供 |
| 2 | 下载 | 使用 WebFetch 或用户手动下载 |
| 3 | 保存 | 保存到 `docs/tech-reference/[技术栈]/[文档名].pdf` |
| 4 | 元数据 | 在 `INDEX.md` 中记录来源 URL、下载日期、文档版本 |
| 5 | 索引更新 | 更新 `INDEX.md`，标注类型为"PDF 下载" |

### 3.3 用户本地文档

| 步骤 | 操作 | 说明 |
|------|------|------|
| 1 | 确认源路径 | 用户提供本地文件路径 |
| 2 | 验证可读 | 使用 Read 工具确认文件存在且可读 |
| 3 | 确定目标路径 | `docs/tech-reference/[技术栈]/[文件名]` |
| 4 | 复制 | 读取源文件内容，写入目标路径 |
| 5 | 元数据 | 在 `INDEX.md` 中记录源路径、导入日期 |
| 6 | 索引更新 | 更新 `INDEX.md`，标注类型为"本地导入" |

## 4. 地址索引说明文档格式

当网页文档无法缓存时，生成地址索引说明文档。格式见 `templates/tech-reference-index.md` 模板。

### 必需字段

| 字段 | 说明 |
|------|------|
| 文档标题 | 原始文档的标题 |
| 根 URL | 文档站点入口地址 |
| 索引生成日期 | ISO 8601 格式 |
| 文档版本 | 如可获取则记录 |
| 目录结构树 | 章节 → URL 映射，支持二级嵌套 |
| 章节摘要 | 每个章节 1-2 句内容摘要 |
| 本地引用说明 | 开发时需查阅哪个章节的哪个 URL |

## 5. INDEX.md 全局索引格式

`docs/tech-reference/INDEX.md` 是所有技术参考文档的统一索引，格式如下：

```markdown
# 技术参考文档索引

> 本文件由 /tech-reference 技能自动维护，请勿手动编辑。

## 索引条目

| 技术栈 | 文档名称 | 类型 | 路径 | 来源 URL | 获取日期 | 版本 |
|--------|----------|------|------|----------|----------|------|
| react | React 官方文档 | 本地缓存 | react/official-docs.md | https://react.dev | 2026-07-09 | 18.2 |
| node | NestJS 文档 | 地址索引 | node/nestjs-docs-index.md | https://nestjs.com | 2026-07-09 | 10.0 |
| postgresql | 性能调优指南 | PDF 下载 | postgresql/performance-tuning.pdf | https://pg.org/... | 2026-07-09 | 16.1 |
| general | API 设计指南 | 本地导入 | general/api-design-guide.pdf | （本地导入） | 2026-07-09 | - |

## 统计

- 文档总数：4
- 按类型：本地缓存 1 / 地址索引 1 / PDF 下载 1 / 本地导入 1
- 按技术栈：react 1 / node 1 / postgresql 1 / general 1
```

## 6. 文档元数据要求

每个本地缓存的 Markdown 文档必须在文件头部包含元数据块：

```markdown
<!--
来源: https://react.dev
获取日期: 2026-07-09
文档版本: 18.2
技术栈: react
获取方式: 网页缓存
-->
```

PDF 文档的元数据记录在 `INDEX.md` 中（PDF 文件本身无法注入元数据）。

## 7. 引用一致性规则

### 7.1 引用路径规范

项目文档（SRS、架构文档、技术偏好）引用技术参考时，必须使用相对于项目根目录的路径：

```markdown
详见技术参考文档：docs/tech-reference/react/official-docs.md#状态管理
```

禁止使用：
- 绝对路径（`e:\0Projects\...`）
- 外部 URL（应先通过 `/tech-reference fetch` 或 `/tech-reference index` 纳入管理后引用本地路径或索引路径）
- 仅描述性引用（如"参见 React 官方文档"）而不给出路径

### 7.2 验证规则

`/tech-reference verify` 检查以下一致性：

| 检查项 | 规则 | 处理 |
|--------|------|------|
| 引用路径存在 | 项目文档中引用的 `docs/tech-reference/` 路径必须对应实际文件 | 不存在标注为"引用断裂" |
| 索引完整性 | `INDEX.md` 中的每个条目必须有对应文件 | 缺失标注为"索引过期" |
| 文件可索引 | `docs/tech-reference/` 中的每个文件应在 `INDEX.md` 有记录 | 未记录标注为"未索引" |
| 孤立文档 | 存在但未被任何项目文档引用的参考文档 | 标注为"孤立文档"（提示而非错误） |

## 8. 版权与合规

- 网页缓存仅用于项目内部参考，不得再分发或公开
- 对明确禁止缓存的站点（如付费文档、明确版权声明），仅生成地址索引
- 用户本地导入的文档版权责任由用户承担
- PDF 下载需遵守原始文档的许可协议