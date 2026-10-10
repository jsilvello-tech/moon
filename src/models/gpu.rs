use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuAdapter {
    pub index: usize,
    pub name: String,
    pub vendor: String,
    pub dedicated_vram_bytes: u64,
    pub shared_vram_bytes: u64,
    pub utilization_percent: Option<f32>,
    pub temperature_celsius: Option<f32>,
    pub clock_mhz: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GpuMetrics {
    pub adapters: Vec<GpuAdapter>,
}
