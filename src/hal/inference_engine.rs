//! Inference Engine HAL Trait
//!
//! Governance: .agent/skills/deploy_on_radxa_rock5/SKILL.md (NPU Optimization)

use std::path::Path;

/// Abstract trait for AI inference operations.
/// Implementations: CpuInferenceEngine (Tier 2/3), MockNpuEngine (Tier 1 Sim), NpuEngine (Tier 1 Real)
pub trait InferenceEngine: Send + Sync {
    /// Run inference on a model with the given input.
    fn infer(&self, model: &Path, input: &[u8]) -> anyhow::Result<Vec<f32>>;
}

/// CPU-based inference engine using ONNX Runtime (Tier 2/3 Fallback).
pub struct CpuInferenceEngine;

impl CpuInferenceEngine {
    pub fn new() -> Self {
        Self
    }
}

impl InferenceEngine for CpuInferenceEngine {
    fn infer(&self, model: &Path, input: &[u8]) -> anyhow::Result<Vec<f32>> {
        log::debug!(
            "CPU InferenceEngine: Running inference on {:?} with {} bytes input",
            model,
            input.len()
        );
        // Placeholder: Use ONNX Runtime for actual implementation
        Ok(vec![0.0; 10]) // Dummy output
    }
}

/// Mock NPU engine for Tier 1 simulation on development hardware.
#[cfg(feature = "mock-hardware")]
pub struct MockNpuEngine;

#[cfg(feature = "mock-hardware")]
impl MockNpuEngine {
    pub fn new() -> Self {
        log::info!("MockNpuEngine initialized (Tier 1 Simulation)");
        Self
    }
}

#[cfg(feature = "mock-hardware")]
impl InferenceEngine for MockNpuEngine {
    fn infer(&self, model: &Path, input: &[u8]) -> anyhow::Result<Vec<f32>> {
        log::info!("MOCK NPU: infer({:?}, {} bytes)", model, input.len());
        // Simulate the operation without actual NPU calls
        Ok(vec![0.5; 10]) // Simulated output
    }
}
