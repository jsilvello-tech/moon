use crate::models::system::SystemInfoMetrics;
use sysinfo::System;

pub struct SystemInfoCollector;

impl SystemInfoCollector {
    pub fn new() -> Self {
        Self
    }

    pub fn collect(&self, _sys: &System) -> SystemInfoMetrics {
        let host_name = System::host_name().unwrap_or_else(|| "Computador Local".to_string());
        let os_name = System::name().unwrap_or_else(|| "Windows".to_string());
        let os_version = System::os_version().unwrap_or_else(|| "11".to_string());
        let kernel_version = System::kernel_version().unwrap_or_default();
        let architecture = System::cpu_arch().unwrap_or_else(|| "x86_64".to_string());
        let uptime_seconds = System::uptime();

        SystemInfoMetrics {
            host_name,
            os_name,
            os_version,
            kernel_version,
            architecture,
            uptime_seconds,
        }
    }
}
