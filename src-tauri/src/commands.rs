use uuid::Uuid;
use crate::config::{generate_new_config, read_config, save_config};
use crate::models::StorageConfig;
use crate::process;
use crate::utils::execute_step;

#[cfg(target_os = "windows")]
use crate::registry::{set_machine_guid, verify_machine_guid};

/// 获取进程状态的命令
#[tauri::command]
pub fn check_cursor_process_status() -> String {
    process::check_cursor_process_status()
}

/// 关闭Cursor进程的命令
#[tauri::command]
pub fn close_cursor_processes() -> Result<(), String> {
    process::close_cursor_processes()
}

/// 修改Cursor IDs的命令
#[tauri::command]
pub fn modify_cursor_ids() -> Result<StorageConfig, String> {
    println!("[流程开始] 开始修改Cursor IDs");
    
    // 步骤1：读取现有配置
    let old_config = match execute_step("读取配置", "backup-config", || {
        match read_config() {
            Ok(config) => Ok(Some(config)),
            Err(e) => {
                // 如果是因为配置文件不存在导致的错误，我们可以继续
                if e.contains("Failed to open config file") && e.contains("No such file") {
                    println!("[Step: 读取配置] 配置文件不存在，将创建新配置");
                    Ok(None)
                } else {
                    Err(format!("读取配置失败: {}", e))
                }
            }
        }
    }) {
        Ok(config) => config,
        Err(e) => return Err(format!("{}", e))
    };
    
    // 步骤2：生成新配置
    let mut new_config = match execute_step("生成IDs", "generate-ids", || {
        Ok(generate_new_config(old_config))
    }) {
        Ok(config) => config,
        Err(e) => return Err(format!("{}", e))
    };
    
    // 使用new_config中的telemetry_machine_id作为新的MachineGuid
    let new_uuid = new_config.telemetry_machine_id.clone();
    println!("[生成ID] 新生成的UUID: {}", new_uuid);
    
    // 步骤3：在Windows系统上，修改系统注册表中的MachineGuid
    #[cfg(target_os = "windows")]
    {
        // 修改MachineGuid
        if let Err(e) = execute_step("修改系统MachineGuid", "update-registry", || {
            set_machine_guid(&new_uuid)
        }) {
            return Err(format!("{}", e));
        }
        
        // 验证MachineGuid是否成功修改
        match execute_step("验证MachineGuid", "verify-registry", || {
            match verify_machine_guid(&new_uuid) {
                Ok(is_consistent) => {
                    if is_consistent {
                        Ok(())
                    } else {
                        Err("系统MachineGuid与telemetry_machine_id不一致".into())
                    }
                },
                Err(e) => Err(e)
            }
        }) {
            Ok(_) => {},
            Err(e) => return Err(format!("{}。请尝试以管理员身份运行此应用程序。", e))
        }
    }
    
    // 步骤4：保存新配置
    if let Err(e) = execute_step("保存配置", "update-config", || {
        save_config(&new_config)
    }) {
        return Err(format!("{}", e));
    }
    
    println!("[流程结束] Cursor IDs修改完成");
    Ok(new_config)
} 