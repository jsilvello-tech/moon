#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod models;
mod monitoring;
mod settings;
mod utils;

use app::MoonMonitorApp;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Inicializa sistema de logs estruturados (tracing)
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,moon_monitor=debug"));

    tracing_subscriber::registry()
        .with(env_filter)
        .with(tracing_subscriber::fmt::layer().with_target(false))
        .init();

    info!("=== MOON MONITOR v0.1.0 ===");
    info!("Slogan: Keep your system in orbit.");
    info!("Iniciando aplicação nativa para Windows...");

    let mut app = MoonMonitorApp::new();
    if let Err(err) = app.run() {
        eprintln!("Erro crítico na execução do Moon Monitor: {err}");
        std::process::exit(1);
    }

    Ok(())
}
