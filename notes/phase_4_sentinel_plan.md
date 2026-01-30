# Phase 4: Sentinel Automation Plan

**Date**: 2026-01-30
**Objective**: Automate the maintenance of project metadata (Catalogs, Links, Knowledge Graph).

---

## Part 1: Implementation Plan 🛠️

### Architecture
We will extend the `agent_tools` crate with new "Sentinel" binaries. These are **Active Maintainers** (Write capability), unlike the **Passive Auditors** (Read-only) of Phase 1-2.

### Sentinel 1: The Cataloger (`sentinel_catalog`)
**Goal**: Keep `project_workflows/SKILL.md` in sync with `src/bin/*.rs`.
*   **Input**: Scans `.agent/tools/src/bin/*.rs`.
*   **Logic**:
    1.  Extracts tool name (filename) and purpose (doc comments `//!`).
    2.  Reads `project_workflows/SKILL.md`.
    3.  Locates the "Binary Registry" table.
    4.  **Action**: Updates the markdown table if new tools are found or descriptions change.
*   **Safety**: Uses a specific comment marker `<!-- sentinel: binary_registry -->` to verify it's editing the right block.

### Sentinel 2: The Reaper (`sentinel_dead_code`)
**Goal**: Identify and flag (or archive) unused knowledge.
*   **Input**: Full file scan of `.agent/skills`, `docs`, `notes`.
*   **Logic**:
    1.  Builds a reference graph (Link -> File).
    2.  Identifies files with In-Degree = 0 (Orphans).
    3.  **Action**: Adds them to `notes/archival_candidates.md` for Agent review.
    *   *Note*: Does not auto-delete (too dangerous).

### Technical Stack
*   **Language**: Rust (reusing `agent_tools::fs` and `agent_tools::parser`).
*   **Dependencies**: `pulldown-cmark` (potentially) or Regex for Markdown table editing.

---

## Part 2: Integration Plan 🔗

### Strategy A: The "Pre-Commit" Guard (Synchronous)
**Mechanism**: Add sentinels to `run_full_audit.sh`.
*   **Pros**: Guarantees catalogs are fresh on every commit.
*   **Cons**: slowing down commits if sentinels are slow (Catalogs are fast, Graphs are slow).
*   **Verdict**: Use for `sentinel_catalog`.

### Strategy B: The "Agent Standup" (Asynchronous)
**Mechanism**: Agent runs a "Daily Sync" workflow at the start of a major task.
*   **Pros**: Keeps the context clean without blocking developers. Best for `sentinel_dead_code`.
*   **Verdict**: Use for heavy analysis tools.

### Strategy C: The "Watcher" (Real-time)
**Mechanism**: A background process (using `cargo watch`).
*   **Command**: `cargo watch -w .agent/tools/src/bin -x "run --bin sentinel_catalog"`
*   **Pros**: Instant updates.
*   **Cons**: Resource usage; requires user setup.

### Integration Roadmap
1.  **Step 1**: Build `sentinel_catalog.rs`.
2.  **Step 2**: Add `sentinel_catalog` to `run_full_audit.sh` (Pre-Commit).
3.  **Step 3**: Build `sentinel_dead_code.rs`.
4.  **Step 4**: Create `/maintenance_daily_sync` workflow for Agent to run Step 3.
