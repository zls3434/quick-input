---
paths:
  - "src/frontend/**/components/**"
platforms:
  claude-code: {enabled: true, path: .claude/rules/ui-code.md}
  cursor: {enabled: true, type: auto-attached, globs: "src/frontend/**/components/**"}
  codex: {enabled: true, section: agents.md}
  windsurf: {enabled: true, mode: append}
  trae: {enabled: true, mode: append}
---
# UI 代码规范

- 组件可访问性达到 WCAG AA 级标准
  - 所有交互元素可通过键盘访问
  - 颜色对比度至少 4.5:1
  - 表单字段必须有关联的 label
- 响应式设计
  - 组件在移动端、平板、桌面均可用
  - 使用响应式断点，禁止固定像素宽度
- 组件复用，禁止重复造轮子
  - 先查找现有组件库再开发
  - 公共组件放入 `components/` 共享目录
- 语义化 HTML
  - 使用 `<nav>` `<main>` `<article>` `<section>` 等语义标签
  - 禁止滥用 `<div>` 替代语义标签
- ARIA 标签正确使用
  - 图标按钮必须有 `aria-label`
  - 动态内容使用 `aria-live` 通知
  - 禁止冗余 ARIA（如 `<button>` 上再加 `role="button"`）
- 键盘导航支持
  - 所有可交互元素支持 Tab 导航
  - 自定义组件实现 `tabindex` 和键盘事件
  - 焦点可见（focus 样式不可移除）

## 示例

**正确**：
```tsx
<button
  aria-label="关闭对话框"
  className="focus:ring-2 focus:ring-blue-500"
  onClick={onClose}
>
  <CloseIcon />
</button>

<nav aria-label="主导航">
  <ul className="flex flex-col md:flex-row">
    <li><a href="/home">首页</a></li>
  </ul>
</nav>
```

**错误**：
```tsx
<div onClick={onClose} style={{cursor:'pointer'}}>  {/* div 不可键盘访问 */}
  <CloseIcon />
</div>

<div className="navigation">  {/* 应使用 <nav> */}
  <div><div>首页</div></div>
</div>
```