# Analysis: Phase 2 (Unified CLI) vs. Atomic Binaries

**Date**: 2026-01-30
**Subject**: Evaluation of Phase 2 "Unified CLI" proposal against `agentic_philosophy` guidelines.

---

## Executive Summary

You are correct. The current Phase 2 proposal ("Unified CLI") **contradicts** the project's core design rules of **Atomic Workflow Design** and **Explicit Dependencies**.

Moving to a monolithic `agent-audit` tool would:
1.  **Break Atomicity**: Workflows would trigger a sub-mode of a giant tool rather than a dedicated binary.
2.  **Obscure Dependencies**: `requires: [check_links]` is clearer than `requires: [agent-audit]`.
3.  **Increase Complexity**: A single binary with 8+ modes is harder to debug/maintain than 8 small binaries.

## Architectural Comparison

### Option A: Unified CLI (Original Plan)
*   **Command**: `agent-audit --check-links`
*   **Structure**: One big `main.rs` with `clap` subcommands.
*   **Pros**: Single binary to build.
*   **Cons**:
    *   **Violates Atomicity**: One tool doing 8 things.
    *   **Fragile**: A compilation error in `check_gherkin` logic breaks `check_links`.
    *   **Agent Context**: Agent must learn CLI flags instead of just running "the link checker".

### Option B: Shared Library + Atomic Binaries (Recommended)
*   **Command**: `check_links.exe`
*   **Structure**: `src/lib.rs` (shared logic) + `src/bin/*.rs` (independent entry points).
*   **Pros**:
    *   **Preserves Atomicity**: 1:1 mapping between Workflow and Tool.
    *   **Code Reuse**: Common file I/O and parsing logic lives in `lib.rs`.
    *   **Resilient**: Broken Gherkin logic doesn't stop Link Checker from compiling/running.
    *   **Agent-Friendly**: Filenames describe purpose (`check_links` vs `agent-audit`).

## Alignment with Philosophy

| Principle | Unified CLI | Atomic Binaries |
|-----------|-------------|-----------------|
| **Atomic Workflow Design** | ❌ Violates | ✅ Aligns |
| **Explicit Dependencies** | ⚠️ Obscures | ✅ Aligns |
| **Rust First** | ✅ Aligns | ✅ Aligns |
| **Fail-Safe** | ❌ Coupled failures | ✅ Isolated failures |

## Recommendation

**Modify Phase 2 Goal**:
*   **From**: "Consolidate into single CLI"
*   **To**: "Refactor to Shared Library (`lib.rs`) for Code Reuse"

This achieves the engineering goal (DRY code, shared types) without sacrificing the architectural goal (Atomicity).
