<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import { onMount } from "svelte";
  import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
  import Tooltip from "$lib/Tooltip.svelte";
  import { hideFloater, resetFloaterKind, showMenu } from "$lib/floater";
  import {
    fadeOutToolbar,
    isMenuVisible,
    showToolbar,
    type ToolbarPayload,
    type ToolbarTab,
  } from "$lib/floater";

  interface ButtonConfig {
    id: string;
    label: string;
    content: string;
    comment: string | null;
    group?: string | null;
  }

  // 悬浮窗按钮分组视图（后端 get_buttons 返回）
  interface GroupView {
    name: string;
    buttons: ButtonConfig[];
  }

  interface ButtonsView {
    groups: GroupView[];
    default_buttons: ButtonConfig[];
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
    opacity: number | null;
    always_on_top: boolean | null;
  }

  // 悬浮窗按钮分组状态：defaultButtons 为未分组按钮（「默认」标签内容），
  // groups 为画像自定义分组（Tab 栏数据源），activeGroup 为当前选中分组名
  // （null 表示「默认」分组）。
  let defaultButtons = $state<ButtonConfig[]>([]);
  let groups = $state<GroupView[]>([]);
  let activeGroup = $state<string | null>(null);
  let loading = $state(true);
  let error = $state<string | null>(null);
  // 当前前台进程生效的注入模式：paste（剪贴板粘贴）/ keystroke（按键模拟）
  // 随按钮列表一起刷新（ConfigSwitched / 启动时），点击按钮时传给 inject_text
  let injectMode = $state<"paste" | "keystroke">("paste");
  let injectingId = $state<string | null>(null);
  let lastError = $state<string | null>(null);
  // 悬浮窗布局：vertical（竖向）| horizontal（横向）
  let layout = $state<"vertical" | "horizontal">("vertical");
  // 布局是否已从配置加载完成：加载前不渲染按钮，避免"横排窗口 + 竖排按钮"
  // 的错位（layout 初始为 vertical，而窗口几何已按配置设为横排）。
  let layoutReady = $state(false);
  let layoutRetries = 0;
  // 悬浮窗透明度百分比（20~100，默认 85；配置持久化；CSS 级视觉实现）
  let overlayOpacityPct = $state(85);
  // 悬浮窗是否置顶（配置持久化）
  let alwaysOnTop = $state(true);
  // 透明度切换档位（百分比）：85%（微透）↔ 30%（通透）两档互切
  const OPACITY_STEPS = [85, 30];

  async function loadLayout() {
    try {
      const s = await invoke<OverlaySettings>("get_overlay_settings");
      const newLayout = s.layout === "horizontal" ? "horizontal" : "vertical";
      if (newLayout !== layout) {
        layout = newLayout;
        // 布局切换后内容重排，触发高度自适应（等待 DOM 更新）
        setTimeout(() => window.dispatchEvent(new CustomEvent("quickinput:adjust-height")), 120);
      }
      overlayOpacityPct =
        s.opacity !== null ? Math.min(100, Math.max(20, s.opacity)) : 85;
      alwaysOnTop = s.always_on_top !== null ? s.always_on_top : true;
      layoutReady = true;
    } catch (e) {
      // 启动竞态兜底：get_overlay_settings 可能在 Rust setup 完成前失败，重试
      console.error("加载悬浮窗设置失败", e);
      if (layoutRetries < 20) {
        layoutRetries += 1;
        setTimeout(() => void loadLayout(), 300);
      } else {
        // 重试耗尽：按默认竖排继续渲染（保证功能可用，不阻塞按钮加载）
        layoutReady = true;
      }
    }
  }

  // 快速切换透明度：循环档位并持久化（百分比）
  async function cycleOpacity() {
    const idx = OPACITY_STEPS.indexOf(overlayOpacityPct);
    const next = OPACITY_STEPS[(idx + 1) % OPACITY_STEPS.length];
    overlayOpacityPct = next;
    try {
      await invoke("set_overlay_opacity", { opacity: next });
    } catch (e) {
      console.error("保存透明度失败", e);
    }
  }

  // 切换横竖布局（后端应用几何并广播 ConfigSwitched，loadLayout 收到后刷新）
  // 布局切换会直接改变窗口几何（触发 onResized），置位标志避免该尺寸变化
  // 被误记为竖排用户自定义高度
  let layoutSwitching = false;
  async function toggleLayout() {
    const next = layout === "vertical" ? "horizontal" : "vertical";
    layoutSwitching = true;
    try {
      await invoke("set_overlay_layout", { layout: next });
    } catch (e) {
      console.error("切换布局失败", e);
    }
    setTimeout(() => {
      layoutSwitching = false;
    }, 800);
  }

  // 隐藏悬浮窗（托盘菜单 / 全局热键可再次显示）
  // 走后端命令：记录用户隐藏意图，避免自愈机制把主动隐藏误判为异常抢显
  function hideOverlay() {
    invoke("hide_overlay").catch((e) => console.error("隐藏悬浮窗失败", e));
  }

  // 切换置顶并持久化（后端同步更新窗口 Z-order 与扩展样式）
  async function toggleAlwaysOnTop() {
    const next = !alwaysOnTop;
    alwaysOnTop = next;
    try {
      await invoke("set_overlay_always_on_top", { enabled: next });
    } catch (e) {
      console.error("保存置顶状态失败", e);
      alwaysOnTop = !next; // 失败回滚
    }
  }

  // 移动按钮：按下即进入窗口拖动模式
  // 同时置位拖动会话标志：后端跟随线程暂停重定位，避免与拖动抢位；
  // 拖动结束（松开左键）由后端按物理按键状态检测复位
  let userDragPending = $state(false);
  function onMoveDown(e: MouseEvent) {
    e.preventDefault();
    userDragPending = true;
    invoke("set_overlay_dragging", { dragging: true }).catch((err) =>
      console.error("置位拖动标志失败", err),
    );
    getCurrentWebviewWindow()
      .startDragging()
      .catch((err) => console.error("拖动悬浮窗失败", err));
  }

  // ---- 顶栏外置浮层（控制按钮组 + 分组标签，悬浮窗上方外侧右对齐）----
  // 浮层由独立透明窗口承载：鼠标移入右上角触发区（右侧 160px × 顶部 40px）
  // 显示，移出后延迟自动隐藏（浮层悬停期间取消），带淡入淡出效果。
  let toolbarShown = false; // overlay 侧记录的顶栏浮层显示状态
  let toolbarHideTimer: ReturnType<typeof setTimeout> | null = null;
  let lastToolbarShowAt = 0;
  let toolbarHideCooldownUntil = 0; // 淡出播放期防重入
  const TOOLBAR_TRIGGER_W = 160;
  const TOOLBAR_TRIGGER_H = 40;
  const TOOLBAR_HIDE_DELAY = 900;

  // 构建顶栏载荷：控制状态 + 分组标签（「默认」+ 画像分组，与悬浮窗 Tab 同源）
  function buildToolbarPayload(): ToolbarPayload {
    const tabs: ToolbarTab[] = [];
    if (showDefaultTab) tabs.push({ name: "默认", active: activeGroup === null });
    for (const g of groups) tabs.push({ name: g.name, active: activeGroup === g.name });
    return { layout, opacityPct: overlayOpacityPct, alwaysOnTop, tabs };
  }

  function showToolbarIfNeeded() {
    if (isMenuVisible()) return; // 右键菜单打开时不覆盖
    if (Date.now() < toolbarHideCooldownUntil) return; // 淡出冷却期
    if (toolbarShown && Date.now() - lastToolbarShowAt < 5000) return; // 已显示且未超刷新窗口
    toolbarShown = true;
    lastToolbarShowAt = Date.now();
    showToolbar(buildToolbarPayload());
  }

  function cancelToolbarHide() {
    if (toolbarHideTimer) {
      clearTimeout(toolbarHideTimer);
      toolbarHideTimer = null;
    }
  }

  // 移出触发区：延迟后淡出隐藏（浮层悬停可取消）
  function scheduleToolbarHide() {
    cancelToolbarHide();
    toolbarHideTimer = setTimeout(() => {
      toolbarHideTimer = null;
      if (!toolbarShown) return;
      toolbarShown = false;
      toolbarHideCooldownUntil = Date.now() + 350; // 淡出播放期防重入
      fadeOutToolbar();
    }, TOOLBAR_HIDE_DELAY);
  }

  // 立即隐藏（失焦/控制动作后浮层已被后端隐藏，仅复位状态）
  function hideToolbarNow() {
    cancelToolbarHide();
    if (!toolbarShown) return;
    toolbarShown = false;
    hideFloater();
  }

  // 启动竞态重试计数：页面加载可能早于 Rust setup 完成 AppState 管理，
  // get_buttons 会临时失败；限次重试直至成功（成功即清零）。
  let buttonLoadRetries = 0;

  async function loadButtons() {
    loading = true;
    error = null;
    try {
      const view = await invoke<ButtonsView>("get_buttons");
      groups = view.groups;
      defaultButtons = view.default_buttons;
      // 默认选中：存在未分组按钮时选中「默认」，否则选中首个分组
      activeGroup =
        view.default_buttons.length > 0 ? null : view.groups[0]?.name ?? null;
      buttonLoadRetries = 0;
      // 注入模式与按钮列表同源（同一配置匹配逻辑），一并刷新
      injectMode = await invoke<"paste" | "keystroke">("get_current_inject_mode");
      // 长按触发阈值（配置管理可调，200~5000ms，缺省 1000）
      const s = await invoke<{ hold_threshold_ms: number | null }>("get_overlay_settings");
      holdThresholdMs = Math.min(5000, Math.max(200, s.hold_threshold_ms ?? 1000));
    } catch (e) {
      console.error(`加载按钮失败: ${e}`);
      if (buttonLoadRetries < 20) {
        buttonLoadRetries += 1;
        // 保持 loading 状态重试：避免重试间隙闪现错误/空状态界面
        setTimeout(() => void loadButtons(), 300);
        return;
      }
      // 重试耗尽才显示错误
      error = `加载按钮失败: ${e}`;
    }
    loading = false;
    // 按钮列表变化可能改变横向布局行数，通知高度自适应（监听在 onMount 注册）
    window.dispatchEvent(new CustomEvent("quickinput:adjust-height"));
  }

  // 当前显示按钮：默认分组（activeGroup === null）或选中分组
  const visibleButtons = $derived(
    activeGroup === null
      ? defaultButtons
      : groups.find((g) => g.name === activeGroup)?.buttons ?? [],
  );
  // Tab 栏渲染条件：存在分组且（有分组按钮或有未分组按钮可显示）
  const showTabs = $derived(groups.length > 0);
  // 「默认」标签显隐：存在其他分组且有未分组按钮
  const showDefaultTab = $derived(groups.length > 0 && defaultButtons.length > 0);

  // ---- 按钮交互：单击输入，长按超过阈值输入后回车 ----
  // 按下（mousedown）即开始注入文本，同时启动定时器：
  // - 阈值内松开：仅输入，不回车（单击）
  // - 按住超过阈值：注入完成后自动补发回车（长按执行）
  // 回车任务排队在文本注入之后，注入慢也不会被回车截断
  // （注入本身为 SendInput 批量注入，毫秒级完成，远快于触发阈值）。
  // 阈值 holdThresholdMs 由配置管理设置（200~5000ms，默认 1000）。
  let holdThresholdMs = $state(1000);
  let holdTimer: ReturnType<typeof setTimeout> | null = null;
  let pressedId: string | null = null;
  let injectPromise: Promise<unknown> | null = null;

  async function onBtnDown(e: MouseEvent, btn: ButtonConfig) {
    // 右键：按下立即弹自定义菜单（不依赖 contextmenu 事件——部分鼠标/触控板
    // 驱动右键不产生 WM_CONTEXTMENU，导致只监听 contextmenu 时菜单无法触发）
    if (e.button === 2) {
      showCtxMenu(e, btn);
      return;
    }
    if (e.button !== 0) return; // 其他键忽略
       if (injectingId !== null) return; // 注入进行中禁止并发
    injectingId = btn.id;
    pressedId = btn.id;
    lastError = null;
    // 1. 按下立即注入文本；注入完成前锁定 injectingId（防止并发注入）
    //    模板按钮左键：占位符 {input} 不输出，光标回退到占位符位置
    const isTpl = isTemplateBtn(btn);
    const outText = isTpl ? removePlaceholder(btn.content) : btn.content;
    const cursorBack = isTpl ? charsAfterPlaceholder(btn.content) : 0;
    injectPromise = invoke("inject_text", { text: outText, cursorBack, mode: injectMode }).catch((err: unknown): void => {
      console.error(`注入失败 [${btn.label}]: ${err}`);
      lastError = `注入失败: ${err}`;
    });
    // 2. 启动长按定时器：超过阈值触发回车（等文本注入完成，避免截断）
    holdTimer = setTimeout(async () => {
      holdTimer = null;
      await injectPromise;
      try {
        await invoke("inject_enter", { mode: injectMode });
      } catch (err) {
        console.error(`回车注入失败: ${err}`);
      }
    }, holdThresholdMs);
  }

  function onBtnUp() {
    // 松开即取消回车（单击仅输入）
    if (holdTimer) {
      clearTimeout(holdTimer);
      holdTimer = null;
    }
    pressedId = null;
    // 兜底恢复焦点：等注入完成后再恢复（注入中途切前台会截断/丢失字符）
    if (injectPromise) {
      const p = injectPromise;
      injectPromise = null;
      void p.finally(() => {
        injectingId = null;
        void invoke("restore_focus").catch(() => {});
      });
    } else {
      void invoke("restore_focus").catch(() => {});
    }
  }

  function onBtnLeave() {
    // 按住拖出按钮范围：取消回车（防误触），已注入文本保留
    if (holdTimer) {
      clearTimeout(holdTimer);
      holdTimer = null;
    }
    pressedId = null;
    if (injectPromise) {
      const p = injectPromise;
      injectPromise = null;
      void p.finally(() => {
        injectingId = null;
        void invoke("restore_focus").catch(() => {});
      });
    } else {
      void invoke("restore_focus").catch(() => {});
    }
  }

  // ---- 按钮右键：自定义菜单 + 模板输入（原生 DOM 实现）----
  // 模板占位符：{input}。content 含 {input} 即为模板按钮（如 git commit -m "{input}"），
  // 位置任意、不依赖引号。右键菜单"模板输入"→ 弹窗填写（如 first init）→ 确认后
  // 合并注入（git commit -m "first init"）。
  // 左键：占位符不输出（其余内容原样注入），并回退光标到占位符位置
  // （如 git commit -m "{input}" 左键输出 git commit -m ""，光标在引号中间）。
  // 非模板按钮菜单项禁用，仅左键快捷输入。
  // 用原生 DOM 而非 Svelte 响应式：contextmenu 事件来自 window capture 委托，
  // 逃逸闭包中的状态更新在 legacy 模式下不保证触发重渲染。
  const PLACEHOLDER = "{input}";

  function isTemplateBtn(btn: ButtonConfig): boolean {
    return btn.content.includes(PLACEHOLDER);
  }

  // 占位符后的字符数（左键注入后光标需回退的步数）
  function charsAfterPlaceholder(content: string): number {
    const idx = content.indexOf(PLACEHOLDER);
    if (idx < 0) return 0;
    return content.length - idx - PLACEHOLDER.length;
  }

  // 移除占位符（左键输出内容）
  function removePlaceholder(content: string): string {
    return content.replace(PLACEHOLDER, "");
  }

  function removeTemplateDialog() {
    document.querySelector(".template-dialog")?.remove();
  }

  // 合并模板：占位符填入内容
  function mergeTemplate(content: string, value: string): string {
    return content.replace(PLACEHOLDER, value);
  }

  // 模板注入（确认后调用）
  async function injectText(text: string, label: string) {
    lastError = null;
    try {
      await invoke("inject_text", { text, mode: injectMode });
    } catch (err) {
      console.error(`注入失败 [${label}]: ${err}`);
      lastError = `注入失败: ${err}`;
    }
  }

  // 弹出模板输入对话框（原生 DOM）
  function showTemplateDialog(btn: ButtonConfig) {
    hideFloater();
    removeTemplateDialog();
    const overlay = document.createElement("div");
    overlay.className = "template-dialog";
    overlay.innerHTML = `
      <div class="dialog-box">
        <div class="dialog-title"></div>
        <input class="template-input" type="text" placeholder="输入模板内容…" />
        <div class="dialog-preview dim">内容将填入引号内</div>
        <div class="dialog-actions">
          <button class="btn-secondary" data-act="cancel">取消</button>
          <button class="btn-primary" data-act="ok">输入</button>
        </div>
      </div>`;
    const title = overlay.querySelector<HTMLElement>(".dialog-title")!;
    title.textContent = `模板输入 — ${btn.label}`;
    const input = overlay.querySelector<HTMLInputElement>(".template-input")!;
    const preview = overlay.querySelector<HTMLElement>(".dialog-preview")!;
    const updatePreview = () => {
      if (input.value) {
        preview.textContent = `将输入：${mergeTemplate(btn.content, input.value)}`;
        preview.classList.remove("dim");
      } else {
        preview.textContent = "内容将填入引号内";
        preview.classList.add("dim");
      }
    };
    input.addEventListener("input", updatePreview);
    // 关闭弹窗：恢复"点击不抢焦点"（NOACTIVATE），并把前台还给原输入窗口
    const close = () => {
      overlay.remove();
      void invoke("set_overlay_focusable", { enabled: false }).catch(() => {});
    };
    const confirm = async () => {
      const text = mergeTemplate(btn.content, input.value);
      close();
      // 先恢复原窗口前台，再注入（粘贴目标为原输入框）
      await invoke("set_overlay_focusable", { enabled: false }).catch(() => {});
      void injectText(text, btn.label);
    };
    input.addEventListener("keydown", (e) => {
      if (e.key === "Enter") void confirm();
      if (e.key === "Escape") close();
    });
    overlay.querySelector<HTMLButtonElement>('[data-act="cancel"]')!.addEventListener("click", close);
    overlay.querySelector<HTMLButtonElement>('[data-act="ok"]')!.addEventListener("click", () => void confirm());
    overlay.addEventListener("mousedown", (e) => e.stopPropagation());
    document.body.appendChild(overlay);
    // 临时切换为可输入：移除 NOACTIVATE 并激活窗口，使键盘焦点进入输入框
    void invoke("set_overlay_focusable", { enabled: true }).then(() => {
      input.focus();
      input.select();
    });
  }

  // 右键菜单去重标记：mousedown 与 contextmenu 兜底事件 200ms 内不重复弹
  let lastMenuAt = 0;
  let lastMenuBtnId: string | null = null;

  // 弹出右键自定义菜单（外置浮层窗口，经 show_floater 渲染）
  function showCtxMenu(e: MouseEvent, btn: ButtonConfig) {
    const now = Date.now();
    if (lastMenuBtnId === btn.id && now - lastMenuAt < 200) return;
    lastMenuAt = now;
    lastMenuBtnId = btn.id;

    removeTemplateDialog();
    const isTpl = isTemplateBtn(btn);
    const items = [
      {
        id: btn.id,
        label: isTpl ? "模板输入…" : "模板输入…（不可用）",
        disabled: !isTpl,
        hint: isTpl ? null : "该按钮内容不含 {input} 占位符",
      },
    ];
    const el = (e.target as HTMLElement | null)?.closest?.(".button-item") as HTMLElement | null;
    const rect = (el ?? (e.target as HTMLElement)).getBoundingClientRect();
    showMenu(
      items,
      { x: rect.left, y: rect.top, w: rect.width, h: rect.height },
      overlayOpacityPct,
    );
  }

  onMount(() => {
    loadButtons();
    loadLayout();

    // 窗口显示由 Rust 侧 on_page_load(Finished) 触发：
    // 窗口初始隐藏（visible:false）避免白屏闪烁，且隐藏窗口中 WebView2
    // 会挂起页面定时器，前端 setTimeout(show) 不可靠。

    // 阻止 mousedown 默认行为：防止 WebView2 点击夺取键盘焦点（保持原输入框焦点）。
    // 控制按钮（.ctrl-btn）与功能按钮的 click 不受 preventDefault 影响，
    // 移动按钮的 startDragging 亦是显式 API 调用。
    // 例外：弹窗（模板输入）内的控件需要正常聚焦打字。
    const blockFocusSteal = (e: MouseEvent) => {
      const target = e.target as HTMLElement | null;
      if (target?.closest?.(".template-dialog")) return;
      e.preventDefault();
    };
    window.addEventListener("mousedown", blockFocusSteal, true);

    // 屏蔽系统右键菜单；按钮右键（委托）弹外置自定义菜单（浮层窗口）。
    // 用 window capture 统一处理：实测 WebView2 中 contextmenu 事件 target
    // 是按钮内 tooltip-wrap 等子元素且不冒泡到按钮绑定，元素级绑定不可靠；
    // capture 必达，closest('.button-item') 反查按钮。
    const handleContextMenu = (e: MouseEvent) => {
      e.preventDefault();
      const t = e.target as HTMLElement | null;
      // 弹窗内右键：保持现状（不重复处理）
      if (t?.closest?.(".template-dialog")) return;
      const el = t?.closest?.(".button-item");
      if (el) {
        const btn = visibleButtons.find((b) => b.id === (el as HTMLElement).dataset.id);
        if (btn) {
          showCtxMenu(e, btn);
          return;
        }
      }
      // 非按钮区域右键：关闭外置菜单与弹窗
      hideFloater();
      removeTemplateDialog();
    };
    window.addEventListener("contextmenu", handleContextMenu, true);

    // 右键按下即弹菜单（capture 兜底）：不依赖按钮上的 onmousedown 绑定
    // 与事件冒泡（覆盖 Svelte 委托/绑定失效等场景）；按钮绑定分支的去重
    // 保证只弹一次。菜单/弹窗内右键不重复弹。
    const handleRightDown = (e: MouseEvent) => {
      if (e.button !== 2) return;
      const t = e.target as HTMLElement | null;
      if (t?.closest?.(".template-dialog")) return;
      const el = t?.closest?.(".button-item");
      if (el) {
        const btn = visibleButtons.find((b) => b.id === (el as HTMLElement).dataset.id);
        if (btn) showCtxMenu(e, btn);
      }
    };
    window.addEventListener("mousedown", handleRightDown, true);

    // 点击悬浮窗任意处关闭外置右键菜单（浮层为独立窗口，悬浮窗内点击均触发）
    const closeMenuOnClick = () => {
      hideFloater();
    };
    window.addEventListener("click", closeMenuOnClick);

    // Rust 侧隐藏浮层后通知（任意关闭路径：菜单项点击/禁用项/淡出等）：
    // 复位浮层类型状态与顶栏显示状态，避免菜单关闭后 currentKind 残留
    // 导致顶栏被 isMenuVisible() 永久拦截、或 toolbarShown 残留导致
    // 触发区 5 秒内不响应（纯状态重置，不再触发 hide，防循环）。
    const unlistenFloaterHidden = listen("floater-hidden", () => {
      resetFloaterKind();
      toolbarShown = false;
      cancelToolbarHide();
    });

    // Rust 侧浮层内容切换通知（show_floater 广播）：顶栏显示时登记
    // 显示状态；tooltip/菜单占用时复位顶栏状态。使 toolbarShown 始终
    // 与真实浮层内容一致——顶栏被 tooltip/菜单顶掉后触发区可立即重显。
    const unlistenFloaterShowing = listen<{ kind: string }>("floater-showing", (e) => {
      if (e.payload.kind === "toolbar") {
        toolbarShown = true;
        lastToolbarShowAt = Date.now();
        cancelToolbarHide();
      } else {
        toolbarShown = false;
        cancelToolbarHide();
      }
    });

    // 监听配置切换事件，收到后自动刷新按钮列表与布局
    const unlisten = listen("ConfigSwitched", () => {
      loadButtons();
      loadLayout();
    });

    // 外置菜单动作转发：菜单项点击 → Rust 隐藏浮层 → 转发回悬浮窗执行
    const unlistenMenuAction = listen<{ id: string }>("floater-menu-action", (e) => {
      const btn = visibleButtons.find((b) => b.id === e.payload.id);
      if (btn && isTemplateBtn(btn)) showTemplateDialog(btn);
    });

    // ---- 顶栏外置浮层：触发区显示 + 自动隐藏 + 动作分发 ----

    // 鼠标在悬浮窗内移动：右上角触发区显示顶栏，其余区域启动延迟隐藏
    const handleOverlayMouseMove = (e: MouseEvent) => {
      const inTrigger =
        e.clientX > window.innerWidth - TOOLBAR_TRIGGER_W && e.clientY < TOOLBAR_TRIGGER_H;
      if (inTrigger) {
        cancelToolbarHide();
        showToolbarIfNeeded();
      } else {
        scheduleToolbarHide();
      }
    };
    window.addEventListener("mousemove", handleOverlayMouseMove, { passive: true });

    // 鼠标离开悬浮窗窗口（移向桌面/其他窗口/顶栏浮层）：启动延迟隐藏。
    // 浮层悬停期间 overlay 收不到 mousemove，须靠 mouseout 兜底；
    // 浮层自身的 hover 上报（floater-hover）会取消该定时器。
    const handleOverlayMouseOut = (e: MouseEvent) => {
      const rt = e.relatedTarget as Node | null;
      if (!rt || !document.contains(rt)) scheduleToolbarHide();
    };
    window.addEventListener("mouseout", handleOverlayMouseOut);

    // 顶栏控制动作分发（与菜单共用 floater-menu-action 通道，Rust 已先隐藏浮层）：
    // toolbar:* 前缀在此处理，按钮菜单 id（按钮 id）由上方 unlistenMenuAction 处理
    const unlistenToolbarAction = listen<{ id: string }>("floater-menu-action", (e) => {
      const id = e.payload.id;
      if (id === "toolbar:hide") {
        hideToolbarNow();
        hideOverlay();
      } else if (id === "toolbar:layout") {
        hideToolbarNow();
        void toggleLayout();
      } else if (id === "toolbar:opacity") {
        hideToolbarNow();
        void cycleOpacity();
      } else if (id === "toolbar:topmost") {
        hideToolbarNow();
        void toggleAlwaysOnTop();
      } else if (id === "toolbar:move") {
        hideToolbarNow();
        // 浮层已隐藏，直接执行拖动逻辑（startDragging 为 Tauri API，无需原生事件）
        onMoveDown(new MouseEvent("mousedown"));
      }
    });

    // 顶栏分组标签切换：浮层保持显示，仅切换悬浮窗当前分组
    const unlistenTabSwitch = listen<{ name: string }>("floater-tab-switch", (e) => {
      activeGroup = e.payload.name === "默认" ? null : e.payload.name;
    });

    // 浮层悬停状态：悬停取消自动隐藏定时器，移出恢复延迟隐藏
    const unlistenHover = listen<{ hovering: boolean }>("floater-hover", (e) => {
      if (e.payload.hovering) cancelToolbarHide();
      else scheduleToolbarHide();
    });

    // 悬浮窗失焦（点击其他窗口/桌面）：立即隐藏顶栏
    const handleOverlayBlur = () => hideToolbarNow();
    window.addEventListener("blur", handleOverlayBlur);

    const win = getCurrentWebviewWindow();

    // ---- 几何记忆：拖动/缩放结束后防抖保存位置与尺寸 ----
    // 尺寸使用 innerSize（客户区）：setSize 的参数语义即客户区，
    // 若用 outerSize（含不可见边框）保存/恢复会造成每次重启尺寸漂移。
    // user_drag 仅在用户拖动会话后为 true（触发后端吸附判定与记忆更新）；
    // 程序性移动（吸附跟随）触发的 onMoved 不带该标志，仅静默更新几何。
    let saveTimer: ReturnType<typeof setTimeout> | null = null;
    const scheduleSaveGeometry = () => {
      if (saveTimer) clearTimeout(saveTimer);
      saveTimer = setTimeout(async () => {
        const wasUserDrag = userDragPending;
        userDragPending = false;
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
            userDrag: wasUserDrag,
          });
        } catch (e) {
          console.error("保存悬浮窗几何失败", e);
        }
      }, 600);
    };
    const unlistenMoved = win.onMoved(() => scheduleSaveGeometry());
    const unlistenResized = win.onResized(() => scheduleSaveGeometry());

    // ---- 悬浮窗高度自适应：按按钮内容调整客户区高度 ----
    // 策略：前端只测量目标高度，单次调用后端原子调整（位置 + 尺寸一次
    // SetWindowPos 生效，无"先改尺寸再改位置"的中间可见态）。
    // 锚定方向由后端解析：吸附时保持贴合边不动（窗口下方/屏顶向下扩展、
    // 窗口上方/屏底向上扩展、左右侧对称扩展）；无吸附时首次保顶边、
    // 之后保底边（符合拖动交互习惯）。
    // 竖排高度由用户控制并跨重启记忆（localStorage）：用户拖动调整高度后
    // 持久保存，内容变化时"用户高度优先、只扩不缩"（按钮增多自动扩展）。
    // 竖排用户自定义高度记忆键（WebView2 localStorage，跨会话持久）
    const VERTICAL_USER_H_KEY = "quickinput.overlay.vertical-user-h";
    // 程序贴合（apply_overlay_height）触发的尺寸变化标志：期间 onResized
    // 不写入用户记忆，避免把程序调整误记为用户自定义高度
    let programmaticResize = false;
    function readUserVerticalH(): number | null {
      try {
        const v = localStorage.getItem(VERTICAL_USER_H_KEY);
        if (v === null) return null;
        const n = Number(v);
        return Number.isFinite(n) && n > 0 ? n : null;
      } catch {
        return null;
      }
    }
    let firstAdjustDone = false;
    let adjustTimer: ReturnType<typeof setTimeout> | null = null;
    const adjustOverlayHeight = async () => {
      const list = document.querySelector<HTMLElement>(".button-list");
      if (!list) return;
      try {
        const [inner, scale] = await Promise.all([win.innerSize(), win.scaleFactor()]);
        // 目标客户区高度（物理像素）：列表实际高度（含四面 6px padding，控制条浮动
        // 不占空间）。不额外加余量，窗口精确贴合内容实现四面等宽。
        const listH = list.scrollHeight;
        const banner = document.querySelector<HTMLElement>(".error-banner");
        const bannerH = banner ? banner.offsetHeight + 8 : 0;
        let targetInnerH = Math.round((listH + bannerH) * scale);
        // 竖排：用户自定义高度优先（不低于内容，内容变多时自动扩展）
        if (layout === "vertical") {
          const userH = readUserVerticalH();
          if (userH !== null) {
            targetInnerH = Math.max(targetInnerH, userH);
          } else if (firstAdjustDone && targetInnerH < inner.height) {
            // 无记忆但当前窗口高于内容（防御）：保持用户高度不缩小
            return;
          }
        }
        // 上限：不超出显示器工作区（竖排按钮很多时防止窗口超出屏幕）
        const maxH = Math.max(window.screen.availHeight * scale - 16, 40);
        targetInnerH = Math.min(targetInnerH, maxH);
        if (Math.abs(inner.height - targetInnerH) > 2) {
          programmaticResize = true;
          await invoke("apply_overlay_height", {
            targetInnerH,
            fallbackKeepTop: !firstAdjustDone,
          });
          setTimeout(() => {
            programmaticResize = false;
          }, 600);
        }
        firstAdjustDone = true;
      } catch (e) {
        console.error("调整悬浮窗高度失败", e);
      }
    };
    const scheduleAdjust = () => {
      if (adjustTimer) clearTimeout(adjustTimer);
      adjustTimer = setTimeout(adjustOverlayHeight, 80);
    };
    // 按钮增删/布局切换统一走事件兜底：
    // - loadButtons 完成后派发（onMount 时的 ResizeObserver 因列表节点
    //   在 loading 阶段尚不存在而绑定失败，故不依赖它）
    // - 布局切换时 loadLayout 派发
    const onAdjustEvt = () => scheduleAdjust();
    window.addEventListener("quickinput:adjust-height", onAdjustEvt);
    // 窗口尺寸变化：
    // - 横排保持自适应贴合内容（按钮换行随宽度变化）
    // - 竖排由用户控制：拖动调整高度后记忆（跨重启）。仅当确为用户调整
    //   （首次加载完成、非程序贴合、非布局切换、窗口高度不低于内容）时记录，
    //   避免启动几何应用/程序贴合/布局切换的尺寸变化被误记为用户高度
    const unlistenResizedAdjust = win.onResized(() => {
      if (layout === "horizontal") {
        scheduleAdjust();
        return;
      }
      if (!firstAdjustDone || programmaticResize || layoutSwitching) return;
      const list = document.querySelector<HTMLElement>(".button-list");
      if (!list) return;
      // 窗口高度不低于内容高度（内容须完整可见）才算用户自定义高度
      const contentH = list.scrollHeight + 12;
      if (innerHeight >= contentH) {
        try {
          localStorage.setItem(VERTICAL_USER_H_KEY, String(innerHeight));
        } catch {
          /* localStorage 不可用时忽略（本次会话仍可拖动保持） */
        }
      }
    });
    // 首次加载兜底（等待 loading 结束与 DOM 渲染）
    setTimeout(() => scheduleAdjust(), 300);

    return () => {
      if (holdTimer) clearTimeout(holdTimer);
      cancelToolbarHide();
      window.removeEventListener("mousedown", blockFocusSteal, true);
      window.removeEventListener("contextmenu", handleContextMenu, true);
      window.removeEventListener("mousedown", handleRightDown, true);
      window.removeEventListener("click", closeMenuOnClick);
      window.removeEventListener("mousemove", handleOverlayMouseMove);
      window.removeEventListener("mouseout", handleOverlayMouseOut);
      window.removeEventListener("blur", handleOverlayBlur);
      hideFloater();
      removeTemplateDialog();
      unlisten.then((fn) => fn());
      unlistenMenuAction.then((fn) => fn());
      unlistenToolbarAction.then((fn) => fn());
      unlistenTabSwitch.then((fn) => fn());
      unlistenHover.then((fn) => fn());
      unlistenMoved.then((fn) => fn());
      unlistenResized.then((fn) => fn());
      unlistenResizedAdjust.then((fn) => fn());
      unlistenFloaterHidden.then((fn) => fn());
      unlistenFloaterShowing.then((fn) => fn());
      window.removeEventListener("quickinput:adjust-height", onAdjustEvt);
    };
  });
