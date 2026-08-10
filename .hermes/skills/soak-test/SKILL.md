---
name: soak-test
description: "生成持续运行压力测试协议，定义压力场景、持续时间、并发数与监控指标，产出测试脚本与报告模板。"
version: 1.0.0
platforms: [macos, linux, windows]
metadata:
  hermes:
    tags: ["testing", "quality"]
    load_mode: on-demand
    model: sonnet
    argument-hint: ""
    user-invocable: true
    allowed-tools:
      - Read
      - Glob
      - Grep
      - Write
      - AskUserQuestion
---

# soak-test — 持续运行压力测试协议

## 技能目的

为系统生成持续运行（Soak）压力测试协议，定义压力场景、持续时长、并发规模与监控指标，产出可执行的测试脚本与报告模板，用于发现内存泄漏、资源耗尽、性能衰减等长时间运行下才会暴露的问题。

## 参数说明

本技能无参数。通过与用户交互确定压力参数。

## 分阶段工作流

### 阶段 1：确定压力场景

- **输入**：项目目录
- **处理**：
  1. 使用 Glob 查找 `docs/srs/` 中的核心用例与性能需求
  2. 使用 Read 读取 SRS 中关于并发用户、峰值负载的描述
  3. 使用 Grep 搜索已有的压测配置（`k6`、`locust`、`jmeter`、`gatling`）
  4. 使用 AskUserQuestion 与用户确认重点压力场景（登录、查询、写入、混合）
- **输出**：压力场景清单

### 阶段 2：定义持续时间和并发数

- **输入**：压力场景清单
- **处理**：
  1. 使用 AskUserQuestion 确认持续运行时长（4小时/8小时/24小时/72小时）
  2. 确认并发用户阶梯（如 100→500→1000→500）
  3. 确认请求速率上限与思考时间（Think Time）
  4. 确定降级判据：错误率超 1%、P95 超阈值、内存持续增长
- **输出**：压力参数定义

### 阶段 3：生成测试脚本

- **输入**：压力参数定义
- **处理**：
  1. 根据技术栈选择压测工具（Node→k6，Python→locust，Java→gatling）
  2. 使用 Write 生成压测脚本 `tests/soak/soak-scenario.{ext}`
  3. 脚本包含：阶梯加压、思考时间、断言、数据准备
  4. 配置测试数据生成与清理逻辑
  5. 生成运行命令与依赖安装说明
- **输出**：压测脚本

### 阶段 4：定义监控指标

- **输入**：压测脚本
- **处理**：
  1. 定义应用层指标：QPS、响应时间（P50/P95/P99）、错误率
  2. 定义资源层指标：CPU、内存、磁盘 I/O、网络
  3. 定义中间件指标：数据库连接池、缓存命中率、队列堆积
  4. 使用 Write 生成监控配置与采集脚本
  5. 定义异常告警阈值
- **输出**：监控指标清单

### 阶段 5：生成报告模板

- **输入**：监控指标清单
- **处理**：
  1. 使用 Write 生成报告模板 `docs/test/soak-report-template.md`
  2. 模板含：测试摘要、场景说明、参数配置、指标趋势占位、异常事件、结论与建议
  3. 预留图表占位（响应时间趋势、内存增长曲线、错误率分布）
  4. 给出通过/失败判据说明
- **输出**：Soak 测试报告模板

## 协作协议引用

- 遵循 `.claude/docs/coding-standards.md` 编码规范
- 压测脚本写入 `tests/soak/` 目录
- 报告模板写入 `docs/test/` 目录
- 压测参数需用户确认后方可执行

## 推荐下一步

使用 `/smoke-check` 在压测前执行冒烟验证。使用 `/perf-profile` 分析压测暴露的瓶颈。使用 `/launch-checklist` 在发布前确认压测通过。