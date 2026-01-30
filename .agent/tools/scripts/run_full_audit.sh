#!/bin/bash

# Agentic Headless Audit Script
# Purpose: Run full project verification using the Phase 2 Rust-based tool suite.
# Usage: .agent/tools/scripts/run_full_audit.sh [-v] [--skip-build]

FAILED=0
VERBOSE=0
SKIP_BUILD=0

for arg in "$@"; do
    case $arg in
        -v) VERBOSE=1 ;;
        --skip-build) SKIP_BUILD=1 ;;
    esac
done

LOG_DEST="/dev/null"
if [ $VERBOSE -eq 1 ]; then
    LOG_DEST="/dev/stdout"
fi

echo "[AUDIT] Starting Full System Audit..."
echo "-------------------------------------"

# 0. Build Tools (unless skipped)
if [ $SKIP_BUILD -eq 0 ]; then
    echo "[SETUP] Building Agent Tools..."
    if ! cargo build --release --manifest-path .agent/tools/Cargo.toml > $LOG_DEST 2>&1; then
        echo "[XX] Tool Build Failed"
        exit 1
    fi
fi

TOOLS_DIR=".agent/tools/target/release"

# Helper function for binaries
run_tool() {
    local name=$1
    local binary=$2
    
    # Check if binary exists
    if [ ! -f "$binary" ]; then
        if [ -f "${binary}.exe" ]; then
             binary="${binary}.exe"
        else
             echo "[XX] [$name] Binary missing: $binary"
             FAILED=1
             return
        fi
    fi

    echo -n "[..] [$name] Running..."
    if eval "$binary > $LOG_DEST 2>&1"; then
        echo -e "\r[OK] [$name] Passed    "
    else
        echo -e "\r[XX] [$name] Failed (Exit Code $?)"
        FAILED=1
    fi
}

# Helper function for shell commands (cargo, etc)
run_cmd() {
    local name=$1
    local cmd=$2
    
    echo -n "[..] [$name] Running..."
    if eval "$cmd > $LOG_DEST 2>&1"; then
        echo -e "\r[OK] [$name] Passed    "
    else
        echo -e "\r[XX] [$name] Failed    "
        FAILED=1
    fi
}

# 1. Standard Rust Checks
# 1. Standard Rust Checks
run_cmd "Cargo Check" "cargo check --quiet"
run_cmd "Cargo Test" "cargo test --quiet"
run_cmd "Cargo Fmt" "cargo fmt --all -- --check"

# 2. Agent Tool Suite
run_tool "Dependency Audit" "$TOOLS_DIR/audit_dependencies"
run_tool "Link Integrity" "$TOOLS_DIR/check_links"
run_tool "Context Check" "$TOOLS_DIR/check_context"
run_tool "Consistency" "$TOOLS_DIR/check_consistency"
run_tool "Infrastructure" "$TOOLS_DIR/check_infrastructure"
run_tool "Constants" "$TOOLS_DIR/check_constants"
run_tool "Gherkin Quality" "$TOOLS_DIR/check_gherkin"

# 3. Tool Alignment (Self-Check)
echo -n "[..] [Tool Alignment] Running..."
# Capture output, allow exit code 2 (Hash Changed)
if out=$("$TOOLS_DIR/check_tool_alignment_skip" 2>&1); then
    echo -e "\r[OK] [Tool Alignment] Passed    "
else
    ret=$?
    if [ $ret -eq 2 ]; then
        echo -e "\r[WW] [Tool Alignment] Hash Changed (Expected)"
    else
        echo -e "\r[XX] [Tool Alignment] Failed    "
        FAILED=1
    fi
fi

echo "-------------------------------------"
if [ $FAILED -eq 0 ]; then
    echo "[OK] SYSTEM HEALTHY (Ready for Commit)"
    exit 0
else
    echo "[XX] SYSTEM UNHEALTHY (Run with -v for details)"
    exit 1
fi

