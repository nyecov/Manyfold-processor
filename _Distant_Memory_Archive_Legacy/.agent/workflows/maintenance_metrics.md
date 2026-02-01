---
description: Review context cost trends.
---

# Maintenance: Metrics Review

<!-- audited_by: .agent/workflows/audit_workflows.md -->

## 🔧 Step 1: Analyze Logs
Run the metrics sentinel to check average context cost.

```bash
cargo run --release --manifest-path .agent/tools/Cargo.toml --bin sentinel_metrics
```

**Output**: Console report.

## 🧠 Step 2: Agent Review
If output indicates `[WW] Context Cost High`:
1.  **Investigate**: Run `.agent/tools/scripts/run_full_audit.sh -v` manually.
2.  **Optimize**: Identify which tool is printing excessive output on success (Success should be silent).
3.  **Fix**: Update the noisy tool to use logging buffers or remove `println!`.
