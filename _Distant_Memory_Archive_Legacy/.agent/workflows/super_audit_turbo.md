---
description: Comprehensive, agent-driven super audit with auto-execution (Turbo Mode).
---
// turbo-all

# 🚀 Super Audit (Turbo Mode)

This workflow consolidates all project audits and refinements into a single, high-speed execution pipeline. It is designed to be executed solely by the AI agent.

## 1. 🛠️ Refinement (Auto-Fixes)

### 1.1 Code Formatting
*   **Goal**: Ensure strict adherence to Rustfmt standards.
*   **Action**: Execute the following command to auto-format all Rust code.
```powershell
cargo fmt --all
```

### 1.2 Link Maintenance (Headless)
*   **Goal**: Detect absolute paths (which break portability).
*   **Action**: Run the link checker.
```powershell
.agent\tools\target\release\check_links.exe
```

## 2. 🛡️ Governance & Hard Audits (System Enforced)

Run all governance binaries to ensure structural integrity, alignment, and consistency.

### 2.1 Dependency & Workflow Integrity
```powershell
.agent\tools\target\release\audit_dependencies.exe
.agent\tools\target\release\check_workflow_skip.exe
```

### 2.2 Gherkin & Test Resources
```powershell
.agent\tools\target\release\check_gherkin.exe
.agent\tools\target\release\check_test_resources.exe
```

### 2.3 Semantic Consistency
```powershell
.agent\tools\target\release\check_consistency.exe
.agent\tools\target\release\check_constants.exe
.agent\tools\target\release\check_context.exe
```

### 2.4 Infrastructure
```powershell
.agent\tools\target\release\check_infrastructure.exe
```

## 3. 🧪 Quality & Verification (The Gauntlet)

### 3.1 Static Analysis (Clippy)
*   **Goal**: Catch common mistakes and potential bugs.
```powershell
cargo clippy --all-targets --all-features -- -D warnings
```

### 3.2 Full Test Suite
*   **Goal**: Verify all logic, UI behavior, and anti-masquerading rules.
*   **Note**: This runs the Cucumber runner which includes the Anti-Masquerading audit.
```powershell
cargo test --release
```

## 4. 📝 Final Report (Agent Analysis)

**Consolidate findings:**
1.  **Refinements Applied**: What did `cargo fmt` change? (If any)
2.  **Governance Status**: Did all binaries pass?
3.  **Test Status**: Did the full suite pass?
4.  **Manual Analysis**: check if there are any obvious logical inconsistencies not caught by tools.

> **Final Action**: if everything passes, declare the codebase **CLEAN**. If issues found, list them as immediate blockers.
