---
description: Full-spectrum project audit (Docs, Tests, and Systems).
---

# Orchestrated Suite: Full Spectrum Audit

The ultimate verification tool for the Manyfold Processor project.

> [!NOTE]
> **Feedback Mandate**: After each step, provide a brief status update (✅/❌ + 1-line summary).

## 1. Documentation Review
*   Invoke `/suite_docs`
*   📢 **Report**: Doc audit status

## 2. Quality & Security Review
*   Invoke `/audit_code_quality`
*   📢 **Report**: fmt/clippy/audit status

## 3. Testing Review
*   Invoke `/suite_tests`
*   📢 **Report**: Test status

## 4. Systems Review
*   Invoke `/audit_infrastructure`
*   📢 **Report**: Infrastructure compliance status

## 5. Final Health Report
*   Consolidate all findings into a single high-level risk assessment for the project.

---

## Alternative: Hard Orchestration

For deterministic, agent-independent execution, run the shell script:
```bash
./scripts/run_full_audit.sh
```
