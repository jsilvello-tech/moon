use crate::models::memory::{MemoryMetrics, ProcessRamSummary};
use crate::monitoring::circular_buffer::CircularBuffer;
use crate::utils::formatters::calculate_percentage;
use sysinfo::System;

pub struct MemoryCollector {
    history: CircularBuffer,
}

impl MemoryCollector {
    pub fn new(history_capacity: usize) -> Self {
        Self {
            history: CircularBuffer::new(history_capacity),
        }
    }

    pub fn resize_history(&mut self, capacity: usize) {
        self.history.resize(capacity);
    }

    pub fn collect(&mut self, sys: &mut System) -> MemoryMetrics {
        sys.refresh_memory();

        let total = sys.total_memory();
        let used = sys.used_memory();
        let available = sys.available_memory();
        let free = sys.free_memory();

        let usage_percent = calculate_percentage(used, total);

        let swap_total = sys.total_swap();
        let swap_used = sys.used_swap();
        let swap_percent = calculate_percentage(swap_used, swap_total);

        self.history.push(usage_percent);

        // Identifica os top 5 processos com maior consumo de RAM
        let mut process_list: Vec<_> = sys
            .processes()
            .iter()
            .map(|(pid, proc_info)| {
                let mem = proc_info.memory();
                let pct = calculate_percentage(mem, total);
                (
                    pid.as_u32(),
                    proc_info.name().to_string_lossy().to_string(),
                    mem,
                    pct,
                )
            })
            .collect();

        // Ordena por consumo de memória decrescente
        process_list.sort_by(|a, b| b.2.cmp(&a.2));

        let top_consumers = process_list
            .into_iter()
            .take(5)
            .map(
                |(pid, name, memory_bytes, memory_percent)| ProcessRamSummary {
                    pid,
                    name,
                    memory_bytes,
                    memory_percent,
                },
            )
            .collect();

        MemoryMetrics {
            total_bytes: total,
            used_bytes: used,
            available_bytes: available,
            free_bytes: free,
            usage_percent,
            swap_total_bytes: swap_total,
            swap_used_bytes: swap_used,
            swap_usage_percent: swap_percent,
            history: self.history.to_vec(),
            top_consumers,
        }
    }
}
