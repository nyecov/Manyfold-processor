---
description: validate `requires:` frontmatter and skill dependency links.
---

# Atomic Audit: Explicit Context (Dependencies)

This standalone workflow validates the explicit dependency graph defined in skill frontmatter.

## 1. Inventory & Parsing
*   Scan all `SKILL.md` files.
*   Parse the YAML frontmatter for the `requires:` block.

## 2. Validation Checks

### A. Presence Check
*   Flag any skill that **lacks** a `requires:` block. 
    *   *Note*: Some root skills (like `project_details`) might genuinely have no dependencies, but most should at least reference `project_details` or `architectural_guidelines`.

### B. Integrity Check (Broken Links)
*   Verify that every skill listed in `requires:` actually exists in `.agent/skills/`.
*   Flag typos (e.g., `requires: deploy_on_radxa` vs `deploy_on_radxa_rock5`).

### C. Circular Dependency Check
*   (Advanced) Trace the graph to ensure there are no infinite loops (A requires B requires A).

## 3. Report
*   **Missing Dependencies**: List skills needing `requires:` added.
*   **Broken Dependencies**: List invalid references.
*   **Graph Health**: Summary of the dependency web.
