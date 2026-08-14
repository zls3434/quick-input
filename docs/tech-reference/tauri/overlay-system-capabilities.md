# Tauri 2 跨平台系统能力技术参考

> 本文档为 QuickInput 实现"置顶浮层 + 点击不抢焦点 + 识别焦点进程 + 注入文本"四大系统级能力的技术参考，涵盖 Windows / macOS / Linux 三平台。

## 一、整体架构

三平台能力通过 Rust 后端统一抽象接口实现：

| 能力 | Windows | macOS | Linux (X11) |
|------|---------|-------|-------------|
| 窗口置顶 | `WS_EX_TOPMOST` | `NSPanel` + `floating` | `override_redirect` + `_NET_WM_STATE_ABOVE` |
| 点击不抢焦点 | `WS_EX_NOACTIVATE` | `.nonactivatingPanel` | `override_redirect` |
| 识别焦点窗口 | `GetForegroundWindow` | `NSWorkspace` / `CGWindowList` | `_NET_ACTIVE_WINDOW` |
| 注入文本 | `SendInput`(UNICODE) | `CGEvent`(需 Accessibility) | `xdotool type` / `wtype` |

## 二、Windows 实现要点

### 2.1 置顶与不抢焦点

通过 Tauri 窗口的 `raw_window_handle` 获取 HWND，用 `windows-rs` 设置扩展样式：

```rust
use raw_window_handle::{HasWindowHandle, RawWindowHandle};
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::*;

let window = app.get_webview_window("overlay")?;
let handle = window.window_handle()?;
if let RawWindowHandle::Win32(win32) = handle.as_raw() {
    let hwnd = HWND(win32.hwnd.get());
    unsafe {
        let ex = GetWindowLongPtrW(hwnd, GWL_EXSTYLE) as u32;
        SetWindowLongPtrW(hwnd, GWL_EXSTYLE,
            (ex | WS_EX_NOACTIVATE | WS_EX_TOPMOST | WS_EX_TOOLWINDOW) as isize);
        // 置顶需配合 SetWindowPos，仅改样式可能不生效（tao issue）
        SetWindowPos(hwnd, HWND_TOPMOST, 0,0,0,0,
            SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE);
        ShowWindow(hwnd, SW_SHOWNOACTIVATE);
    }
}
```

要点：`WS_EX_TOPMOST` 与 `SetWindowPos(HWND_TOPMOST)` 需配合使用；处理 `WM_MOUSEACTIVATE` 返回 `MA_NOACTIVATE`。

### 2.2 识别焦点窗口/进程

```rust
let hwnd = GetForegroundWindow();
let mut pid = 0u32;
GetWindowThreadProcessId(hwnd, Some(&mut pid));
let mut buf = [0u16; 512];
let len = GetWindowTextW(hwnd, &mut buf);
let title = String::from_utf16_lossy(&buf[..len as usize]);
```

### 2.3 注入文本（SendInput Unicode）

```rust
use windows::Win32::UI::Input::KeyboardAndMouse::*;
use windows::Win32::UI::Input::*;

for ch in text.encode_utf16() {
    let inputs = [
        INPUT { r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 { ki: KEYBDINPUT {
                wVk: VIRTUAL_KEY_CODE(0), wScan: ch,
                dwFlags: KEYEVENTF_UNICODE, time: 0, dwExtraInfo: 0 }}},
        INPUT { r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 { ki: KEYBDINPUT {
                wVk: VIRTUAL_KEY_CODE(0), wScan: ch,
                dwFlags: KEYEVENTF_UNICODE | KEYEVENTF_KEYUP, time: 0, dwExtraInfo: 0 }}},
    ];
    SendInput(&inputs, std::mem::size_of::<INPUT>() as i32);
}
```

注意事项：`win-text-inject` crate 处理了剪贴板泄漏、修饰键冲突、UIPI 权限（向高完整性窗口注入受限）等问题，建议参考。

## 三、macOS 实现要点

### 3.1 NSPanel（置顶 + 不抢焦点）

使用社区插件 `tauri-nspanel` 将 `WebviewWindow` 底层转换为 `NSPanel`：

```rust
use tauri_nspanel::{ManagerExt, WebviewWindowExt};
let panel = app.get_webview_window("overlay").unwrap().to_panel().unwrap();
panel.set_style_mask(1 << 7); // NSWindowStyleMaskNonActivatingPanel
panel.set_level(NSMainMenuWindowLevel + 1);
panel.set_collection_behaviour(
    NSWindowCollectionBehaviorCanJoinAllSpaces
    | NSWindowCollectionBehaviorFullScreenAuxiliary);
panel.set_becomes_key_only_if_needed(true);
panel.order_front_regardless();
```

### 3.2 注入文本（CGEvent）

macOS 需要 **Accessibility 权限**。用 `cgevents` crate：

```rust
use cgevents::prelude::*;
fn inject(text: &str) {
    type_string(text, TapLocation::Session).ok();  // 需已授权
}
```

权限检测：`CGPreflightPostEventAccess()` → `CGRequestPostEventAccess()`。发布前需代码签名与公证。

## 四、Linux 实现要点

- X11：`xdotool type --clearmodifiers --delay 0 <text>`，或 `x11rb` 的 XTEST 扩展。
- 焦点窗口：查询 `_NET_ACTIVE_WINDOW` 原子。
- Wayland：`wtype` 注入，`override_redirect` 不适用（需 layer-shell），作为受限降级场景。

## 五、推荐依赖

```toml
tauri = "2"
raw-window-handle = "1"
[target.'cfg(windows)'.dependencies]
windows = { version = "0.58", features = ["Win32_Foundation","Win32_UI_WindowsAndMessaging","Win32_UI_Input_KeyboardAndMouse"] }
[target.'cfg(target_os = "macos")'.dependencies]
tauri-nspanel = { git = "https://github.com/ahkohd/tauri-nspanel", branch = "v2" }
cgevents = "0.10"
[target.'cfg(target_os = "linux")'.dependencies]
x11rb = { version = "0.14", features = ["allow-unsafe-code"] }
active-win-pos-rs = "0.11"   # 跨平台焦点窗口检测
```

## 六、来源

- Tauri 讨论 #14084：Windows 覆盖层实现 https://github.com/tauri-apps/tauri/discussions/14084
- Tauri 讨论 #15251：macOS NSPanel 方案 https://github.com/orgs/tauri-apps/discussions/15251
- `win-text-inject` https://docs.rs/win-text-inject
- `cgevents` https://lib.rs/crates/cgevents
- `tauri-nspanel` https://github.com/ahkohd/tauri-nspanel
- `active-win-pos-rs` https://github.com/dimusic/active-win-pos-rs
- `x11rb` https://docs.rs/x11rb
