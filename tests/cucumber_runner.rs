//! Cucumber BDD Test Runner
//!
//! Run with: cargo test --test cucumber_runner
//!
//! References:
//! - [testing_philosophy](../.agent/skills/testing_philosophy/SKILL.md)
//! - [gherkin_style_guide](../.agent/skills/gherkin_style_guide/SKILL.md)

use cucumber::World;

mod steps;

// Import step modules to register them with Cucumber
// The `#[given]`, `#[when]`, `#[then]` attributes register steps at compile time
#[allow(unused_imports)]
use steps::{given_api_steps, given_steps, then_api_steps, then_steps, when_api_steps, when_steps};

use steps::world::DashboardWorld;

#[tokio::main]
async fn main() {
    // 🛡️ Phase 1: Anti-Masquerading Reliability Audit
    // This ensures no UI steps are "masquerading" as API calls without DOM verification
    println!("🧪 Running Reliability Audit (Anti-Masquerading)...");
    let audit_status = std::process::Command::new("cargo")
        .args([
            "run",
            "--manifest-path",
            ".agent/tools/Cargo.toml",
            "--bin",
            "audit_masquerading",
            "--quiet",
        ])
        .status()
        .expect("Failed to execute reliability audit tool");

    if !audit_status.success() {
        // Findings are printed by the audit tool itself to stdout/stderr
        std::process::exit(1);
    }

    // 🚀 Phase 2: Cucumber Feature Execution
    DashboardWorld::cucumber()
        .max_concurrent_scenarios(1) // 🛡️ Prevent interference in shared localhost environment
        .run_and_exit("tests/features")
        .await;
}
