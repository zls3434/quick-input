<!-- Software Engineering Studios -->
# 前端代码规则

## 适用范围
`src/frontend/**` 下所有文件

## 规范
- 组件化：所有 UI 元素封装为可复用组件，单一职责
- TypeScript：必须使用 TypeScript，禁止 `any` 类型
- 状态管理：使用框架推荐的方案，避免组件内管理全局状态
- 命名：组件名 PascalCase，文件名与组件名一致
- 样式隔离：使用 CSS Modules 或 CSS-in-JS
- 响应式设计：组件必须支持响应式布局
- 可访问性：满足 WCAG AA 标准