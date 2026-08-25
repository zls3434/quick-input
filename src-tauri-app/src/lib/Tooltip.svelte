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
    // 锚点取整个按钮（.button-item）。wrap 是 display:contents 且为按钮的
    // 父级（`<Tooltip><button>`），closest 向上找不到内部按钮，须直接取子元素。
    const btnEl = (el.firstElementChild as HTMLElement | null) ??
      el.closest(".button-item") ??
      el.parentElement ??
      el;
    const r = btnEl.getBoundingClientRect();
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
    /* 不生成盒子：按钮直接参与 .button-list 的 flex 布局（布局不变），
       同时鼠标事件仍从按钮冒泡至此触发 tooltip，覆盖整个按钮区域 */
    display: contents;
  }
</style>
