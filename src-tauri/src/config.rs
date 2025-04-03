use std::fs;
use std::io::{Read, Write};
use std::path::PathBuf;
use rand::{Rng, thread_rng};
use uuid::Uuid;
use crate::models::{ConfigFile, StorageConfig};

/// 查找Cursor配置文件路径
pub fn get_config_path() -> Result<PathBuf, String> {
    println!("[Step: 寻找配置文件] 正在查找Cursor的配置文件路径...");
    
    let home_dir = dirs::home_dir().ok_or_else(|| "Could not find home directory".to_string())?;
    
    let config_path = if cfg!(target_os = "windows") {
        home_dir.join("AppData").join("Roaming").join("Cursor").join("User").join("globalStorage").join("storage.json")
    } else if cfg!(target_os = "macos") {
        home_dir.join("Library").join("Application Support").join("Cursor").join("storage.json")
    } else {
        home_dir.join(".config").join("Cursor").join("storage.json")
    };
    
    println!("[Step: 寻找配置文件] 配置文件路径: {:?}", config_path);
    
    // 确保目录存在
    if let Some(parent) = config_path.parent() {
        if !parent.exists() {
            println!("[Step: 寻找配置文件] 创建父目录: {:?}", parent);
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create directory: {}", e))?;
        }
    }
    
    // 如果文件不存在，创建一个空的JSON对象
    if !config_path.exists() {
        println!("[Step: 寻找配置文件] 配置文件不存在，创建新文件");
        let empty_config = ConfigFile {
            telemetry_machine_id: String::new(),
            telemetry_mac_machine_id: String::new(),
            telemetry_dev_device_id: String::new(),
            telemetry_sqm_id: String::new(),
        };
        
        let json = serde_json::to_string_pretty(&empty_config)
            .map_err(|e| format!("Failed to serialize empty config: {}", e))?;
        
        let mut file = fs::File::create(&config_path)
            .map_err(|e| format!("Failed to create config file: {}", e))?;
        
        file.write_all(json.as_bytes())
            .map_err(|e| format!("Failed to write empty config file: {}", e))?;
    } else {
        println!("[Step: 寻找配置文件] 配置文件已存在");
    }
    
    Ok(config_path)
}

/// 读取现有配置
pub fn read_config() -> Result<ConfigFile, String> {
    println!("[Step: 读取配置] 正在读取现有配置文件...");
    
    let config_path = get_config_path()?;
    
    let mut file = fs::File::open(&config_path)
        .map_err(|e| format!("Failed to open config file: {}", e))?;
    
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|e| format!("Failed to read config file: {}", e))?;
    
    if contents.trim().is_empty() {
        println!("[Step: 读取配置] 配置文件为空，返回空配置");
        return Ok(ConfigFile {
            telemetry_machine_id: String::new(),
            telemetry_mac_machine_id: String::new(),
            telemetry_dev_device_id: String::new(),
            telemetry_sqm_id: String::new(),
        });
    }
    
    println!("[Step: 读取配置] 成功解析配置文件");
    
    serde_json::from_str(&contents)
        .map_err(|e| format!("Failed to parse config file: {}", e))
}

/// 生成新配置
pub fn generate_new_config(old_config: Option<ConfigFile>) -> StorageConfig {
    println!("[Step: 生成IDs] 正在生成新的Cursor IDs...");
    
    // 生成64个随机十六进制字符作为machine_id和mac_machine_id
    let mut rng = thread_rng();
    let mut generate_hex_id = || {
        let mut bytes = [0u8; 32]; // 32字节 = 64个十六进制字符
        rng.fill(&mut bytes);
        bytes.iter().map(|b| format!("{:02x}", b)).collect::<String>()
    };
    
    // 生成大括号包围的大写UUID作为sqm_id
    let generate_sqm_id = || {
        let uuid = Uuid::new_v4();
        format!("{{{}}}", uuid.to_string().to_uppercase())
    };
    
    let mut new_config = StorageConfig {
        telemetry_machine_id: generate_hex_id(),
        telemetry_mac_machine_id: generate_hex_id(),
        telemetry_dev_device_id: Uuid::new_v4().to_string(),
        telemetry_sqm_id: String::new(),
    };
    
    // Keep the SQM ID if it exists, otherwise generate a new one
    if let Some(old) = old_config {
        if !old.telemetry_sqm_id.is_empty() {
            println!("[Step: 生成IDs] 保留现有SQM ID");
            new_config.telemetry_sqm_id = old.telemetry_sqm_id;
        } else {
            println!("[Step: 生成IDs] 生成新的SQM ID");
            new_config.telemetry_sqm_id = generate_sqm_id();
        }
    } else {
        println!("[Step: 生成IDs] 生成新的SQM ID");
        new_config.telemetry_sqm_id = generate_sqm_id();
    }
    
    println!("[Step: 生成IDs] 新的IDs生成完成");
    
    new_config
}

/// 保存配置到文件
pub fn save_config(config: &StorageConfig) -> Result<(), String> {
    println!("[Step: 保存配置] 正在保存新的配置...");
    
    let config_path = get_config_path()?;
    
    let config_file = ConfigFile {
        telemetry_machine_id: config.telemetry_machine_id.clone(),
        telemetry_mac_machine_id: config.telemetry_mac_machine_id.clone(),
        telemetry_dev_device_id: config.telemetry_dev_device_id.clone(),
        telemetry_sqm_id: config.telemetry_sqm_id.clone(),
    };
    
    let json = serde_json::to_string_pretty(&config_file)
        .map_err(|e| format!("Failed to serialize config: {}", e))?;
    
    // Create a backup before overwriting
    if config_path.exists() {
        println!("[Step: 备份配置] 创建配置文件备份");
        let timestamp = chrono::Local::now().format("backup_%Y%m%d_%H%M%S");
        let backup_path = config_path.with_extension(format!("json.{}", timestamp));
        if let Err(e) = fs::copy(&config_path, &backup_path) {
            eprintln!("Warning: Failed to create backup: {}", e);
        } else {
            println!("[Step: 备份配置] 备份文件已保存至: {:?}", backup_path);
        }
    }
    
    let mut file = fs::File::create(&config_path)
        .map_err(|e| format!("Failed to create config file: {}", e))?;
    
    file.write_all(json.as_bytes())
        .map_err(|e| format!("Failed to write config file: {}", e))?;
    
    println!("[Step: 保存配置] 新配置已保存至: {:?}", config_path);
    
    // Set read-only if requested
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        println!("[Step: 设置权限] 设置文件权限为只读");
        
        let perms = fs::Permissions::from_mode(0o444);
        if let Err(e) = fs::set_permissions(&config_path, perms) {
            eprintln!("Warning: Failed to set permissions: {}", e);
            return Err(format!("Failed to set permissions: {}", e));
        } else {
            println!("[Step: 设置权限] 文件权限已设置");
        }
    }
    
    Ok(())
} 