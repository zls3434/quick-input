# QuickInput

桌面快捷输入工具：一个常驻屏幕的置顶悬浮窗，显示可自定义的按钮面板，点击按钮即把对应的命令或文本注入到当前焦点窗口。面向开发者高频命令（git、kubectl、docker 等）设计，也适用于任意重复输入场景。

浮层支持竖向/横向两种布局、透明度与置顶调节，并可通过全局快捷键随时显示/隐藏；不同应用自动切换各自的按钮组，全程不抢焦点、不打断输入。

## 功能特性

- **置顶悬浮按钮面板**：常驻屏幕右上角，默认 85% 微透，点击按钮将文本注入当前焦点窗口
- **一键文本注入**：基于 `SendInput + KEYEVENTF_UNICODE`，支持中文等多字节 Unicode 字符
- **按进程切换按钮组**：不同应用（VS Code、Windows Terminal 等）自动显示各自的按钮配置
- **悬浮窗控制按钮**：右上角浮动工具条（隐藏 / 布局切换 / 透明度 / 置顶 / 移动）
- **横竖两种布局**：竖向（屏幕右侧）与横向（屏幕底部居中）一键切换，各自独立记忆位置与大小
- **透明度两档快捷切换**：85% ↔ 30% 互切，档位持久化
- **系统托盘**：显示/隐藏浮层、重置悬浮窗位置和大小、打开配置管理、退出应用
- **全局快捷键**：默认 `Ctrl+Shift+Space` 显示/隐藏浮层，可在配置管理中修改并检测与其他软件的热键冲突
- **配置管理界面**：可视化编辑按钮、管理进程映射，以及基础配置（开机自启动、默认置顶、默认布局、默认透明度、快捷键）
- **配置导入导出**：TOML 格式，便于备份与分享
- **开机自启**：可选随系统登录自动运行
- **焦点保护**：点击浮层按钮不会让当前输入框失去焦点
- **防误触设计**：悬浮窗不响应 Windows 屏幕快捷布局（Snap / 最大化），位置恢复自动钳制在屏幕内

## 快速开始

### 使用现成版本

- 绿色版：直接运行 `QuickInput.exe`
- 安装包：`QuickInput_0.1.0_x64-setup.exe`，安装后可从开始菜单启动

首次启动会在配置目录生成默认配置（见下文）。

### 从源码构建

环境要求：Node.js 18+、Rust stable 工具链、Windows 10/11（自带 WebView2 运行时）。

```bash
cd src-tauri-app
npm install
npm run tauri:dev    # 开发模式（热重载）
npm run tauri:build  # 生产构建（生成安装包）
```

## 使用说明

### 浮层控制

浮层右上角浮动控制条（左 → 右）：

| 按钮 | 作用 |
|------|------|
| 隐藏 | 隐藏浮层（托盘菜单或全局快捷键唤回） |
| 布局 | 切换竖向 / 横向布局（各自记忆位置与大小） |
| 透明度 | 85% ↔ 30% 两档互切 |
| 置顶 | 切换浮层是否始终置顶 |
| 移动 | 按住拖动浮层到任意位置 |

默认快捷键 `Ctrl+Shift+Space` 显示/隐藏浮层；热键可在「配置管理 → 悬浮窗 → 快捷键」中修改，保存时自动检测是否与其他软件冲突。

### 系统托盘

托盘图标右键菜单：

| 菜单项 | 作用 |
|--------|------|
| 显示/隐藏浮层 | 切换浮层可见性 |
| 重置悬浮窗位置和大小 | 清除记忆的几何，恢复默认位置与尺寸 |
| 配置管理 | 打开设置窗口 |
| 退出 | 退出应用（带确认对话框） |

### 配置管理

设置窗口（托盘菜单「配置管理」打开）包含三个页签：

- **默认按钮**：增删改全局按钮（未匹配任何进程时显示）
- **应用映射**：为指定进程（如 `Code.exe`）绑定专属按钮组
- **悬浮窗 / 基础配置**：开机自启动、默认置顶、默认透明度（不透明 / 85% / 70% / 45% / 30%）、默认布局、快捷键

