---
name: Testing and Verification
description: Guidelines for Behavior Driven Development (BDD) using Gherkin and Cucumber.
---

# Testing Strategy: BDD

The software behavior is described and verified using **Behavior Driven Development (BDD)**.

## 1. Core Philosophy
*   **Tests are Specifications**: The "truth" of what the software does is defined in `.feature` files, not in the code features themselves.
*   **Language**: Gherkin (Given/When/Then).
*   **Tooling**: `cucumber` crate (Rust).

## 2. Directory Structure
```
tests/
├── features/           <-- The executable specifications
│   ├── archive_processing.feature
│   └── model_metadata.feature
├── steps/              <-- Rust "Glue" code
│   ├── mod.rs
│   └── archive_steps.rs
└── integration.rs      <-- Main Cucumber test runner
```

## 3. Workflow
1.  **Describe**: Write a Scenario in a `.feature` file.
    *   *Example*: "Given a ZIP file, When it is processed, Then it should extract 3 models."
2.  **Fail**: Run `cargo test` and watch it fail (undefined steps).
3.  **Implement**: Write the Step Definitions in Rust.
4.  **Pass**: Fix application logic until the test passes.

## 4. Hardware Tiers (Context)
*   **Standard Tests**: Run in Docker/CI on generic hardware (Tier 2).
*   **Hardware Verification**: As per `workshop_log.md`, specific hardware testing (Tier 1/3) is deferred until v1.0. BDD tests should generally be hardware-agnostic where possible.
