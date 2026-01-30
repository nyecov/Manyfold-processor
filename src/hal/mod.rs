//! Hardware Abstraction Layer (HAL)
//!
//! Governance: .agent/skills/architectural_guidelines/SKILL.md
//!
//! This module defines abstract traits for hardware-specific operations,
//! enabling Tier 1 (Radxa Rock 5) logic to be simulated on Tier 2 (Generic) hardware.

// HAL traits are scaffolding - allow dead code until integrated with processing logic
#![allow(dead_code)]

mod image_processor;
mod inference_engine;

// Re-exports are for future integration - allow unused for now
#[allow(unused_imports)]
pub use image_processor::{CpuImageProcessor, ImageProcessor};
#[allow(unused_imports)]
pub use inference_engine::{CpuInferenceEngine, InferenceEngine};

#[cfg(feature = "mock-hardware")]
pub use image_processor::MockRgaProcessor;
#[cfg(feature = "mock-hardware")]
pub use inference_engine::MockNpuEngine;

/// Selects the appropriate HAL implementation based on environment.
pub fn select_hal() -> (Box<dyn ImageProcessor>, Box<dyn InferenceEngine>) {
    #[cfg(feature = "mock-hardware")]
    {
        log::info!("HAL: Using MOCK hardware implementations (Tier 1 Simulation)");
        (
            Box::new(MockRgaProcessor::new()),
            Box::new(MockNpuEngine::new()),
        )
    }
    #[cfg(not(feature = "mock-hardware"))]
    {
        log::info!("HAL: Using CPU fallback implementations (Tier 2/3)");
        (
            Box::new(CpuImageProcessor::new()),
            Box::new(CpuInferenceEngine::new()),
        )
    }
}
