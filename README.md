# QuickInput

桌面快捷输入工具：一个常驻屏幕的置顶悬浮窗，显示可自定义的按钮面板，点击按钮即把对应的命令或文本注入到当前焦点窗口。面向开发者高频命令（git、kubectl、docker 等）设计，也适用于客服话术、日常回复等任意重复输入场景。

支持竖向/横向两种布局、透明度与置顶调节、窗口边缘吸附跟随，并可通过全局快捷键随时显示/隐藏；不同应用自动切换各自的按钮组，全程不抢焦点、不打断输入。内置模板按钮与动态占位符，长按自动回车，老游戏等不响应粘贴的窗口可切换按键模拟注入。

## 功能特性

- **置顶悬浮按钮面板**：常驻屏幕，默认 85% 微透，点击按钮将文本注入当前焦点窗口
- **两种注入模式**：`paste`（剪贴板粘贴，默认，支持中文等多字节 Unicode）与 `keystroke`（扫描码按键模拟，兼容 DirectInput 老游戏与自绘输入框）
- **模板按钮**：`content` 含 `{input}` 即为模板——左键注入后光标自动停在占位符处（如 `git commit -m "{input}"` 光标落在引号中间），右键弹出模板输入框填写后合并注入
- **动态占位符**：`{date}`、`{time}`、`{clipboard}` 注入瞬间由后端展开，无需弹窗
- **长按回车**：按住按钮超过阈值（200~5000ms 可调）输出内容后自动回车，输入即执行
- **按进程切换按钮组**：不同应用（VS Code、Windows Terminal 等）自动显示各自的按钮配置；支持默认映射兜底
- **分组 Tab**：画像可定义多个分组，悬浮窗顶部显示 Tab 标签一键切换
- **窗口吸附跟随**：拖动悬浮窗靠近目标窗口边缘自动吸附，并跟随目标窗口移动/缩放
- **横竖两种布局**：竖向（屏幕右侧）与横向（屏幕底部居中）一键切换，各自独立记忆位置与大小；竖排高度可拖动调整并跨重启记忆
- **透明度两档快捷切换**：85% ↔ 30% 互切，档位持久化
- **悬浮窗控制按钮**：右上角浮动工具条（隐藏 / 布局切换 / 透明度 / 置顶 / 移动）
- **系统托盘**：显示/隐藏浮层、重置悬浮窗位置和大小、打开配置管理、退出应用
- **全局快捷键**：默认 `Ctrl+Shift+Space` 显示/隐藏浮层，可修改并自动检测与其他软件的热键冲突
- **配置管理界面**：四个页签（通用设置 / 默认按钮 / 应用映射 / 关于），可视化编辑按钮与映射，配置导入导出（TOML）
- **自动更新**：关于页检查更新、一键更新，可开启启动时自动检查；更新通道内置国内镜像加速
- **开机自启**：可选随系统登录自动运行
- **焦点保护**：点击浮层按钮不会让当前输入框失去焦点
- **防误触设计**：悬浮窗不响应 Windows 屏幕快捷布局（Snap / 最大化），位置恢复自动钳制在屏幕内

## 下载安装

