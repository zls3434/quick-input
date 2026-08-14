# 测试报告

## 1. 文本注入冒烟测试（2026-08-13）

### 执行摘要

| 项目 | 值 |
|------|-----|
| 测试日期 | 2026-08-13 |
| 关键路径 | 按钮点击 → Tauri 命令 → SendInput 注入到焦点窗口 |
| 结果 | **SMOKE PASS** |

### 验证方法

由于无法直接驱动运行中的 WebView2 前端（Chrome DevTools 仅支持 Chrome），采用等价机制验证：

1. 打开记事本并聚焦（确认前台窗口标题为 `无标题 - Notepad`）
2. 使用与 `inject_windows.rs` 完全相同的 Win32 调用 `SendInput` + `KEYEVENTF_UNICODE`，注入标记文本 `QUICKINPUT_SMOKE_123`
3. 检查注入结果

### 测试结果

| 检查项 | 结果 |
|--------|------|
| `INPUT` 结构体大小（x64）| 40 字节（正确）|
| `SendInput` 返回 | 40 / 40（全部事件成功）|
| `GetLastError` | 0（无错误）|
| 目标窗口标题变化 | `*QUICKINPUT_SMOKE_123 - Notepad`（文字已落入记事本）|

### 结论

文本注入机制验证通过。`SendInput` + `KEYEVENTF_UNICODE` 能成功将 Unicode 文本注入到焦点窗口，记事本标题栏出现 `*` 前缀与标记文本，证明字符已真正输入。

应用侧 `inject_windows.rs` 使用 Rust `windows` crate 自动生成的完整 `INPUT` 结构体（x64 下 40 字节），与本次验证的机制一致，且 `inject_text` 命令在 SendInput 失败时会返回错误。结合 22 个单元测试全通过，判定注入功能可用。

## 2. 核心闭环验收（2026-08-14）

通过 Chrome DevTools Protocol（CDP）驱动浮层页面模拟点击按钮，以记事本为注入目标，验证"按钮点击 → Tauri 命令 → SendInput → 焦点窗口"完整链路。

| 验证项 | 结果 |
|--------|------|
| 按钮列表加载（Git Status / Git Commit / Get Pods） | PASS |
| 模拟点击按钮触发 `inject_text`（无错误提示） | PASS |
| `git status` 精确注入记事本 | PASS |
| `git commit -m ""`（含引号横杠） | PASS |
| `HELLO 123` / `a-b-c 123`（数字横杠） | PASS |
| `quote"test"`（双引号） | PASS |
| `Mixed Case 42!`（混合大小写） | PASS |
| `你好世界`（中文多字节 UTF-16） | PASS |
| 注入时浮层不抢焦点（前台保持记事本） | PASS |

结论：核心闭环完整可用，注入引擎对引号、横杠、数字、大小写、中文均稳定工作。测试初期出现的"丢字符/乱码"经排查为测试脚本读取时机过早（SendInput 事件入队后需少量时间被目标窗口处理），非应用缺陷；延长读取等待后全部精确匹配。

## 3. 功能验收（2026-08-14）

| 验收项 | 方法 | 结果 |
|--------|------|------|
| 全局热键注册 | 外部 `RegisterHotKey` 探测同组合键返回 1409（已被占用） | PASS |
| 热键按下触发 | 真实按键（人工验证） | PASS |
| 托盘图标 | 枚举窗口确认 `tray_icon_app` 存在 | PASS |
| 托盘菜单逻辑 | 代码审查（toggle/settings/quit 与命令绑定正确） | PASS |
| 进程切换自动换按钮组 | 添加 explorer.exe 专属按钮 → Win+D 切换前台 → 浮层按钮自动切换（双向） | PASS |
| 配置管理 CRUD | 增/删/改/查、重复 ID 报错、空 content 校验、删除不存在报错 | PASS |
| 配置导入导出 | 导出生成 TOML 文件（内容校验通过）→ 修改配置 → 导入后恢复导出状态 | PASS |

## 4. 焦点保持验证（2026-08-14）

针对"点击浮层按钮后原输入框失焦"问题，通过 `GetGUIThreadInfo` 检测前台窗口线程的 active/focus 状态。

| 检查项 | 点击前 | 点击后 |
|--------|--------|--------|
| 记事本线程 active | Notepad | Notepad |
| 记事本线程 focus | RichEditD2DPT | RichEditD2DPT |
| 前台窗口 | Notepad | Notepad |

真实鼠标点击浮层按钮后，前台窗口与键盘焦点均保持不变，注入正常。已在 VS Code 实测确认（人工验证）。

## 5. 验收中发现并修复的问题

| 问题 | 根因 | 修复 |
|------|------|------|
| 配置校验失败污染内存 | `add_button` 等命令先改内存再校验，失败时脏数据残留，导致后续 `delete_button` 等操作持续失败 | 改为"克隆-修改-校验-应用"模式，校验通过后才应用到实际配置 |
| 点击浮层按钮夺取键盘焦点 | WebView2 点击请求键盘焦点（键盘焦点与窗口激活分离），在 VS Code 等对焦点敏感的应用中导致原输入框失焦 | 前端拦截 `mousedown` 默认行为（拖动区域除外）；后端 FocusGuard 增强为同时记录并恢复前台窗口与键盘焦点窗口 |

## 6. 总体结论

截至 2026-08-14，QuickInput 全部核心功能与验收项通过：文本注入（含 Unicode）、全局热键、系统托盘、进程切换按钮组、配置管理 CRUD、配置导入导出、焦点保持。已知限制：全局热键的"按下触发"无法通过注入按键自动化模拟（Windows 系统级行为），已通过人工按键验证确认。
