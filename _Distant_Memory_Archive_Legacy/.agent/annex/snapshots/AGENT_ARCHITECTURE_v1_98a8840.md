# AI Agent Architecture: Version 98a8840 (Jan 2026)

> **Snapshot**: `98a8840`
> **Date**: 2026-01-31
> **Branch**: `Self-healing-cylce`

This document serves as the canonical reference for the AI Agent system architecture at this specific commit. It details the philosophy, structure, and mechanisms that govern agentic interaction within the Manyfold Processor project.

---

## 1. Core Philosophy: Defensive Orchestration

**Principle**: "Agentic 'vibecoding' introduces broken logic, missed concepts, and bloat. The system must defend itself against the agent."

Defined in [Agentic Philosophy](.agent/skills/agentic_philosophy/SKILL.md), this mandates:
1.  **Ghost in the Machine**: The `.agent/` directory is an operating system, not a config folder.
2.  **Explicit Context**: Agents must load context explicitly via `requires:` frontmatter.
3.  **Governance Integration**: Every new feature must be audited by a companion tool/workflow.

---

## 2. Directory Structure

```
.agent/
├── annex/             # Historical context ("Why we failed")
├── metrics/           # Cost tracking logs (CSVs)
├── skills/            # Active knowledge ("How to do X")
│   ├── governance_integration/
│   │   ├── SKILL.md
│   │   └── examples/  # Reference implementation
│   └── ...
├── tools/             # Rust binaries (Headless Audits)
│   ├── src/bin/
│   └── scripts/
└── workflows/         # Procedure definitions (.md)
    ├── atomics/       # Single-purpose checks
    └── suites/        # Orchestrated logic
```

---

## 3. The Self-Healing Cycle

The system is designed to detect and correct drift automatically.

| Phase | Mechanism | Tooling |
|-------|-----------|---------|
| **1. Validation** | Pre-commit hooks run audits | `run_full_audit.sh` |
| **2. Detection** | Tools flag structural issues | `audit_dependencies`, `check_links` |
| **3. Correction** | Sentinels update state | `sentinel_catalog` |
| **4. Integration** | Workflows guide coverage | `/feature_governance_integration` |

**Example**:
- If a user adds a skill but forgets to list it in the catalog, `sentinel_catalog` detects the new file and auto-updates `project_workflows/SKILL.md`.

---

## 4. Token Efficiency Strategy

**Goal**: Minimize context cost ("The Invisible Tax") while maximizing correctness.

### Hybrid Mode Protocol
Workflows split tasks into two types:
1.  **🔧 Script (Headless)**: Deterministic checks (file existence, regex, links). **Cost: ~0 tokens**.
2.  **🧠 Agent (Semantic)**: Understanding intent, code logic, design quality. **Cost: Variable**.

### Artifacts vs. Tools
- **Tools (Rust)**: High-speed, zero-cost structural validation.
- **Skills/Workflows (Markdown)**: High-context semantic guidance.

**Optimization**:
- `check_workflow_skip`: Analyzes git diffs to skip entire audit steps if dependencies haven't changed.
- `check_context`: Soft-warns if files exceed token budget (e.g., >200 lines).

---

## 5. Governance Integration Framework

**Problem**: New features introduce new gaps (e.g., "Skill Examples" added a new folder structure).

**Solution**:
1.  **Trigger**: User invokes `/feature_governance_integration`.
2.  **Analysis**: Workflow guides decision (Script vs Agent).
3.  **Implementation**: Create new tool (`check_skill_examples.rs`) + workflow (`audit_skill_examples.md`).
4.  **Integration**: Add to `run_full_audit.sh` and catalogs.

**Self-Application**:
The workflow now includes a "Recursive Check" phase, requiring new artifacts to pass their own audits before completion.

---

## 6. Key Components

### Skills (Knowledge)
- **Strategy vs. Reference**: `SKILL.md` (Strategy) is separated from `examples/` (Reference) to keep context light.
- **Requires Frontmatter**: `requires: [dep1, dep2]` enforces dependency loading.

### Workflows (Action)
- **Atomic**: Does one thing (e.g., `audit_deps`).
- **Suite**: Orchestrates atomics (e.g., `suite_full_audit`).
- **Sentinel**: Runs automatically/silently (`maintenance_cleanup`).

### Tools (Enforcement)
- **Rust-based**: `audit_dependencies`, `check_links`, `check_context`.
- **Pre-commit**: `run_full_audit.sh` acts as the final gatekeeper.

---

## 7. Warning Resolution

When tools flag issues that are intentional (e.g., a large file that *needs* to be large):
1.  **Analyze**: Use `Warning_Resolution_Template.md`.
2.  **Suppress**: Add `<!-- context_warning_reviewed: HASH -->`.
3.  **Versioning**: If file changes, hash mismatches → warning reappears.

---

## 8. Status at Commit `98a8840`

- **System Health**: ✅ HEALTHY (All audits passing).
- **Latest Feature**: Skill Examples & Governance Integration.
- **Known Gaps**: See `notes/governance_integration_gaps.md` (Sentinel pattern, Catalog validation).

---

**See Also**:
- `.agent/skills/agentic_philosophy/SKILL.md`
- `.agent/skills/governance_integration/SKILL.md`
- `docs/Development_Workflow.md`
