#[cfg(target_os = "windows")]
use winreg::{enums::*, RegKey};

use crate::utils::emit_step_status;

/// 读取Windows系统注册表MachineGuid
#[cfg(target_os = "windows")]
pub fn get_machine_guid() -> Result<String, String> {
    println!("[Step: 读取MachineGuid] 正在读取系统MachineGuid...");
    
    // 尝试打开注册表并读取
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    
    match hklm.open_subkey_with_flags(r"SOFTWARE\Microsoft\Cryptography", KEY_READ) {
        Ok(crypto_key) => {
            match crypto_key.get_value::<String, _>("MachineGuid") {
                Ok(guid) => {
                    println!("[Step: 读取MachineGuid] 当前系统MachineGuid: {}", guid);
                    Ok(guid)
                },
                Err(e) => {
                    let error = format!("无法读取MachineGuid: {}", e);
                    println!("[Step: 读取MachineGuid] {}", error);
                    Err(error)
                }
            }
        },
        Err(e) => {
            let error = format!("无法打开Cryptography注册表项以进行读取: {}", e);
            println!("[Step: 读取MachineGuid] {}", error);
            Err(error)
        }
    }
}

/// 校验Windows系统注册表MachineGuid是否与预期值一致
#[cfg(target_os = "windows")]
pub fn verify_machine_guid(expected_guid: &str) -> Result<bool, String> {
    println!("[Step: 校验MachineGuid] 正在校验系统MachineGuid...");
    
    match get_machine_guid() {
        Ok(current_guid) => {
            let is_consistent = current_guid == expected_guid;
            if is_consistent {
                println!("[Step: 校验MachineGuid] 校验成功: 系统MachineGuid与应用设置一致");
            } else {
                println!("[Step: 校验MachineGuid] 校验失败: 系统MachineGuid({})与预期值({})不一致", 
                          current_guid, expected_guid);
            }
            Ok(is_consistent)
        },
        Err(e) => Err(e)
    }
}

/// 修改Windows系统注册表MachineGuid
#[cfg(target_os = "windows")]
pub fn set_machine_guid(new_guid: &str) -> Result<(), String> {
    println!("[Step: 修改MachineGuid] 正在修改系统MachineGuid...");
    
    // 先读取并备份当前的MachineGuid
    let original_guid = match get_machine_guid() {
        Ok(guid) => {
            println!("[Step: 修改MachineGuid] 已备份原始MachineGuid");
            Some(guid)
        },
        Err(e) => {
            println!("[Step: 修改MachineGuid] 无法读取原始MachineGuid: {}", e);
            // 在没有管理员权限的情况下，不能读取可能也不能写入，所以报错退出
            if e.contains("拒绝访问") || e.contains("access is denied") || e.contains("权限") {
                let error = format!("无法读取MachineGuid：{}。请以管理员身份运行。", e);
                emit_step_status("update-registry", "error");
                return Err(error);
            }
            None
        }
    };
    
    // 不再尝试设置注册表文件权限，直接通过注册表API修改
    println!("[Step: 修改MachineGuid] 正在使用注册表API修改MachineGuid...");
    
    // 尝试打开注册表并修改
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    
    match hklm.open_subkey_with_flags(r"SOFTWARE\Microsoft\Cryptography", KEY_WRITE) {
        Ok(crypto_key) => {
            match crypto_key.set_value("MachineGuid", &new_guid) {
                Ok(_) => {
                    println!("[Step: 修改MachineGuid] 成功修改系统MachineGuid为: {}", new_guid);
                    
                    // 校验修改是否成功
                    match get_machine_guid() {
                        Ok(current_guid) => {
                            if current_guid == new_guid {
                                Ok(())
                            } else {
                                let error = format!("修改后的MachineGuid与预期不符，可能是权限不足");
                                println!("[Step: 修改MachineGuid] {}", error);
                                
                                // 如果有原始值，尝试恢复
                                if let Some(orig_guid) = original_guid {
                                    println!("[Step: 修改MachineGuid] 尝试恢复原始MachineGuid");
                                    let _ = crypto_key.set_value("MachineGuid", &orig_guid);
                                }
                                
                                Err(error)
                            }
                        },
                        Err(e) => {
                            let error = format!("无法验证MachineGuid修改: {}", e);
                            println!("[Step: 修改MachineGuid] {}", error);
                            
                            // 如果有原始值，尝试恢复
                            if let Some(orig_guid) = original_guid {
                                println!("[Step: 修改MachineGuid] 尝试恢复原始MachineGuid");
                                let _ = crypto_key.set_value("MachineGuid", &orig_guid);
                            }
                            
                            Err(error)
                        }
                    }
                },
                Err(e) => {
                    let error = format!("无法修改MachineGuid: {}，请以管理员身份运行", e);
                    println!("[Step: 修改MachineGuid] {}", error);
                    Err(error)
                }
            }
        },
        Err(e) => {
            let error = format!("无法打开Cryptography注册表项以进行写入: {}，请以管理员身份运行", e);
            println!("[Step: 修改MachineGuid] {}", error);
            Err(error)
        }
    }
} 