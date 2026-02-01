---
description: Governance workflow to audit the integrity, logic, and redundancy of the workflow library itself.
---

# Atomic Audit: Workflow Governance

This standalone workflow audits the `.agent/workflows/` library to ensure the "Agentic Operating System" remains clean, efficient, and non-redundant.

## 1. Inventory & Classification
*   List all files in `.agent/workflows/`.
*   Classify each as **Atomic** (Standalone) or **Suite** (Orchestrator).

## 2. Integrity Checks

### A. Atomicity Rule
*   **Check**: Atomic workflows MUST NOT call other workflows. They must contain raw steps only.
*   **Logic**: If an Atomic calls another Atomic, it implies a hidden Orchestrator or tight coupling.

### B. Orchestration Rule
*   **Check**: Suites MUST primarily call Atomics. They should avoid inline heavy lifting.
*   **Logic**: Suites are for *flow control*, not *work execution*.

### C. Redundancy Scan
*   **Matrix**: Compare purpose of all workflows.
    *   *Example*: Do we have `audit_test_logic` and `audit_gherkin` doing the same thing?
*   **Action**: Flag any workflow that overlaps >80% with another. Consolidate if found.

### D. Step Logic Validation
*   **Check**: Are steps actionable "Verbs"? (e.g., "Scan", "Verify", "Update" vs "Think about").
*   **Check**: Do steps have clear Pass/Fail criteria?

### E. Logical Sequencing
*   **Check**: Ensure destructive/reorganizational steps happen *before* verification/linking steps.
    *   *Bad Pattern*: `check_links` -> `move_files` (Links break instantly).
    *   *Good Pattern*: `move_files` -> `check_links` -> `verify_build`.
*   **Mandate**: Workflows must end in a stable, verified state.

## 3. Report
*   **Sequencing Errors**: [List of workflows with bad order]
*   **Redundancy**: [None / List of duplicates]
*   **Logic Gaps**: [List of vague instructions]
*   **Output**: Health score of the workflow library.
