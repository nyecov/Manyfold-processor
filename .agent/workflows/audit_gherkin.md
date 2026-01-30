---
description: Testing audit of Gherkin scenario limits, behavior-focus, and traceability links.
---

# Atomic Audit: Gherkin Logic

This standalone workflow verifies that BDD feature files are logically sound and comply with quality standards.

> [!IMPORTANT]
> **Headless First Protocol**: Run the token-efficient script before manual analysis.

## 0. Headless Execution (Preferred)
```powershell
.agent\tools\target\release\check_gherkin.exe
```
*   **If `[OK]`**: Report pass, skip manual steps.
*   **If `[XX]`**: Proceed to manual analysis below.

---

## 1. Quality Standards (Manual Fallback)
*   **Scenario Limits**: No more than **5 scenarios** per `.feature` file.
*   **Step Count**: Aim for 3–7 steps per scenario.
*   **Traceability**: Each feature/scenario must link to a relevant skill or document.

## 2. Semantic Verification
*   **Behavior focus**: Steps must describe *what* happens, not *how* (Implementation-agnostic).
*   **Prefix Verification**: 
    *   Standard keywords (`Given`, `When`, `Then`) for UI.
    *   `_API` prefixed keywords for Backend.
*   **Sentence Reuse**: Allow identical sentences across standard and `_API` layers.

## 3. Logical Contradictions
*   Detect redundant scenarios that test identical behaviors.
*   Flag scenarios with contradictory logic (same Given/When, conflicting Then).

## 4. Report
*   List all violations with file paths and line numbers.
