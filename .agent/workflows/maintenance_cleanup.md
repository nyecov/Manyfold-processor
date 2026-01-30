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

## 🧠 Step 2: Agent Review
Read `notes/archival_candidates.md`.
For each listed file:

1.  **Analyze**: Read the file. Is it valuable?
    *   **Yes**: It is "Forgotten Knowledge". Add a link to it in a relevant `SKILL.md` or `README.md` to "revive" it.
    *   **No**: It is "Stale". Move it to `.agent/annex/` (and update internal links if any, though orphans usually have none).

2.  **Report**: Update the user on what was archived vs revived.
