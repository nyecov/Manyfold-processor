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
*   **Location**: `tests/steps/`
*   **Standard (UI) Layer**:
    *   `given_steps.rs`: All `#[given]` UI steps.
    *   `when_steps.rs`: All `#[when]` UI steps.
    *   `then_steps.rs`: All `#[then]` UI steps.
*   **API Layer**:
    *   `given_api_steps.rs`: All `#[given]` API steps.
    *   `when_api_steps.rs`: All `#[when]` API steps.
    *   `then_api_steps.rs`: All `#[then]` API steps.

> [!IMPORTANT]
> A step MUST be placed in its designated file even if it does not currently have a Twin.

## 5. IDE Integration (VS Code)
For a mature testing experience, the project is configured for native VS Code execution.

### Mandatory Extensions:
*   **rust-analyzer**: For Rust intelligence and Codelens.
*   **CodeLLDB**: Required for debugging Gherkin steps.
*   **Cucumber (Gherkin) Full Support**: For syntax highlighting and step navigation.

### VS Code Plugin Configuration (`.vscode/settings.json`):
```json
{
    "cucumberautocomplete.steps": ["tests/steps/**/*.rs"],
    "cucumberautocomplete.syncfeatures": "tests/features/*.feature",
    "cucumberautocomplete.strictGherkinCompletion": true
}
```

### Running & Debugging:
*   **Run All**: Execute `cargo test --test cucumber_runner`.
*   **Selective Run (IDE)**: Use **Codelens** (Run/Debug) directly above scenarios in `.feature` files.
*   **Debugger**: Use the `Debug Cucumber` configurations in the **Run and Debug** side panel.

## 6. Tooling & Workflows
*   **Crate**: `cucumber` (Rust).
*   **Pre-Hook**: The test runner executes an automatic anti-masquerading audit before every run.
*   **Validation**:
    *   `/audit_gherkin`: Verifies Gherkin semantics.
    *   `/audit_step_glue`: Verifies Rust-to-Gherkin mapping and file placement.
    *   `/audit_masquerading`: Verifies UI steps don't bypass the UI layer.

---

## See Also
*   **Standards**: [gherkin_style_guide](../gherkin_style_guide/SKILL.md)
*   **Governance**: [testing_philosophy](../testing_philosophy/SKILL.md)
