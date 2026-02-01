---
description: Alignment audit of Gherkin steps to Rust definitions and Twin-linking verification.
---

# Atomic Audit: Step Glue Alignment

This standalone workflow ensures that the Gherkin "Truth" is correctly implemented in Rust.

## 1. Mapping Verification
*   Ensure every `#[given/when/then]` attribute in Rust matches a step in the feature files.
*   Verify that the context (Standard/UI vs. `_API`) is correctly mapped.

## 2. Twin-Linking Audit
*   Check that corresponding UI and API steps are cross-linked.
*   **Gherkin**: `# [twin: Step Name]`
*   **Rust**: `// [twin: Step Name]` above the attribute.

## 3. Dead Code Detection
*   Flag any step definitions in Rust that are not used in any feature file.
*   Flag any Gherkin steps that lack a corresponding implemention in Rust.

## 4. Report
*   Highlight misalignments between specifications and implementation.
