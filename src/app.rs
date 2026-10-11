use crate::models::process::ProcessSortOrder;
use crate::monitoring::collector::{CentralCollector, TelemetrySnapshot};
use crate::settings::{AppSettings, SettingsManager};
use crate::utils::formatters::{
    format_bytes_gib, format_frequency, format_storage_bytes, format_uptime,
};
use slint::{ComponentHandle, ModelRc, VecModel};
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;
use tracing::info;

slint::include_modules!();

struct CollectorSharedState {
    interval_secs: u64,
    is_paused: bool,
    search_query: String,
    sort_order: ProcessSortOrder,
}

pub struct MoonMonitorApp {
    settings_manager: SettingsManager,
}

impl MoonMonitorApp {
    pub fn new() -> Self {
        Self {
            settings_manager: SettingsManager::new(),
        }
    }

    pub fn run(&mut self) -> Result<(), slint::PlatformError> {
        info!("Inicializando Moon Monitor v0.1.0...");

        let main_window = MainWindow::new()?;
        let settings = self.settings_manager.load();

        // Aplica configurações iniciais à interface
        main_window.set_dark_mode(settings.dark_mode);
        main_window.set_update_interval_secs(settings.update_interval_secs as i32);
        main_window.set_animations_enabled(settings.animations_enabled);
        main_window.set_is_paused(settings.is_paused);
        main_window.set_app_version("0.1.0".into());
        main_window.set_config_path(self.settings_manager.get_path_display().into());

        // Estado compartilhado entre a UI e a thread de coleta
        let shared_state = Arc::new(Mutex::new(CollectorSharedState {
            interval_secs: settings.update_interval_secs,
            is_paused: settings.is_paused,
            search_query: String::new(),
            sort_order: ProcessSortOrder::CpuDesc,
        }));
        let condvar = Arc::new(Condvar::new());

        let is_running = Arc::new(AtomicBool::new(true));

        // Inicializa a thread de coleta em segundo plano
        let thread_shared = Arc::clone(&shared_state);
        let thread_condvar = Arc::clone(&condvar);
        let thread_running = Arc::clone(&is_running);
        let weak_ui = main_window.as_weak();

        thread::Builder::new()
            .name("moon-collector".into())
            .spawn(move || {
                info!("Thread de telemetria centralizada iniciada com sucesso.");
                let mut collector = CentralCollector::new(settings.update_interval_secs);

                while thread_running.load(Ordering::Relaxed) {
                    let (interval_secs, is_paused, search_query, sort_order) = {
                        let lock = thread_shared.lock().unwrap();
                        (
                            lock.interval_secs,
                            lock.is_paused,
                            lock.search_query.clone(),
                            lock.sort_order,
                        )
                    };

                    collector.set_interval(interval_secs);

                    if !is_paused {
                        let snapshot = collector.collect_snapshot(&search_query, sort_order);

                        // Envia snapshot para a thread principal do Slint de forma segura
                        let result = weak_ui.upgrade_in_event_loop(move |ui| {
                            Self::apply_snapshot_to_ui(&ui, snapshot);
                        });

                        if result.is_err() {
                            // A janela da interface foi fechada
                            info!("Janela principal fechada. Encerrando ciclo de telemetria.");
                            break;
                        }
                    }

                    // Pausa entre amostragens respeitando o intervalo ou despertando imediatamente em buscas
                    let sleep_duration = if is_paused {
                        Duration::from_millis(500)
                    } else {
                        Duration::from_secs(interval_secs.max(1))
                    };

                    let lock = thread_shared.lock().unwrap();
                    let _ = thread_condvar.wait_timeout(lock, sleep_duration);
                }

                info!("Thread de telemetria finalizada de forma segura.");
            })
            .expect("Falha ao inicializar thread de monitoramento");

        // Callbacks de interação do usuário
        let cb_shared = Arc::clone(&shared_state);
        let cb_condvar = Arc::clone(&condvar);

        // 1. Busca de Processos
        let search_state = Arc::clone(&cb_shared);
        let search_cv = Arc::clone(&cb_condvar);
        main_window.on_search_changed(move |query| {
            if let Ok(mut lock) = search_state.lock() {
                lock.search_query = query.to_string();
            }
            search_cv.notify_one();
        });

        // 2. Ordenação de Processos
        let sort_state = Arc::clone(&cb_shared);
        let sort_cv = Arc::clone(&cb_condvar);
        main_window.on_sort_changed(move |sort_id| {
            if let Ok(mut lock) = sort_state.lock() {
                lock.sort_order = if sort_id == 1 {
                    ProcessSortOrder::MemoryDesc
                } else {
                    ProcessSortOrder::CpuDesc
                };
            }
            sort_cv.notify_one();
        });

        // 3. Seleção de Processo
        main_window.on_process_selected(move |pid, name, cpu, mem| {
            info!("Processo selecionado: PID {pid} ({name}) - CPU {cpu}, RAM {mem}");
        });

        // 4. Alternância de Tema
        let mgr_theme = SettingsManager::new();
        let ui_handle_theme = main_window.as_weak();
        main_window.on_toggle_theme(move || {
            if let Some(ui) = ui_handle_theme.upgrade() {
                let dark = ui.get_dark_mode();
                let mut current = mgr_theme.load();
                current.dark_mode = dark;
                mgr_theme.save(&current);
            }
        });

        // 5. Alteração de Intervalo
        let mgr_interval = SettingsManager::new();
        let interval_state = Arc::clone(&cb_shared);
        let interval_cv = Arc::clone(&cb_condvar);
        main_window.on_set_interval(move |secs| {
            let s = secs as u64;
            if let Ok(mut lock) = interval_state.lock() {
                lock.interval_secs = s;
            }
            let mut current = mgr_interval.load();
            current.update_interval_secs = s;
            mgr_interval.save(&current);
            interval_cv.notify_one();
        });

        // 6. Microanimações
        let mgr_anim = SettingsManager::new();
        let ui_handle_anim = main_window.as_weak();
        main_window.on_toggle_animations(move || {
            if let Some(ui) = ui_handle_anim.upgrade() {
                let enabled = ui.get_animations_enabled();
                let mut current = mgr_anim.load();
                current.animations_enabled = enabled;
                mgr_anim.save(&current);
            }
        });

        // 7. Pausa / Retomada
        let mgr_pause = SettingsManager::new();
        let pause_state = Arc::clone(&cb_shared);
        let pause_cv = Arc::clone(&cb_condvar);
        let ui_handle_pause = main_window.as_weak();
        main_window.on_toggle_pause(move || {
            if let Some(ui) = ui_handle_pause.upgrade() {
                let paused = ui.get_is_paused();
                if let Ok(mut lock) = pause_state.lock() {
                    lock.is_paused = paused;
                }
                let mut current = mgr_pause.load();
                current.is_paused = paused;
                mgr_pause.save(&current);
                pause_cv.notify_one();
            }
        });

        // 8. Restaurar Padrões
        let mgr_restore = SettingsManager::new();
        let restore_state = Arc::clone(&cb_shared);
        let restore_cv = Arc::clone(&cb_condvar);
        let ui_handle_restore = main_window.as_weak();
        main_window.on_restore_defaults(move || {
            let defaults = AppSettings::default();
            mgr_restore.save(&defaults);
            if let Ok(mut lock) = restore_state.lock() {
                lock.interval_secs = defaults.update_interval_secs;
                lock.is_paused = defaults.is_paused;
            }
            if let Some(ui) = ui_handle_restore.upgrade() {
                ui.set_dark_mode(defaults.dark_mode);
                ui.set_update_interval_secs(defaults.update_interval_secs as i32);
                ui.set_animations_enabled(defaults.animations_enabled);
                ui.set_is_paused(defaults.is_paused);
            }
            restore_cv.notify_one();
        });

        info!("Janela do Moon Monitor exibida. Entrando no loop principal do Slint...");
        main_window.run()?;

        // Sinaliza o encerramento gracioso para a thread de coleta
        is_running.store(false, Ordering::Relaxed);
        info!("Moon Monitor encerrado com sucesso.");
        Ok(())
    }

