---
description: Audits the Test Resource Registry for orphans, broken links, and metadata quality.
---

# Audit Test Resources

This workflow validates the integrity and quality of the binary test asset registry.

### 🔧 Step 1: Headless Validation
Runs the Rust binary to check for orphans, broken links, schema violations, and hash integrity.
// turbo
```bash
cd .agent/tools && cargo run --bin check_test_resources -- --base-dir "../../test_resources" --manifest "../../test_resources/manifest.yaml" --fix
```

### 🧠 Step 2: Semantic Analysis (AGENT-ONLY)
Review `test_resources/manifest.yaml`.
1.  Look for resources tagged `needs-triage`.
2.  If found, investigate the file and update the `description` and `tags` to reflect its actual content and purpose.
3.  Remove the `needs-triage` and `auto-generated` tags once fixed.
4.  Verify that existing descriptions are meaningful (e.g., "File for testing" is bad; "STL with flipped normals for parser stress test" is good).

### Decision
- **PASS**: No errors from binaries, and no `needs-triage` tags remaining.
- **FAIL**: Binary errors found, or `needs-triage` items still exist.
