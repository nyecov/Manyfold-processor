# Analysis: Phase 4 Integration Strategies
**Date**: 2026-01-30
**Subject**: Pro/Con Analysis of Sentinel Integration Vectors
**Reference**: `notes/phase_4_sentinel_plan.md`

This document provides a detailed breakdown of the three proposed integration strategies for Sentinel Automation.

---

## Strategy A: The "Pre-Commit" Guard (Synchronous)
**Mechanism**: Sentinels run as part of `.git/hooks/pre-commit` (via `run_full_audit.sh`).

### ✅ Pros
1.  **Guaranteed Consistency**: It is *impossible* for a commit to exist in the history where the Code and the Catalog are out of sync. This is the "Nuclear Option" against drift.
2.  **Zero Drift Window**: Unlike periodic checks, there is no time window where the system is misaligned.
3.  **Zero Agent Cost**: The computation happens on the developer's machine (User CPU), costing 0 tokens.
4.  **Simulated CI**: Matches what the CI/CD pipeline would likely do.

### ❌ Cons
1.  **Commit Latency**: If the Sentinels take 5 seconds to run, every commit takes 5+ seconds. This adds friction to the "Loop".
2.  **Blocking Friction**: A developer checking in a "WIP" might get blocked by a "Documentation Table Outdated" error, forcing them to run the tool or use `--no-verify`.
3.  **Local Setup Required**: Every developer must install the hook (though Phase 3 already enforces this).

### 🎯 Verdict: Essential for "Hard" Metadata
Best for **`sentinel_catalog`**. The cost of a stale catalog (Agent hallucination) outweighs the minor friction of updating a table on commit.

---

## Strategy B: The "Agent Standup" (Asynchronous)
**Mechanism**: The Agent runs a `/maintenance_daily` workflow at the start of a session conform.

### ✅ Pros
1.  **Deep Analysis Allowed**: Can perform heavy operations (graph theory, full text search) that would be too slow for a pre-commit hook.
2.  **Semantic Judgment**: The Agent can interpret the Sentinel's output. For example, "Dead Code" might actually be "Reserved for Future". The Agent can decide to *ignore* rather than *delete*.
3.  **Non-Blocking**: Never stops a developer from working.
4.  **No User Setup**: Entirely contained within the Agent's workflow definitions.

### ❌ Cons
1.  **Reactive Window**: Drift accumulates until the Agent decides to run the maintenance task.
2.  **Token Cost**: Requires Agent tokens to invoke the workflow and process the results.
3.  **Discipline Dependent**: If the user/agent forgets to run the specific workflow, the maintenance never happens.

### 🎯 Verdict: Essential for "Soft" Hygiene
Best for **`sentinel_dead_code`** (The Reaper). Archiving content requires judgement and shouldn't happen automatically during a git commit.

---

## Strategy C: The "Watcher" (Real-Time)
**Mechanism**: A background process (e.g., `cargo watch`) updates files instantly as they change.

### ✅ Pros
1.  **Best UX**: Documentation updates "magically" as code is written. No manual steps, no wait times.
2.  **Zero Latency**: No commit delay.
3.  **Flow State**: Keeps the developer in the zone; the environment maintains itself.

### ❌ Cons
1.  **Resource Heavy**: Requires a file system watcher running constantly, consuming RAM/CPU.
2.  **"Magic" Confusion**: Files changing automatically on disk can confuse editors (VS Code reloading files) or git clients (unstaged changes appearing repeatedly).
3.  **Environment Complexity**: Requires external tools (`cargo-watch`, `nodemon`) and complex shell setups. Hard to standardize across all user environments.

### 🎯 Verdict: Luxury / Optional
A "Nice to Have" for power users, but **too fragile** to be the primary project strategy. It cannot be guaranteed to be running.

---

## Summary Recommendation

**Adopt a Hybrid Approach**:

1.  **Use Strategy A (Pre-Commit)** for **Low-Cost / High-Value** consistency checks (e.g., keeping the `SKILL.md` tool list updated).
    *   *Reason*: It must never be wrong.
2.  **Use Strategy B (Agent Standup)** for **High-Cost / Judgment-Heavy** cleanups (e.g., finding dead skills).
    *   *Reason*: It requires safety and context.
3.  **Discard Strategy C** for now.
    *   *Reason*: Too much complexity for the marginal gain over Strategy A.
