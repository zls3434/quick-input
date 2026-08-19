<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";

  interface ButtonConfig {
    id: string;
    label: string;
    content: string;
    comment: string | null;
  }

  interface AppProfile {
    process_name: string;
    name: string | null;
    buttons: ButtonConfig[];
  }

  interface RunningProcess {
    process_name: string;
    window_title: string;
  }

  let buttons = $state<ButtonConfig[]>([]);
  let profiles = $state<AppProfile[]>([]);
  let activeTab = $state<"buttons" | "profiles" | "overlay">("buttons");
  let error = $state<string | null>(null);

  // 悬浮窗布局状态
  let overlayLayout = $state<"vertical" | "horizontal">("vertical");
  let layoutSaving = $state(false);
  // 基础配置：自启动 / 默认置顶 / 默认透明度
  let autostartEnabled = $state(false);
  let overlayTopmost = $state(true);
  let overlayOpacity = $state(85);
  let settingsSaving = $state(false);

  // 快捷键配置：显示/隐藏悬浮窗
  let showOverlayShortcut = $state("CTRL+SHIFT+SPACE");
  // 捕获模式：true 时监听按键组合
  let shortcutCapturing = $state(false);
  // 冲突检测结果提示
  let shortcutStatus = $state<{ kind: "ok" | "conflict" | "err"; text: string } | null>(null);

  // 透明度可选档位
  const OPACITY_OPTIONS = [100, 85, 70, 45, 30];

  async function loadShortcuts() {
    try {
      const s = await invoke<{ show_overlay: string | null }>("get_shortcuts");
      showOverlayShortcut = s.show_overlay?.trim() || "CTRL+SHIFT+SPACE";
    } catch (e) {
      console.error("读取快捷键配置失败", e);
    }
  }

  // 捕获键盘组合键：修饰键 + 一个按键
  function onShortcutKeydown(e: KeyboardEvent) {
    if (!shortcutCapturing) return;
    e.preventDefault();
    e.stopPropagation();
    const mods: string[] = [];
    if (e.ctrlKey) mods.push("CTRL");
    if (e.shiftKey) mods.push("SHIFT");
    if (e.altKey) mods.push("ALT");
    if (e.metaKey) mods.push("WIN");
    const key = e.code.startsWith("Key")
      ? e.code.slice(3)
      : e.code.startsWith("Digit")
        ? e.code.slice(5)
        : e.code.startsWith("Numpad")
          ? `NUMPAD${e.code.slice(6)}`
          : e.key.length === 1
            ? e.key.toUpperCase()
            : e.key.toUpperCase();
    // 仅修饰键按下时等待主键
    if (["CONTROL", "SHIFT", "ALT", "META", "WIN"].includes(key)) return;
    if (!e.ctrlKey && !e.shiftKey && !e.altKey && !e.metaKey) {
      shortcutStatus = { kind: "err", text: "请至少包含一个修饰键（Ctrl/Shift/Alt/Win）" };
      return;
    }
    const combo = [...mods, key].join("+");
    shortcutCapturing = false;
    void saveShortcut(combo);
  }

  async function saveShortcut(combo: string) {
    shortcutStatus = null;
    try {
      // 后端执行：格式校验 → 冲突检测 → 保存 → 重注册热键
      const saved = await invoke<string>("set_shortcut", { name: "show_overlay", value: combo });
      showOverlayShortcut = saved;
      shortcutStatus = { kind: "ok", text: "已保存，新快捷键即时生效" };
    } catch (e) {
      shortcutStatus = { kind: "conflict", text: `${e}` };
    }
  }

  async function checkShortcutAvailability(combo: string) {
    shortcutStatus = null;
    try {
      const available = await invoke<boolean>("check_shortcut_available", { shortcut: combo });
      shortcutStatus = available
        ? { kind: "ok", text: "该快捷键可用" }
        : { kind: "conflict", text: "该快捷键已被其他软件占用" };
    } catch (e) {
      shortcutStatus = { kind: "err", text: `${e}` };
    }
  }

  async function loadOverlayLayout() {
    try {
      const s = await invoke<{ layout: string; opacity: number | null; always_on_top: boolean | null }>(
        "get_overlay_settings"
      );
      overlayLayout = s.layout === "horizontal" ? "horizontal" : "vertical";
      overlayOpacity = s.opacity ?? 85;
      overlayTopmost = s.always_on_top ?? true;
    } catch (e) {
      error = `读取悬浮窗设置失败: ${e}`;
    }
  }

  async function loadAutostart() {
    try {
      autostartEnabled = await invoke<boolean>("is_autostart_enabled");
    } catch (e) {
      console.error("读取自启动状态失败", e);
    }
  }

  async function toggleAutostart() {
    if (settingsSaving) return;
    settingsSaving = true;
    error = null;
    const target = !autostartEnabled;
    try {
      await invoke("toggle_autostart", { enable: target });
      autostartEnabled = target;
    } catch (e) {
      error = `切换开机自启动失败: ${e}`;
    } finally {
      settingsSaving = false;
    }
  }

  async function toggleOverlayTopmost() {
    if (settingsSaving) return;
    settingsSaving = true;
    error = null;
    const target = !overlayTopmost;
    try {
      await invoke("set_overlay_always_on_top", { enabled: target });
      overlayTopmost = target;
    } catch (e) {
      error = `切换默认置顶失败: ${e}`;
    } finally {
      settingsSaving = false;
    }
  }

  async function setOverlayOpacity(v: number) {
    if (settingsSaving || overlayOpacity === v) return;
    settingsSaving = true;
    error = null;
    try {
      await invoke("set_overlay_opacity", { opacity: v });
      overlayOpacity = v;
    } catch (e) {
      error = `设置默认透明度失败: ${e}`;
    } finally {
      settingsSaving = false;
    }
  }

  async function switchLayout(target: "vertical" | "horizontal") {
    if (layoutSaving || overlayLayout === target) return;
    layoutSaving = true;
    error = null;
    try {
      await invoke("set_overlay_layout", { layout: target });
      overlayLayout = target;
    } catch (e) {
      error = `切换布局失败: ${e}`;
    } finally {
      layoutSaving = false;
    }
  }

  // 编辑表单状态
  // editMode 独立于 editId：新增时用户输入 ID 后 editId 非空，
  // 不能再用 editId 是否为空来判断模式（否则一输入就变成"编辑"）
  let editing = $state(false);
  let editMode = $state<"new" | "edit">("new");
  let editId = $state("");
  let editLabel = $state("");
  let editContent = $state("");
  let editComment = $state("");
  let saveError = $state<string | null>(null);

  async function loadButtons() {
    error = null;
    try {
      buttons = await invoke<ButtonConfig[]>("get_all_buttons");
    } catch (e) {
      error = `加载失败: ${e}`;
    }
  }

  async function loadProfiles() {
    error = null;
    try {
      profiles = await invoke<AppProfile[]>("get_profiles");
    } catch (e) {
      error = `加载应用映射失败: ${e}`;
    }
  }

  // ---- 应用画像表单状态 ----
  let profEditing = $state(false);
  let profEditingOriginal = $state("");
  let profProcessName = $state("");
  let profName = $state("");
  // 已选按钮（完整 ButtonConfig，保留原 id/label/content/comment）
  let profSelectedButtons = $state<ButtonConfig[]>([]);
  let profSaveError = $state<string | null>(null);
  // 运行进程列表（供绑定进程选择）
  let runningProcesses = $state<RunningProcess[]>([]);
  let processesLoading = $state(false);

  async function loadRunningProcesses() {
    processesLoading = true;
    try {
      runningProcesses = await invoke<RunningProcess[]>("list_window_processes");
    } catch (e) {
      runningProcesses = [];
    } finally {
      processesLoading = false;
    }
  }

  function onPickProcess(ev: Event) {
    const v = (ev.currentTarget as HTMLSelectElement).value;
    if (v) profProcessName = v;
  }

  // 候选按钮：合并默认按钮与所有映射的按钮，按 id 去重，排除已选
  const candidateButtons = $derived.by(() => {
    const all = [...buttons, ...profiles.flatMap((p) => p.buttons)];
    const seen = new Set<string>();
    const unique: ButtonConfig[] = [];
    for (const b of all) {
      if (!seen.has(b.id)) {
        seen.add(b.id);
        unique.push(b);
      }
    }
    const selectedIds = new Set(profSelectedButtons.map((b) => b.id));
    return unique.filter((b) => !selectedIds.has(b.id));
  });

  function startNewProfile() {
    profEditing = true;
    profEditingOriginal = "";
    profProcessName = "";
    profName = "";
    profSelectedButtons = [];
    profSaveError = null;
    loadRunningProcesses();
  }

  function startEditProfile(p: AppProfile) {
    profEditing = true;
    profEditingOriginal = p.process_name;
    profProcessName = p.process_name;
    profName = p.name ?? "";
    profSelectedButtons = p.buttons.map((b) => ({ ...b }));
    profSaveError = null;
    loadRunningProcesses();
  }

  function cancelProfileEdit() {
    profEditing = false;
    profSaveError = null;
  }

  function addToSelected(btn: ButtonConfig) {
    profSelectedButtons = [...profSelectedButtons, { ...btn }];
  }

  function removeFromSelected(index: number) {
    profSelectedButtons = profSelectedButtons.filter((_, i) => i !== index);
  }

  async function saveProfile() {
    profSaveError = null;
    const name = profProcessName.trim();
    if (!name) {
      profSaveError = "进程名不能为空";
      return;
    }
    const btnList = profSelectedButtons.map((b) => ({ ...b }));
    const displayName = profName.trim() || null;

    try {
      if (profEditingOriginal === "") {
        await invoke("add_profile", { processName: name, buttons: btnList, name: displayName });
      } else {
        await invoke("update_profile", { processName: name, buttons: btnList, name: displayName });
      }
      profEditing = false;
      await loadProfiles();
    } catch (e) {
      profSaveError = `${e}`;
    }
  }

  async function deleteProfile(processName: string) {
    if (!confirm(`确定删除应用映射 "${processName}" 吗？`)) return;
    try {
      await invoke("delete_profile", { processName });
      await loadProfiles();
    } catch (e) {
      error = `删除映射失败: ${e}`;
    }
  }

  function startNew() {
    editMode = "new";
    editId = "";
    editLabel = "";
    editContent = "";
    editComment = "";
    saveError = null;
    editing = true;
  }

  function startEdit(btn: ButtonConfig) {
    editMode = "edit";
    editId = btn.id;
    editLabel = btn.label;
    editContent = btn.content;
    editComment = btn.comment ?? "";
    saveError = null;
    editing = true;
  }

  function cancelEdit() {
    editing = false;
    saveError = null;
  }

  async function saveNew() {
    saveError = null;
    if (!editId.trim() || !editLabel.trim() || !editContent.trim()) {
      saveError = "ID、标签、内容不能为空";
      return;
    }
    try {
      await invoke("add_button", {
        id: editId.trim(),
        label: editLabel.trim(),
        content: editContent.trim(),
        comment: editComment.trim() || null,
      });
      editing = false;
      await loadButtons();
    } catch (e) {
      saveError = `${e}`;
    }
  }

  async function saveEdit() {
    saveError = null;
    if (!editLabel.trim() || !editContent.trim()) {
      saveError = "标签、内容不能为空";
      return;
    }
    try {
      await invoke("update_button", {
        id: editId,
        label: editLabel.trim(),
        content: editContent.trim(),
        comment: editComment.trim() || null,
      });
      editing = false;
      await loadButtons();
    } catch (e) {
      saveError = `${e}`;
    }
  }

  async function deleteBtn(id: string) {
    if (!confirm(`确定删除按钮 "${id}" 吗？`)) return;
    try {
      await invoke("delete_button", { id });
      await loadButtons();
    } catch (e) {
      error = `删除失败: ${e}`;
    }
  }

  function closeWindow() {
    const win = getCurrentWebviewWindow();
    win.hide();
  }

  // 导出配置到 TOML 文件
  async function exportConfig() {
    try {
      await invoke("export_config");
      error = null;
    } catch (e) {
      error = `导出失败: ${e}`;
    }
  }

  // 从 TOML 文件导入配置
  async function importConfig() {
    try {
      await invoke("import_config");
      await loadButtons();
      await loadProfiles();
      error = null;
    } catch (e) {
      error = `导入失败: ${e}`;
    }
  }

  // 恢复默认按钮与进程映射（保留悬浮窗/快捷键设置）
  async function resetConfigToDefault() {
    if (!confirm("将恢复默认按钮与进程映射，替换当前所有自定义按钮（悬浮窗与快捷键设置保留）。确定继续？")) return;
    try {
      await invoke("reset_config_to_default");
      await loadButtons();
      await loadProfiles();
      error = null;
    } catch (e) {
      error = `恢复默认配置失败: ${e}`;
    }
  }

  onMount(() => {
    loadButtons();
    loadProfiles();
    loadOverlayLayout();
    loadAutostart();
    loadShortcuts();
  });
