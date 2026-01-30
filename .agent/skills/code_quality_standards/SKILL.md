---
name: Code Quality Standards
description: Mandatory standards for Rust code formatting, linting, and security auditing.
requires: [project_details, agentic_philosophy]
---

# Code Quality Standards

This project enforces strict code quality and security standards to ensure long-term maintainability and reliability on hardware.

## 1. Rust Formatting (`rustfmt`)
*   **Mandate**: All code must be formatted using `cargo fmt`.
*   **Configuration**: Use default `rustfmt` settings unless a `.rustfmt.toml` is provided.
*   **Check**: `cargo fmt --all -- --check`

## 2. Linting (`clippy`)
*   **Mandate**: Code must be free of clippy warnings.
*   **Severity**: "Strict" - all warnings are treated as errors in audit suites.
*   **Check**: `cargo clippy --all-targets --all-features -- -D warnings`
*   **Allowed Exceptions**: Any `#[allow(clippy::...)]` must be accompanied by a comment explaining the rationale.

## 3. Security Auditing (`cargo-audit`)
*   **Mandate**: Dependencies must be audited for known vulnerabilities.
*   **Check**: `cargo audit`
*   **Frequency**: Run as part of every full spectrum audit.

## 4. Project Hygiene (Structural Integrity)
To prevent structural degradation and "documentation rot":
*   **Zero Duplication**: Explicitly forbid duplicated headers, sections, or functional blocks within the same document (e.g., no double `### Implementation Mandate`).
*   **No Orphan Files**: Every file in `.agent/` or `docs/` must be linked from at least one other document or catalog.
*   **Relative Path Integrity**: Mandatory use of relative paths for all internal cross-linking.

## 5. Clean Code Mandates
*   **DRY (Don't Repeat Yourself)**: Shared logic across the API/UI layers or documentation must be abstracted into common modules or foundational skills.
*   **The "No-Placeholder" Policy**: 
    *   Forbid the use of `TODO`, `FIXME`, or `unimplemented!()` in any file that is part of a production build or verified audit.
    *   Exceptions must include a specific issue reference or architectural rationale.
*   **Layer Isolation (Testing)**: 
    *   **UI Step Definitions** (`*_steps.rs`) MUST strictly confine logic to navigational, visual, and DOM-level verification. They are forbidden from performing direct protocol assertions (e.g., status codes, JSON schema) if an API Twin exists.
    *   **API Step Definitions** (`*_API_steps.rs`) MUST confine logic to protocol-level, data-integrity, and state-machine verification.
    *   **Twin Logic Sharing**: Logic shared between layers must be abstracted into the `world.rs` or a shared helper module, never duplicated across layer-specific files.
*   **Dead Code Elimination**: Strictly forbid unused imports, functions, or variables (implicitly handled by `clippy`, but enforced as a project mandate).

## 6. Documentation Quality
*   **Mandate**: All public modules and non-trivial functions should have doc comments (`///`).
*   **Semantic Segmentation**: Documents MUST strictly adhere to the **Strategy vs. Methodology vs. Reference** categorization to avoid "semantic pollution."

---

## See Also
*   **Workflows**: [audit_code_quality](../../workflows/audit_code_quality.md)
*   **Governance**: [architectural_guidelines](../architectural_guidelines/SKILL.md)