</script>

<svelte:head>
  <title>QuickInput</title>
</svelte:head>

<main
  class="quickinput-overlay"
  class:layout-horizontal={layout === "horizontal"}
  style="opacity: {overlayOpacityPct / 100}"
>
  <!-- 顶栏（控制按钮组 + 分组标签）已外置为悬浮窗上方外侧浮层，
       鼠标靠近右上角触发显示，见 $lib/floater.ts showToolbar -->
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
  {:else if defaultButtons.length === 0 && groups.length === 0}
    <div class="empty-state">
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" width="24" height="24">
        <path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"/>
        <path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z"/>
      </svg>
      <p>暂无快捷按钮</p>
      <p class="hint">编辑 default.toml 添加按钮</p>
    </div>
  {:else if !layoutReady}
    <div class="empty-state">
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" width="24" height="24">
        <circle cx="12" cy="12" r="10" stroke-dasharray="30 70" stroke-linecap="round">
          <animateTransform attributeName="transform" type="rotate" from="0 12 12" to="360 12 12" dur="1s" repeatCount="indefinite"/>
        </circle>
      </svg>
      <p>加载中…</p>
    </div>
  {:else}
    {#if lastError}
      <div class="error-banner">{lastError}</div>
    {/if}
    <div class="button-list">
      {#if visibleButtons.length === 0}
        <div class="empty-state">
          <p>暂无快捷按钮</p>
          <p class="hint">编辑 default.toml 添加按钮</p>
        </div>
      {:else}
        {#each visibleButtons as btn (btn.id)}
          <Tooltip text={btn.comment} opacityPct={overlayOpacityPct}>
            <button
              class="button-item"
              class:is-clicking={injectingId === btn.id}
              class:is-template={isTemplateBtn(btn)}
              data-id={btn.id}
              disabled={injectingId !== null && injectingId !== btn.id}
              onmousedown={(e) => onBtnDown(e, btn)}
              onmouseup={onBtnUp}
              onmouseleave={onBtnLeave}
            >
              <span class="button-label">{btn.label}</span>
              {#if btn.comment}
                <span class="button-comment">{btn.comment}</span>
              {/if}
            </button>
          </Tooltip>
        {/each}
      {/if}
    </div>

    <!-- 模板输入弹窗由原生 JS 动态创建（见 showTemplateDialog）；右键菜单已外置浮层窗口 -->
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
    transition: opacity 0.15s ease;
  }

  /* 右上角控制按钮条已外置为顶栏浮层（见 floater/+page.svelte），此处不再定义 */

  .button-list {
    /* 不拉伸：高度由按钮内容决定（配合高度自适应，窗口贴合内容实现四面 padding 等宽） */
    flex: 0 0 auto;
    overflow-y: auto;
    /* 四面等宽：统一 6px，避免上下/左右边距不一致 */
    padding: 6px;
    display: flex;
    flex-direction: column;
    gap: 2px;
    /* 竖排按钮靠上对齐、单列（默认 flex-start） */
  }

  /* 横向布局：按钮水平排列、自动换行、更紧凑 */
  .layout-horizontal .button-list {
    flex-direction: row;
    flex-wrap: wrap;
    align-items: stretch;
    align-content: flex-start;
    /* 横排按钮靠左对齐（工具栏习惯，窗口宽度由用户控制） */
    justify-content: flex-start;
    gap: 4px;
    padding: 6px;
    /* 不纵向拉伸：scrollHeight 反映真实内容行数，
       避免高度自适应目标随窗口高度虚高导致死循环 */
    flex: 0 0 auto;
    /* 横向单行/少行场景不出现滚动条；按钮溢出时由高度自适应承载，
       避免横排模式出现滚动条（竖排模式保留 overflow-y: auto） */
    overflow: hidden;
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

  /* 模板按钮标识：右侧小标记 */
  .button-item.is-template .button-label::after {
    content: "T";
    margin-left: 5px;
    font-size: 9px;
    color: #7aa2f7;
    border: 1px solid #7aa2f7;
    border-radius: 3px;
    padding: 0 3px;
    vertical-align: 1px;
  }

  /* 模板输入弹窗（原生 DOM 动态创建，同样须 :global） */
  :global(.template-dialog) {
    position: fixed;
    inset: 0;
    z-index: 1100;
    display: flex;
    align-items: center;
    justify-content: center;
    background: rgba(0, 0, 0, 0.35);
  }
  :global(.dialog-box) {
    width: 260px;
    background: rgba(40, 40, 44, 0.98);
    border: 1px solid rgba(255, 255, 255, 0.14);
    border-radius: 10px;
    padding: 12px;
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.5);
  }
  :global(.dialog-title) {
    font-size: 12px;
    color: #bbb;
    margin-bottom: 8px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  :global(.template-input) {
    width: 100%;
    box-sizing: border-box;
    padding: 7px 9px;
    background: rgba(0, 0, 0, 0.35);
    border: 1px solid rgba(255, 255, 255, 0.16);
    border-radius: 6px;
    color: #e0e0e0;
    font-size: 13px;
    outline: none;
  }
  :global(.template-input:focus) {
    border-color: #7aa2f7;
  }
  :global(.dialog-preview) {
    margin-top: 7px;
    font-size: 11px;
    color: #9ece6a;
    word-break: break-all;
  }
  :global(.dialog-preview.dim) {
    color: #888;
  }
  :global(.dialog-actions) {
    display: flex;
    justify-content: flex-end;
    gap: 8px;
    margin-top: 10px;
  }
  :global(.dialog-actions .btn-secondary),
  :global(.dialog-actions .btn-primary) {
    padding: 5px 14px;
    border-radius: 6px;
    border: 1px solid rgba(255, 255, 255, 0.16);
    background: none;
    color: #ccc;
    font-size: 12px;
    cursor: pointer;
  }
  :global(.dialog-actions .btn-secondary:hover) {
    background: rgba(255, 255, 255, 0.1);
  }
  :global(.dialog-actions .btn-primary) {
    border-color: #7aa2f7;
    color: #7aa2f7;
  }
  :global(.dialog-actions .btn-primary:hover) {
    background: rgba(122, 162, 247, 0.2);
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

  /* 分组 Tab 标签已外置为顶栏浮层（见 floater/+page.svelte），此处不再定义 */

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