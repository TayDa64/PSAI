//! Overlay/fallback graphics backend using basic terminal rendering

use crate::graphics::backend::{BackendType, Capabilities, GraphicsBackend, Region};
use anyhow::Result;

pub struct OverlayBackend {
    capabilities: Capabilities,
    initialized: bool,
}

impl OverlayBackend {
    pub fn new() -> Result<Self> {
        Ok(OverlayBackend {
            capabilities: Capabilities {
                max_width: 800,
                max_height: 600,
                color_depth: 8,
                supports_transparency: false,
                supports_animation: false,
                effective_resolution: 1.0,
                latency_ms: 5.0,
            },
            initialized: false,
        })
    }
}

impl GraphicsBackend for OverlayBackend {
    fn backend_type(&self) -> BackendType {
        BackendType::Overlay
    }

    fn init(&mut self) -> Result<()> {
        tracing::info!("Initializing overlay fallback backend");
        self.initialized = true;
        Ok(())
    }

    fn capabilities(&self) -> Capabilities {
        self.capabilities.clone()
    }

    fn render_image(&mut self, region: &Region, _image_data: &[u8]) -> Result<()> {
        tracing::debug!("Rendering ASCII art placeholder at {:?}", region);
        // Real implementation would convert image to ASCII art
        Ok(())
    }

    fn render_video_frame(&mut self, region: &Region, _frame_data: &[u8]) -> Result<()> {
        tracing::debug!("Rendering video frame as ASCII at {:?}", region);
        Ok(())
    }

    fn clear_region(&mut self, region: &Region) -> Result<()> {
        tracing::debug!("Clearing region {:?}", region);
        Ok(())
    }

    fn supports_resolution(&self, width: u32, height: u32) -> bool {
        width <= self.capabilities.max_width && height <= self.capabilities.max_height
    }

    fn benchmark(&mut self) -> Result<f32> {
        Ok(1.0)
    }
}
