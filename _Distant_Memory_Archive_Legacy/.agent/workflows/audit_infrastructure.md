---
description: Systems audit of codebase and infrastructure alignment with design truth.
---

# Atomic Audit: Infrastructure & System

This standalone workflow verifies that the actual project implementation matches architectural mandates.

<!-- depends: Cargo.toml -->
<!-- depends: Dockerfile -->
<!-- depends: compose.yml -->
<!-- depends: .agent/skills/environment_constraints/SKILL.md -->
<!-- depends: .agent/skills/architectural_guidelines/SKILL.md -->
<!-- depends: .agent/tools/src/bin/check_infrastructure.rs -->

---

## Execution Protocol

> [!NOTE]
> **Hybrid Mode**: This workflow uses both headless scripts (🔧) and agent analysis (🧠).

### 🔧 Step 1: Headless Infrastructure Check
```powershell
.agent\tools\target\release\check_infrastructure.exe
```
**Covers**: compose.yml memory, Dockerfile presence, Cargo.toml presence, Python file count

*   **If `[OK]`**: Proceed to Agent steps.
*   **If `[XX]`**: Fix infrastructure issues, then proceed.

---

### 🧠 Step 2: Architectural Alignment (AGENT-ONLY)
> Script cannot do this — requires comparing code to mandates.

*   Verify code follows `architectural_guidelines`:
    *   Rust-first mandate
    *   Container-native design
    *   API-based Manyfold interaction
*   **Action**: Flag and fix deviations.

---

### 🧠 Step 3: Environment Constraint Compliance (AGENT-ONLY)
> Script cannot do this — requires cross-referencing constraints.

*   Verify `compose.yml` values match `environment_constraints` skill.
*   Check Dockerfile stages align with deployment requirements.
*   **Action**: Sync values with canonical sources.

---

### 🧠 Step 4: Dependency Audit (AGENT-ONLY)
> Script cannot do this — requires semantic understanding of dependencies.

*   Check `Cargo.toml` dependencies are minimal and justified.
*   Flag unused or redundant dependencies.
*   **Action**: Remove bloat.

---

### 🧠 Step 5: Python Phase-Out Progress (AGENT-ONLY)
> Script counts files but cannot assess migration progress.

*   Check if Python files are legacy wrappers or active code.
*   Assess progress toward Rust-first migration.
*   **Action**: Document migration status.

---

## Report

| Finding | Source |
|---------|--------|
| Missing compose/Dockerfile | 🔧 Script |
| Memory Config | 🔧 Script |
| Python File Count | 🔧 Script |
| Architectural Deviations | 🧠 Agent |
| Constraint Mismatches | 🧠 Agent |
| Dependency Bloat | 🧠 Agent |
| Python Migration Status | 🧠 Agent |
