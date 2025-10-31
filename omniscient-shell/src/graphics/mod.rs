//! Graphics backend negotiation and abstraction

pub mod backend;
pub mod notcurses_backend;
pub mod kitty_backend;
pub mod overlay_backend;

use anyhow::Result;
use crate::utils::config::GraphicsConfig;
pub use backend::{GraphicsBackend, BackendType, Capabilities, Region};

/// Negotiate and initialize the best available graphics backend
pub fn negotiate_backend(config: &GraphicsConfig) -> Result<Box<dyn GraphicsBackend>> {
    let mut backends_to_try = vec![config.preferred.as_str()];
    backends_to_try.extend(config.fallback.iter().map(|s| s.as_str()));

    for backend_name in backends_to_try {
        match try_backend(backend_name, config) {
            Ok(backend) => return Ok(backend),
            Err(e) => {
                tracing::warn!("Failed to initialize {} backend: {}", backend_name, e);
                continue;
            }
        }
    }

    // Final fallback to overlay
    tracing::warn!("All preferred backends failed, falling back to overlay");
    try_backend("overlay", config)
}

fn try_backend(name: &str, config: &GraphicsConfig) -> Result<Box<dyn GraphicsBackend>> {
    match name {
        "notcurses" => {
            #[cfg(feature = "notcurses")]
            {
                Ok(Box::new(notcurses_backend::NotcursesBackend::new()?))
            }
            #[cfg(not(feature = "notcurses"))]
            {
                anyhow::bail!("Notcurses support not compiled in")
            }
        }
        "kitty" => Ok(Box::new(kitty_backend::KittyBackend::new()?)),
        "overlay" => {
            #[cfg(feature = "overlay")]
            {
                Ok(Box::new(overlay_backend::OverlayBackend::new()?))
            }
            #[cfg(not(feature = "overlay"))]
            {
                // Fallback to basic terminal rendering
                Ok(Box::new(overlay_backend::OverlayBackend::new()?))
            }
        }
        _ => anyhow::bail!("Unknown graphics backend: {}", name),
    }
}
