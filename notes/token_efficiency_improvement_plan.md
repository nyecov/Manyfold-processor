# Token Efficiency Improvement Plan v2

**Created**: 2026-01-30
**Baseline Commit**: `3395d359ea3464832f3e6bc5e1607ddf8b42da4e`
**Status**: Active backlog for v0.4+

---

## Phase 1: Complete Script Coverage (High Priority) ✅ DONE

### Goal
Convert remaining manual workflows to headless Rust scripts.

| Workflow | Script to Create | Token Savings | Status |
|----------|-----------------|---------------|--------|
| `audit_consistency` | `check_consistency.rs` | ~15,000/run | ✅ |
| `audit_constants` | `check_constants.rs` | ~8,000/run | ✅ |
| `audit_context` | `check_context.rs` | ~12,000/run | ✅ |
| `audit_infrastructure` | `check_infrastructure.rs` | ~8,000/run | ✅ |

**Estimated Total Savings**: ~43,000 tokens/full audit

---

## Phase 1.5: Root Workflow Optimization (HIGH PRIORITY)

### Problem
The `/audit_tool_alignment` ROOT workflow costs **~25,000-40,000 tokens** per run. This is necessary for reliability but wasteful when tools haven't changed.

### Goal
Implement **Skip Condition** to avoid re-auditing unchanged tools.

### Implementation

#### Option A: Git-Based Detection
Add to `/audit_tool_alignment` workflow:
```markdown
## Skip Condition
*   Run: `git diff HEAD~1 --name-only | grep '.agent/tools/'`
*   **If empty**: Tools unchanged since last commit → SKIP audit.
*   **If non-empty**: Proceed with full audit.
```

#### Option B: Hash-Based Caching
Create `.agent/tools/.audit_cache`:
```yaml
last_audit: 2026-01-30T21:00:00
tools_hash: abc123def456  # SHA256 of all .rs files
result: PASS
```

**Logic**:
1.  Compute current `tools_hash` (concat all `.rs` file contents → SHA256)
2.  If `tools_hash == cached_hash` AND `result == PASS` → SKIP
3.  Else → Run full audit, update cache

#### Option C: Partial Headless (Future)
Create `check_tool_registry.rs` that verifies:
*   All binaries compile
*   All workflow references exist
*   **Cannot headless**: Semantic alignment (stays agent-only forever)

### Token Savings

| Scenario | Without Skip | With Skip |
|----------|--------------|-----------|
| First Run | ~25,000-40,000 | ~25,000-40,000 |
| **Subsequent Run (No Changes)** | ~25,000-40,000 | **~500** |
| Subsequent Run (Tools Changed) | ~25,000-40,000 | ~25,000-40,000 |

**Net Impact**: ~95% reduction on consecutive runs without tool changes.

---

## Phase 2: Unified CLI (Medium Priority)

### Goal
Consolidate all audit binaries into a single CLI tool.

```bash
# Before (4 commands)
.agent/tools/target/release/audit_dependencies.exe
.agent/tools/target/release/check_links.exe
.agent/tools/target/release/check_gherkin.exe

# After (1 command)
.agent/tools/target/release/agent-audit --all
.agent/tools/target/release/agent-audit --deps --links
```

**Benefits**: Simpler invocation, shared code, easier maintenance.

---

## Phase 3: Pre-Commit Integration (Medium Priority)

### Goal
Enforce health checks before every git commit.

**Implementation**:
1.  Add `.git/hooks/pre-commit` that runs `run_full_audit.sh`
2.  Block commits with exit code 1

**Benefits**: Pre-emptive self-healing (Level 3 in philosophy).

---

## Phase 4: Sentinel Automation (Lower Priority)

### Goal
Implement proactive watchers for drift detection.

| Sentinel | Function |
|----------|----------|
| **Catalog Sentinel** | Auto-update `project_workflows` on file creation |
| **Dead Skill Detector** | Flag skills with no references |
| **Orphan Doc Detector** | Flag docs not linked from skills |

**Implementation**: File system watcher (Rust `notify` crate) or Git hook.

---

## Phase 5: Metrics & Comparison (Lower Priority)

### Goal
Track real-world token usage for validation.

**Metrics to Track**:
*   Tokens per `/suite_full_audit` run
*   Manual fallback trigger rate
*   Time-to-detection for regressions

**Compare Against**: `notes/token_efficiency_milestone.md` baseline.

---

## Success Criteria

| Phase | Metric | Target |
|-------|--------|--------|
| 1 | Workflow script coverage | 100% |
| 2 | CLI binary count | 1 |
| 3 | Pre-commit hook active | Yes |
| 4 | Sentinel count | 2+ |
| 5 | Token tracking active | Yes |
