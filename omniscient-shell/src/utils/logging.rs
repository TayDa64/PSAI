//! Logging utilities

use tracing::{info, warn, error};

pub fn log_startup(version: &str) {
    info!("Omniscient Shell v{} starting", version);
}

pub fn log_config_loaded(path: &str) {
    info!("Configuration loaded from: {}", path);
}

pub fn log_graphics_selected(backend: &str) {
    info!("Graphics backend selected: {}", backend);
}

pub fn log_error_with_hint(error: &str, hint: Option<&str>) {
    error!("{}", error);
    if let Some(h) = hint {
        warn!("Recovery hint: {}", h);
    }
}
