---
name: Cucumber Rust Reference
description: Technical implementation details for using the Cucumber crate in Rust.
---

# Cucumber & Rust: Implementation Reference

This skill provides the technical details for implementing the "glue" code that connects Gherkin steps to Rust logic.

## 1. Context & State: The `World`
The `World` struct holds state for a scenario and is reset after each run.

```rust
use cucumber::World;
use std::convert::Infallible;

#[derive(Debug, World, Default)]
pub struct AppWorld {
    pub response_code: u16,
}

impl cucumber::WorldInit for AppWorld {
    type Error = Infallible;
}
```

## 2. Step Definitions
Use attributes to map Gherkin sentences to Rust functions.

```rust
use cucumber::{given, when, then};

// [twin: Given_API the service is running]
#[given("the service is running")]
async fn service_up(world: &mut AppWorld) {
    // Rust logic...
}
```

## 3. Twin Linking in Rust
Cross-link corresponding steps for AI traceability.
*   **Format**: Use `// [twin: Step Name]` above the step attribute.

## 4. File Organization
To ensure modularity and ease of lookup, step definitions MUST be organized into separate files based on their keyword and layer.

### Mandated Structure:
*   **Location**: `tests/Testing/steps/`
*   **Standard (UI) Layer**:
    *   `Given_steps.rs`: All `#[given]` UI steps.
    *   `When_steps.rs`: All `#[when]` UI steps.
    *   `Then_steps.rs`: All `#[then]` UI steps.
*   **API Layer**:
    *   `Given_API_steps.rs`: All `#[given]` API steps.
    *   `When_API_steps.rs`: All `#[when]` API steps.
    *   `Then_API_steps.rs`: All `#[then]` API steps.

> [!IMPORTANT]
> A step MUST be placed in its designated file even if it does not currently have a Twin.

## 5. Tooling & Workflows
*   **Crate**: `cucumber` (Rust).
*   **Validation**:
    *   `/audit_gherkin`: Verifies Gherkin semantics.
    *   `/audit_step_glue`: Verifies Rust-to-Gherkin mapping and file placement.

---

## See Also
*   **Standards**: [gherkin_style_guide](../gherkin_style_guide/SKILL.md)
*   **Governance**: [testing_philosophy](../testing_philosophy/SKILL.md)
