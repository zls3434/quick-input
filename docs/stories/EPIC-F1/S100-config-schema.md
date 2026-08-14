---
状态: 草案
作者: zls3434
更新日期: 2026-08-10
所属 Epic: EPIC-F1
对应模块: M4
优先级: P0
---

# S100 用户故事：配置数据结构与校验规则

**用户故事**：作为开发者，我希望定义 TOML 配置的数据结构与校验规则，以便各模块基于稳定契约读取配置。

## 验收标准（Given-When-Then）

- **AC1-1**：Given 一个合法的配置文件，When 解析它，Then 能正确读取按钮列表（每个按钮含 `id`、`label`、`content`、`comment` 字段）。
- **AC1-2**：Given 一个配置项缺失必填字段，When 解析它，Then 返回明确错误并指出缺失字段。
- **AC1-3**：Given 一个类型错误的配置项，When 解析它，Then 返回类型校验错误，不崩溃。
- **AC1-4**：Given 配置结构定义，When 项目编译时，Then 数据结构使用强类型（struct + serde）。

## 任务清单

1. 定义 `ConfigFile`、`ButtonConfig`、`AppProfile` 数据结构
2. 使用 `serde` derive 序列化/反序列化注解
3. 定义必填/可选字段校验规则（`Default` + serde 默认值）
4. 编写单元测试覆盖合法与非法配置解析
5. 创建测试夹具工厂 `create_button_fixture()`

## 关联需求

- N7 配置文件持久化与批量自定义（TOML）
- 概念 V3 按应用自动切换配置

## 技术路由

- 后端 Rust 程序员：`rust` 注入层 + serde 数据建模
- 数据库/数据校验：无 DB，走 serde 校验

## 估算

- 1 人日（小）