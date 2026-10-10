use sysinfo::{Pid, System};

pub struct SelfMonitor {
    pid: u32,
}

impl SelfMonitor {
    pub fn new() -> Self {
        Self {
            pid: std::process::id(),
        }
    }

    /// Retorna (cpu_usage_percent, memory_bytes) do próprio processo do Moon Monitor.
    pub fn collect(&self, sys: &System) -> (f32, u64) {
        let sys_pid = Pid::from_u32(self.pid);
        if let Some(proc_info) = sys.process(sys_pid) {
            (proc_info.cpu_usage(), proc_info.memory())
        } else {
            (0.0, 0)
        }
    }
}
