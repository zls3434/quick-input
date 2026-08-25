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

  interface ShowPayload {
    kind: "tooltip" | "menu";
    text?: string | null;
    items?: MenuItem[] | null;
  }

  let kind = $state<"tooltip" | "menu">("tooltip");
  let text = $state("");
  let items = $state<MenuItem[]>([]);
  // 浮层相对锚点方向（tooltip 箭头用）：above=true → 箭头朝下指向按钮
  let above = $state(true);

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
      reportSize();
    });
    const unorient = listen<{ above: boolean }>("floater://orient", (e) => {
      above = e.payload.above;
    });
    const unhide = listen("floater://hide", () => {
      text = "";
      items = [];
    });
    // 兜底：页面 JS 因后台节流挂起期间到达的显示请求（事件丢失），
    // 页面启动后主动取回最近一次请求，保证浮层内容不丢。
    void invoke<ShowPayload | null>("floater_pull_pending")
      .then((pending) => {
        if (!pending) return;
        kind = pending.kind;
        text = pending.text ?? "";
        items = pending.items ?? [];
        reportSize();
      })
      .catch(() => {});
    return () => {
      void unshow.then((f) => f());
      void unorient.then((f) => f());
      void unhide.then((f) => f());
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
</style>
