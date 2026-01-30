---
name: Cucumber Gherkin Reference
description: Detailed syntax guide and implementation reference for using Cucumber with Rust.
---

# Cucumber & Gherkin Reference (Rust)

This skill provides technical details on implementing BDD tests using the `cucumber` crate.

## 1. Gherkin Syntax
Gherkin uses a set of keywords to define behavior.

*   **Feature**: High-level description of a software feature.
*   **Scenario**: A specific example/test case.
*   **Given**: The initial context (setup).
*   **When**: The event or action.
*   **Then**: The expected outcome (assertion).
*   **And/But**: Connectors.

### Example
```gherkin
Feature: Archive Processing

  Scenario: Extracting a valid ZIP
    Given I have a ZIP file named "test_model.zip" with 3 STLs
    When I process the archive
    Then I should see 3 separate model files
    And the original ZIP should be removed
```

## 2. Rust Implementation (`cucumber` crate)

### The `World` Struct
The `World` holds the state for a scenario. It is reset for each scenario.

```rust
use cucumber::World;
use std::convert::Infallible;

#[derive(Debug, World, Default)]
pub struct TestWorld {
    pub file_path: Option<String>,
    pub extracted_count: usize,
}

// Error type for the World (usually Infallible for simple tests)
impl cucumber::WorldInit for TestWorld {
    type Error = Infallible;
}
```

### Step Definitions
Use the attributes `#[given]`, `#[when]`, `#[then]` with regex or literal strings.

```rust
use cucumber::{given, when, then};

#[given(regex = r#"I have a ZIP file named "(\S+)" with (\d+) STLs"#)]
async fn create_zip(world: &mut TestWorld, name: String, count: usize) {
    world.file_path = Some(name);
    // Logic to create dummy zip...
}

#[when("I process the archive")]
async fn process_archive(world: &mut TestWorld) {
    // Call the actual application logic...
    // world.extracted_count = ...
}

#[then(regex = r#"I should see (\d+) separate model files"#)]
async fn verify_count(world: &mut TestWorld, expected: usize) {
    assert_eq!(world.extracted_count, expected);
}
```

## 3. Running Tests
You generally set up a `tests/integration.rs` entry point.

```rust
use cucumber::WorldInit;

fn main() {
    futures::executor::block_on(TestWorld::run("tests/features"));
}
```

## 4. Best Practices

### Core Principles
1.  **Behavior, Not Implementation**: Describe *what* the system does, not *how*. No UI selectors or implementation details.
2.  **One Behavior Per Scenario**: Keep scenarios focused on a single intent. Split if multiple outcomes are needed.
3.  **Business Language**: Use a consistent shared vocabulary. Avoid synonyms for the same concept.
4.  **Deterministic**: Scenarios must not depend on time, randomness, or order of execution.

### Gherkin Mechanics
5.  **Strict Keywords**:
    *   **Given**: Setup only.
    *   **When**: The single action under test.
    *   **Then**: Observable outcomes / assertions.
6.  **Step Count**: Aim for 3–7 steps. If it grows, the scenario is too complex.
7.  **No Logic in Steps**: Avoid conditional logic (`if/else`) in scenarios or step definitions.
8.  **Reusable but Specific**: Avoid generic "do everything" steps. Reuse language, not abstractions.

### Data & Structure
9.  **Explicit Data**: Use concrete values over placeholders unless generating permutations.
10. **Scenario Outlines**: Use only for true data permutations (same logic, different inputs).
11. **Avoid UI Phrasing**: Use "When I save the order" instead of "When I click Save".
12. **Assert Outcomes**: Verify results, not that a function was called.

### Maintenance
13. **Cohesion**: one feature file = one business capability. Split if >20 scenarios.
14. **Strategic Tagging**: Use tags (`@slow`, `@api`) for filtering, not for logic.
15. **No Technical Glue**: Waits, retries, and cleanups belong in the test runner, not the feature file.
16. **Living Documentation**: Review feature files like code. Delete outdated ones.

### Execution
17. **Thin Steps**: Step definitions should delegate to helper methods/services.
18. **Fast Feedback**: Most scenarios should run in seconds.
19. **Fail Loudly**: Assertion messages must explain the *business* failure.
20. **Enforce Style**: Use linters and code review to maintain Gherkin quality.

> **Rule of Thumb**: If a product owner can’t review the feature file and say “yes, that’s correct behavior”, the Gherkin is wrong.

