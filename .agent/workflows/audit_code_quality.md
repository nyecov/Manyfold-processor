---
description: Performs code formatting, linting, and security checks on the Rust codebase.
---

# Atomic Audit: Code Quality & Security

This standalone workflow verifies that the codebase adheres to the project's quality and security mandates.

> [!NOTE]
> **Feedback Mandate**: After each check, provide a brief status update (✅/❌ + 1-line summary).

## 1. Formatting Check
*   Run `cargo fmt --all -- --check`
*   **Goal**: Ensure consistent style across all contributors.
*   📢 **Report**: ✅/❌ Formatting status

## 2. Static Analysis (Clippy)
*   Run `cargo clippy --all-targets --all-features -- -D warnings`
*   **Goal**: Catch common mistakes, idiomatic improvements, and potential bugs.
*   📢 **Report**: ✅/❌ Clippy status + warning count

## 3. Security Audit
*   Run `cargo audit`
*   **Goal**: Identify vulnerable dependencies in `Cargo.lock`.
*   📢 **Report**: ✅/❌ Security status + vulnerability count

## 4. Reporting
*   Consolidate any warnings or errors into a quality report.
*   **Mandate**: Any failure in this workflow blocks the "Healthy" status in Full Spectrum Audits.
