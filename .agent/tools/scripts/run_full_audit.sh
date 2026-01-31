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

# ------------------------------------------------------------------
# Helper Functions
# ------------------------------------------------------------------

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

# ------------------------------------------------------------------
# Main Logic (Wrapped for Metrics)
# ------------------------------------------------------------------

main_audit() {
    echo "[AUDIT] Starting Full System Audit..."
    echo "-------------------------------------"

    # 0. Build Tools (unless skipped)
    if [ $SKIP_BUILD -eq 0 ]; then
        echo "[SETUP] Building Agent Tools..."
        if ! cargo build --release --manifest-path .agent/tools/Cargo.toml > $LOG_DEST 2>&1; then
            echo "[XX] Tool Build Failed"
            return 1
        fi
    fi

    TOOLS_DIR=".agent/tools/target/release"

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
    run_tool "Skill Examples" "$TOOLS_DIR/check_skill_examples"

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

    # 4. Phase 4 Sentinels
    echo -n "[..] [Sentinel: Catalog] Running..."
    # Allowed to fail (Exit 1 means "Updated", User must stage)
    if "$TOOLS_DIR/sentinel_catalog" > $LOG_DEST 2>&1; then
        echo -e "\r[OK] [Sentinel: Catalog] Aligned    "
    else
        RET=$?
        if [ $RET -eq 1 ]; then
            echo -e "\r[XX] [Sentinel: Catalog] Catalog Updated (Please Stage Changes)"
            FAILED=1
        else
            echo -e "\r[XX] [Sentinel: Catalog] Error (Exit $RET)"
            FAILED=1
        fi
    fi

    echo "-------------------------------------"
    if [ $FAILED -eq 0 ]; then
        echo "[OK] SYSTEM HEALTHY (Ready for Commit)"
        return 0
    else
        echo "[XX] SYSTEM UNHEALTHY (Run with -v for details)"
        return 1
    fi
}

# ------------------------------------------------------------------
# Invocation & Metrics
# ------------------------------------------------------------------

# Ensure metrics directory (silently)
mkdir -p .agent/metrics 2>/dev/null

TEMP_LOG=$(mktemp 2>/dev/null || echo ".agent/metrics/temp.audit.log")

# Run Main Audit, capturing output to both TTY and Temp Log
main_audit 2>&1 | tee "$TEMP_LOG"
RET=${PIPESTATUS[0]}  # Capture exit code of main_audit

# Record Metrics (Context Cost)
if [ -f "$TEMP_LOG" ]; then
    BYTES=$(wc -c < "$TEMP_LOG")
    TS=$(date +%s)
    # Format: Timestamp, Bytes, ExitCode
    echo "$TS,$BYTES,$RET" >> .agent/metrics/audit_log.csv
    rm "$TEMP_LOG"
fi

exit $RET
