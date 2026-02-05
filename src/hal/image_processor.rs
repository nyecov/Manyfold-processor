//! Image Processing HAL Trait
//!
//! Governance: .agent/skills/deploy_on_radxa_rock5/SKILL.md (RGA Optimization)

use std::path::Path;
use tracing::{debug, instrument};

/// Abstract trait for image processing operations.
/// Implementations: CpuImageProcessor (Tier 2/3), MockRgaProcessor (Tier 1 Sim), RgaProcessor (Tier 1 Real)
pub trait ImageProcessor: Send + Sync {
    /// Resize an image to the specified dimensions.
    fn resize(&self, input: &Path, output: &Path, width: u32, height: u32) -> anyhow::Result<()>;

    /// Convert image format (e.g., JPG -> WebP).
    fn convert(&self, input: &Path, output: &Path, format: &str) -> anyhow::Result<()>;
}

/// CPU-based image processor (Tier 2/3 Fallback).
pub struct CpuImageProcessor;

impl CpuImageProcessor {
    pub fn new() -> Self {
        Self
    }
}

impl ImageProcessor for CpuImageProcessor {
    #[instrument(skip(self))]
    fn resize(&self, input: &Path, output: &Path, width: u32, height: u32) -> anyhow::Result<()> {
        debug!(
            "CPU ImageProcessor: Resizing {:?} to {}x{}",
            input,
            width,
            height
        );
        let img = image::open(input)?;
        let resized = img.resize(width, height, image::imageops::FilterType::Lanczos3);
        resized.save(output)?;
        Ok(())
    }

    #[instrument(skip(self))]
    fn convert(&self, input: &Path, output: &Path, format: &str) -> anyhow::Result<()> {
        debug!("CPU ImageProcessor: Converting {:?} to {}", input, format);
        let img = image::open(input)?;
        // Simple conversion by saving with the target extension/format
        img.save(output)?;
        Ok(())
    }
}

/// Mock RGA processor for Tier 1 simulation on development hardware.
#[cfg(feature = "mock-hardware")]
pub struct MockRgaProcessor;

#[cfg(feature = "mock-hardware")]
impl MockRgaProcessor {
    pub fn new() -> Self {
        log::info!("MockRgaProcessor initialized (Tier 1 Simulation)");
        Self
    }
}

#[cfg(feature = "mock-hardware")]
impl ImageProcessor for MockRgaProcessor {
    fn resize(&self, input: &Path, output: &Path, width: u32, height: u32) -> anyhow::Result<()> {
        log::info!(
            "MOCK RGA: resize({:?}, {}x{}) -> {:?}",
            input,
            width,
            height,
            output
        );
        // Simulate the operation without actual RGA calls
        std::fs::copy(input, output)?;
        Ok(())
    }

    fn convert(&self, input: &Path, output: &Path, format: &str) -> anyhow::Result<()> {
        log::info!("MOCK RGA: convert({:?}, {}) -> {:?}", input, format, output);
        // Simulate the operation without actual RGA calls
        std::fs::copy(input, output)?;
        Ok(())
    }
}
