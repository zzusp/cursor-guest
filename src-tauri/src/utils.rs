use std::sync::Mutex;
use tauri::AppHandle;
use tauri::Manager;

// 全局存储App句柄的静态变量
static mut APP_HANDLE: Option<tauri::AppHandle> = None;

// 使用Mutex包装String来存储进程状态
lazy_static::lazy_static! {
    pub static ref PROCESS_STATUS: Mutex<String> = Mutex::new(String::new());
}

// Windows API兼容性检查
#[cfg(target_os = "windows")]
pub fn check_api_compatibility() -> Result<(), String> {
    use std::ptr;
    use winapi::um::libloaderapi::{GetProcAddress, LoadLibraryA};
    
    unsafe {
        // 尝试加载comctl32.dll
        let comctl_lib = LoadLibraryA(b"comctl32.dll\0".as_ptr() as *const i8);
        if comctl_lib.is_null() {
            return Err("无法加载comctl32.dll".to_string());
        }
        
        // 检查TaskDialogIndirect是否可用
        let task_dialog_fn = GetProcAddress(comctl_lib, b"TaskDialogIndirect\0".as_ptr() as *const i8);
        if task_dialog_fn.is_null() {
            return Err("TaskDialogIndirect函数不可用，您的操作系统版本可能过低".to_string());
        }
        
        Ok(())
    }
}

/// 发送步骤状态更新函数
pub fn emit_step_status(step_id: &str, status: &str) {
    println!("[Event] 发送步骤状态更新: {} -> {}", step_id, status);
    
    // 这个函数在非Tauri环境下不会实际发送事件
    #[cfg(not(test))]
    if let Some(app_handle) = get_app_handle() {
        let _ = app_handle.emit_all("step-status-update", serde_json::json!({
            "stepId": step_id,
            "status": status,
            "error": status == "error"
        }));
    }
}

/// 设置App句柄的函数
pub fn set_app_handle(handle: AppHandle) {
    unsafe {
        APP_HANDLE = Some(handle);
    }
}

/// 获取App句柄的函数
pub fn get_app_handle() -> Option<AppHandle> {
    unsafe {
        APP_HANDLE.clone()
    }
}

/// 执行步骤并处理错误
pub fn execute_step<T, F>(step_name: &str, step_id: &str, operation: F) -> Result<T, String>
where F: FnOnce() -> Result<T, String>
{
    println!("[Step: {}] 开始执行...", step_name);
    emit_step_status(step_id, "in-progress");
    
    match operation() {
        Ok(result) => {
            println!("[Step: {}] 执行成功", step_name);
            emit_step_status(step_id, "completed");
            Ok(result)
        },
        Err(e) => {
            let error = format!("步骤'{}'执行失败: {}", step_name, e);
            println!("[Step: {}] {}", step_name, error);
            emit_step_status(step_id, "error");
            Err(error)
        }
    }
}

// 显示错误消息对话框，避免使用TaskDialogIndirect
#[cfg(target_os = "windows")]
pub fn show_error_message(title: &str, message: &str) {
    use std::ffi::OsStr;
    use std::iter::once;
    use std::os::windows::ffi::OsStrExt;
    use std::ptr::null_mut;
    use winapi::um::winuser::{MB_ICONERROR, MB_OK, MessageBoxW};

    let title_wide: Vec<u16> = OsStr::new(title).encode_wide().chain(once(0)).collect();
    let message_wide: Vec<u16> = OsStr::new(message).encode_wide().chain(once(0)).collect();

    unsafe {
        MessageBoxW(
            null_mut(),
            message_wide.as_ptr(),
            title_wide.as_ptr(),
            MB_OK | MB_ICONERROR,
        );
    }
} 