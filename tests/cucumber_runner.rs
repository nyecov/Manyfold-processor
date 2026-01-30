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
    DashboardWorld::cucumber()
        .run("tests/Testing/Features")
        .await;
}
