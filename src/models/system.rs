use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfoMetrics {
    pub host_name: String,
    pub os_name: String,
    pub os_version: String,
    pub kernel_version: String,
    pub architecture: String,
    pub uptime_seconds: u64,
}

impl Default for SystemInfoMetrics {
    fn default() -> Self {
        Self {
            host_name: "Detectando...".to_string(),
            os_name: "Windows".to_string(),
            os_version: "11".to_string(),
            kernel_version: "".to_string(),
            architecture: "x86_64".to_string(),
            uptime_seconds: 0,
        }
    }
}
