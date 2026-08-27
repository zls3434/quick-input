# QuickInput 桌面应用（Tauri）

本目录是 QuickInput 的 Tauri 应用本体（SvelteKit 前端 + Rust 后端）。项目介绍、功能说明与配置文档见[仓库根 README](../README.md)。

## 常用命令

```bash
npm install        # 安装依赖
npm run tauri:dev  # 开发模式（热重载）
npm run tauri:build# 生产构建（安装包 + MSI；构建后自动复制/重启根目录 QuickInput.exe 绿色版）
npm run check      # svelte-check 类型检查
```

## 目录速览

- `src/` — SvelteKit 前端（悬浮窗 / 浮层 / 设置三页面）
- `src-tauri/` — Rust 后端（注入引擎、焦点监听、托盘、悬浮窗几何与吸附、updater）
- `scripts/` — `postbuild.mjs`（SvelteKit hash 对齐）、`copy-exe.mjs`（绿色版复制与重启）、`make-update-json.mjs`（生成 updater 元数据）

## 推荐 IDE

[VS Code](https://code.visualstudio.com/) + [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)。
