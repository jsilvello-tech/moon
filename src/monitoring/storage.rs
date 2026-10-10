use crate::models::storage::{DiskVolume, StorageMetrics};
use crate::utils::formatters::calculate_percentage;
use sysinfo::Disks;

pub struct StorageCollector;

impl StorageCollector {
    pub fn new() -> Self {
        Self
    }

    pub fn collect(&self) -> StorageMetrics {
        let disks = Disks::new_with_refreshed_list();

        let volumes = disks
            .iter()
            .map(|disk| {
                let name = disk.name().to_string_lossy().to_string();
                let mount_point = disk.mount_point().to_string_lossy().to_string();
                let file_system = disk.file_system().to_string_lossy().to_string();
                let total = disk.total_space();
                let available = disk.available_space();
                let used = total.saturating_sub(available);
                let usage_percent = calculate_percentage(used, total);

                DiskVolume {
                    name,
                    mount_point,
                    file_system,
                    total_space_bytes: total,
                    available_space_bytes: available,
                    used_space_bytes: used,
                    usage_percent,
                    is_removable: disk.is_removable(),
                }
            })
            .collect();

        StorageMetrics { volumes }
    }
}
