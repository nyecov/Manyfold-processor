---
description: Semantic audit of skill organization, chopping, and Strategy vs. Reference separation.
---

# Atomic Audit: Skill Context

This standalone workflow evaluates the semantic organization of the project's knowledge base.

<!-- depends: .agent/skills/project_workflows/SKILL.md -->
<!-- depends: .agent/skills/agentic_philosophy/SKILL.md -->
<!-- depends: .agent/annex/README.md -->
<!-- depends: .agent/tools/src/bin/check_context.rs -->

---

## Execution Protocol

> [!NOTE]
> **Hybrid Mode**: This workflow uses both headless scripts (🔧) and agent analysis (🧠).

### 🔧 Step 1: Headless Structure Check
```powershell
.agent\tools\target\release\check_context.exe
```
**Covers**: Skill file sizes (>200 lines), missing SKILL.md, annex README

*   **If `[OK]`**: Proceed to Agent steps.
*   **If `[XX]`**: Split oversized skills, add missing files, then proceed.

---

### 🧠 Step 2: Strategy vs Reference Separation (AGENT-ONLY)
> Script cannot do this — requires semantic classification.

*   Verify skills are correctly classified:
    *   **Strategy**: "How to approach X" (Philosophy, Testing, etc.)
    *   **Reference**: "Details about Y" (3MF Spec, API Endpoints, etc.)
*   **Action**: Reclassify or split mixed-mode skills.

---

### 🧠 Step 3: Semantic Overlap Detection (AGENT-ONLY)
> Script cannot do this — requires understanding skill purposes.

*   Identify skills with overlapping scope.
*   Check for duplicate information across skills.
*   **Action**: Merge or deduplicate.

---

### 🧠 Step 4: Orphan Skill Detection (AGENT-ONLY)
> Script cannot do this — requires checking usage across project.

*   Identify skills not referenced by any workflow or other skill.
*   Check for skills with no `requires:` backlinks.
*   **Action**: Archive to annex or add references.

---

### 🧠 Step 5: Annex Candidates (AGENT-ONLY)
> Script cannot do this — requires judgment on staleness.

*   Identify skills that should be archived to annex.
*   Check for outdated content no longer relevant.
*   **Action**: Move to annex with deprecation note.

---

## Report

| Finding | Source |
|---------|--------|
| Oversized Skills | 🔧 Script |
| Missing SKILL.md | 🔧 Script |
| Strategy/Reference Mix | 🧠 Agent |
| Overlapping Skills | 🧠 Agent |
| Orphan Skills | 🧠 Agent |
| Annex Candidates | 🧠 Agent |
