---
name: perf-profile
description: "性能分析和瓶颈识别，从前端、后端到数据库的全栈性能剖析与优化建议生成。"
version: 1.0.0
platforms: [macos, linux, windows]
metadata:
  hermes:
    tags: ["maintenance", "quality"]
    load_mode: on-demand
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
---

# perf-profile — 性能分析和瓶颈识别

## 技能目的

对项目进行全栈性能分析，识别前端、后端与数据库各层的性能瓶颈，生成包含量化指标与优先级排序的优化建议报告，帮助团队在发布前消除主要性能风险。

## 参数说明

本技能无参数。自动扫描项目代码与配置文件。

## 分阶段工作流

### 阶段 1：识别性能指标

- **输入**：项目目录
- **处理**：
  1. 使用 Glob 查找性能基准文件（如 `benchmarks/`、`perf/`、`.lighthouserc`）
  2. 使用 Read 读取 `docs/srs/` 中关于性能需求（NFR）的章节
  3. 使用 AskUserQuestion 与用户确认关注的性能指标阈值
  4. 使用 Grep 搜索代码中已有的埋点与监控配置
- **输出**：性能指标清单与阈值基线

### 阶段 2：分析前端性能

- **输入**：性能指标清单
- **处理**：
  1. 使用 Glob 查找前端构建配置（webpack/vite/rollup）
  2. 使用 Bash 运行构建产物体积分析（`du`、bundle-analyzer）
  3. 使用 Grep 检查图片资源是否懒加载、是否使用现代格式
  4. 评估 Core Web Vitals 三项指标：LCP（最大内容绘制）、FID/INP（首次输入延迟）、CLS（累积布局偏移）
  5. 检查关键渲染路径（阻塞资源、首屏 JS 体积）
- **输出**：前端性能分析结果

### 阶段 3：分析后端性能

- **输入**：前端分析结果
- **处理**：
  1. 使用 Grep 查找 API 端点定义与中间件链
  2. 使用 Read 读取路由处理逻辑，识别同步阻塞、N+1 查询
  3. 使用 Bash 检查是否存在内存泄漏风险（长连接、未释放资源）
  4. 评估 API 响应时间分布，识别 P95/P99 慢端点
  5. 检查缓存策略（Redis/Memory/HTTP 缓存头）
- **输出**：后端性能分析结果

### 阶段 4：分析数据库性能

- **输入**：后端分析结果
- **处理**：
  1. 使用 Grep 查找 SQL 查询语句与 ORM 调用
  2. 使用 Bash 运行 `EXPLAIN` 分析慢查询执行计划（若可连接数据库）
  3. 检查索引覆盖情况，识别全表扫描风险
  4. 评估连接池配置与事务范围
  5. 检查是否存在过度查询、缺失分页等问题
- **输出**：数据库性能分析结果

### 阶段 5：生成优化建议

- **输入**：全部分析结果
- **处理**：
  1. 按影响面与修复成本对所有瓶颈排序
  2. 为每条建议给出量化预期收益（如"LCP 预计降低 1.2s"）
  3. 使用 Write 生成报告 `docs/perf/profile-report.md`
  4. 报告包含：指标基线表、瓶颈清单、优化建议、验收标准
- **输出**：性能优化报告

## 协作协议引用

- 遵循 `.claude/docs/coding-standards.md` 编码规范
- 性能报告写入 `docs/perf/` 目录
- 优化建议需用户批准后方可执行

## 推荐下一步

使用 `/security-audit` 同步进行安全审计。使用 `/soak-test` 生成压力测试协议验证优化效果。使用 `/tech-debt` 将性能相关债务纳入跟踪。