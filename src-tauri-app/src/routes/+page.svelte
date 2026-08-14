<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import { onMount } from "svelte";
  import Tooltip from "$lib/Tooltip.svelte";

  interface ButtonConfig {
    id: string;
    label: string;
    content: string;
    comment: string | null;
  }

  let buttons = $state<ButtonConfig[]>([]);
  let loading = $state(true);
  let error = $state<string | null>(null);
  let injectingId = $state<string | null>(null);
  let lastError = $state<string | null>(null);

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

    // 阻止 mousedown 默认行为：防止 WebView2 点击夺取键盘焦点（保持原输入框焦点）
    // 拖动区域（data-tauri-drag-region）由系统处理拖拽，跳过不拦截
    const blockFocusSteal = (e: MouseEvent) => {
      if ((e.target as HTMLElement | null)?.closest("[data-tauri-drag-region]")) return;
      e.preventDefault();
    };
    window.addEventListener("mousedown", blockFocusSteal, true);

    // 监听配置切换事件，收到后自动刷新按钮列表
    const unlisten = listen("ConfigSwitched", () => {
      loadButtons();
    });
    return () => {
      window.removeEventListener("mousedown", blockFocusSteal, true);
      unlisten.then((fn) => fn());
    };
  });
</script>

<svelte:head>
  <title>QuickInput</title>
</svelte:head>

<main class="quickinput-overlay">
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