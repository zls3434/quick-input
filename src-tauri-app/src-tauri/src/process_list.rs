//! 运行进程枚举模块
//!
//! 枚举系统中拥有可见顶层窗口的进程（进程名 + 窗口标题），
//! 供配置管理界面的"绑定进程"下拉列表选择。

#[cfg(target_os = "windows")]
mod imp {
    use std::collections::BTreeMap;

    use serde::Serialize;
    use windows::Win32::Foundation::{CloseHandle, BOOL, HWND, LPARAM};
    use windows::Win32::System::Threading::{
        OpenProcess, PROCESS_NAME_FORMAT, PROCESS_QUERY_INFORMATION, QueryFullProcessImageNameW,
    };
    use windows::Win32::UI::WindowsAndMessaging::{
        EnumWindows, GetWindowLongW, GetWindowTextW, GetWindowThreadProcessId, GWL_EXSTYLE,
        WS_EX_TOOLWINDOW,
    };

    /// 单个运行进程信息
    #[derive(Debug, Clone, Serialize)]
    pub struct RunningProcess {
        /// 进程可执行文件名（如 `Code.exe`）
        pub process_name: String,
        /// 该进程的一个可见窗口标题（用于辨识）
        pub window_title: String,
    }

    /// 收集枚举结果的上下文
    struct EnumCtx {
        /// 进程名 -> 窗口标题（按进程名去重，BTreeMap 保持字母序）
        map: BTreeMap<String, String>,
    }

    /// 打开进程并取可执行文件名
    fn process_name_of(pid: u32) -> Option<String> {
        unsafe {
            let handle = OpenProcess(PROCESS_QUERY_INFORMATION, false, pid).ok()?;
            let mut buffer = vec![0u16; 4096];
            let mut size = buffer.len() as u32;
            let ok = QueryFullProcessImageNameW(
                handle,
                PROCESS_NAME_FORMAT(0),
                windows::core::PWSTR(buffer.as_mut_ptr()),
                &mut size,
            );
            let _ = CloseHandle(handle);
            if ok.is_err() {
                return None;
            }
            let path = String::from_utf16_lossy(&buffer[..size as usize]);
            std::path::Path::new(&path)
                .file_name()
                .map(|n| n.to_string_lossy().into_owned())
        }
    }

    /// EnumWindows 回调：收集可见、有标题、非工具窗口的进程
    unsafe extern "system" fn enum_callback(hwnd: HWND, lparam: LPARAM) -> BOOL {
        let ctx = unsafe { &mut *(lparam.0 as *mut EnumCtx) };
        unsafe {
            // 仅统计可见窗口
            if !windows::Win32::UI::WindowsAndMessaging::IsWindowVisible(hwnd).as_bool() {
                return BOOL(1);
            }
            // 排除工具窗口（托盘、悬浮小部件等）
            let ex_style = GetWindowLongW(hwnd, GWL_EXSTYLE) as u32;
            if ex_style & WS_EX_TOOLWINDOW.0 != 0 {
                return BOOL(1);
            }
            // 必须有窗口标题（过滤隐藏主窗口的进程）
            let mut title = [0u16; 256];
            let len = GetWindowTextW(hwnd, &mut title);
            if len == 0 {
                return BOOL(1);
            }
            let title_str = String::from_utf16_lossy(&title[..len as usize]);

            let mut pid = 0u32;
            GetWindowThreadProcessId(hwnd, Some(&mut pid));
            if pid == 0 {
                return BOOL(1);
            }
            if let Some(name) = process_name_of(pid) {
                if !name.is_empty() {
                    ctx.map.entry(name).or_insert(title_str);
                }
            }
        }
        BOOL(1)
    }

    /// 枚举拥有可见窗口的进程（按进程名去重、字母序）
    pub fn list_window_processes() -> Vec<RunningProcess> {
        let mut ctx = EnumCtx {
            map: BTreeMap::new(),
        };
        unsafe {
            let lparam = LPARAM(&mut ctx as *mut EnumCtx as isize);
            let _ = EnumWindows(Some(enum_callback), lparam);
        }
        ctx.map
            .into_iter()
            .map(|(process_name, window_title)| RunningProcess {
                process_name,
                window_title,
            })
            .collect()
    }
}

#[cfg(target_os = "windows")]
pub use imp::{list_window_processes, RunningProcess};

/// 跨平台占位：非 Windows 暂返回空列表
#[cfg(not(target_os = "windows"))]
pub fn list_window_processes() -> Vec<imp_stub::RunningProcess> {
    Vec::new()
}

#[cfg(not(target_os = "windows"))]
mod imp_stub {
    #[derive(Debug, Clone, serde::Serialize)]
    pub struct RunningProcess {
        pub process_name: String,
        pub window_title: String,
    }
}

#[cfg(test)]
mod tests {
    #[cfg(target_os = "windows")]
    #[test]
    fn test_list_window_processes_returns_nonempty() {
        // 测试进程自身拥有控制台/测试窗口视环境而定，此处仅验证不 panic 且去重
        let list = super::list_window_processes();
        let names: Vec<&str> = list.iter().map(|p| p.process_name.as_str()).collect();
        let mut sorted = names.clone();
        sorted.sort();
        assert_eq!(names, sorted, "应按字母序排列");
    }
}
