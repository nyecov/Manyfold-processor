# Testing Strategy: Dual-Track Verification

<!-- audited_by: .agent/workflows/audit_gherkin.md -->

We employ a **Dual-Track Verification** strategy to maximize speed and stability while ensuring coverage.

## 1. The Separation of Concerns

Tests are split into two distinct directories based on their **Target** and **Level**:

| Level | Directory | Target | Role |
| :--- | :--- | :--- | :--- |
| **High Level** | `tests/features/high_level_ui/` | **WebUI** | Verify the "Golden Path" and user experience. |
| **Low Level** | `tests/features/processing_logic/` | **File System/API** | Verify business logic, permutations, and edge cases. |

## 2. Rules of Engagement

### High Level (UI)
*   **Goal**: Prove the feature exists and works for the happy path.
*   **Quantity**: Minimal. **1 Scenario per feature**.
*   **Constraints**:
    *   MUST use real UI steps (e.g., `When I click...`).
    *   MUST NOT overlap with low-level logic variations.

### Low Level (Logic)
*   **Goal**: Prove the logic is correct for all inputs.
*   **Quantity**: Comprehensive. As many scenarios as needed.
*   **Constraints**:
    *   **NO UI STEPS**. All interaction must be via `_API` steps or file system operations.
    *   **Twin-Linking**: MUST include a header pointing to its UI twin.
        ```gherkin
        # Twin-UI: tests/features/high_level_ui/feature_name_ui.feature
        Feature: ...
        ```
    *   **Twin-Lock**: The `Background` must match the UI twin's background (using `_API` variants).

## 3. The "Generic Inspector" Pattern
Do not write custom Rust steps for every specific data field. Use the generic inspector to query `datapackage.json`:

```gherkin
Then_API project "my-project" metadata "title" should be "My Project"
Then_API project "my-project" metadata "resources[0].name" should be "model.stl"
```

## 4. Anti-Masquerading
*   **Strict Separation**: A test in `processing_logic/` using "When I click..." is a compliance violation.
*   **Tool Enforcement**: `check_gherkin` audits will enforce directory structure and linking.
