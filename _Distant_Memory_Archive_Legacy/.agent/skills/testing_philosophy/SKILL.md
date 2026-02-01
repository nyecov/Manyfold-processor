---
name: Testing Philosophy
description: High-level strategy and governing mandates for Behavior Driven Development (BDD).
requires: [gherkin_style_guide, cucumber_rust_reference, project_details, agentic_philosophy]
---

# Testing Philosophy: Strategy & Governance

<!-- audited_by: .agent/workflows/audit_gherkin.md -->
<!-- audited_by: .agent/workflows/audit_masquerading.md -->

The software behavior is governed by a **Dual-Track Verification** strategy, ensuring reliability across both user-facing and internal interfaces.

## 1. Core Mandates
*   **Gherkin is the Interface**: Cucumber `.feature` files are the official, language-based source of truth for all behavior.
*   **Dual-Track Verification**: Every feature MUST be verified through both **UI Testing** (End-to-End) and **API Testing** (Logic/Data).
*   **Rust Backend**: All step definitions and test logic must be implemented in Rust.
*   **Hardware Tier Strategy**:
    *   **Tier 2 (Standard)**: Primary target for automated verification (Docker/CI).
    *   **Tier 1/3 (Hardware)**: Specific verification is deferred until v1.0.

## 2. Prohibited UI Masquerading (MANDATORY)
*   **Definition**: A test step using UI-centric language (e.g., "I click the button") MUST NOT bypass the UI by hitting an API endpoint directly without verifying the UI element's presence.
*   **Verification**: Steps claiming to interact with the UI MUST verify the element exists (e.g., by scanning the DOM or checking static HTML) before proceeding with the action.
*   **Consequence**: Direct API interaction for setup should use API-centric language (e.g., "I clear the timeline via API").

## 3. Auditor Verification (Poisoned Step Strategy)
To ensure our automated governance tools (in `.agent/tools/`) are effective, developers should periodically use the **Poisoned Step** technique:
*   **Action**: Intentionally introduce a step that violates a mandate (e.g., a UI step hitting an API directly without DOM verification).
*   **Verification**: Run the test suite or specific audit workflow (e.g., `/audit_masquerading`).
*   **Success Criteria**: The audit MUST fail and provide a clear reason. If it passes, the auditor logic is stale or broken and must be fixed.
*   **Cleanup**: Immediately remove the poisoned step after verification.

## 4. Speed vs. Reliability
*   **API Substitution Policy**: Once a feature is verified by a foundational UI test, subsequent tests may use **API-based steps** to bypass slow UI interactions for setup or verification.
*   **Twin Linking Intent**: UI and API layers must remain logically connected to allow for automated test optimization and high-speed fallback.

## 3. Directory Structure
```
tests/
├── features/       <-- Executable specifications (.feature)
└── steps/          <-- Rust implementation (.rs)
```

## 4. Operational Verification
To maintain the "Rust First" policy, tests are executed natively via the IDE:
*   **Discovery**: Features are found in `tests/features/`.
*   **Execution**: Use the **VS Code Test Explorer** or **Codelens** inside `.feature` files.
*   **Debugging**: Set breakpoints in `tests/steps/*.rs` and use the **Debug Cucumber** launch configuration.
*   **Gatekeeping**: Every run is preceded by an automated anti-masquerading audit.

## 5. Test Resources
*   **Location**: `test_resources/` (in project root).
*   **Purpose**: Stores binary assets (STL, Images) required for integration tests.
*   **Policy**: Only small, representative files. Do not commit large datasets.

---

## See Also
*   **Methodology**: [gherkin_style_guide](../gherkin_style_guide/SKILL.md) (How to write features).
*   **Implementation**: [cucumber_rust_reference](../cucumber_rust_reference/SKILL.md) (Rust technical lookup).
