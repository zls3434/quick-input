<!--
状态: 已批准
作者: tech-architect
更新日期: 2026-07-09
-->

# 技术参考文档目录

本目录存放项目设计与选型期间获取的技术参考文档，包括框架、中间件、三方服务的官方文档、SDK 和参考示例。

## 目录结构

按技术栈分子目录组织：

- `[技术栈]/`：特定技术栈的参考文档（如 `react/`、`node/`、`postgresql/`）
- `middleware/`：中间件参考文档（跨技术栈）
- `services/`：三方服务参考文档
- `general/`：通用参考文档

## 管理方式

通过 `/tech-reference` 技能管理本目录：

- `/tech-reference fetch`：获取在线文档（网页缓存或 PDF 下载）
- `/tech-reference index`：为无法缓存的网页文档生成地址索引
- `/tech-reference import`：导入本地文档
- `/tech-reference list`：查看文档清单
- `/tech-reference verify`：验证引用一致性

## 索引文件

`INDEX.md` 是本目录所有文档的统一索引，由 `/tech-reference` 技能自动维护，请勿手动编辑。

## 管理规范

详见 `.studio/docs/tech-reference-management.md`。