</script>

<svelte:head>
  <title>QuickInput 设置</title>
</svelte:head>

<main class="settings-window">
  <header class="drag-region" data-tauri-drag-region>
    <span class="title">QuickInput 设置</span>
    <button class="close-btn" onclick={closeWindow} aria-label="关闭">✕</button>
  </header>

  <div class="content">
    {#if error}
      <div class="error-banner">{error}</div>
    {/if}

    <div class="tab-bar">
      <button
        class="tab"
        class:active={activeTab === "buttons"}
        onclick={() => (activeTab = "buttons")}
      >
        默认按钮
      </button>
      <button
        class="tab"
        class:active={activeTab === "profiles"}
        onclick={() => (activeTab = "profiles")}
      >
        应用映射
      </button>
      <button
        class="tab"
        class:active={activeTab === "overlay"}
        onclick={() => (activeTab = "overlay")}
      >
        悬浮窗
      </button>
    </div>

    {#if activeTab === "buttons"}
      <div class="toolbar">
        <span class="count">{buttons.length} 个按钮</span>
        <button class="btn-primary" onclick={startNew}>+ 新增</button>
      </div>

      {#if editing}
        <div class="edit-form">
          <h3>{editMode === "new" ? "新增按钮" : "编辑按钮"}</h3>
          {#if saveError}
            <div class="form-error">{saveError}</div>
          {/if}
          {#if editMode === "new"}
            <label>
              ID <input bind:value={editId} placeholder="唯一标识，如 git-status" />
            </label>
          {/if}
          <label>
            标签 <input bind:value={editLabel} placeholder="显示名称，如 Git Status" />
          </label>
          <label>
            内容 <input bind:value={editContent} placeholder="注入文本/命令，如 git status" />
          </label>
          <label>
            注释 <input bind:value={editComment} placeholder="悬浮注释说明（可选）" />
          </label>
          <div class="form-actions">
            <button class="btn-cancel" onclick={cancelEdit}>取消</button>
            <button class="btn-primary" onclick={editMode === "new" ? saveNew : saveEdit}>保存</button>
          </div>
        </div>
      {/if}

      <div class="button-list">
        {#each buttons as btn (btn.id)}
          <div class="button-row">
            <div class="button-info">
              <span class="btn-label">{btn.label}</span>
              <span class="btn-id">{btn.id}</span>
              <span class="btn-content">{btn.content}</span>
              {#if btn.comment}
                <span class="btn-comment">{btn.comment}</span>
              {/if}
            </div>
            <div class="button-actions">
              <button class="btn-edit" onclick={() => startEdit(btn)}>编辑</button>
              <button class="btn-delete" onclick={() => deleteBtn(btn.id)}>删除</button>
            </div>
          </div>
        {/each}
      </div>
    {:else if activeTab === "profiles"}
      <div class="toolbar">
        <span class="count">{profiles.length} 个应用映射</span>
        <button class="btn-primary" onclick={startNewProfile}>+ 新增映射</button>
      </div>

      {#if profEditing}
        <div class="edit-form">
          <h3>{profEditingOriginal === "" ? "新增应用映射" : "编辑应用映射"}</h3>
          {#if profSaveError}
            <div class="form-error">{profSaveError}</div>
          {/if}
          <label>
            映射名称（可选） <input bind:value={profName} placeholder="自定义显示名，如 我的浏览器；留空显示进程名" />
          </label>
          <label>
            绑定进程 <input bind:value={profProcessName} placeholder="可手动输入，如 Code.exe" />
          </label>
          <label>
            从运行窗口选择
            <div class="process-picker">
              <select onchange={onPickProcess} class="process-select">
                <option value="">{processesLoading ? "加载中..." : `选择进程（共 ${runningProcesses.length} 个）`}</option>
                {#each runningProcesses as p (p.process_name)}
                  <option value={p.process_name}>{p.process_name} — {p.window_title.slice(0, 40)}</option>
                {/each}
              </select>
              <button class="btn-secondary" onclick={loadRunningProcesses}>刷新</button>
            </div>
          </label>

          <div class="picker-section">
            <div class="section-title">已选按钮（{profSelectedButtons.length}）</div>
            {#if profSelectedButtons.length === 0}
              <div class="picker-empty">尚未选择按钮</div>
            {:else}
              <div class="picker-list">
                {#each profSelectedButtons as b, i (b.id)}
                  <div class="picker-row">
                    <div class="picker-info">
                      <span class="p-label">{b.label}</span>
                      <span class="p-content">{b.content}</span>
                    </div>
                    <button class="btn-delete" onclick={() => removeFromSelected(i)}>移除</button>
                  </div>
                {/each}
              </div>
            {/if}
          </div>

          <div class="picker-section">
            <div class="section-title">从现有按钮添加</div>
            {#if candidateButtons.length === 0}
              <div class="picker-empty">没有可选按钮，请先在"默认按钮"或其他映射中创建</div>
            {:else}
              <div class="picker-list">
                {#each candidateButtons as b (b.id)}
                  <div class="picker-row">
                    <div class="picker-info">
                      <span class="p-label">{b.label}</span>
                      <span class="p-content">{b.content}</span>
                    </div>
                    <button class="btn-edit" onclick={() => addToSelected(b)}>添加</button>
                  </div>
                {/each}
              </div>
            {/if}
          </div>
          <div class="form-actions">
            <button class="btn-cancel" onclick={cancelProfileEdit}>取消</button>
            <button class="btn-primary" onclick={saveProfile}>保存</button>
          </div>
        </div>
      {/if}

      <div class="button-list">
        {#each profiles as p (p.process_name)}
          <div class="button-row">
            <div class="button-info">
              <span class="btn-label">{p.name || p.process_name}</span>
              <span class="btn-id">{p.name ? p.process_name : ""} {p.buttons.length} 个按钮</span>
            </div>
            <div class="button-actions">
              <button class="btn-edit" onclick={() => startEditProfile(p)}>编辑</button>
              <button class="btn-delete" onclick={() => deleteProfile(p.process_name)}>删除</button>
            </div>
          </div>
        {/each}
      </div>
    {:else if activeTab === "overlay"}
      <div class="overlay-settings">
        <h3>基础配置</h3>

        <div class="cfg-row">
          <div class="cfg-info">
            <div class="cfg-name">开机自启动</div>
            <div class="cfg-desc">系统登录后自动运行 QuickInput</div>
          </div>
          <button
            class="toggle"
            class:on={autostartEnabled}
            disabled={settingsSaving}
            onclick={toggleAutostart}
            aria-label="开机自启动"
          >
            <span class="toggle-knob"></span>
          </button>
        </div>

        <div class="cfg-row">
          <div class="cfg-info">
            <div class="cfg-name">默认置顶</div>
            <div class="cfg-desc">悬浮窗始终显示在其他窗口之上</div>
          </div>
          <button
            class="toggle"
            class:on={overlayTopmost}
            disabled={settingsSaving}
            onclick={toggleOverlayTopmost}
            aria-label="默认置顶"
          >
            <span class="toggle-knob"></span>
          </button>
        </div>

        <div class="cfg-block">
          <div class="cfg-name">默认透明度</div>
          <div class="cfg-desc">新窗口启动时的透明度档位（悬浮窗按钮可随时切换 85% ↔ 30%）</div>
          <div class="opacity-options">
            {#each OPACITY_OPTIONS as v (v)}
              <button
                class="opacity-option"
                class:active={overlayOpacity === v}
                disabled={settingsSaving}
                onclick={() => setOverlayOpacity(v)}
              >
                {v === 100 ? "不透明" : `${v}%`}
              </button>
            {/each}
          </div>
        </div>

        <div class="cfg-block">
          <div class="cfg-name">快捷键</div>
          <div class="cfg-desc">全局快捷键，按下即可显示/隐藏悬浮窗</div>
          <div class="shortcut-row">
            <span class="shortcut-key">{shortcutCapturing ? "请按下新的组合键…" : showOverlayShortcut}</span>
            {#if shortcutCapturing}
              <button
                class="btn-cancel"
                onclick={() => {
                  shortcutStatus = null;
                  shortcutCapturing = false;
                }}
              >
                取消
              </button>
            {:else}
              <button
                class="btn-secondary"
                disabled={settingsSaving}
                onclick={() => {
                  shortcutStatus = null;
                  shortcutCapturing = true;
                  // 捕获区获得焦点后才能接收 keydown
                  setTimeout(() => {
                    document.querySelector<HTMLElement>(".shortcut-capture")?.focus();
                  }, 0);
                }}
              >
                修改
              </button>
            {/if}
          </div>
          {#if shortcutCapturing}
            <div class="shortcut-capture" onkeydown={onShortcutKeydown} tabindex="0">
              按下新的组合键（如 Ctrl+Alt+F2）。仅按修饰键会继续等待主键。
            </div>
          {:else if shortcutStatus}
            <div
              class="shortcut-status"
              class:ok={shortcutStatus.kind === "ok"}
              class:conflict={shortcutStatus.kind === "conflict"}
            >
              {shortcutStatus.text}
            </div>
          {/if}
        </div>

        <div class="cfg-block">
          <div class="cfg-name">默认布局</div>
          <div class="cfg-desc">新窗口启动时的排列方式，切换后立即生效</div>
          <div class="layout-options">
            <button
              class="layout-option"
              class:active={overlayLayout === "vertical"}
              disabled={layoutSaving}
              onclick={() => switchLayout("vertical")}
            >
              <div class="layout-preview layout-preview-v">
                <span></span><span></span><span></span>
              </div>
              <div class="layout-option-text">
                <div class="layout-name">竖向排列</div>
                <div class="layout-desc">默认位于屏幕右上方</div>
              </div>
            </button>
            <button
              class="layout-option"
              class:active={overlayLayout === "horizontal"}
              disabled={layoutSaving}
              onclick={() => switchLayout("horizontal")}
            >
              <div class="layout-preview layout-preview-h">
                <span></span><span></span><span></span>
              </div>
              <div class="layout-option-text">
                <div class="layout-name">横向排列</div>
                <div class="layout-desc">默认位于屏幕下方任务栏上方居中</div>
              </div>
            </button>
          </div>
        </div>

        <p class="layout-hint">
          拖动悬浮窗右上角移动按钮可移动位置，位置会被记忆，下次启动悬浮窗将停留在
          上次的位置；两种布局各自独立记忆。基础配置保存后立即生效并持久化。
        </p>
      </div>
    {/if}

    <div class="share-toolbar">
      <button class="btn-secondary" onclick={exportConfig}>导出配置</button>
      <button class="btn-secondary" onclick={importConfig}>导入配置</button>
      <button class="btn-secondary btn-warn" onclick={resetConfigToDefault}>恢复默认配置</button>
    </div>
  </div>
</main>

<style>
  :global(html), :global(body) {
    height: 100%;
    margin: 0;
    overflow: hidden;
    background: transparent !important;
    font-family: system-ui, -apple-system, sans-serif;
    font-size: 13px;
    color: #e0e0e0;
  }

  .settings-window {
    display: flex;
    flex-direction: column;
    height: 100vh;
    background: rgba(32, 32, 35, 0.97);
    border-radius: 8px;
    overflow: hidden;
  }

  .drag-region {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 8px 12px;
    background: rgba(24, 24, 27, 0.95);
    border-bottom: 1px solid rgba(255, 255, 255, 0.08);
    -webkit-app-region: drag;
    user-select: none;
    flex-shrink: 0;
  }

  .title {
    font-size: 13px;
    font-weight: 600;
    color: #ccc;
  }

  .close-btn {
    -webkit-app-region: no-drag;
    background: none;
    border: none;
    color: #888;
    cursor: pointer;
    font-size: 14px;
    padding: 2px 6px;
    border-radius: 4px;
  }
  .close-btn:hover { background: rgba(255,255,255,0.1); color: #fff; }

  .content {
    flex: 1;
    overflow-y: auto;
    padding: 12px;
  }

  .error-banner {
    padding: 8px 12px;
    background: rgba(231, 76, 60, 0.15);
    color: #e74c3c;
    border-radius: 4px;
    margin-bottom: 8px;
  }

  .toolbar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 12px;
  }

  .tab-bar {
    display: flex;
    gap: 0;
    margin-bottom: 12px;
    border-bottom: 1px solid rgba(255,255,255,0.08);
  }
  .tab {
    background: none;
    border: none;
    border-bottom: 2px solid transparent;
    color: #888;
    cursor: pointer;
    font-size: 12px;
    padding: 6px 14px;
    transition: all 0.15s;
  }
  .tab:hover { color: #ccc; }
  .tab.active {
    color: #4a7cff;
    border-bottom-color: #4a7cff;
  }

  .count { color: #888; font-size: 12px; }

  .btn-primary {
    background: #4a7cff;
    color: #fff;
    border: none;
    border-radius: 4px;
    padding: 6px 14px;
    cursor: pointer;
    font-size: 12px;
  }
  .btn-primary:hover { background: #3a6aee; }

  .btn-cancel {
    background: rgba(255,255,255,0.08);
    color: #ccc;
    border: none;
    border-radius: 4px;
    padding: 6px 14px;
    cursor: pointer;
    font-size: 12px;
  }
  .btn-cancel:hover { background: rgba(255,255,255,0.15); }

  .share-toolbar {
    display: flex;
    gap: 8px;
    margin-top: 14px;
    padding-top: 10px;
    border-top: 1px solid rgba(255,255,255,0.08);
  }

  .overlay-settings h3 {
    margin: 4px 0 12px 0;
    font-size: 13px;
    font-weight: 600;
  }
  .cfg-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
    padding: 10px 0;
    border-bottom: 1px solid rgba(255,255,255,0.06);
  }
  .cfg-info { min-width: 0; }
  .cfg-name {
    font-size: 13px;
    font-weight: 600;
    color: #e0e0e0;
  }
  .cfg-desc {
    font-size: 11px;
    color: #888;
    margin-top: 2px;
  }
  .cfg-block {
    padding: 12px 0;
    border-bottom: 1px solid rgba(255,255,255,0.06);
  }
  .cfg-block:last-of-type { border-bottom: none; }

  .toggle {
    position: relative;
    width: 40px;
    height: 22px;
    border-radius: 11px;
    border: 1px solid rgba(255,255,255,0.12);
    background: rgba(255,255,255,0.08);
    cursor: pointer;
    flex-shrink: 0;
    transition: background 0.15s, border-color 0.15s;
    padding: 0;
  }
  .toggle-knob {
    position: absolute;
    top: 2px;
    left: 2px;
    width: 16px;
    height: 16px;
    border-radius: 50%;
    background: #aaa;
    transition: left 0.15s, background 0.15s;
  }
  .toggle.on {
    background: rgba(74,124,255,0.35);
    border-color: #4a7cff;
  }
  .toggle.on .toggle-knob {
    left: 20px;
    background: #4a7cff;
  }
  .toggle:disabled { opacity: 0.5; cursor: default; }

  .opacity-options {
    display: flex;
    gap: 6px;
    margin-top: 8px;
    flex-wrap: wrap;
  }
  .opacity-option {
    padding: 5px 14px;
    background: rgba(255,255,255,0.04);
    border: 1px solid rgba(255,255,255,0.1);
    border-radius: 4px;
    color: #aaa;
    cursor: pointer;
    font-size: 12px;
    transition: all 0.12s;
  }
  .opacity-option:hover { background: rgba(255,255,255,0.08); color: #ccc; }
  .opacity-option.active {
    border-color: #4a7cff;
    background: rgba(74,124,255,0.15);
    color: #7ca5ff;
  }
  .opacity-option:disabled { opacity: 0.5; cursor: default; }

  .shortcut-row {
    display: flex;
    align-items: center;
    gap: 8px;
    margin-top: 8px;
  }
  .shortcut-key {
    flex: 1;
    padding: 6px 10px;
    background: rgba(0,0,0,0.3);
    border: 1px solid rgba(255,255,255,0.12);
    border-radius: 4px;
    font-family: Consolas, monospace;
    font-size: 13px;
    color: #7ca5ff;
    letter-spacing: 0.5px;
    user-select: none;
  }
  .shortcut-capture {
    margin-top: 8px;
    padding: 8px 10px;
    background: rgba(74,124,255,0.1);
    border: 1px dashed #4a7cff;
    border-radius: 4px;
    font-size: 12px;
    color: #9db9ff;
    outline: none;
  }
  .shortcut-status {
    margin-top: 8px;
    font-size: 12px;
    padding: 6px 10px;
    border-radius: 4px;
  }
  .shortcut-status.ok {
    color: #4caf50;
    background: rgba(76,175,80,0.1);
  }
  .shortcut-status.conflict {
    color: #e74c3c;
    background: rgba(231,76,60,0.1);
  }
  .layout-options {
    display: flex;
    gap: 10px;
  }
  .layout-option {
    flex: 1;
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 12px;
    background: rgba(255,255,255,0.03);
    border: 1px solid rgba(255,255,255,0.08);
    border-radius: 6px;
    cursor: pointer;
    color: #ccc;
    text-align: left;
    transition: all 0.12s;
  }
  .layout-option:hover { background: rgba(255,255,255,0.06); }
  .layout-option.active {
    border-color: #4a7cff;
    background: rgba(74,124,255,0.12);
  }
  .layout-option:disabled { opacity: 0.6; cursor: default; }
  .layout-preview {
    display: flex;
    gap: 3px;
    background: rgba(0,0,0,0.35);
    border-radius: 4px;
    padding: 6px;
    flex-shrink: 0;
  }
  .layout-preview span {
    background: rgba(255,255,255,0.35);
    border-radius: 2px;
    display: block;
  }
  .layout-preview-v {
    flex-direction: column;
    width: 28px;
    height: 44px;
  }
  .layout-preview-v span { height: 8px; width: 100%; }
  .layout-preview-h {
    flex-direction: row;
    width: 44px;
    height: 28px;
  }
  .layout-preview-h span { width: 8px; height: 100%; }
  .layout-option-text { min-width: 0; }
  .layout-name {
    font-size: 13px;
    font-weight: 600;
    color: #e0e0e0;
  }
  .layout-desc {
    font-size: 11px;
    color: #888;
    margin-top: 3px;
  }
  .layout-hint {
    font-size: 11px;
    color: #777;
    line-height: 1.6;
    margin: 14px 0 0 0;
  }
  .btn-secondary {
    background: rgba(255,255,255,0.08);
    color: #ccc;
    border: none;
    border-radius: 4px;
    padding: 6px 14px;
    cursor: pointer;
    font-size: 12px;
  }
  .btn-secondary:hover { background: rgba(255,255,255,0.15); }
  .btn-warn { color: #e8a33d; }
  .btn-warn:hover { background: rgba(232,163,61,0.15); }

  .edit-form {
    background: rgba(255,255,255,0.05);
    border-radius: 6px;
    padding: 12px;
    margin-bottom: 12px;
  }
  .edit-form h3 {
    margin: 0 0 10px 0;
    font-size: 13px;
    font-weight: 600;
  }
  .edit-form label {
    display: block;
    margin-bottom: 8px;
    font-size: 12px;
    color: #aaa;
  }
  .edit-form input {
    display: block;
    width: 100%;
    margin-top: 3px;
    padding: 6px 8px;
    background: rgba(0,0,0,0.3);
    border: 1px solid rgba(255,255,255,0.1);
    border-radius: 4px;
    color: #e0e0e0;
    font-size: 12px;
    box-sizing: border-box;
  }
  .edit-form input:focus {
    outline: none;
    border-color: #4a7cff;
  }
  .form-error {
    color: #e74c3c;
    font-size: 12px;
    margin-bottom: 8px;
  }

  .picker-section {
    margin-bottom: 10px;
  }

  .process-picker {
    display: flex;
    gap: 6px;
    margin-top: 3px;
  }
  .process-select {
    flex: 1;
    padding: 6px 8px;
    background: rgba(0,0,0,0.3);
    border: 1px solid rgba(255,255,255,0.1);
    border-radius: 4px;
    color: #e0e0e0;
    font-size: 12px;
    box-sizing: border-box;
  }
  .process-select:focus {
    outline: none;
    border-color: #4a7cff;
  }
  .section-title {
    font-size: 12px;
    color: #aaa;
    margin-bottom: 6px;
  }
  .picker-list {
    display: flex;
    flex-direction: column;
    gap: 4px;
    max-height: 160px;
    overflow-y: auto;
  }
  .picker-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 8px;
    padding: 5px 8px;
    background: rgba(255,255,255,0.03);
    border-radius: 4px;
    border: 1px solid rgba(255,255,255,0.06);
  }
  .picker-row:hover { background: rgba(255,255,255,0.06); }
  .picker-info {
    display: flex;
    flex-direction: column;
    gap: 1px;
    min-width: 0;
    flex: 1;
  }
  .p-label {
    font-size: 12px;
    font-weight: 600;
  }
  .p-content {
    font-size: 11px;
    color: #999;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .picker-empty {
    font-size: 11px;
    color: #666;
    padding: 6px 0;
  }
  .form-actions {
    display: flex;
    gap: 8px;
    justify-content: flex-end;
    margin-top: 10px;
  }

  .button-list {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .button-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 8px 10px;
    background: rgba(255,255,255,0.03);
    border-radius: 5px;
    border: 1px solid rgba(255,255,255,0.06);
  }
  .button-row:hover {
    background: rgba(255,255,255,0.06);
  }

  .button-info {
    display: flex;
    flex-direction: column;
    gap: 2px;
    min-width: 0;
    flex: 1;
  }
  .btn-label {
    font-weight: 600;
    font-size: 13px;
  }
  .btn-id {
    font-size: 10px;
    color: #666;
    font-family: monospace;
  }
  .btn-content {
    font-size: 11px;
    color: #999;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .btn-comment {
    font-size: 11px;
    color: #6a9;
    font-style: italic;
  }

  .button-actions {
    display: flex;
    gap: 4px;
    flex-shrink: 0;
    margin-left: 8px;
  }

  .btn-edit {
    background: rgba(74,124,255,0.15);
    color: #7ca5ff;
    border: none;
    border-radius: 3px;
    padding: 4px 10px;
    cursor: pointer;
    font-size: 11px;
  }
  .btn-edit:hover { background: rgba(74,124,255,0.25); }

  .btn-delete {
    background: rgba(231,76,60,0.15);
    color: #e74c3c;
    border: none;
    border-radius: 3px;
    padding: 4px 10px;
    cursor: pointer;
    font-size: 11px;
  }
  .btn-delete:hover { background: rgba(231,76,60,0.25); }
</style>