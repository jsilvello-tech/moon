use crate::models::cpu::{CoreMetric, CpuMetrics};
use crate::monitoring::circular_buffer::CircularBuffer;
use sysinfo::{Components, System};

pub struct CpuCollector {
    history: CircularBuffer,
}

impl CpuCollector {
    pub fn new(history_capacity: usize) -> Self {
        Self {
            history: CircularBuffer::new(history_capacity),
        }
    }

    pub fn resize_history(&mut self, capacity: usize) {
        self.history.resize(capacity);
    }

    pub fn collect(&mut self, sys: &mut System) -> CpuMetrics {
        // Atualiza a telemetria da CPU. O sysinfo calcula a diferença entre chamadas.
        sys.refresh_cpu_all();

        let cpus = sys.cpus();
        let logical_count = cpus.len();
        let physical_count = sys.physical_core_count().unwrap_or(logical_count);

        let brand_name = cpus
            .first()
            .map(|c| c.brand().trim().to_string())
            .unwrap_or_else(|| "Processador Genérico".to_string());

        let vendor_id = cpus
            .first()
            .map(|c| c.vendor_id().trim().to_string())
            .unwrap_or_default();

        let total_usage = sys.global_cpu_usage();

        // Amostra média ou máxima de frequência
        let avg_freq = if !cpus.is_empty() {
            let sum: u64 = cpus.iter().map(|c| c.frequency()).sum();
            sum / cpus.len() as u64
        } else {
            0
        };

        // Coleta o uso detalhado de cada núcleo lógico
        let cores = cpus
            .iter()
            .enumerate()
            .map(|(i, c)| CoreMetric {
                index: i + 1,
                usage_percent: c.cpu_usage(),
                frequency_mhz: c.frequency(),
            })
            .collect();

        // Tenta detectar a temperatura via componentes de hardware
        let components = Components::new_with_refreshed_list();
        let temperature_celsius = components
            .iter()
            .find(|comp| {
                let label = comp.label().to_lowercase();
                label.contains("cpu") || label.contains("core") || label.contains("package") || label.contains("tctl")
            })
            .and_then(|comp| comp.temperature());

        // Adiciona ao buffer histórico
        self.history.push(total_usage);

        CpuMetrics {
            brand_name,
            vendor_id,
            total_usage_percent: total_usage,
            physical_core_count: physical_count,
            logical_core_count: logical_count,
            frequency_mhz: avg_freq,
            temperature_celsius,
            cores,
            history: self.history.to_vec(),
        }
    }
}
