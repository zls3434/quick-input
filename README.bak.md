# QuickInput

桌面快捷输入工具：一个常驻屏幕的置顶浮层，显示可自定义的按钮面板，点击按钮即把对应的命令或文本注入到当前焦点窗口。面向开发者高频命令（git、kubectl、docker 等）设计，也适用于任意重复输入场景。

## 功能特性

- **置顶浮层按钮面板**：全局热键 `Ctrl+Shift+Space` 显示/隐藏，窗口不抢焦点
- **一键文本注入**：点击按钮将文本注入当前焦点窗口，基于 `SendInput + KEYEVENTF_UNICODE`，支持中文等多字节 Unicode 字符
- **按进程切换按钮组**：不同应用（VS Code、Windows Terminal 等）自动显示各自的按钮配置
- **系统托盘**：显示/隐藏浮层、打开配置管理、退出应用
- **配置管理界面**：可视化增删改按钮、管理进程映射，保存后浮层实时刷新
- **配置导入导出**：TOML 格式，便于备份与分享
- **开机自启**：可选随系统启动
- **焦点保护**：点击浮层按钮不会让当前输入框失去焦点

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

### 浮层与热键

- 按 `Ctrl+Shift+Space` 显示或隐藏浮层
- 浮层默认置顶显示，点击按钮将文本注入当前焦点窗口
- 浮层不抢焦点：点击按钮不会导致当前输入框失去焦点，注入后焦点保持在原输入框

### 系统托盘

托盘图标右键菜单：

| 菜单项 | 作用 |
|--------|------|
| 显示/隐藏浮层 | 切换浮层可见性 |
| 配置管理 | 打开配置管理窗口 |
| 退出 | 退出应用（带确认对话框） |

### 按进程切换按钮组

当焦点切换到已配置的应用（如 VS Code）时，浮层自动显示该应用的专属按钮；切回未配置的应用时显示默认按钮组。

## 配置文件

配置存储在系统配置目录：

- Windows: `%APPDATA%\com.quickinput.desktop\default.toml`

首次启动自动生成默认配置，格式如下：

```toml
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

| 字段 | 说明 |
|------|------|
| `buttons` | 默认按钮组（未匹配任何进程时显示） |
| `buttons[].id` | 按钮唯一标识 |
| `buttons[].label` | 按钮显示名 |
| `buttons[].content` | 点击后注入的文本 |
| `buttons[].comment` | 按钮注释（浮层中以灰字显示） |
| `profiles` | 进程映射：`process_name` 匹配前台进程的 exe 名，`buttons` 为该进程的专属按钮 |

也可在配置管理界面可视化编辑，或通过导出/导入功能备份与迁移。

## 项目结构

```
├── src-tauri-app/               # Tauri 应用（前端 + 应用后端）
│   ├── src/                     # SvelteKit 前端
│   │   ├── routes/
│   │   │   ├── +page.svelte         # 浮层页面（按钮面板）
│   │   │   └── settings/+page.svelte# 配置管理页面
│   │   └── lib/Tooltip.svelte       # 悬浮注释组件
│   ├── src-tauri/              # Rust 后端
│   │   └── src/
│   │       ├── lib.rs              # 命令注册、配置管理、焦点监听
│   │       ├── inject_windows.rs   # Windows 文本注入（SendInput）
│   │       ├── focus_guard.rs      # 焦点保护（前台 + 键盘焦点恢复）
│   │       ├── focus_watcher.rs    # 前台进程变化监听
│   │       ├── focus_detector.rs   # 前台进程名识别
│   │       ├── global_shortcut.rs  # 全局热键
│   │       ├── tray.rs             # 系统托盘
│   │       └── window.rs           # 浮层窗口配置
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
