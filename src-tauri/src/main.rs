#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod config;
mod commands;
mod models;
mod process;
mod registry;
mod utils;

use utils::set_app_handle;

fn main() {
    // 检查API兼容性
    #[cfg(target_os = "windows")]
    {
        if let Err(e) = utils::check_api_compatibility() {
            utils::show_error_message("兼容性错误", &format!("兼容性检查失败: {}", e));
            std::process::exit(1);
        }
    }

    println!("🚀 启动Cursor Guest应用程序");
    
    // 在Windows上提示用户需要管理员权限
    #[cfg(target_os = "windows")]
    {
        println!("注意：修改系统注册表需要管理员权限，请以管理员身份运行此应用程序");
    }
    
    tauri::Builder::default()
        .setup(|app| {
            // 存储App句柄以便后续使用
            set_app_handle(app.handle());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::close_cursor_processes,
            commands::modify_cursor_ids,
            commands::check_cursor_process_status
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
} 
