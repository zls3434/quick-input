<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import { onMount } from "svelte";

  interface MenuItem {
    id: string;
    label: string;
    disabled: boolean;
    hint: string | null;
  }

  // 顶栏浮层载荷（与 $lib/floater.ts ToolbarPayload 对应）
  interface ToolbarTab {
    name: string;
    active: boolean;
  }
  interface ToolbarPayload {
    layout: "vertical" | "horizontal";
    opacityPct: number;
    alwaysOnTop: boolean;
    tabs: ToolbarTab[];
  }

  type FloaterKind = "tooltip" | "menu" | "toolbar";

  interface ShowPayload {
    kind: FloaterKind;
    text?: string | null;
    items?: MenuItem[] | null;
    toolbar?: ToolbarPayload | null;
  }

  let kind = $state<FloaterKind>("tooltip");
  let text = $state("");
  let items = $state<MenuItem[]>([]);
  let toolbar = $state<ToolbarPayload | null>(null);
  // 浮层相对锚点方向（tooltip 箭头用）：above=true → 箭头朝下指向按钮
  let above = $state(true);
  // 淡出动画进行中（自动隐藏前播放，结束后自行隐藏）
  let fading = $state(false);

  // 渲染后测量内容尺寸并上报（两阶段定位：先测量后显示，避免闪烁）
  function reportSize() {
    requestAnimationFrame(() => {
      const el = document.body;
      void invoke("floater_ready", { width: el.scrollWidth, height: el.scrollHeight }).catch(
        (e) => console.error("上报浮层尺寸失败", e),
      );
    });
  }

  onMount(() => {
    const unshow = listen<ShowPayload>("floater://show", (e) => {
      kind = e.payload.kind;
      text = e.payload.text ?? "";
      items = e.payload.items ?? [];
      toolbar = e.payload.toolbar ?? null;
      fading = false;
      reportSize();
    });
    const unorient = listen<{ above: boolean }>("floater://orient", (e) => {
      above = e.payload.above;
    });
    const unhide = listen("floater://hide", () => {
      text = "";
      items = [];
      toolbar = null;
      fading = false;
    });
    // 自动隐藏前淡出：播放 150ms 过渡后自行隐藏（窗口不立即移走，
    // 淡出过程可见）。非顶栏类型忽略（tooltip/menu 走立即隐藏路径）。
    const unfade = listen("floater://fadeout", () => {
      if (kind !== "toolbar" || !toolbar) return;
      fading = true;
      setTimeout(() => {
        fading = false;
        void invoke("hide_floater").catch(() => {});
      }, 150);
    });
    // 兜底：页面 JS 因后台节流挂起期间到达的显示请求（事件丢失），
    // 页面启动后主动取回最近一次请求，保证浮层内容不丢。
    void invoke<ShowPayload | null>("floater_pull_pending")
      .then((pending) => {
        if (!pending) return;
        kind = pending.kind;
        text = pending.text ?? "";
        items = pending.items ?? [];
        toolbar = pending.toolbar ?? null;
        reportSize();
      })
      .catch(() => {});
    return () => {
      void unshow.then((f) => f());
      void unorient.then((f) => f());
      void unhide.then((f) => f());
      void unfade.then((f) => f());
    };
  });

  function onItemClick(item: MenuItem) {
    if (item.disabled) {
      // 禁用项：仅关闭菜单，不转发动作
      void invoke("hide_floater").catch(() => {});
      return;
    }
    void invoke("floater_action", { id: item.id }).catch((e) => console.error("菜单动作失败", e));
  }

  // ---- 顶栏浮层交互 ----

  // 顶栏控制动作：转发到悬浮窗前端执行（Rust 隐藏浮层后 emit 回 overlay）
  function act(id: string) {
    void invoke("floater_action", { id }).catch((e) => console.error("顶栏动作失败", e));
  }

  // 分组标签切换：本地更新高亮 + 转发切换（浮层保持显示，避免切换后闪烁）
  function switchTab(name: string) {
    if (!toolbar) return;
    toolbar = { ...toolbar, tabs: toolbar.tabs.map((t) => ({ ...t, active: t.name === name })) };
    void invoke("floater_tab_switch", { name }).catch((e) => console.error("分组切换失败", e));
  }

  // 悬停状态上报：悬浮窗前端据此取消/恢复自动隐藏定时器
  function reportHover(hovering: boolean) {
    void invoke("floater_hover", { hovering }).catch(() => {});
  }
</script>

