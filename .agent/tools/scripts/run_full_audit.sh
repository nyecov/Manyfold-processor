#!/bin/bash

# Agentic Headless Audit Script
# Purpose: Run full project verification and output a minimal summary for AI Agents.
# Usage: .agent/tools/scripts/run_full_audit.sh [-v]
# Flags:
#   -v  Verbose mode (show full output)

FAILED=0
VERBOSE=0

if [ "$1" == "-v" ]; then
    VERBOSE=1
fi

LOG_DEST="/dev/null"
if [ $VERBOSE -eq 1 ]; then
    LOG_DEST="/dev/stdout"
fi

echo "[AUDIT] Starting Agentic Audit..."
echo "--------------------------------"

# Helper function
run_check() {
    local label=$1
    local cmd=$2
    
    if eval "$cmd > $LOG_DEST 2>&1"; then
        echo "[OK] [$label] Passed"
    else
        echo "[XX] [$label] Failed"
        FAILED=1
    fi
}

# 1. Documentation Integrity
if [ -f "compose.yml" ]; then
    echo "[OK] [Docs] compose.yml found"
else
    echo "[XX] [Docs] compose.yml MISSING"
    FAILED=1
fi

# 2. Code Quality
run_check "Code" "cargo fmt --all -- --check"

# 3. Code Safety
run_check "Code" "cargo clippy --all-targets --all-features -- -D warnings"

# 4. Testing
run_check "Test" "cargo test --quiet"

# 5. Infra (Memory Check)
if grep -q "memory: 1G" compose.yml; then
    echo "[OK] [Infra] Memory Limits Compliant (1G)"
else
    echo "[XX] [Infra] Memory Limits Violation"
    FAILED=1
fi

echo "--------------------------------"
if [ $FAILED -eq 0 ]; then
    echo "[OK] SYSTEM HEALTHY (Ready for Commit)"
    exit 0
else
    echo "[XX] SYSTEM UNHEALTHY (Run with -v for details)"
    exit 1
fi
