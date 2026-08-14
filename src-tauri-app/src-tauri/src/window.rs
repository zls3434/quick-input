//! 浮层窗口系统模块 (M1)
//!
//! 负责 QuickInput 置顶浮层窗口的创建、系统级置顶/不抢焦点样式设置、
//! 以及窗口句柄访问。首期实现 Windows 平台。

use tauri::{AppHandle, Manager, WebviewWindow};

/// 浮层窗口的 label（与 tauri.conf.json 中的 label 一致）
pub const OVERLAY_WINDOW_LABEL: &str = "overlay";

/// 获取浮层窗口句柄
pub fn get_overlay_window(app: &AppHandle) -> Option<WebviewWindow> {
    app.get_webview_window(OVERLAY_WINDOW_LABEL)
}

/// 应用置顶窗口系统级样式
///
/// 在 Tauri 窗口创建后，通过原生 API 设置窗口扩展样式以确保：
/// 1. 窗口始终置顶 (WS_EX_TOPMOST / HWND_TOPMOST)
/// 2. 点击不激活/不抢焦点 (WS_EX_NOACTIVATE)
/// 3. 不在任务栏显示 (WS_EX_TOOLWINDOW)
pub fn apply_overlay_styles(app: &AppHandle) -> Result<(), anyhow::Error> {
    #[cfg(target_os = "windows")]
    {
        apply_windows_overlay_styles(app)?;
    }

    #[cfg(target_os = "macos")]
    {
        // macOS 使用 NSPanel 实现，预留
        let _ = app;
    }

    #[cfg(target_os = "linux")]
    {
        // Linux 使用 override_redirect 实现，预留
        let _ = app;
    }

    Ok(())
}

/// Windows 平台：通过 HWND 设置置顶/不抢焦点扩展样式
#[cfg(target_os = "windows")]
fn apply_windows_overlay_styles(app: &AppHandle) -> Result<(), anyhow::Error> {
    use raw_window_handle::{HasWindowHandle, RawWindowHandle};
    use windows::Win32::Foundation::HWND;
    use windows::Win32::UI::WindowsAndMessaging::*;

    let window = get_overlay_window(app)
        .ok_or_else(|| anyhow::anyhow!("浮层窗口 (overlay) 未找到"))?;

    let handle = window.window_handle()?;
    let raw = handle.as_raw();

    #[cfg(target_os = "windows")]
    if let RawWindowHandle::Win32(win32) = raw {
        let hwnd = HWND(win32.hwnd.get() as *mut std::ffi::c_void);

        unsafe {
            // 设置扩展样式：不抢焦点 + 置顶 + 工具窗口（不在任务栏）
            let ex_style = GetWindowLongPtrW(hwnd, GWL_EXSTYLE);
            let new_style = ex_style
                | (WS_EX_NOACTIVATE.0 as isize)
                | (WS_EX_TOPMOST.0 as isize)
                | (WS_EX_TOOLWINDOW.0 as isize);
            SetWindowLongPtrW(hwnd, GWL_EXSTYLE, new_style);

            // 确保 Z-order 置顶（仅改样式可能不生效）
            let _ = SetWindowPos(
                hwnd,
                HWND_TOPMOST,
                0, 0, 0, 0,
                SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE,
            );

            // 显示但不激活窗口
            let _ = ShowWindow(hwnd, SW_SHOWNOACTIVATE);
        }
    }

    Ok(())
}