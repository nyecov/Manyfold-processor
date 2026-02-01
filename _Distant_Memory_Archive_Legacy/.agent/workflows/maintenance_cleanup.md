---
description: Identifies and cleans up "dead" knowledge (orphan files).
---

# Maintenance: Cleanup & Archival

<!-- audited_by: .agent/workflows/audit_workflows.md -->

## 🔧 Step 1: Sentinel Analysis (Dead Code)
Run the "Reaper" to identify files with zero incoming links.

```bash
cargo run --release --manifest-path .agent/tools/Cargo.toml --bin sentinel_dead_code
```

**Output**: `notes/archival_candidates.md`


## 🧠 Step 2: Classification & Review (Manual)

### Filter 1: The Reaper's List (`archival_candidates.md`)
*   **Action**: Review files flagged by Sentinel.
*   **Archival**: If valuable but stale, move to `.agent/annex/`.
*   **Revival**: If valuable and active, link from `SKILL.md` or `README.md`.
*   **Deletion**: If truly useless junk, delete.

### Filter 2: Staleness Check (Live Files)
Even if linked, files may be "stale" (Historical).

| Indicator | Action | Example |
| :-- | :-- | :-- |
| **Static Snapshots** | Archive | Hardware specs frozen at a date |
| **Decision Logs** | Archive | "Why we chose X" (unless Strategy) |
| **Post-Mortems** | Archive | Incident analysis |
| **Version Notes** | Archive | "Changes in v0.2" |

## 🔧 Step 3: Execution
1.  Move files to `.agent/annex/[category]/`.
2.  Update links if necessary.
