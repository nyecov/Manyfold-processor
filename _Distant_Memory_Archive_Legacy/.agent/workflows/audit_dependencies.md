---
description: validate `requires:` frontmatter and skill dependency links.
---

# Atomic Audit: Explicit Context (Dependencies)

This standalone workflow validates the explicit dependency graph defined in skill frontmatter.

<!-- depends: .agent/skills/project_workflows/SKILL.md -->
<!-- depends: .agent/skills/kb_linking/SKILL.md -->
<!-- depends: .agent/tools/src/bin/audit_dependencies.rs -->

---

## Execution Protocol

> [!NOTE]
> **Hybrid Mode**: This workflow uses both headless scripts (🔧) and agent analysis (🧠).

### 🔧 Step 1: Headless Integrity Check
```powershell
.agent\tools\target\release\audit_dependencies.exe
```
**Covers**: Broken `requires:` references (typos, non-existent skills)

*   **If `[OK]`**: Proceed to Agent steps.
*   **If `[XX]`**: Fix broken references, then proceed.

---

### 🧠 Step 2: Presence Check (AGENT-ONLY)
> Script cannot do this — requires judgment on which skills genuinely need no dependencies.

*   Scan all `SKILL.md` files.
*   Flag any skill that **lacks** a `requires:` block.
*   **Exception**: Root skills like `project_details` may legitimately have no dependencies.
*   **Action**: Propose `requires:` additions for flagged skills.

---

### 🧠 Step 3: Circular Dependency Detection (AGENT-ONLY)
> Script cannot do this — requires graph traversal with cycle detection.

*   Trace the dependency graph (A → B → C → A?).
*   Flag any cycles that would cause infinite loops.
*   **Action**: Break cycles by removing or restructuring dependencies.

---

## Report

| Finding | Source |
|---------|--------|
| Broken References | 🔧 Script |
| Missing `requires:` | 🧠 Agent |
| Circular Dependencies | 🧠 Agent |