从 [GitHub Releases](https://github.com/zls3434/quick-input/releases/latest) 获取：

| 资产 | 说明 |
|------|------|
| `QuickInput_<版本>_x64-setup.exe` | NSIS 安装包（推荐，支持应用内自动更新） |
| `QuickInput_<版本>_x64_portable.exe` | 单文件绿色版，下载即用，配置仍读用户目录 |

> 国内直连 GitHub 若超时，可在下载链接前加镜像前缀（如 `https://ghfast.top/`）；应用内自动更新已内置镜像通道，无需手动处理。

首次启动会在配置目录生成默认配置（见下文）。

## 快速开始

### 使用现成版本

- 绿色版：直接运行 `QuickInput_<版本>_x64_portable.exe`
- 安装包：运行 `QuickInput_<版本>_x64-setup.exe`，安装后可从开始菜单启动

### 从源码构建

环境要求：Node.js 18+、Rust stable 工具链、Windows 10/11（自带 WebView2 运行时）。

```bash
cd src-tauri-app
npm install
npm run tauri:dev    # 开发模式（热重载）
npm run tauri:build  # 生产构建（生成安装包 + 绿色版）
```

构建完成后 `scripts/copy-exe.mjs` 自动把单文件 exe 复制到项目根目录 `QuickInput.exe` 并重启，便于本地实测。

## 使用说明

### 浮层控制

浮层右上角浮动控制条（左 → 右）：

| 按钮 | 作用 |
|------|------|
| 隐藏 | 隐藏浮层（托盘菜单或全局快捷键唤回） |
| 布局 | 切换竖向 / 横向布局（各自记忆位置与大小） |
| 透明度 | 85% ↔ 30% 两档互切 |
| 置顶 | 切换浮层是否始终置顶 |
| 移动 | 按住拖动浮层到任意位置；靠近窗口边缘自动吸附 |

默认快捷键 `Ctrl+Shift+Space` 显示/隐藏浮层；热键可在「配置管理 → 通用设置 → 快捷键」中修改，保存时自动检测是否与其他软件冲突。

### 系统托盘

托盘图标右键菜单：

| 菜单项 | 作用 |
|--------|------|
| 显示/隐藏浮层 | 切换浮层可见性 |
| 重置悬浮窗位置和大小 | 清除记忆的几何，恢复默认位置与尺寸 |
| 配置管理 | 打开设置窗口 |
| 退出 | 退出应用（带确认对话框） |

### 配置管理

设置窗口（托盘菜单「配置管理」打开）包含四个页签：

- **通用设置**：开机自启动、默认置顶、默认透明度（不透明 / 85% / 70% / 45% / 30%）、长按触发时间、默认布局、快捷键、配置管理（导入 / 导出 / 恢复默认）
- **默认按钮**：增删改全局按钮（未匹配任何进程时显示），支持分组与拖拽排序
- **应用映射**：为指定进程（如 `Code.exe`）绑定专属按钮组与注入模式，支持默认映射兜底
- **关于**：版本信息、检查更新、一键更新、启动时自动检查更新

### 模板按钮与动态占位符

```toml
[[buttons]]
id = "git-commit"
label = "Commit"
content = "git commit -m \"{input}\""    # 左键光标停在引号中间，右键弹出输入框

[[buttons]]
id = "stamp"
label = "时间戳"
content = "[{date} {time}] "              # 注入时展开为 [2026-08-27 14:30:05]
```

交互速查：

| 操作 | 行为 |
|------|------|
| 左键 | 注入内容（模板按钮光标停在 `{input}` 处） |
| 长按超阈值 | 注入内容 + 回车（阈值见通用设置「长按触发时间」） |
| 右键 | 模板按钮：弹出模板输入对话框 |

完整说明见 `config.example.toml` 头部注释。

## 配置文件

配置存储在系统配置目录：

- Windows: `%APPDATA%\com.quickinput.desktop\default.toml`

首次启动自动生成默认配置，节结构如下：

```toml
[overlay]
layout = "vertical"          # vertical | horizontal
opacity = 85                 # 20~100，默认 85
always_on_top = true         # 默认置顶
hold_threshold_ms = 1000     # 长按触发回车的阈值（200~5000）
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
group = "Git"                # 设置界面分组（可选）

[[profiles]]
process_name = "Code.exe"
name = "VS Code"
inject_mode = "paste"        # paste | keystroke（可选，画像级注入模式）

[[profiles.groups]]
name = "格式化"

[[profiles.groups.buttons]]
id = "fmt"
label = "Format"
content = "cargo fmt"
comment = "格式化 Rust 代码"
```

字段说明：

| 节 / 字段 | 说明 |
|-----------|------|
| `[overlay]` | 悬浮窗设置：布局、透明度、置顶、长按阈值、各布局记忆的位置与尺寸 |
| `[shortcuts]` | 全局快捷键（`show_overlay` 为显示/隐藏浮层） |
| `[[buttons]]` | 默认按钮组（未匹配任何进程时显示） |
| `buttons[].group` | 设置界面分组（不产生悬浮窗 Tab） |
| `default_buttons` / `default_inject_mode` | 默认映射：未命中画像时使用的按钮与注入模式 |
| `[[profiles]]` | 进程映射：`process_name` 匹配前台进程 exe 名 |
| `profiles[].inject_mode` | 画像级注入模式（`paste` 剪贴板 / `keystroke` 按键模拟） |
| `[[profiles.groups]]` | 画像分组：悬浮窗顶部 Tab 的依据，未分组按钮归「默认」标签 |

也可在配置管理界面可视化编辑，或通过通用设置中的导出/导入功能备份与迁移。

## 自动更新

- 检查更新入口：配置管理 → 关于 → 检查更新；可开启「启动时自动检查更新」
- 更新包经 minisign 签名，客户端强制公钥校验，镜像仅作传输层，被篡改的安装包会被拒装
- 更新端点内置国内镜像（GitHub 直连兜底），国内用户无需配置即可正常检查与下载

## 项目结构

```
├── src-tauri-app/               # Tauri 应用（前端 + 应用后端）
│   ├── src/                     # SvelteKit 前端
│   │   ├── routes/
│   │   │   ├── +page.svelte          # 悬浮窗页面（按钮面板 + 控制条）
│   │   │   ├── floater/+page.svelte  # 浮层页面（tooltip / 右键菜单 / 按钮组）
│   │   │   └── settings/+page.svelte # 配置管理页面（四页签）
│   │   └── lib/Tooltip.svelte        # 悬浮注释组件
│   ├── src-tauri/               # Rust 后端
│   │   └── src/
│   │       ├── lib.rs               # 命令注册、配置管理、焦点监听、应用装配
│   │       ├── inject.rs            # 注入引擎抽象（paste / keystroke 模式）
│   │       ├── inject_windows.rs    # Windows 注入实现（SendInput / 扫描码）
│   │       ├── focus_guard.rs       # 焦点保护（前台 + 键盘焦点恢复）
│   │       ├── focus_watcher.rs     # 前台进程监听
│   │       ├── focus_detector.rs    # 前台进程名识别
│   │       ├── global_shortcut.rs   # 全局热键（配置驱动 + 冲突检测）
│   │       ├── tray.rs              # 系统托盘（含重置悬浮窗几何）
│   │       ├── window.rs            # 悬浮窗几何/样式/吸附跟随
│   │       ├── floater.rs           # 浮层窗口（tooltip / 菜单 / 按钮组）
│   │       ├── target_window.rs     # 吸附目标窗口判定
│   │       ├── placeholder.rs       # 动态占位符展开（{date}/{time}/{clipboard}）
│   │       └── process_list.rs      # 拥有可见窗口的进程枚举
│   └── scripts/                 # postbuild（hash 对齐）/ copy-exe（绿色版）/ make-update-json（更新元数据）
├── src/backend/                 # quickinput-config 配置库（独立 crate：模型、默认值、校验、管理器）
├── config.example.toml          # 配置示例（含模板按钮与占位符完整说明）
└── docs/                        # 设计文档（产品概念、Epics、Stories、测试报告）
```

## 测试

```bash
# 配置库单元测试（模型校验、管理器、进程匹配等）
cd src/backend && cargo test

# Tauri 应用 Rust 单元测试
cd src-tauri-app/src-tauri && cargo test
```

冒烟测试与功能验收记录见 `docs/test/smoke-report.md`。

## 许可证

[MIT](LICENSE)
