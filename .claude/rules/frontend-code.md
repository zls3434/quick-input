---
paths:
  - "src/frontend/**"
platforms:
  claude-code: {enabled: true, path: .claude/rules/frontend-code.md}
  cursor: {enabled: true, type: auto-attached, globs: "src/frontend/**"}
  codex: {enabled: true, section: agents.md}
  windsurf: {enabled: true, mode: append}
  trae: {enabled: true, mode: append}
---
# 前端代码规范

- 所有组件必须使用 TypeScript 编写，禁止使用 `any` 类型
  - 需要明确类型时应使用 `unknown` 并进行类型收窄
  - 禁止使用 `// @ts-ignore` 或 `// @ts-nocheck` 绕过类型检查
- 状态管理统一使用 store（如 Zustand / Redux Toolkit / Pinia），禁止散落的 `useState` 导致状态难以追踪
  - 跨组件共享状态必须放入全局 store
  - 仅组件内部局部状态可使用 `useState`
- 所有 API 调用必须通过统一的 API 层（如 `src/frontend/api/`），禁止在组件中直接使用 `fetch` / `axios`
  - API 层负责错误处理、鉴权头注入、重试策略
- 所有用户可见文本必须支持 i18n 国际化
  - 禁止在 JSX 中硬编码中英文字符串
  - 使用 `t('key')` 形式引用翻译键
- 组件遵循单一职责原则，一个组件只做一件事
  - 超过 300 行的组件必须拆分
- 所有 props 必须定义 TypeScript 接口类型，禁止使用隐式 `any`
  - 使用 `interface` 或 `type` 显式声明 Props
- 禁止使用内联样式（`style={{...}}`），必须使用 CSS 模块或 Tailwind 等方案
  - 动态样式应通过 className 切换实现

## 示例

**正确**：
```tsx
// 组件通过 API 层调用，状态来自 store，文本支持 i18n
interface UserCardProps {
  userId: string;
}

export function UserCard({ userId }: UserCardProps) {
  const user = useUserStore((s) => s.users[userId]);
  const { t } = useTranslation();

  return (
    <div className="rounded-lg bg-white p-4 shadow">
      <h3 className="text-lg font-bold">{t('user.profile')}</h3>
      <p>{user?.name}</p>
    </div>
  );
}
```

**错误**：
```tsx
// 直接 fetch、内联样式、硬编码文本、any 类型
export function UserCard(props: any) {
  const [user, setUser] = useState(null);

  useEffect(() => {
    fetch(`/api/users/${props.userId}`)
      .then((r) => r.json())
      .then(setUser);
  }, []);

  return (
    <div style={{ padding: '16px', background: 'white' }}>
      <h3>用户资料</h3>
      <p>{user?.name}</p>
    </div>
  );
}
```