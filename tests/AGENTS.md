<!-- Software Engineering Studios -->
# 测试代码规则

## 适用范围
`tests/**` 下所有文件

## 规范
- 命名：`should [期望行为] when [前置条件]`
- 结构：AAA（Arrange-Act-Assert）三段分隔
- 隔离：每个测试独立运行，不依赖执行顺序
- 确定性：结果稳定可重复
- 禁止硬编码环境数据
- 使用 mock/stub，不依赖外部服务