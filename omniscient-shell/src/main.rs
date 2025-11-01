//! Omniscient Shell - AI-native companion shell extending PowerShell
//!
//! Phase 1: Core + TUI implementation

use anyhow::Result;
use clap::Parser;
use tracing::{info, warn};

mod graphics;
mod notifications;
mod platform;
mod shell;
mod state;
mod tui;
mod utils;
mod workspace;

// Media features (optional)
#[cfg(feature = "media")]
mod media;

// Conditionally compile OAuth module based on omniscience feature
#[cfg(feature = "omniscience")]
mod agents;
#[cfg(feature = "omniscience")]
mod oauth;
#[cfg(feature = "omniscience")]
mod security;

// Use shim when omniscience is disabled
#[cfg(not(feature = "omniscience"))]
mod oauth_shim;
#[cfg(not(feature = "omniscience"))]
use oauth_shim as oauth;

use crate::tui::dashboard::Dashboard;
use crate::utils::config::{load_config, Config};

/// Omniscient Shell - AI-native companion shell extending PowerShell
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Skip omniscience initialization even if the feature is compiled in
    #[arg(long)]
    no_omniscience: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Parse command line arguments
    let args = Args::parse();

    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    info!("Omniscient Shell v0.1.0 starting...");

    // Log omniscience status
    #[cfg(feature = "omniscience")]
    {
        if args.no_omniscience {
            info!("Omniscience features disabled via --no-omniscience flag");
            info!("To enable omniscience: remove --no-omniscience flag");
        } else {
            info!("Omniscience features enabled");
        }
    }

    #[cfg(not(feature = "omniscience"))]
    {
        info!("Omniscience features not available (feature not compiled)");
        info!("To enable omniscience: build with --features omniscience");
        if args.no_omniscience {
            warn!("--no-omniscience flag is redundant when omniscience feature is not compiled");
        }
    }

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
        anyhow::bail!(
            "Unsupported config version: {}. Expected 0.1. Please update your config file.",
            config.version
        );
    }

    // Initialize graphics backend
    let graphics_backend = graphics::negotiate_backend(&config.graphics)?;
    info!(
        "Graphics backend selected: {:?}",
        graphics_backend.backend_type()
    );

    // Initialize PowerShell integration
    let shell_integration = shell::PowerShellIntegration::new()?;
    info!("PowerShell integration initialized");

    // Create and run dashboard
    let mut dashboard = Dashboard::new(config, graphics_backend, shell_integration)?;
    info!("Dashboard initialized, starting main loop...");

    dashboard.run().await?;

    info!("Omniscient Shell shutting down");
    Ok(())
}
