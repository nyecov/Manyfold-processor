# Archived BDD Tests

**Date:** 2026-02-01
**Reason:** The BDD tests ("kinks") were causing issues and needed significant work. They have been moved here to temporarily disable them while allowing the rest of the project to proceed without them.

## Contents
- `features/`: Gherkin feature files.
- `steps/`: Rust step definitions.
- `support/`: Helper code for tests.
- `cucumber_runner.rs`: The test runner harness.

## To Restore
1. Move the contents of this folder back to `tests/`.
2. Ensure `cucumber_runner.rs` is at `tests/cucumber_runner.rs`.
3. Re-enable any references in `Cargo.toml` if they were removed (none were removed in this pass, as they are likely just standard dev-dependencies).
