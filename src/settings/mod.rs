use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tracing::{info, warn};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub dark_mode: bool,
    pub update_interval_secs: u64,
    pub animations_enabled: bool,
    pub is_paused: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            dark_mode: true,
            update_interval_secs: 2,
            animations_enabled: true,
            is_paused: false,
        }
    }
}

pub struct SettingsManager {
    file_path: PathBuf,
}

impl SettingsManager {
    pub fn new() -> Self {
        let file_path = Self::resolve_settings_path();
        Self { file_path }
    }

    /// Retorna o caminho do arquivo de configuração do usuário no Windows (%APPDATA%/moon-monitor/settings.json).
    pub fn resolve_settings_path() -> PathBuf {
        if let Some(proj_dirs) = ProjectDirs::from("com", "moon-monitor", "MoonMonitor") {
            let config_dir = proj_dirs.config_dir();
            config_dir.join("settings.json")
        } else {
            PathBuf::from("settings.json")
        }
    }

    pub fn get_path_display(&self) -> String {
        self.file_path.to_string_lossy().to_string()
    }

    /// Carrega as configurações do disco com fallback resiliente para valores padrão.
    pub fn load(&self) -> AppSettings {
        if !self.file_path.exists() {
            info!("Arquivo de configurações não encontrado. Criando padrões em {:?}", self.file_path);
            let default_settings = AppSettings::default();
            self.save(&default_settings);
            return default_settings;
        }

        match fs::read_to_string(&self.file_path) {
            Ok(content) => match serde_json::from_str::<AppSettings>(&content) {
                Ok(mut settings) => {
                    // Valida que o intervalo é suportado (1, 2 ou 5)
                    if settings.update_interval_secs != 1
                        && settings.update_interval_secs != 2
                        && settings.update_interval_secs != 5
                    {
                        settings.update_interval_secs = 2;
                    }
                    settings
                }
                Err(err) => {
                    warn!("Configurações corrompidas ({err}). Restaurando padrões.");
                    AppSettings::default()
                }
            },
            Err(err) => {
                warn!("Erro ao ler configurações ({err}). Utilizando padrões.");
                AppSettings::default()
            }
        }
    }

    /// Salva as configurações de forma atômica no diretório do usuário.
    pub fn save(&self, settings: &AppSettings) {
        if let Some(parent) = self.file_path.parent() {
            if let Err(e) = fs::create_dir_all(parent) {
                warn!("Não foi possível criar diretório de configurações: {e}");
            }
        }

        match serde_json::to_string_pretty(settings) {
            Ok(json) => {
                if let Err(e) = fs::write(&self.file_path, json) {
                    warn!("Erro ao salvar arquivo de configurações: {e}");
                } else {
                    info!("Configurações salvas em {:?}", self.file_path);
                }
            }
            Err(e) => {
                warn!("Falha ao serializar configurações: {e}");
            }
        }
    }
}
