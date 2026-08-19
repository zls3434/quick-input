//! 焦点切换监听模块 (S141)
//!
//! 使用 `SetWinEventHook(EVENT_SYSTEM_FOREGROUND)` 监听前台窗口切换，
//! 事件驱动（非轮询），通过 mpsc 通道将前台窗口句柄传递给消费方。
//! 首期实现 Windows 平台。

use std::sync::mpsc;
use std::sync::Mutex;
use std::thread::{self, JoinHandle};

use windows::Win32::Foundation::HWND;
use windows::Win32::System::Threading::GetCurrentThreadId;
use windows::Win32::UI::WindowsAndMessaging::{
    DispatchMessageW, GetMessageW, MSG, PostThreadMessageW, TranslateMessage,
};

/// 全局通道发送端（供 WinEvent 回调使用）
static FOREGROUND_SENDER: Mutex<Option<mpsc::Sender<isize>>> = Mutex::new(None);

/// 诊断探针：钩子注册状态（0=线程未跑 1=注册失败 2=注册成功）
pub static HOOK_STATUS: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
/// 诊断探针：消息循环已处理的消息数（0=循环未运行或无消息）
pub static LOOP_TICKS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
/// 诊断探针：回调已触发次数
pub static CALLBACK_HITS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

/// 前台切换回调函数（WINEVENTPROC 签名）
///
/// 当前台窗口切换时，将新窗口 HWND 通过全局通道发送。
unsafe extern "system" fn foreground_change_callback(
    _hook_handle: HWINEVENTHOOK,
    _event: u32,
    hwnd: HWND,
    _id_object: i32,
    _id_child: i32,
    _id_event_thread: u32,
    _dwms_event_time: u32,
) {
    if let Ok(sender_guard) = FOREGROUND_SENDER.lock() {
        if let Some(ref sender) = *sender_guard {
            CALLBACK_HITS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            let _ = sender.send(hwnd.0 as isize);
        }
    }
}

/// 焦点切换监听器
///
/// 启动后台线程注册 WinEvent 钩子，前台切换时通过 `Receiver<isize>` 推送 HWND。
/// 调用 `stop()` 安全终止。
pub struct FocusWatcher {
    /// 后台线程句柄
    thread_handle: Option<JoinHandle<()>>,
    /// 事件接收端（调用方通过 `take_receiver()` 获取后消费）
    receiver: Option<mpsc::Receiver<isize>>,
    /// 后台线程 ID（用于 PostThreadMessageW 发送 WM_QUIT）
    thread_id: u32,
}

impl FocusWatcher {
    /// 创建并启动焦点监听器
    ///
    /// 启动专用线程，注册 `SetWinEventHook(EVENT_SYSTEM_FOREGROUND)`，
    /// 运行消息循环直到收到 WM_QUIT。
    pub fn start() -> Self {
        let (tx, rx) = mpsc::channel::<isize>();

        // 设置全局发送端
        *FOREGROUND_SENDER.lock().unwrap() = Some(tx);

        let (ready_tx, ready_rx) = std::sync::mpsc::channel::<u32>();

        let handle = thread::spawn(move || {
            // 记录当前线程 ID
            let thread_id = unsafe { GetCurrentThreadId() };

            // 注册 WinEvent 钩子
            let hook = register_hook();
            HOOK_STATUS.store(
                if hook.0.is_null() { 1 } else { 2 },
                std::sync::atomic::Ordering::SeqCst,
            );

            let _ = ready_tx.send(thread_id);

            // 消息循环
            let mut msg = MSG::default();
            loop {
                let ret = unsafe { GetMessageW(&mut msg, None, 0, 0) };
                LOOP_TICKS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                if ret.0 == 0 {
                    // 收到 WM_QUIT，退出循环
                    break;
                }
                unsafe {
                    let _ = TranslateMessage(&msg);
                    let _ = DispatchMessageW(&msg);
                }
            }

            // 退出后取消钩子注册
            if !hook.0.is_null() {
                unsafe {
                    let _ = windows::Win32::UI::Accessibility::UnhookWinEvent(hook);
                }
            }
        });

        let thread_id = ready_rx.recv().unwrap_or(0);

        FocusWatcher {
            thread_handle: Some(handle),
            receiver: Some(rx),
            thread_id,
        }
    }

    /// 取出事件接收端（调用方消费 mpsc 通道）
    ///
    /// 调用后 `FocusWatcher` 不再持有接收端，但监听线程仍在运行。
    pub fn take_receiver(&mut self) -> Option<mpsc::Receiver<isize>> {
        self.receiver.take()
    }

    /// 安全终止监听器
    ///
    /// 向后台线程发送 WM_QUIT 消息，等待线程退出后自动取消钩子。
    pub fn stop(&mut self) {
        if let Some(handle) = self.thread_handle.take() {
            if self.thread_id != 0 {
                unsafe {
                    let _ = PostThreadMessageW(self.thread_id, 0x0012, None, None); // WM_QUIT = 0x0012
                }
            }
            let _ = handle.join();
        }
    }
}

impl Drop for FocusWatcher {
    fn drop(&mut self) {
        self.stop();
    }
}

// ============================================================
// Win32 封装
// ============================================================

/// 前台切换钩子句柄类型
type HWINEVENTHOOK = windows::Win32::UI::Accessibility::HWINEVENTHOOK;

/// 注册前台切换事件钩子
fn register_hook() -> HWINEVENTHOOK {
    use windows::Win32::UI::Accessibility::{SetWinEventHook, WINEVENTPROC};
    use windows::Win32::UI::WindowsAndMessaging::{EVENT_SYSTEM_FOREGROUND, WINEVENT_OUTOFCONTEXT};

    let callback: WINEVENTPROC = Some(foreground_change_callback);
    unsafe {
        SetWinEventHook(
            EVENT_SYSTEM_FOREGROUND,
            EVENT_SYSTEM_FOREGROUND,
            None,
            callback,
            0,
            0,
            WINEVENT_OUTOFCONTEXT,
        )
    }
}

// ============================================================
// 单元测试
// ============================================================

#[cfg(test)]
mod tests {
    use super::FocusWatcher;

    #[cfg(target_os = "windows")]
    #[test]
    fn test_focus_watcher_start_stop() {
        let mut watcher = FocusWatcher::start();
        std::thread::sleep(std::time::Duration::from_millis(50));
        watcher.stop();
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn test_focus_watcher_drop_stops() {
        let watcher = FocusWatcher::start();
        std::thread::sleep(std::time::Duration::from_millis(50));
        drop(watcher);
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn test_focus_watcher_multiple_cycles() {
        for _ in 0..3 {
            let mut watcher = FocusWatcher::start();
            std::thread::sleep(std::time::Duration::from_millis(30));
            watcher.stop();
        }
    }

    // 注：曾有 test_focus_watcher_receives_real_event 真实事件测试——依赖
    // 真实窗口环境（打开 notepad 触发前台切换），不稳定且已无必要：
    // 应用已改用 500ms 轮询检测前台进程（见 lib.rs run_focus_listener），
    // WinEvent 通道不再被应用消费，故移除。
}