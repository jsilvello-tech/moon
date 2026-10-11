use crate::models::{
    cpu::CpuMetrics,
    gpu::GpuMetrics,
    memory::MemoryMetrics,
    process::{ProcessDetail, ProcessSortOrder},
    storage::StorageMetrics,
    system::SystemInfoMetrics,
};
use crate::monitoring::{
    cpu::CpuCollector, gpu::GpuCollector, memory::MemoryCollector, processes::ProcessesCollector,
    self_monitor::SelfMonitor, storage::StorageCollector, system::SystemInfoCollector,
};
use sysinfo::System;

#[derive(Debug, Clone)]
pub struct TelemetrySnapshot {
    pub cpu: CpuMetrics,
    pub memory: MemoryMetrics,
    pub gpu: GpuMetrics,
    pub storage: StorageMetrics,
    pub processes: Vec<ProcessDetail>,
    pub total_process_count: usize,
    pub system_info: SystemInfoMetrics,
    pub self_cpu_percent: f32,
    pub self_memory_bytes: u64,
}

pub struct CentralCollector {
    sys: System,
    cpu_collector: CpuCollector,
    memory_collector: MemoryCollector,
    gpu_collector: GpuCollector,
    storage_collector: StorageCollector,
    processes_collector: ProcessesCollector,
    system_collector: SystemInfoCollector,
    self_monitor: SelfMonitor,
    interval_secs: u64,
}

impl CentralCollector {
    pub fn new(interval_secs: u64) -> Self {
        let history_points = Self::calc_history_points(interval_secs);
        let mut sys = System::new_all();

        // Primeira leitura para estabelecer a linha de base de CPU
        sys.refresh_cpu_all();

        Self {
            sys,
            cpu_collector: CpuCollector::new(history_points),
            memory_collector: MemoryCollector::new(history_points),
            gpu_collector: GpuCollector::new(),
            storage_collector: StorageCollector::new(),
            processes_collector: ProcessesCollector::new(),
            system_collector: SystemInfoCollector::new(),
            self_monitor: SelfMonitor::new(),
            interval_secs,
        }
    }

    /// Calcula o número de pontos necessários para cobrir 60 segundos de telemetria.
    pub fn calc_history_points(interval_secs: u64) -> usize {
        let secs = interval_secs.max(1);
        (60 / secs).max(12) as usize
    }

    /// Atualiza o intervalo e redimensiona os buffers de histórico.
    pub fn set_interval(&mut self, interval_secs: u64) {
        self.interval_secs = interval_secs;
        let points = Self::calc_history_points(interval_secs);
        self.cpu_collector.resize_history(points);
        self.memory_collector.resize_history(points);
    }

    /// Executa uma rodada completa de coleta não-bloqueante de todas as fontes de dados.
    pub fn collect_snapshot(
        &mut self,
        search_query: &str,
        sort_order: ProcessSortOrder,
    ) -> TelemetrySnapshot {
        let cpu = self.cpu_collector.collect(&mut self.sys);
        let memory = self.memory_collector.collect(&mut self.sys);
        let gpu = self.gpu_collector.collect();
        let storage = self.storage_collector.collect();
        let (processes, total_process_count) =
            self.processes_collector
                .collect(&mut self.sys, search_query, sort_order, 250);
        let system_info = self.system_collector.collect(&self.sys);
        let (self_cpu, self_ram) = self.self_monitor.collect(&self.sys);

        TelemetrySnapshot {
            cpu,
            memory,
            gpu,
            storage,
            processes,
            total_process_count,
            system_info,
            self_cpu_percent: self_cpu,
            self_memory_bytes: self_ram,
        }
    }
}