### 按进程切换按钮组

当焦点切换到已配置的应用（如 VS Code）时，浮层自动显示该应用的专属按钮；切回未配置的应用时显示默认按钮组。

## 配置文件

配置存储在系统配置目录：

- Windows: `%APPDATA%\com.quickinput.desktop\default.toml`

首次启动自动生成默认配置，节结构如下：

```toml
[overlay]
layout = "vertical"          # vertical | horizontal
opacity = 85                 # 20~100，默认 85
always_on_top = true         # 默认置顶
vertical_x = 1580            # 竖向布局记忆位置/尺寸（逻辑坐标）
vertical_y = 64
vertical_w = 300
vertical_h = 400

[shortcuts]
show_overlay = "CTRL+SHIFT+SPACE"   # 显示/隐藏浮层快捷键

[[buttons]]
id = "git-status"
label = "Git Status"
content = "git status"
comment = "查看当前工作区状态"

[[profiles]]
process_name = "Code.exe"

[[profiles.buttons]]
id = "fmt"
label = "Format"
content = "cargo fmt"
comment = "格式化 Rust 代码"
```

字段说明：

| 节 / 字段 | 说明 |
|-----------|------|
| `[overlay]` | 悬浮窗设置：布局、透明度、置顶、各布局记忆的位置与尺寸 |
| `[shortcuts]` | 全局快捷键（`show_overlay` 为显示/隐藏浮层） |
| `[[buttons]]` | 默认按钮组（未匹配任何进程时显示） |
| `buttons[].id` | 按钮唯一标识 |
| `buttons[].label` | 按钮显示名 |
| `buttons[].content` | 点击后注入的文本 |
| `buttons[].comment` | 按钮注释（浮层中以灰字显示） |
| `[[profiles]]` | 进程映射：`process_name` 匹配前台进程的 exe 名，`buttons` 为该进程的专属按钮 |

也可在配置管理界面可视化编辑，或通过导出/导入功能备份与迁移。

## 项目结构

```
├── src-tauri-app/               # Tauri 应用（前端 + 应用后端）
│   ├── src/                     # SvelteKit 前端
│   │   ├── routes/
│   │   │   ├── +page.svelte         # 浮层页面（按钮面板 + 控制条）
│   │   │   └── settings/+page.svelte# 配置管理页面（按钮/映射/基础配置）
│   │   └── lib/Tooltip.svelte       # 悬浮注释组件
│   ├── src-tauri/              # Rust 后端
│   │   └── src/
│   │       ├── lib.rs              # 命令注册、配置管理、焦点监听、设置窗口
│   │       ├── inject_windows.rs   # Windows 文本注入（SendInput）
│   │       ├── focus_guard.rs      # 焦点保护（前台 + 键盘焦点恢复）
│   │       ├── focus_watcher.rs    # 前台进程变化监听（WinEvent 事件驱动）
│   │       ├── focus_detector.rs   # 前台进程名识别
│   │       ├── global_shortcut.rs  # 全局热键（配置驱动 + 冲突检测）
│   │       ├── tray.rs             # 系统托盘（含重置悬浮窗几何）
│   │       └── window.rs           # 浮层窗口几何/样式（布局、置顶、防 Snap）
│   └── scripts/postbuild.mjs       # 构建后 SvelteKit hash 对齐
├── src/backend/                # quickinput-config 配置库（独立 crate）
│   └── src/config/             # 配置模型、默认值、校验、管理器
└── docs/                       # 设计文档（产品概念、Epics、Stories、测试报告）
```

## 测试

```bash
# 配置库单元测试（模型校验、管理器、进程匹配等）
cd src/backend && cargo test

# Tauri 应用 Rust 单元测试
cd src-tauri-app/src-tauri && cargo test
```

冒烟测试与功能验收记录见 `docs/test/smoke-report.md`。
