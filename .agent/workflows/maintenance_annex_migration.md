---
description: Audit skills, docs, and notes for content that should migrate to the annex.
---

# Atomic Maintenance: Annex Migration Check

This standalone workflow identifies content that should be archived in the annex.

## 1. Inventory Scan
*   List all files in:
    *   `.agent/skills/`
    *   `docs/`
    *   `notes/`

## 2. Classification Criteria

### Should Move to Annex ✅
| Indicator | Example |
| :-- | :-- |
| **Static Snapshots** | Hardware specs frozen at a point in time |
| **Decision Logs** | "Why we chose X over Y" |
| **Post-Mortems** | Incident analysis, failed approaches |
| **Version-Specific Notes** | "Changes in v0.2" |
| **Deprecated Content** | Superseded by newer skills |

### Should Stay ❌
| Indicator | Example |
| :-- | :-- |
| **Operational Guidance** | "How to deploy" |
| **Active Specifications** | API contracts, file formats |
| **Current Standards** | Code quality rules, linking mandates |

## 3. Analysis Checks

For each file, evaluate:
1.  **Temporal Binding**: Does the content reference a specific date/version without ongoing relevance?
2.  **Operational vs Historical**: Is this "how to do X" (skill) or "why we did X" (annex)?
3.  **Staleness Risk**: If outdated, would it mislead rather than inform?

## 4. Report

Generate a migration recommendation:
```
## Migration Candidates
| File | Reason | Recommended Action |
| :-- | :-- | :-- |
| notes/old_analysis.md | Version-specific | Move to annex |
| skills/legacy_tool/SKILL.md | Deprecated | Move to annex |
```

## 5. Execution (Optional)
*   If approved, move identified files to `.agent/annex/` with date prefix.
*   Update any broken links in remaining files.

---

## See Also
*   **Annex Purpose**: [annex README](../.agent/annex/README.md)
*   **Context Audit**: [audit_context](./audit_context.md)
*   **Linking Standards**: [kb_linking skill](../.agent/skills/kb_linking/SKILL.md)
