use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessRamSummary {
    pub pid: u32,
    pub name: String,
    pub memory_bytes: u64,
    pub memory_percent: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryMetrics {
    pub total_bytes: u64,
    pub used_bytes: u64,
    pub available_bytes: u64,
    pub free_bytes: u64,
    pub usage_percent: f32,
    pub swap_total_bytes: u64,
    pub swap_used_bytes: u64,
    pub swap_usage_percent: f32,
    pub history: Vec<f32>,
    pub top_consumers: Vec<ProcessRamSummary>,
}

impl Default for MemoryMetrics {
    fn default() -> Self {
        Self {
            total_bytes: 0,
            used_bytes: 0,
            available_bytes: 0,
            free_bytes: 0,
            usage_percent: 0.0,
            swap_total_bytes: 0,
            swap_used_bytes: 0,
            swap_usage_percent: 0.0,
            history: Vec::new(),
            top_consumers: Vec::new(),
        }
    }
}
