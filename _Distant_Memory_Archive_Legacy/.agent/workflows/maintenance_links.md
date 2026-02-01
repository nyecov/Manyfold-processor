---
description: Synchronize and update relative links across project skills, workflows, docs, and notes.
---

# Atomic Maintenance: Link Synchronization

This standalone workflow maintains the integrity of the "web of knowledge" by ensuring all internal links are relative and functional.

<!-- depends: .agent/skills/kb_linking/SKILL.md -->
<!-- depends: .agent/workflows -->
<!-- depends: docs -->
<!-- depends: notes -->
<!-- depends: .agent/tools/src/bin/check_links.rs -->

---

## Execution Protocol

> [!NOTE]
> **Hybrid Mode**: This workflow uses both headless scripts (🔧) and agent analysis (🧠).

### 🔧 Step 1: Headless Absolute Path Detection
```powershell
.agent\tools\target\release\check_links.exe
```
**Covers**: Absolute paths (`C:/`, `/home/`, `file:///`)

*   **If `[OK]`**: Proceed to Agent steps.
*   **If `[XX]`**: Convert to relative paths, then proceed.

---

### 🧠 Step 2: Broken Link Detection (AGENT-ONLY)
> Script cannot do this — requires checking if link targets exist.

*   Scan all markdown links.
*   Verify each target file exists.
*   **Action**: Fix or remove broken links.

---

### 🧠 Step 3: Link Enrichment (AGENT-ONLY)
> Script cannot do this — requires semantic understanding of relationships.

*   Identify mentions of skill names without links.
*   Add lateral links between peer skills (e.g., `3mf_specification` ↔ `stl_specification`).
*   Ensure workflows link to their foundation skills.
*   **Action**: Add missing links.

---

### 🧠 Step 4: Link Logic Verification (AGENT-ONLY)
> Script cannot do this — requires understanding linking semantics.

*   Verify links follow vertical, lateral, or contextual logic (per `kb_linking` skill).
*   Check for orphaned documents with no incoming links.
*   **Action**: Add appropriate backlinks.

---

## Report

| Finding | Source |
|---------|--------|
| Absolute Paths | 🔧 Script |
| Broken Links | 🧠 Agent |
| Missing Links | 🧠 Agent |
| Orphaned Docs | 🧠 Agent |
