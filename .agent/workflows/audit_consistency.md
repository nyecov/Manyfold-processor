---
description: Logical audit of internal and cross-document mandate consistency.
---

# Atomic Audit: Document Consistency

This standalone workflow verifies the logical integrity of the documentation itself.

<!-- depends: .agent/skills/kb_linking/SKILL.md -->
<!-- depends: .agent/skills/code_quality_standards/SKILL.md -->
<!-- depends: docs/Documentation_Quality_Comparison.md -->
<!-- depends: notes/token_efficiency_improvement_plan.md -->
<!-- depends: .agent/tools/src/bin/check_consistency.rs -->

---

## Execution Protocol

> [!NOTE]
> **Hybrid Mode**: This workflow uses both headless scripts (🔧) and agent analysis (🧠).

### 🔧 Step 1: Headless Placeholder Scan
```powershell
.agent\tools\target\release\check_consistency.exe
```
**Covers**: TODOs, TBDs, FIXMEs, PLACEHOLDERs

*   **If `[OK]`**: Proceed to Agent steps.
*   **If `[XX]`**: Review flagged placeholders, then proceed.

---

### 🧠 Step 2: Internal Consistency (AGENT-ONLY)
> Script cannot do this — requires semantic understanding.

*   Verify that each document's conclusions follow from its premises.
*   Check for contradictory statements within the same document.
*   **Action**: Flag and fix logical inconsistencies.

---

### 🧠 Step 3: Cross-Document Mandates (AGENT-ONLY)
> Script cannot do this — requires cross-file semantic analysis.

*   Scan for contradictions between documents.
    *   Example: Doc A says "Use X", Doc B says "Do not use X".
*   Verify that version numbers, paths, and filenames are consistent.
*   **Action**: Reconcile conflicting statements.

---

### 🧠 Step 4: Risk Identification (AGENT-ONLY)
> Script cannot do this — requires reasoning about architectural logic.

*   Flag logical fallacies in architectural reasoning.
*   Identify outdated or orphaned guidelines.
*   **Action**: Update or archive stale content.

---

## Report

| Finding | Source |
|---------|--------|
| Placeholders (TODO/TBD) | 🔧 Script |
| Internal Contradictions | 🧠 Agent |
| Cross-Doc Conflicts | 🧠 Agent |
| Logical Fallacies | 🧠 Agent |
