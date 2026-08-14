---
状态: 草案
作者: zls3434
更新日期: 2026-08-10
所属 Epic: EPIC-C2
对应模块: M5
优先级: P1
依赖: S130
---

# S131 用户故事：悬浮注释显示

**用户故事**：作为用户，我希望悬停按钮时显示注释说明，以便了解按钮用途。

## 验收标准（Given-When-Then）

- **AC2-1**：Given 按钮列表渲染完成，When 鼠标悬停在按钮上，Then 弹出 Tooltip 显示该按钮的 `comment` 字段内容。
- **AC2-2**：Given 按钮的 `comment` 为空，When 悬停，Then 不显示 Tooltip（无空框）。
- **AC2-3**：Given Tooltip 显示，When 鼠标移出按钮区域，Then Tooltip 自动消失。
- **AC2-4**：Given Tooltip 内容较长，When 显示，Then 自动换行且不超过面板边界。

## 任务清单

1. 实现 Tooltip 组件（纯 CSS 或 Tauri 原生 Tooltip）
2. 将 Tooltip 集成到按钮组件中
3. 处理空注释（不显示）
4. 处理 Tooltip 边界溢出（自动定位）
5. 编写前端组件测试覆盖悬停/移出/空注释场景

## 关联需求

- V4 悬浮注释说明
- N5 悬浮显示注释说明

## 技术路由

- 前端 Svelte 程序员：Tooltip 组件
- 依赖 S130 按钮渲染

## 估算

- 1 人日（小）