---
description: Full-spectrum project audit (Docs, Tests, and Systems).
---

# Orchestrated Suite: Full Spectrum Audit

The ultimate verification tool for the Manyfold Processor project.

<!-- depends: .agent/workflows/audit_tool_alignment.md -->
<!-- depends: .agent/workflows/suite_docs.md -->
<!-- depends: .agent/workflows/audit_code_quality.md -->
<!-- depends: .agent/workflows/suite_tests.md -->
<!-- depends: .agent/workflows/audit_infrastructure.md -->
<!-- depends: .agent/tools/src/bin/check_workflow_skip.rs -->

---

## Execution Protocol

> [!NOTE]
> **Feedback Mandate**: After each step, provide a brief status update (✅/❌ + 1-line summary).

---

### 🔧 Step -1: Workflow Skip Detection
```powershell
.agent\tools\target\release\check_workflow_skip.exe
```
*   Parse output to determine which workflows have unchanged dependencies.
*   Workflows marked `[SKIP]` can bypass their 🔧 Script steps (agent steps still run).
*   Workflows marked `[RUN]` proceed with full execution.

**Example Output**:
```
[SKIP] audit_gherkin - No dependencies changed
[RUN]  audit_dependencies - Modified: .agent/skills
```

---

### 🧠 Step 0: Tool Alignment Check (AGENT-ONLY)

> [!CAUTION]
> **Prerequisite**: If `.agent/tools/` changed, this MUST run first.

*   Invoke `/audit_tool_alignment`
*   📢 **Report**: Tool-workflow alignment status
*   **If FAIL**: Stop. Fix tool alignment before proceeding.
*   **If `check_workflow_skip` said `[SKIP]` for tools**: Can skip this step.

---

### Step 1: Documentation Review
*   Invoke `/suite_docs`
*   📢 **Report**: Doc audit status
*   🔧 Script steps can be skipped if dependencies unchanged.
*   🧠 Agent steps always run.

---

### Step 2: Quality & Security Review
*   Invoke `/audit_code_quality`
*   📢 **Report**: fmt/clippy/audit status

---

### Step 3: Testing Review
*   Invoke `/suite_tests`
*   📢 **Report**: Test status
*   🔧 Script steps can be skipped if dependencies unchanged.
*   🧠 Agent steps always run.

---

### Step 4: Systems Review
*   Invoke `/audit_infrastructure`
*   📢 **Report**: Infrastructure compliance status
*   🔧 Script steps can be skipped if dependencies unchanged.
*   🧠 Agent steps always run.

---

### Step 5: Final Health Report
*   Consolidate all findings into a single high-level risk assessment.
*   Generate summary table of 🔧 Script vs 🧠 Agent findings.

---

## Token Efficiency Summary

| Scenario | Tokens |
|----------|--------|
| First Run (All Changed) | ~63,000 |
| Subsequent Run (Nothing Changed) | **~350** (Step -1 & Pre-Checks) |
| Targeted Run (Some Changed) | ~30,000-45,000 |

---

## Alternative: Hard Orchestration

For deterministic, agent-independent execution of scripts only:
```powershell
.agent\tools\target\release\check_workflow_skip.exe
.agent\tools\target\release\audit_dependencies.exe
.agent\tools\target\release\check_gherkin.exe
.agent\tools\target\release\check_links.exe
.agent\tools\target\release\check_consistency.exe
.agent\tools\target\release\check_constants.exe
.agent\tools\target\release\check_context.exe
.agent\tools\target\release\check_infrastructure.exe
```

> **Note**: Hard orchestration only runs 🔧 Script steps. 🧠 Agent-Only steps require full workflow execution.
