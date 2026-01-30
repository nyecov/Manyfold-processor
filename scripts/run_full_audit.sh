#!/bin/bash
# Deterministic Full Audit Script
# Governance: .agent/workflows/suite_full_audit.md
#
# This script provides "Hard Orchestration" for the project audit,
# eliminating agent-parsing fragility by mechanically executing checks.

set -e  # Exit on first failure

echo "=== Manyfold Processor: Full Spectrum Audit ==="
echo "Date: $(date)"
echo ""

# Phase 1: Code Quality
echo "[1/4] Code Formatting Check..."
cargo fmt --all -- --check
echo "✅ Formatting: PASS"

echo "[2/4] Static Analysis (Clippy)..."
cargo clippy --all-targets --all-features -- -D warnings
echo "✅ Linting: PASS"

# Phase 3: Testing
echo "[3/4] Running Tests..."
cargo test --all-features
echo "✅ Tests: PASS"

# Phase 4: Security (Optional - requires cargo-audit)
echo "[4/4] Security Audit..."
if command -v cargo-audit &> /dev/null; then
    cargo audit
    echo "✅ Security: PASS"
else
    echo "⚠️  Security: SKIPPED (cargo-audit not installed)"
fi

echo ""
echo "=== AUDIT COMPLETE: ALL CHECKS PASSED ==="
