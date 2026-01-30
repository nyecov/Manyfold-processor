---
description: Synchronize and update relative links across project skills, workflows, docs, and notes.
---

# Atomic Maintenance: Link Synchronization

This standalone workflow maintains the integrity of the "web of knowledge" by ensuring all internal links are relative and functional.

> [!IMPORTANT]
> **Headless First Protocol**: Run the token-efficient script before manual analysis.

## 0. Headless Execution (Preferred)
```powershell
.agent\tools\target\release\check_links.exe
```
*   **If `[OK]`**: Report pass, skip manual steps.
*   **If `[XX]`**: Proceed to manual analysis below to fix violations.

---

## 1. Inventory (Manual Fallback)
*   List all markdown files in `.agent/`, `docs/`, `notes/`, and `tests/`.

## 2. Scan & Update
*   Identify mentions of skill names or document titles without links.
*   **Mandate**: Convert all absolute paths to **relative paths**.
*   Verify that links follow the vertical, lateral, or contextual logic defined in `kb_linking`.

## 3. Enrichment
*   Add lateral links between peer skills (e.g., `3mf_specification` <-> `stl_specification`).
*   Ensure workflows link to their foundation skills.

## 4. Verification
*   Check for broken links (pointing to non-existent files) and report them for manual fix or removal.
*   Batch update using `multi_replace_file_content`.
