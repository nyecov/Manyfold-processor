---
description: Testing audit of Gherkin scenario limits, behavior-focus, and traceability links.
---

# Atomic Audit: Gherkin Logic

This standalone workflow verifies that BDD feature files are logically sound and comply with quality standards.

<!-- depends: tests/Testing/Features -->
<!-- depends: .agent/skills/gherkin_style_guide/SKILL.md -->
<!-- depends: .agent/skills/testing_philosophy/SKILL.md -->
<!-- depends: .agent/tools/src/bin/check_gherkin.rs -->

---

## Execution Protocol

> [!NOTE]
> **Hybrid Mode**: This workflow uses both headless scripts (🔧) and agent analysis (🧠).

### 🔧 Step 1: Headless Scenario Count
```powershell
.agent\tools\target\release\check_gherkin.exe
```
**Covers**: Files with >5 scenarios per feature file

*   **If `[OK]`**: Proceed to Agent steps.
*   **If `[XX]`**: Split oversized feature files, then proceed.

---

### 🧠 Step 2: Step Count Analysis (AGENT-ONLY)
> Script cannot do this — requires parsing Given/When/Then steps.

*   Check each scenario has 3-7 steps.
*   Flag scenarios with too many or too few steps.
*   **Action**: Refactor overly verbose or thin scenarios.

---

### 🧠 Step 3: Behavior Focus (AGENT-ONLY)
> Script cannot do this — requires semantic understanding of step wording.

*   Verify steps describe *what* happens, not *how*.
    *   ❌ "When I click the green button"
    *   ✅ "When I save the model"
*   **Action**: Reword implementation-focused steps.

---

### 🧠 Step 4: Traceability (AGENT-ONLY)
> Script cannot do this — requires matching scenarios to skills/docs.

*   Verify each feature/scenario links to a relevant skill or document.
*   Check for orphan scenarios with no traceability.
*   **Action**: Add traceability links.

---

### 🧠 Step 5: Prefix Verification (AGENT-ONLY)
> Script cannot do this — requires parsing step keywords.

*   Verify standard keywords (`Given`, `When`, `Then`) for UI tests.
*   Verify `_API` prefixes for backend tests.
*   **Action**: Fix incorrect prefixes.

---

## Report

| Finding | Source |
|---------|--------|
| >5 Scenarios | 🔧 Script |
| Step Count | 🧠 Agent |
| Behavior Focus | 🧠 Agent |
| Traceability | 🧠 Agent |
| Prefix Errors | 🧠 Agent |
