# Phase 5: Metrics & Comparison Plan

**Date**: 2026-01-30
**Objective**: establish a feedback loop for efficiency validation.
**Constraint**: We cannot directly access "LLM API Token Usage" from inside the Docker container or shell scripts.
**Proxy Metric**: **Context Load (Bytes)**. The definition of "Cost" for an Agent is the number of bytes it must read to understand the state.

---

## 1. Analysis: The "Context Cost" Proxy
Phase 5 Goal is "Track real-world token usage".
Since Token Count ≈ Byte Count / 4, we can instrument the *Output Size* of our tools.
outputting 50KB of logs = costly.
outputting 2KB of logs = cheap.

**Metric 1: Cost to Audit**
*   Measure the `stdout` + `stderr` size of `run_full_audit.sh`.
*   Target: < 2000 Bytes (approx 500 tokens) for a "Clean Pass".

**Metric 2: Cost to Fix**
*   Measure the size of diffs generated or logs read during a failure.
*   (Harder to automate, treated as "Manual Sampling").

---

## 2. Implementation Plan 🛠️
We will create a **Metrics Sentinel** (`sentinel_metrics`) or simply enhance `run_full_audit.sh`.
Given simplicity, enhancing `run_full_audit.sh` is better.

### A. Instrumentation
Modify `run_full_audit.sh` to:
1.  Capture all output to a temporary buffer.
2.  Count bytes (`wc -c`).
3.  Append to a CSV log file `.agent/metrics/audit_log.csv`.
    *   Format: `Timestamp, GitHash, Status, OutputBytes, DurationSec`

### B. Visualization
Create a simple CLI tool `view_metrics.sh` (or `sentinel_metrics.rs`) to parse the CSV and show trends.
*   "Average Audit Cost: 1.2KB (-15% this week)"

---

## 3. Integration Plan 🔗

### A. Skill Integration: `observability_standards`
*   Update `observability_standards/SKILL.md`.
*   Define "Token Budget": "Atomic operations should produce < 50 lines of output on success."

### B. Workflow Integration: `/suite_full_audit`
*   The audit workflow already runs `run_full_audit.sh`.
*   The instrumentation will be *silent* and automatic.
*   We add a new **Review Step** to `/maintenance_metrics` workflow:
    *   Step 1: Run `sentinel_metrics` (to read CSV).
    *   Step 2: Compare against Baseline (`token_efficiency_milestone.md`).

### C. The Feedback Loop
1.  Agent runs Audit.
2.  Audit logs size.
3.  Weekly `maintenance_metrics` checks if Audit Size is creeping up (e.g., verbose logging left on).
4.  Agent fixes observability spam.

---

## 4. Execution Steps
1.  Create `.agent/metrics/` directory.
2.  Modify `run_full_audit.sh` to implement "Quiet capturing" and CSV logging.
3.  Create `sentinel_metrics.rs` (to analyze CSV).
4.  Create `.agent/workflows/maintenance_metrics.md`.