    /// Atualiza os modelos declarativos da interface Slint a partir do snapshot
    fn apply_snapshot_to_ui(ui: &MainWindow, snap: TelemetrySnapshot) {
        // 1. Identificação do Sistema
        ui.set_hostname(snap.system_info.host_name.into());
        ui.set_os_version(
            format!(
                "{} {}",
                snap.system_info.os_name, snap.system_info.os_version
            )
            .into(),
        );
        ui.set_architecture(snap.system_info.architecture.into());
        ui.set_uptime(format_uptime(snap.system_info.uptime_seconds).into());

        // 2. CPU
        ui.set_cpu_name(snap.cpu.brand_name.into());
        ui.set_cpu_vendor(snap.cpu.vendor_id.into());
        ui.set_cpu_usage(snap.cpu.total_usage_percent);
        ui.set_cpu_physical_cores(snap.cpu.physical_core_count as i32);
        ui.set_cpu_logical_cores(snap.cpu.logical_core_count as i32);
        ui.set_cpu_frequency(format_frequency(snap.cpu.frequency_mhz).into());
        let temp_str = snap
            .cpu
            .temperature_celsius
            .map(|t| format!("{:.1} °C", t))
            .unwrap_or_else(|| "Não disponível".to_string());
        ui.set_cpu_temperature(temp_str.into());

        // Histórico de CPU
        let cpu_hist_model = Rc::new(VecModel::from(snap.cpu.history));
        ui.set_cpu_history(ModelRc::from(cpu_hist_model));

        // Núcleos individuais
        let core_items: Vec<CoreUsage> = snap
            .cpu
            .cores
            .into_iter()
            .map(|c| CoreUsage {
                core_index: c.index as i32,
                usage: c.usage_percent,
                frequency_mhz: c.frequency_mhz as i32,
            })
            .collect();
        let cores_model = Rc::new(VecModel::from(core_items));
        ui.set_cpu_cores(ModelRc::from(cores_model));

        // 3. RAM
        ui.set_ram_total(format_bytes_gib(snap.memory.total_bytes).into());
        ui.set_ram_used(format_bytes_gib(snap.memory.used_bytes).into());
        ui.set_ram_available(format_bytes_gib(snap.memory.available_bytes).into());
        ui.set_ram_free(format_bytes_gib(snap.memory.free_bytes).into());
        ui.set_ram_usage_percent(snap.memory.usage_percent);
        ui.set_swap_total(format_bytes_gib(snap.memory.swap_total_bytes).into());
        ui.set_swap_used(format_bytes_gib(snap.memory.swap_used_bytes).into());
        ui.set_swap_usage_percent(snap.memory.swap_usage_percent);

        let ram_hist_model = Rc::new(VecModel::from(snap.memory.history));
        ui.set_ram_history(ModelRc::from(ram_hist_model));

        let top_ram_items: Vec<TopProcessRam> = snap
            .memory
            .top_consumers
            .into_iter()
            .map(|p| TopProcessRam {
                pid: p.pid as i32,
                name: p.name.into(),
                memory_formatted: format_bytes_gib(p.memory_bytes).into(),
                memory_percent: p.memory_percent,
            })
            .collect();
        let top_ram_model = Rc::new(VecModel::from(top_ram_items));
        ui.set_top_memory_processes(ModelRc::from(top_ram_model));

        // 4. GPU
        let has_gpu = !snap.gpu.adapters.is_empty();
        ui.set_has_gpu_detected(has_gpu);

        if let Some(first_gpu) = snap.gpu.adapters.first() {
            ui.set_gpu_name(first_gpu.name.clone().into());
            ui.set_gpu_vram(format_bytes_gib(first_gpu.dedicated_vram_bytes).into());
            ui.set_gpu_usage(
                first_gpu
                    .utilization_percent
                    .map(|u| format!("{:.0}%", u))
                    .unwrap_or_else(|| "Não disponível".to_string())
                    .into(),
            );
            ui.set_gpu_temperature(
                first_gpu
                    .temperature_celsius
                    .map(|t| format!("{:.0} °C", t))
                    .unwrap_or_else(|| "Não disponível".to_string())
                    .into(),
            );
        } else {
            ui.set_gpu_name("Não disponível".into());
            ui.set_gpu_vram("Não disponível".into());
            ui.set_gpu_usage("Não disponível".into());
            ui.set_gpu_temperature("Não disponível".into());
        }

        let gpu_adapter_items: Vec<GpuAdapterInfo> = snap
            .gpu
            .adapters
            .into_iter()
            .map(|a| GpuAdapterInfo {
                index: a.index as i32,
                name: a.name.into(),
                vendor: a.vendor.into(),
                dedicated_vram: format_bytes_gib(a.dedicated_vram_bytes).into(),
                shared_vram: format_bytes_gib(a.shared_vram_bytes).into(),
                utilization: a
                    .utilization_percent
                    .map(|u| format!("{:.0}%", u))
                    .unwrap_or_else(|| "Não disponível".to_string())
                    .into(),
                temperature: a
                    .temperature_celsius
                    .map(|t| format!("{:.0} °C", t))
                    .unwrap_or_else(|| "Não disponível".to_string())
                    .into(),
                clock: a
                    .clock_mhz
                    .map(|c| format!("{} MHz", c))
                    .unwrap_or_else(|| "Não disponível".to_string())
                    .into(),
            })
            .collect();
        let gpu_adapters_model = Rc::new(VecModel::from(gpu_adapter_items));
        ui.set_gpu_adapters(ModelRc::from(gpu_adapters_model));

        // 5. Armazenamento
        let storage_items: Vec<StorageItem> = snap
            .storage
            .volumes
            .into_iter()
            .map(|v| StorageItem {
                name: v.name.into(),
                mount_point: v.mount_point.into(),
                fs_type: v.file_system.into(),
                total_gb: format_storage_bytes(v.total_space_bytes).into(),
                used_gb: format_storage_bytes(v.used_space_bytes).into(),
                free_gb: format_storage_bytes(v.available_space_bytes).into(),
                usage_percent: v.usage_percent,
            })
            .collect();
        let storage_model = Rc::new(VecModel::from(storage_items));
        ui.set_storage_items(ModelRc::from(storage_model));

        // 6. Processos
        ui.set_total_process_count(snap.total_process_count as i32);
        let process_items: Vec<ProcessItem> = snap
            .processes
            .into_iter()
            .map(|p| ProcessItem {
                pid: p.pid as i32,
                name: p.name.into(),
                cpu_percent: p.cpu_usage_percent,
                cpu_formatted: format!("{:.1}%", p.cpu_usage_percent).into(),
                memory_formatted: format_bytes_gib(p.memory_bytes).into(),
                memory_bytes: p.memory_bytes as i32,
            })
            .collect();
        let proc_model = Rc::new(VecModel::from(process_items));
        ui.set_processes(ModelRc::from(proc_model));

        // 7. Auto-monitoramento de telemetria
        ui.set_self_cpu_usage(format!("{:.1}%", snap.self_cpu_percent).into());
        ui.set_self_ram_usage(format_bytes_gib(snap.self_memory_bytes).into());
    }
}
