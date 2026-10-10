use crate::models::process::{ProcessDetail, ProcessSortOrder};
use sysinfo::{ProcessesToUpdate, System};

pub struct ProcessesCollector;

impl ProcessesCollector {
    pub fn new() -> Self {
        Self
    }

    pub fn collect(
        &self,
        sys: &mut System,
        search_query: &str,
        sort_order: ProcessSortOrder,
        limit: usize,
    ) -> (Vec<ProcessDetail>, usize) {
        // Atualiza a tabela de processos
        sys.refresh_processes(ProcessesToUpdate::All, true);

        let query = search_query.trim().to_lowercase();

        let mut list: Vec<ProcessDetail> = sys
            .processes()
            .iter()
            .filter_map(|(pid, proc_info)| {
                let name = proc_info.name().to_string_lossy().to_string();
                if query.is_empty() || name.to_lowercase().contains(&query) {
                    Some(ProcessDetail {
                        pid: pid.as_u32(),
                        name,
                        cpu_usage_percent: proc_info.cpu_usage(),
                        memory_bytes: proc_info.memory(),
                        disk_read_bytes: proc_info.disk_usage().read_bytes,
                        disk_written_bytes: proc_info.disk_usage().written_bytes,
                    })
                } else {
                    None
                }
            })
            .collect();

        let total_matching = list.len();

        match sort_order {
            ProcessSortOrder::CpuDesc => {
                list.sort_by(|a, b| {
                    b.cpu_usage_percent
                        .partial_cmp(&a.cpu_usage_percent)
                        .unwrap_or(std::cmp::Ordering::Equal)
                        .then_with(|| b.memory_bytes.cmp(&a.memory_bytes))
                });
            }
            ProcessSortOrder::MemoryDesc => {
                list.sort_by(|a, b| {
                    b.memory_bytes
                        .cmp(&a.memory_bytes)
                        .then_with(|| {
                            b.cpu_usage_percent
                                .partial_cmp(&a.cpu_usage_percent)
                                .unwrap_or(std::cmp::Ordering::Equal)
                        })
                });
            }
        }

        if list.len() > limit {
            list.truncate(limit);
        }

        (list, total_matching)
    }
}
