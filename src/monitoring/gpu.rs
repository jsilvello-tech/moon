use crate::models::gpu::{GpuAdapter, GpuMetrics};

pub struct GpuCollector;

impl GpuCollector {
    pub fn new() -> Self {
        Self
    }

    #[cfg(windows)]
    pub fn collect(&self) -> GpuMetrics {
        use windows::Win32::Graphics::Dxgi::{CreateDXGIFactory1, IDXGIFactory1};

        let mut adapters = Vec::new();

        unsafe {
            // Tenta instanciar a factory DXGI
            if let Ok(factory) = CreateDXGIFactory1::<IDXGIFactory1>() {
                let mut index = 0;
                while let Ok(adapter) = factory.EnumAdapters1(index) {
                    if let Ok(desc) = adapter.GetDesc1() {
                        // Converte o nome UTF-16 do adaptador
                        let name_len = desc
                            .Description
                            .iter()
                            .position(|&c| c == 0)
                            .unwrap_or(desc.Description.len());
                        let name = String::from_utf16_lossy(&desc.Description[..name_len])
                            .trim()
                            .to_string();

                        // Identifica fabricante pelo VendorId PCI
                        let vendor = match desc.VendorId {
                            0x10DE => "NVIDIA Corporation",
                            0x1002 => "Advanced Micro Devices, Inc. (AMD)",
                            0x8086 => "Intel Corporation",
                            0x1414 => "Microsoft Basic Render Driver",
                            0x5143 => "Qualcomm Technologies, Inc.",
                            _ => "Adaptador Gráfico",
                        }
                        .to_string();

                        // Ignora adaptadores de renderização puramente por software se houver outros
                        let is_software = desc.VendorId == 0x1414;

                        // Se não for puramente software ou se for o único até o momento
                        if !is_software || adapters.is_empty() {
                            adapters.push(GpuAdapter {
                                index: index as usize + 1,
                                name,
                                vendor,
                                dedicated_vram_bytes: desc.DedicatedVideoMemory as u64,
                                shared_vram_bytes: desc.SharedSystemMemory as u64,
                                utilization_percent: None, // Exige driver proprietário ou PDH
                                temperature_celsius: None,
                                clock_mhz: None,
                            });
                        }
                    }
                    index += 1;
                }
            }
        }

        GpuMetrics { adapters }
    }

    #[cfg(not(windows))]
    pub fn collect(&self) -> GpuMetrics {
        GpuMetrics {
            adapters: Vec::new(),
        }
    }
}