{#if kind === "tooltip" && text}
  <div class="tooltip" class:above class:below={!above} role="tooltip">{text}</div>
{:else if kind === "menu" && items.length > 0}
  <div class="ctx-menu">
    {#each items as item}
      <button
        class="ctx-item"
        class:disabled={item.disabled}
        aria-disabled={item.disabled}
        onclick={() => onItemClick(item)}
      >
        {item.label}
      </button>
      {#if item.hint}
        <div class="ctx-hint">{item.hint}</div>
      {/if}
    {/each}
  </div>
{:else if kind === "toolbar" && toolbar}
  <div
    class="toolbar"
    class:fading
    role="toolbar"
    tabindex="-1"
    aria-label="悬浮窗顶栏"
    onmouseenter={() => reportHover(true)}
    onmouseleave={() => reportHover(false)}
  >
    <!-- 控制按钮组（隐藏 / 布局 / 透明度 / 置顶 / 移动） -->
    <div class="tb-ctrl">
      <button
        class="tb-btn"
        title="隐藏悬浮窗（托盘或全局热键唤回）"
        aria-label="隐藏悬浮窗"
        onclick={() => act("toolbar:hide")}
      >
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" width="13" height="13">
          <path d="M3 10s3.5-6 9-6 9 6 9 6-3.5 6-9 6-9-6-9-6z" />
          <circle cx="12" cy="10" r="2.5" />
          <path d="M4 20L20 4" />
        </svg>
      </button>
      <button
        class="tb-btn"
        title="切换横竖布局（当前：{toolbar.layout === 'vertical' ? '竖向' : '横向'}）"
        aria-label="切换横竖布局"
        onclick={() => act("toolbar:layout")}
      >
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" width="13" height="13">
          <rect x="3" y="6" width="10" height="12" rx="1.5" />
          <path d="M16 9h4M18 7l-2 2 2 2" />
          <path d="M16 15h4M18 13l-2 2 2 2" />
        </svg>
      </button>
      <button
        class="tb-btn"
        class:is-dimmed={toolbar.opacityPct <= 30}
        title="透明度 {toolbar.opacityPct}%（点击切换）"
        aria-label="切换透明度"
        onclick={() => act("toolbar:opacity")}
      >
        <svg viewBox="0 0 24 24" stroke="currentColor" stroke-width="2" width="13" height="13" fill="none">
          <circle cx="12" cy="12" r="8" />
          <path d="M12 4a8 8 0 0 1 0 16z" fill="currentColor" stroke="none" />
        </svg>
      </button>
      <button
        class="tb-btn"
        class:is-active={toolbar.alwaysOnTop}
        title="{toolbar.alwaysOnTop ? '已置顶' : '未置顶'}（点击切换）"
        aria-label="切换置顶"
        onclick={() => act("toolbar:topmost")}
      >
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" width="13" height="13">
          <path d="M9 3h6" />
          <path d="M10 3v5l-3 4h10l-3-4V3" />
          <path d="M12 12v9" />
        </svg>
      </button>
      <button
        class="tb-btn tb-move"
        title="按住拖动悬浮窗"
        aria-label="移动悬浮窗"
        onmousedown={(e) => {
          e.preventDefault();
          act("toolbar:move");
        }}
      >
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" width="13" height="13">
          <path d="M12 3v18M3 12h18" />
          <path d="M12 3l-2.5 2.5M12 3l2.5 2.5M12 21l-2.5-2.5M12 21l2.5-2.5M3 12l2.5-2.5M3 12l2.5 2.5M21 12l-2.5-2.5M21 12l-2.5 2.5" />
        </svg>
      </button>
    </div>
    {#if toolbar.tabs.length > 0}
      <div class="tb-sep" aria-hidden="true"></div>
      <div class="tb-tabs">
        {#each toolbar.tabs as tab}
          <button
            class="tb-tab"
            class:active={tab.active}
            onclick={() => switchTab(tab.name)}
          >{tab.name}</button>
        {/each}
      </div>
    {/if}
  </div>
{/if}

<style>
  :global(html),
  :global(body) {
    margin: 0;
    padding: 0;
    background: transparent !important;
    /* 内容决定窗口尺寸：body 不填满视口，由内容撑开以便测量 */
    width: max-content;
    /* 弹窗不允许出现滚动条（WebView2 overlay 滚动条）：
       保留 overflow 默认（visible）以保证 tooltip 箭头（绝对定位伪元素）
       计入 scrollWidth/scrollHeight 测量、且窗口尺寸正确容纳箭头 */
    scrollbar-width: none;
    -ms-overflow-style: none;
  }
  :global(html::-webkit-scrollbar),
  :global(body::-webkit-scrollbar) {
    display: none;
    width: 0;
    height: 0;
  }

  /* ---- tooltip 视觉（与原 Tooltip.svelte 一致） ---- */
  .tooltip {
    position: relative;
    max-width: 240px;
    padding: 5px 9px;
    background: rgba(20, 20, 22, 0.95);
    border: 1px solid rgba(255, 255, 255, 0.12);
    border-radius: 5px;
    color: #d0d0d0;
    font-size: 11px;
    line-height: 1.4;
    word-break: break-word;
    white-space: normal;
    text-align: left;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.35);
  }
  /* 浮层在锚点上方：箭头朝下指向按钮 */
  .tooltip.above::after {
    content: "";
    position: absolute;
    top: 100%;
    left: 50%;
    transform: translateX(-50%);
    border: 5px solid transparent;
    border-top-color: rgba(20, 20, 22, 0.95);
  }
  /* 浮层在锚点下方：箭头朝上指向按钮 */
  .tooltip.below::after {
    content: "";
    position: absolute;
    bottom: 100%;
    left: 50%;
    transform: translateX(-50%);
    border: 5px solid transparent;
    border-bottom-color: rgba(20, 20, 22, 0.95);
  }

  /* ---- 右键菜单视觉（与原 ctx-menu 一致） ---- */
  .ctx-menu {
    min-width: 150px;
    background: rgba(40, 40, 44, 0.98);
    border: 1px solid rgba(255, 255, 255, 0.12);
    border-radius: 8px;
    padding: 4px;
    box-shadow: 0 6px 20px rgba(0, 0, 0, 0.45);
  }
  .ctx-item {
    display: block;
    width: 100%;
    padding: 7px 10px;
    background: none;
    border: none;
    border-radius: 5px;
    color: #e0e0e0;
    font-size: 12px;
    text-align: left;
    cursor: pointer;
  }
  .ctx-item:hover:not(:disabled) {
    background: rgba(122, 162, 247, 0.18);
  }
  .ctx-item:disabled {
    color: #777;
    cursor: default;
  }
  .ctx-hint {
    padding: 4px 10px 6px;
    font-size: 10px;
    color: #888;
  }

  /* ---- 顶栏浮层视觉（与悬浮窗内控制按钮/分组标签同款深色胶囊） ---- */
  .toolbar {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 4px 6px;
    background: rgba(28, 28, 30, 0.92);
    border: 1px solid rgba(255, 255, 255, 0.14);
    border-radius: 10px;
    box-shadow: 0 4px 16px rgba(0, 0, 0, 0.4);
    /* 淡入：每次显示播放；淡出由 .fading 状态驱动 transition */
    opacity: 1;
    animation: tb-fade-in 0.18s ease;
    transition: opacity 0.15s ease;
  }
  .toolbar.fading {
    opacity: 0;
  }
  @keyframes tb-fade-in {
    from {
      opacity: 0;
    }
    to {
      opacity: 1;
    }
  }

  .tb-ctrl {
    display: flex;
    align-items: center;
    gap: 2px;
  }
  .tb-btn {
    width: 22px;
    height: 22px;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 0;
    border: none;
    border-radius: 5px;
    background: rgba(40, 40, 44, 0.85);
    color: #9a9a9a;
    cursor: pointer;
    transition: background 0.12s, color 0.12s;
  }
  .tb-btn:hover {
    background: rgba(255, 255, 255, 0.14);
    color: #ddd;
  }
  .tb-btn svg {
    pointer-events: none; /* 保证整按钮命中区域 */
  }
  .tb-move {
    cursor: grab;
  }
  .tb-move:active {
    cursor: grabbing;
  }
  /* 透明度按钮：处于半透明状态时高亮提示 */
  .tb-btn.is-dimmed {
    background: rgba(122, 184, 255, 0.22);
    color: #7ab8ff;
  }
  /* 置顶按钮：置顶激活态高亮 */
  .tb-btn.is-active {
    background: rgba(122, 184, 255, 0.22);
    color: #7ab8ff;
  }

  /* 控制按钮组与分组标签组分隔符 */
  .tb-sep {
    width: 1px;
    height: 16px;
    background: rgba(255, 255, 255, 0.14);
    margin: 0 2px;
  }

  .tb-tabs {
    display: flex;
    align-items: center;
    gap: 4px;
  }
  .tb-tab {
    flex: 0 0 auto;
    padding: 2px 9px;
    border: none;
    border-radius: 5px;
    background: rgba(40, 40, 44, 0.85); /* 与按钮组/控制按钮同款深色 */
    color: #9a9a9a;
    font-size: 11px;
    line-height: 1.4;
    cursor: pointer;
    transition: background 0.12s, color 0.12s;
  }
  .tb-tab:hover {
    background: rgba(255, 255, 255, 0.14);
    color: #ddd;
  }
  .tb-tab.active {
    background: rgba(122, 162, 247, 0.22);
    color: #7ab8ff;
  }
</style>
