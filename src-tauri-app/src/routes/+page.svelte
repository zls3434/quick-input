<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import { onMount } from "svelte";
  import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
  import { PhysicalPosition, PhysicalSize } from "@tauri-apps/api/dpi";
  import Tooltip from "$lib/Tooltip.svelte";

  interface ButtonConfig {
    id: string;
    label: string;
    content: string;
    comment: string | null;
  }

  interface OverlaySettings {
    layout: string;
    vertical_x: number | null;
    vertical_y: number | null;
    vertical_w: number | null;
    vertical_h: number | null;
    horizontal_x: number | null;
    horizontal_y: number | null;
    horizontal_w: number | null;
  }

  let buttons = $state<ButtonConfig[]>([]);
  let loading = $state(true);
  let error = $state<string | null>(null);
  let injectingId = $state<string | null>(null);
  let lastError = $state<string | null>(null);
  // 悬浮窗布局：vertical（竖向）| horizontal（横向）
  let layout = $state<"vertical" | "horizontal">("vertical");
  // 拖动标题栏高度（横向布局自适应高度的组成部分）
  const TITLE_H = 26;

  async function loadLayout() {
    try {
      const s = await invoke<OverlaySettings>("get_overlay_settings");
      const newLayout = s.layout === "horizontal" ? "horizontal" : "vertical";
      if (newLayout !== layout) {
        layout = newLayout;
        // 布局切换后内容重排，触发高度自适应（等待 DOM 更新）
        setTimeout(() => window.dispatchEvent(new CustomEvent("quickinput:adjust-height")), 120);
      }
    } catch (e) {
      console.error("加载悬浮窗设置失败", e);
    }
  }

  async function loadButtons() {
    loading = true;
    error = null;
    try {
      buttons = await invoke<ButtonConfig[]>("get_buttons");
    } catch (e) {
      error = `加载按钮失败: ${e}`;
      console.error(e);
    } finally {
      loading = false;
      // 按钮列表变化可能改变横向布局行数，通知高度自适应（监听在 onMount 注册）
      window.dispatchEvent(new CustomEvent("quickinput:adjust-height"));
    }
  }

  async function handleClick(btn: ButtonConfig) {
    if (injectingId !== null) return; // 防止重复点击
    injectingId = btn.id;
    lastError = null;
    try {
      // 调用 M2 注入引擎，将按钮 content 注入到当前焦点输入框
      await invoke("inject_text", { text: btn.content });
    } catch (e) {
      // AC3-3: 注入失败不崩溃，输出控制台错误日志
      console.error(`注入失败 [${btn.label}]: ${e}`);
      lastError = `注入失败: ${e}`;
    } finally {
      injectingId = null;
    }
  }

  onMount(() => {
    loadButtons();
    loadLayout();

    // 阻止 mousedown 默认行为：防止 WebView2 点击夺取键盘焦点（保持原输入框焦点）
    // 拖动区域（data-tauri-drag-region）由系统处理拖拽，跳过不拦截
    const blockFocusSteal = (e: MouseEvent) => {
      if ((e.target as HTMLElement | null)?.closest("[data-tauri-drag-region]")) return;
      e.preventDefault();
    };
    window.addEventListener("mousedown", blockFocusSteal, true);

    // 监听配置切换事件，收到后自动刷新按钮列表与布局
    const unlisten = listen("ConfigSwitched", () => {
      loadButtons();
      loadLayout();
    });

    const win = getCurrentWebviewWindow();

    // ---- 几何记忆：拖动/缩放结束后防抖保存位置与尺寸 ----
    // 尺寸使用 innerSize（客户区）：setSize 的参数语义即客户区，
    // 若用 outerSize（含不可见边框）保存/恢复会造成每次重启尺寸漂移。
    let saveTimer: ReturnType<typeof setTimeout> | null = null;
    const scheduleSaveGeometry = () => {
      if (saveTimer) clearTimeout(saveTimer);
      saveTimer = setTimeout(async () => {
        try {
          const [pos, inner, scale] = await Promise.all([
            win.outerPosition(),
            win.innerSize(),
            win.scaleFactor(),
          ]);
          await invoke("save_overlay_geometry", {
            layout,
            x: Math.round(pos.x / scale),
            y: Math.round(pos.y / scale),
            w: Math.round(inner.width / scale),
            h: Math.round(inner.height / scale),
          });
        } catch (e) {
          console.error("保存悬浮窗几何失败", e);
        }
      }, 600);
    };
    const unlistenMoved = win.onMoved(() => scheduleSaveGeometry());
    const unlistenResized = win.onResized(() => scheduleSaveGeometry());

    // ---- 横向布局高度自适应：按按钮行数调整客户区高度 ----
    // 位移策略：首次调整（启动加载）保持顶边不动，避免窗口整体下移；
    // 之后的调整（用户拖宽导致换行变化）保持底边不动，符合拖动交互习惯。
    let firstAdjustDone = false;
    let adjustTimer: ReturnType<typeof setTimeout> | null = null;
    const adjustHorizontalHeight = async () => {
      if (layout !== "horizontal") return;
      const list = document.querySelector<HTMLElement>(".button-list");
      if (!list) return;
      try {
        const [pos, inner, outer, scale] = await Promise.all([
          win.outerPosition(),
          win.innerSize(),
          win.outerSize(),
          win.scaleFactor(),
        ]);
        // 目标客户区高度（逻辑像素）：标题栏 + 列表实际高度（含 padding）+ 底部余量
        const listH = list.scrollHeight;
        const banner = document.querySelector<HTMLElement>(".error-banner");
        const bannerH = banner ? banner.offsetHeight + 8 : 0;
        const targetInnerH = Math.round((TITLE_H + listH + bannerH + 4) * scale);
        if (Math.abs(inner.height - targetInnerH) > 2) {
          // 位移补偿：首次保持顶边，之后保持底边
          const chrome = outer.height - inner.height;
          const newOuterH = targetInnerH + chrome;
          const newY = firstAdjustDone ? pos.y + outer.height - newOuterH : pos.y;
          await win.setSize(new PhysicalSize(inner.width, targetInnerH));
          await win.setPosition(new PhysicalPosition(pos.x, newY));
        }
        firstAdjustDone = true;
      } catch (e) {
        console.error("调整悬浮窗高度失败", e);
      }
    };
    const scheduleAdjust = () => {
      if (adjustTimer) clearTimeout(adjustTimer);
      adjustTimer = setTimeout(adjustHorizontalHeight, 80);
    };
    // 按钮增删/布局切换/窗口缩放统一走事件兜底：
    // - loadButtons 完成后派发（onMount 时的 ResizeObserver 因列表节点
    //   在 loading 阶段尚不存在而绑定失败，故不依赖它）
    // - 布局切换时 loadLayout 派发
    // - 窗口尺寸变化（用户拖宽改变换行）由 onResized 触发
    const onAdjustEvt = () => scheduleAdjust();
    window.addEventListener("quickinput:adjust-height", onAdjustEvt);
    const unlistenResizedAdjust = win.onResized(() => scheduleAdjust());
    // 首次加载兜底（等待 loading 结束与 DOM 渲染）
    setTimeout(() => scheduleAdjust(), 300);

    return () => {
      window.removeEventListener("mousedown", blockFocusSteal, true);
      unlisten.then((fn) => fn());
      unlistenMoved.then((fn) => fn());
      unlistenResized.then((fn) => fn());
      unlistenResizedAdjust.then((fn) => fn());
      window.removeEventListener("quickinput:adjust-height", onAdjustEvt);
    };
  });
