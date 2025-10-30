//! Omniscient Shell - AI-native companion shell extending PowerShell
//! 
//! Phase 1: Core + TUI implementation

use anyhow::Result;
use tracing::{info, warn};
use tracing_subscriber;

mod utils;
mod shell;
mod tui;
mod graphics;

use crate::utils::config::{Config, load_config};
use crate::tui::dashboard::Dashboard;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into())
        )
        .init();

    info!("Omniscient Shell v0.1.0 starting...");

    // Load configuration
    let config = match load_config() {
        Ok(cfg) => {
            info!("Configuration loaded successfully");
            cfg
        }
        Err(e) => {
            warn!("Failed to load config, using defaults: {}", e);
            Config::default()
        }
    };

    // Validate schema version
    if config.version != "0.1" {
        anyhow::bail!("Unsupported config version: {}. Expected 0.1. Please update your config file.", config.version);
    }

    // Initialize graphics backend
    let graphics_backend = graphics::negotiate_backend(&config.graphics)?;
    info!("Graphics backend selected: {:?}", graphics_backend.backend_type());

    // Initialize PowerShell integration
    let mut shell_integration = shell::PowerShellIntegration::new()?;
    info!("PowerShell integration initialized");

    // Create and run dashboard
    let mut dashboard = Dashboard::new(config, graphics_backend, shell_integration)?;
    info!("Dashboard initialized, starting main loop...");
    
    dashboard.run().await?;

    info!("Omniscient Shell shutting down");
    Ok(())
}
