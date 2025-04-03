use std::process::Command;
use encoding_rs::GBK;
use crate::utils::PROCESS_STATUS;

/// 关闭Cursor进程
pub fn close_cursor_processes() -> Result<(), String> {
    println!("[Step: 检查进程] 正在检查并关闭Cursor进程...");
    
    if cfg!(target_os = "windows") {
        let output = Command::new("taskkill")
            .args(&["/F", "/IM", "Cursor.exe"])
            .output()
            .map_err(|e| format!("Failed to execute command: {}", e))?;
        
        if !output.status.success() {
            // 使用GBK编码来解码中文错误消息
            let (cow, _encoding_used, _had_errors) = GBK.decode(&output.stderr);
            let error = cow.into_owned();
            
            if error.contains("not found") || error.contains("没有找到") || error.contains("找不到") {
                let status = "未发现运行中的Cursor进程";
                println!("[Step: 检查进程] {}", status);
                
                // 更新状态
                let mut process_status = PROCESS_STATUS.lock().unwrap();
                *process_status = status.to_string();
                
                // 未找到进程仍然视为成功
                return Ok(());
            } else {
                let status = format!("关闭Cursor失败: {}", error);
                println!("[Step: 检查进程] {}", status);
                
                // 更新状态
                let mut process_status = PROCESS_STATUS.lock().unwrap();
                *process_status = status;
                
                return Err(format!("Failed to close Cursor: {}", error));
            }
        } else {
            let status = "成功关闭Cursor进程";
            println!("[Step: 检查进程] {}", status);
            
            // 更新状态
            let mut process_status = PROCESS_STATUS.lock().unwrap();
            *process_status = status.to_string();
        }
    } else {
        let output = Command::new("pkill")
            .arg("Cursor")
            .output()
            .map_err(|e| format!("Failed to execute command: {}", e))?;
        
        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            // 对于pkill，通常返回码1表示没有匹配的进程
            // macOS和Linux pkill通常在找不到进程时会返回非零状态码
            if output.status.code() == Some(1) && error.is_empty() {
                let status = "未发现运行中的Cursor进程";
                println!("[Step: 检查进程] {}", status);
                
                // 更新状态
                let mut process_status = PROCESS_STATUS.lock().unwrap();
                *process_status = status.to_string();
                
                // 未找到进程仍然视为成功
                return Ok(());
            } else if !error.is_empty() {
                let status = format!("关闭Cursor失败: {}", error);
                println!("[Step: 检查进程] {}", status);
                
                // 更新状态
                let mut process_status = PROCESS_STATUS.lock().unwrap();
                *process_status = status;
                
                return Err(format!("Failed to close Cursor: {}", error));
            } else {
                let status = "未发现运行中的Cursor进程";
                println!("[Step: 检查进程] {}", status);
                
                // 更新状态
                let mut process_status = PROCESS_STATUS.lock().unwrap();
                *process_status = status.to_string();
                
                return Ok(());
            }
        } else {
            let status = "成功关闭Cursor进程";
            println!("[Step: 检查进程] {}", status);
            
            // 更新状态
            let mut process_status = PROCESS_STATUS.lock().unwrap();
            *process_status = status.to_string();
        }
    }
    
    println!("[Step: 检查进程] 完成Cursor进程检查");
    Ok(())
}

/// 获取进程状态
pub fn check_cursor_process_status() -> String {
    let status = PROCESS_STATUS.lock().unwrap();
    status.clone()
} 