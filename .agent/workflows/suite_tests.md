---
description: Combined audit of Gherkin logic and Rust step alignment.
---

# Orchestrated Suite: Testing Audit

This suite performs a comprehensive audit of the BDD testing stack by invoking atomic standalones.

## 1. Specification Audit
*   Invoke `/audit_gherkin`
*   Verify scenario quality, behavior-focus, and layer prefixes.

## 2. Implementation Audit
*   Invoke `/audit_step_glue`
*   Verify alignment between Gherkin steps and Rust definitions, including Twin-linking.

## 3. Findings Consolidation
*   Synthesize reports into a unified testing health overview.
