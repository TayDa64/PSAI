//! Graphics backend trait and types

use anyhow::Result;

/// Backend type identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackendType {
    Notcurses,
    Kitty,
    Overlay,
}

/// Graphics capabilities
#[derive(Debug, Clone)]
pub struct Capabilities {
    pub max_width: u32,
    pub max_height: u32,
    pub color_depth: u8,
    pub supports_transparency: bool,
    pub supports_animation: bool,
    pub effective_resolution: f32, // Benchmark score
    pub latency_ms: f32,
}

impl Default for Capabilities {
    fn default() -> Self {
        Capabilities {
            max_width: 1920,
            max_height: 1080,
            color_depth: 24,
            supports_transparency: false,
            supports_animation: false,
            effective_resolution: 1.0,
            latency_ms: 10.0,
        }
    }
}

/// Screen region for rendering
#[derive(Debug, Clone)]
pub struct Region {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

/// Graphics backend trait
pub trait GraphicsBackend: Send {
    /// Get backend type
    fn backend_type(&self) -> BackendType;

    /// Initialize backend and negotiate capabilities
    fn init(&mut self) -> Result<()>;

    /// Get supported capabilities
    fn capabilities(&self) -> Capabilities;

    /// Render an image in the specified region
    fn render_image(&mut self, region: &Region, image_data: &[u8]) -> Result<()>;

    /// Render a video frame in the specified region
    fn render_video_frame(&mut self, region: &Region, frame_data: &[u8]) -> Result<()>;

    /// Clear the specified region
    fn clear_region(&mut self, region: &Region) -> Result<()>;

    /// Check if the backend supports the given resolution
    fn supports_resolution(&self, width: u32, height: u32) -> bool;

    /// Benchmark the backend (returns effective resolution score)
    fn benchmark(&mut self) -> Result<f32>;
}
