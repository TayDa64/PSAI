//! FFmpeg integration (stub for Phase 4)

use anyhow::Result;
use std::path::Path;

/// FFmpeg wrapper for media processing
pub struct FFmpegProcessor {
    // Configuration
}

impl FFmpegProcessor {
    pub fn new() -> Self {
        FFmpegProcessor {}
    }

    /// Generate thumbnail
    pub async fn generate_thumbnail(
        &self,
        _input: &Path,
        _output: &Path,
        _width: u32,
        _height: u32,
    ) -> Result<()> {
        #[cfg(feature = "media")]
        {
            // Real implementation would use ffmpeg-next
            tracing::info!("Generating thumbnail (stub)");
            Ok(())
        }
        #[cfg(not(feature = "media"))]
        {
            anyhow::bail!("Media support not compiled in. Enable the 'media' feature.")
        }
    }

    /// Extract video frame
    pub async fn extract_frame(
        &self,
        _input: &Path,
        _output: &Path,
        _timestamp_s: f64,
    ) -> Result<()> {
        tracing::info!("Extracting frame (stub)");
        Ok(())
    }

    /// Generate waveform
    pub async fn generate_waveform(&self, _input: &Path, _output: &Path) -> Result<()> {
        tracing::info!("Generating waveform (stub)");
        Ok(())
    }
}

impl Default for FFmpegProcessor {
    fn default() -> Self {
        Self::new()
    }
}
