<script lang="ts">
  // Tooltip 悬浮注释组件（S131）
  // 悬停按钮显示 comment，移出后隐藏；空注释不显示。
  // 内容渲染在外置浮层窗口（floater）中，避免超出悬浮窗被裁剪。
  import { hideTooltip, showTooltip } from "$lib/floater";

  let {
    text,
    opacityPct,
    children,
  }: {
    text: string | null;
    opacityPct: number;
    children: import("svelte").Snippet;
  } = $props();
  let wrap: HTMLSpanElement | undefined = $state();
  // 显示防抖：鼠标快速掠过按钮（停留不足阈值）不触发 tooltip，避免闪烁。
  // 隐藏防抖：hover 快速切换按钮时（旧按钮 leave → 新按钮 enter），
  // 延迟 hide 让新按钮的 show 覆盖旧内容，避免 tooltip 闪断。
  const SHOW_DELAY = 80;
  const HIDE_DELAY = 120;
  let showTimer: ReturnType<typeof setTimeout> | undefined = $state();
  let hideTimer: ReturnType<typeof setTimeout> | undefined = $state();
  // 本次显示对应的浮层令牌：hide 时回传，防止旧按钮的延迟隐藏误杀新 tooltip
  let token: number | undefined = $state();

  function show() {
    if (!text) return;
    const el = wrap;
    if (!el) return;
    // 锚点取整个按钮（.button-item）。wrap 是 display:contents 且为按钮的
    // 父级（`<Tooltip><button>`），closest 向上找不到内部按钮，须直接取子元素。
    const btnEl = (el.firstElementChild as HTMLElement | null) ??
      el.closest(".button-item") ??
      el.parentElement ??
      el;
    const r = btnEl.getBoundingClientRect();
    token = showTooltip(text, { x: r.left, y: r.top, w: r.width, h: r.height }, opacityPct);
  }

  function scheduleShow() {
    if (!text) return;
    clearTimeout(hideTimer);
    clearTimeout(showTimer);
    showTimer = setTimeout(show, SHOW_DELAY);
  }

  function scheduleHide() {
    clearTimeout(showTimer);
    clearTimeout(hideTimer);
    hideTimer = setTimeout(() => {
      hideTooltip(token);
      token = undefined;
    }, HIDE_DELAY);
  }

  // 组件卸载（如按钮列表刷新）时：清理未触发的定时器；若浮层仍显示着
  // 本按钮的 tooltip，主动关闭，避免残留浮层。令牌校验保证不会误杀
  // 已由其他按钮接管的 tooltip。
  $effect(() => {
    return () => {
      clearTimeout(showTimer);
      clearTimeout(hideTimer);
      hideTooltip(token);
      token = undefined;
    };
  });
</script>

<span
  bind:this={wrap}
  class="tooltip-wrap"
  role="presentation"
  onmouseenter={scheduleShow}
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
