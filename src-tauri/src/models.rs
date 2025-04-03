use serde::{Deserialize, Serialize};

/// 用于前端和后端间传输的配置结构
#[derive(Debug, Serialize, Deserialize)]
pub struct StorageConfig {
    pub telemetry_machine_id: String,
    pub telemetry_mac_machine_id: String,
    pub telemetry_dev_device_id: String,
    pub telemetry_sqm_id: String,
}

/// 与存储格式匹配的配置文件结构
#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigFile {
    #[serde(rename = "telemetry.machineId")]
    pub telemetry_machine_id: String,
    #[serde(rename = "telemetry.macMachineId")]
    pub telemetry_mac_machine_id: String,
    #[serde(rename = "telemetry.devDeviceId")]
    pub telemetry_dev_device_id: String,
    #[serde(rename = "telemetry.sqmId")]
    pub telemetry_sqm_id: String,
} 