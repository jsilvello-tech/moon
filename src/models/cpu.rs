use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreMetric {
    pub index: usize,
    pub usage_percent: f32,
    pub frequency_mhz: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuMetrics {
    pub brand_name: String,
    pub vendor_id: String,
    pub total_usage_percent: f32,
    pub physical_core_count: usize,
    pub logical_core_count: usize,
    pub frequency_mhz: u64,
    pub temperature_celsius: Option<f32>,
    pub cores: Vec<CoreMetric>,
    pub history: Vec<f32>,
}

impl Default for CpuMetrics {
    fn default() -> Self {
        Self {
            brand_name: "Detectando processador...".to_string(),
            vendor_id: "".to_string(),
            total_usage_percent: 0.0,
            physical_core_count: 0,
            logical_core_count: 0,
            frequency_mhz: 0,
            temperature_celsius: None,
            cores: Vec::new(),
            history: Vec::new(),
        }
    }
}
