# Token Efficiency Analysis v2: Full Suite with Root Workflow

**Date**: 2026-01-30
**Git Commit**: `3395d359ea3464832f3e6bc5e1607ddf8b42da4e` (baseline)

---

## Executive Summary

The addition of `/audit_tool_alignment` as a ROOT workflow introduces a **fixed overhead cost** but **prevents cascading errors** from misaligned tools. The trade-off is worthwhile for reliability.

---

## Token Cost Breakdown: `/suite_full_audit`

### Current Structure (with Root Workflow)

| Step | Workflow | Method | Est. Tokens |
|------|----------|--------|-------------|
| **0** | `/audit_tool_alignment` | **AGENT-ONLY** | **~25,000-40,000** ⚠️ |
| 1 | `/suite_docs` | Headless First | ~500-1,000 |
| 2 | `/audit_code_quality` | Mixed | ~5,000 |
| 3 | `/suite_tests` | Headless First | ~500-1,000 |
| 4 | `/audit_infrastructure` | Headless First | ~500 |
| 5 | Final Report | Agent | ~1,000 |
| **TOTAL** | | | **~33,000-48,000** |

### Comparison

| Scenario | OLD (No Tools) | NEW (7 Scripts, No Root) | **NEW (7 Scripts + Root)** |
|----------|----------------|--------------------------|----------------------------|
| Green Build | ~55,000 | ~2,500 | **~28,000-43,000** |
| Broken Build | ~55,000 | ~8,000 | **~33,000-48,000** |

---

## The `/audit_tool_alignment` Tax

### Why It's Expensive

| Task | Requires | Tokens |
|------|----------|--------|
| Read Cargo.toml | Agent file read | ~200 |
| Read 7 Rust files | 7 × ~100 lines × 4 tokens | ~2,800 |
| Read 8 workflow files | 8 × ~40 lines × 4 tokens | ~1,280 |
| Semantic comparison | Agent reasoning | ~15,000+ |
| Correction (if needed) | Agent edits | ~5,000+ |
| Verification | Re-read files | ~2,000 |
| **TOTAL** | | **~25,000-40,000** |

### The Tax Justification

> **If tools are misaligned, ALL downstream headless checks produce garbage.**

Without `/audit_tool_alignment`:
- A script might check the wrong directories
- Exit codes might be inverted
- Output format might not match workflow expectations
- **Result**: False positives/negatives → misleading reports

---

## Drawbacks Analysis

### 1. Fixed Overhead (Critical)
*   **Problem**: ~25,000-40,000 tokens spent before any "real" work begins.
*   **Impact**: Eats into the ~55,000 token savings achieved by headless scripts.
*   **Mitigation**: Only run when tools have changed (see below).

### 2. Execution Time
*   **Problem**: Reading 7 Rust files + 8 workflows + semantic analysis = slow.
*   **Impact**: User waits longer for first feedback.
*   **Mitigation**: Cache results; skip if no `.agent/tools/` changes since last run.

### 3. Cannot Parallelize
*   **Problem**: Agent-only = sequential. No headless speedup possible.
*   **Impact**: Bottleneck at start of every audit suite.
*   **Mitigation**: Accept as governance cost.

### 4. Overkill for Stable Projects
*   **Problem**: If tools haven't changed, re-auditing is waste.
*   **Current**: No mechanism to detect "tools changed since last audit".
*   **Mitigation**: Add git-based change detection.

---

## Recommendations

### Short-Term: Conditional Execution

Add to `/audit_tool_alignment`:
```markdown
## Skip Condition
*   If `git diff HEAD~1 --name-only | grep '.agent/tools/'` returns empty:
*   **SKIP** this audit (tools unchanged).
```

**Token Savings**: ~25,000-40,000 on consecutive runs without tool changes.

### Medium-Term: Caching

Create `.agent/tools/.audit_cache`:
```yaml
last_audit: 2026-01-30T21:00:00
tools_hash: abc123
result: PASS
```

If `tools_hash` matches current state → skip audit.

### Long-Term: Partial Headless

Create `check_tool_registry.rs` that does:
- Verify all binaries compile
- Verify all workflow references exist
- **Cannot do**: Semantic alignment (agent-only forever)

**Token Savings**: ~10,000 (structural checks become headless).

---

## Revised Savings Summary

| Scenario | Tokens | vs. OLD (~55,000) |
|----------|--------|-------------------|
| First Run (Green) | ~33,000 | **40%** ⬇️ |
| First Run (Broken) | ~48,000 | **13%** ⬇️ |
| **Subsequent Run (No Tool Changes)** | **~2,500** | **95%** ⬇️ |
| Subsequent Run (Tool Changes) | ~33,000 | 40% ⬇️ |

---

## Conclusion

| Aspect | Assessment |
|--------|------------|
| **Worth It?** | ✅ Yes — Reliability over speed |
| **Primary Drawback** | Fixed overhead on every first run |
| **Mitigation Path** | Skip condition + caching |
| **Net Benefit** | Prevents garbage-in-garbage-out |
