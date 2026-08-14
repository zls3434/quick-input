<script lang="ts">
  // Tooltip 悬浮注释组件（S131）
  // 悬停按钮显示 comment，移出后隐藏；空注释不显示。
  let { text, children }: { text: string | null; children: import("svelte").Snippet } = $props();
  let visible = $state(false);

  function show() {
    visible = !!text;
  }
  function hide() {
    visible = false;
  }
</script>

<span
  class="tooltip-wrap"
  role="presentation"
  onmouseenter={show}
  onmouseleave={hide}
>
  {@render children()}
  {#if visible && text}
    <span class="tooltip" role="tooltip">{text}</span>
  {/if}
</span>

<style>
  .tooltip-wrap {
    position: relative;
    display: inline-flex;
    width: 100%;
  }

  .tooltip {
    position: absolute;
    bottom: calc(100% + 6px); /* 定位在按钮上方 */
    left: 50%;
    transform: translateX(-50%);
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
    z-index: 100;
    pointer-events: none; /* 不拦截鼠标事件 */
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.35);
    /* 不超出面板边界 */
    max-height: 200px;
    overflow-y: auto;
  }

  /* 小三角指示器 */
  .tooltip::after {
    content: '';
    position: absolute;
    top: 100%;
    left: 50%;
    transform: translateX(-50%);
    border: 5px solid transparent;
    border-top-color: rgba(20, 20, 22, 0.95);
  }
</style>