</script>

<svelte:head>
  <title>QuickInput</title>
</svelte:head>

<main class="quickinput-overlay" class:layout-horizontal={layout === "horizontal"}>
  <div class="drag-region" data-tauri-drag-region>QuickInput</div>

  {#if loading}
    <div class="empty-state">
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" width="24" height="24">
        <circle cx="12" cy="12" r="10" stroke-dasharray="30 70" stroke-linecap="round">
          <animateTransform attributeName="transform" type="rotate" from="0 12 12" to="360 12 12" dur="1s" repeatCount="indefinite"/>
        </circle>
      </svg>
      <p>加载中…</p>
    </div>
  {:else if error}
    <div class="empty-state">
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" width="24" height="24">
        <circle cx="12" cy="12" r="10"/><line x1="12" y1="8" x2="12" y2="12"/><line x1="12" y1="16" x2="12.01" y2="16"/>
      </svg>
      <p class="error-text">{error}</p>
    </div>
  {:else if buttons.length === 0}
    <div class="empty-state">
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" width="24" height="24">
        <path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"/>
        <path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z"/>
      </svg>
      <p>暂无快捷按钮</p>
      <p class="hint">编辑 default.toml 添加按钮</p>
    </div>
  {:else}
    {#if lastError}
      <div class="error-banner">{lastError}</div>
    {/if}
    <div class="button-list">
      {#each buttons as btn (btn.id)}
        <button
          class="button-item"
          class:is-clicking={injectingId === btn.id}
          data-id={btn.id}
          disabled={injectingId !== null}
          onclick={() => handleClick(btn)}
        >
          <Tooltip text={btn.comment}>
            <span class="button-label">{btn.label}</span>
          </Tooltip>
          {#if btn.comment}
            <span class="button-comment">{btn.comment}</span>
          {/if}
        </button>
      {/each}
    </div>
  {/if}

  <!-- 缩放手柄视觉指示器（右下角） -->
  <div class="resize-handle" aria-hidden="true">
    <svg viewBox="0 0 12 12" width="10" height="10">
      <line x1="8" y1="12" x2="12" y2="8" stroke="#555" stroke-width="1.5"/>
      <line x1="4" y1="12" x2="12" y2="4" stroke="#555" stroke-width="1.5"/>
      <line x1="0" y1="12" x2="12" y2="0" stroke="#555" stroke-width="1.5"/>
    </svg>
  </div>
</main>

<style>
  :global(html),
  :global(body) {
    height: 100%;
    margin: 0;
    overflow: hidden;
    background: transparent !important;
  }

  .quickinput-overlay {
    position: relative;
    display: flex;
    flex-direction: column;
    height: 100vh;
    background: rgba(28, 28, 30, 0.92);
    border-radius: 10px;
    overflow: hidden;
    color: #e0e0e0;
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Microsoft YaHei', sans-serif;
    user-select: none;
    -webkit-user-select: none;
  }

  .drag-region {
    -webkit-app-region: drag;
    height: 26px;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 11px;
    font-weight: 600;
    letter-spacing: 1px;
    color: #8a8a8a;
    background: rgba(255, 255, 255, 0.04);
    cursor: grab;
    flex-shrink: 0;
  }

  .button-list {
    flex: 1;
    overflow-y: auto;
    padding: 4px 6px;
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  /* 横向布局：按钮水平排列、自动换行、更紧凑 */
  .layout-horizontal .button-list {
    flex-direction: row;
    flex-wrap: wrap;
    align-items: stretch;
    align-content: flex-start;
    gap: 4px;
    padding: 4px 8px;
    /* 不纵向拉伸：scrollHeight 反映真实内容行数，
       避免高度自适应目标随窗口高度虚高导致死循环 */
    flex: 0 0 auto;
  }
  .layout-horizontal .button-item {
    width: auto;
    min-width: 72px;
    flex: 0 1 auto;
    align-items: center;
    text-align: center;
    padding: 4px 10px;
  }
  .layout-horizontal .button-comment {
    display: none; /* 横向条形空间有限，隐藏注释行（悬浮 Tooltip 仍可用） */
  }
  .layout-horizontal .error-banner {
    width: 100%;
  }

  .button-list::-webkit-scrollbar {
    width: 4px;
  }
  .button-list::-webkit-scrollbar-track {
    background: transparent;
  }
  .button-list::-webkit-scrollbar-thumb {
    background: rgba(255, 255, 255, 0.15);
    border-radius: 2px;
  }

  .button-item {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    width: 100%;
    padding: 6px 10px;
    border: none;
    border-radius: 6px;
    background: rgba(255, 255, 255, 0.06);
    color: #e0e0e0;
    cursor: pointer;
    text-align: left;
    transition: background 0.12s;
    font-family: inherit;
    -webkit-app-region: no-drag;
  }
  .button-item:hover {
    background: rgba(255, 255, 255, 0.12);
  }
  .button-item:disabled {
    opacity: 0.6;
    cursor: default;
  }
  .button-item.is-clicking {
    background: rgba(255, 255, 255, 0.15);
  }

  .button-label {
    font-size: 13px;
    font-weight: 500;
    line-height: 1.3;
  }

  .button-comment {
    font-size: 10px;
    color: #888;
    margin-top: 1px;
    line-height: 1.2;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    max-width: 100%;
  }

  .empty-state {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 8px;
    color: #6a6a6a;
    font-size: 13px;
    text-align: center;
    padding: 16px;
  }
  .empty-state svg { opacity: 0.45; }
  .empty-state .hint { font-size: 11px; color: #555; }
  .error-text { color: #e74c3c; font-size: 12px; }

  .error-banner {
    margin: 4px 8px;
    padding: 4px 8px;
    background: rgba(231, 76, 60, 0.15);
    color: #e74c3c;
    font-size: 11px;
    border-radius: 4px;
    flex-shrink: 0;
  }

  /* 缩放手柄指示器（右下角） */
  .resize-handle {
    position: absolute;
    right: 2px;
    bottom: 2px;
    width: 14px;
    height: 14px;
    display: flex;
    align-items: center;
    justify-content: center;
    opacity: 0.5;
    cursor: nwse-resize;
    pointer-events: none; /* 不拦截窗口原生缩放 */
    -webkit-app-region: no-drag;
  }
  .resize-handle:hover {
    opacity: 0.9;
  }
</style>