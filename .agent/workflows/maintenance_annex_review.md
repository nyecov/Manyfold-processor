---
description: Review annex content for candidate retrieval (move to active) or staleness updates.
---

# Atomic Maintenance: Annex Review

This standalone workflow audits the `.agent/annex/` to identify historical content that should be revived, updated, or flagged as misleading.

## 1. Inventory
*   List all files in `.agent/annex/`.

## 2. Analysis Criteria

### Candidate for Retrieval (Move to Active)
*   **Lost Knowledge**: Contains logic/rules missing from current `skills/` or `docs/`.
*   **Relevance**: Topic has become active again (e.g., a "future" feature that is now "in progress").
*   **Action**: Suggest moving to `skills/` (if standard) or `docs/` (if specific guidance).

### Needs Refresh (Stale/Misleading)
*   **Misleading**: Contains advice that is now dangerous or strictly forbidden by current architecture.
*   **Confusing**: Users/Agents might mistake this historical behavior for current rules.
*   **Action**:
    *   Add `> [!WARNING]` header: "This document is historical and contains obsolete patterns."
    *   Or, if the *topic* is active but the *content* is old, suggest creating a new Skill superseding this.

### Refinement (Drafts)
*   **Unfinished**: A draft or partial note deemed useful.
*   **Action**: Move to `notes/` for active development.

## 3. Review Process
For each file in the annex:
1.  Check creation date.
2.  Compare against current `project_details` and `architectural_guidelines`.
3.  Determine if it serves better as **Active Guidance** or **Historical Record**.

## 4. Report
Generate a review summary:
```
## Annex Review Findings
| File | Status | Recommendation |
| :-- | :-- | :-- |
| annex/old_feature.md | Relevant | Move to active `docs/feature.md` |
| annex/v0.1_specs.md | Misleading | Add Obsolete Warning Header |
| annex/draft_api.md | Unfinished | Move to `notes/` for refinement |
```

---

## See Also
*   **Annex Purpose**: [annex README](../.agent/annex/README.md)
*   **Migration Check**: [maintenance_annex_migration](./maintenance_annex_migration.md) (The inverse process)
