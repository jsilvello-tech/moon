use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LunarPhase {
    Crescent, // < 30%
    Gibbous,  // 30% a 69%
    Full,     // 70% a 89%
    Eclipse,  // >= 90%
}

impl LunarPhase {
    /// Determina a fase lunar com base na utilização percentual da CPU.
    pub fn from_cpu_usage(cpu_usage: f32) -> Self {
        if cpu_usage >= 90.0 {
            LunarPhase::Eclipse
        } else if cpu_usage >= 70.0 {
            LunarPhase::Full
        } else if cpu_usage >= 30.0 {
            LunarPhase::Gibbous
        } else {
            LunarPhase::Crescent
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            LunarPhase::Crescent => "Lua Crescente (Carga Leve)",
            LunarPhase::Gibbous => "Lua Gibosa (Carga Moderada)",
            LunarPhase::Full => "Lua Cheia (Carga Elevada)",
            LunarPhase::Eclipse => "Eclipse (Carga Crítica)",
        }
    }

    pub fn phase_code(&self) -> i32 {
        match self {
            LunarPhase::Crescent => 0,
            LunarPhase::Gibbous => 1,
            LunarPhase::Full => 2,
            LunarPhase::Eclipse => 3,
        }
    }
}
