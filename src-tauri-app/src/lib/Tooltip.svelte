<script lang="ts">
  // Tooltip 悬浮注释组件（S131）
  // 悬停按钮显示 comment，移出后隐藏；空注释不显示。
  // 内容渲染在外置浮层窗口（floater）中，避免超出悬浮窗被裁剪。
  import { hideTooltip, showTooltip } from "$lib/floater";

  let { text, children }: { text: string | null; children: import("svelte").Snippet } = $props();
  let wrap: HTMLSpanElement | undefined = $state();
  // 防抖隐藏：hover 快速切换按钮时（旧按钮 leave → 新按钮 enter），
  // 延迟 hide 让新按钮的 show 覆盖旧内容，避免 tooltip 闪断。
  let hideTimer: ReturnType<typeof setTimeout> | undefined = $state();

  function show() {
    if (!text) return;
    clearTimeout(hideTimer);
    const el = wrap;
    if (!el) return;
    const r = el.getBoundingClientRect();
    showTooltip(text, { x: r.left, y: r.top, w: r.width, h: r.height });
  }

  function scheduleHide() {
    clearTimeout(hideTimer);
    hideTimer = setTimeout(() => hideTooltip(), 120);
  }
</script>

<span
  bind:this={wrap}
  class="tooltip-wrap"
  role="presentation"
  onmouseenter={show}
  onmouseleave={scheduleHide}
>
  {@render children()}
</span>

<style>
  .tooltip-wrap {
    position: relative;
    display: inline-flex;
    width: 100%;
  }
</style>
