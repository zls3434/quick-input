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
    buttons: ButtonConfig[];
  }

  let buttons = $state<ButtonConfig[]>([]);
  let profiles = $state<AppProfile[]>([]);
  let activeTab = $state<"buttons" | "profiles">("buttons");
  let error = $state<string | null>(null);

  // 编辑表单状态
  let editing = $state(false);
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
  let profButtonsText = $state("");
  let profSaveError = $state<string | null>(null);

  function startNewProfile() {
    profEditing = true;
    profEditingOriginal = "";
    profProcessName = "";
    profButtonsText = "";
    profSaveError = null;
  }

  function startEditProfile(p: AppProfile) {
    profEditing = true;
    profEditingOriginal = p.process_name;
    profProcessName = p.process_name;
    // 每行一个按钮：内容
    profButtonsText = p.buttons.map((b) => b.content).join("\n");
    profSaveError = null;
  }

  function cancelProfileEdit() {
    profEditing = false;
    profSaveError = null;
  }

  async function saveProfile() {
    profSaveError = null;
    const name = profProcessName.trim();
    if (!name) {
      profSaveError = "进程名不能为空";
      return;
    }
    // 把每行内容构造成按钮（id/label 用行内容，comment 为空）
    const btnList: ButtonConfig[] = profButtonsText
      .split("\n")
      .map((line) => line.trim())
      .filter((line) => line.length > 0)
      .map((line, i) => ({
        id: `${name}-${i}`,
        label: line,
        content: line,
        comment: null,
      }));

    try {
      if (profEditingOriginal === "") {
        await invoke("add_profile", { processName: name, buttons: btnList });
      } else {
        await invoke("update_profile", { processName: name, buttons: btnList });
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
    editId = "";
    editLabel = "";
    editContent = "";
    editComment = "";
    saveError = null;
    editing = true;
  }

  function startEdit(btn: ButtonConfig) {
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

  onMount(() => {
    loadButtons();
    loadProfiles();
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
    </div>

    {#if activeTab === "buttons"}
      <div class="toolbar">
        <span class="count">{buttons.length} 个按钮</span>
        <button class="btn-primary" onclick={startNew}>+ 新增</button>
      </div>

      {#if editing}
        <div class="edit-form">
          <h3>{editId === "" ? "新增按钮" : "编辑按钮"}</h3>
          {#if saveError}
            <div class="form-error">{saveError}</div>
          {/if}
          {#if editId === ""}
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
            <button class="btn-primary" onclick={editId === "" ? saveNew : saveEdit}>保存</button>
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
    {:else}
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
            进程名 <input bind:value={profProcessName} placeholder="如 Code.exe、WindowsTerminal.exe" />
          </label>
          <label>
            按钮内容（每行一个）
            <textarea bind:value={profButtonsText} rows="5" placeholder="git status&#10;git pull&#10;git commit -m "></textarea>
          </label>
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
              <span class="btn-label">{p.process_name}</span>
              <span class="btn-id">{p.buttons.length} 个按钮</span>
            </div>
            <div class="button-actions">
              <button class="btn-edit" onclick={() => startEditProfile(p)}>编辑</button>
              <button class="btn-delete" onclick={() => deleteProfile(p.process_name)}>删除</button>
            </div>
          </div>
        {/each}
      </div>
    {/if}

    <div class="share-toolbar">
      <button class="btn-secondary" onclick={exportConfig}>导出配置</button>
      <button class="btn-secondary" onclick={importConfig}>导入配置</button>
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
  .edit-form textarea {
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
    resize: vertical;
    font-family: monospace;
  }
  .edit-form textarea:focus {
    outline: none;
    border-color: #4a7cff;
  }
  .form-error {
    color: #e74c3c;
    font-size: 12px;
    margin-bottom: 8px;
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