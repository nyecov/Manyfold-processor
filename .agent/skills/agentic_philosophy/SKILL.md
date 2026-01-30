---
name: Agentic Philosophy
description: Core governing philosophy for AI-agent-driven development (Defensive Orchestration).
requires: [project_details, project_workflows]
---

# Agentic Engineering Philosophy

<!-- audited_by: .agent/workflows/audit_context.md -->

> "Agentic 'vibecoding' introduces broken logic, missed concepts, and bloat at every step—*even with human supervision*."

This skill codifies the **Defensive Orchestration** paradigm that governs how AI agents interact with this project.

## 🧠 Core Philosophy

### 1. The "Ghost in the Machine" Strategy
The `.agent/` folder is not config—it is a **Ghost Operating System** that runs parallel to user code.
*   `constants.yml` is **Law**, not Suggestion.
*   Agents must read and obey constraints before proposing changes.

### 2. The "Annex" Strategy (Combating Knowledge Rot)
*   **Active** (`.agent/skills/`): "How to do X".
*   **Historical** (`.agent/annex/`): "Why we tried Y and it failed".
*   Separation prevents confusion and re-inventing failed approaches.

### 3. The "Meta-Governance" Strategy
*   Workflows audit workflows.
*   Complexity breeds entropy; automation checks are mandatory.
*   Example: `/audit_workflows` verifies `/audit_consistency` is logically sound.

---

## 📐 Design Rules

| Rule | Principle |
|------|-----------|
| **Diátaxis for Machines** | "Fail if X" not "You might want to check X". |
| **Explicit Dependencies** | `requires:` frontmatter forces context loading. |
| **Atomic Workflow Design** | Atomics do one thing; Suites orchestrate Atomics. |
| **Rust First** | Complex analysis in Rust. Shell/Python only for trivial scans. |

---

## 🔄 The Self-Healing Cycle

| Level | Strategy |
|-------|----------|
| **Reactive** | Run `/suite_full_audit`. Fix detected drift. |
| **Proactive** | Sentinels auto-update catalogs on file creation. |
| **Pre-Emptive** | Simulated audits block commits that degrade health. |

---

## 💰 Token Efficiency (Hybrid Mode)

Workflows use a **Hybrid Model** where scripts handle atomic checks and agents handle semantic analysis.

### Workflow Step Types
| Marker | Type | Cost | Coverage |
|--------|------|------|----------|
| 🔧 | Script | ~0 tokens | Atomic, structural checks |
| 🧠 | Agent-Only | Variable | Semantic, cross-reference analysis |

### Correctness Over Efficiency
> **Mandate**: Scripts ONLY substitute atomic steps they can actually perform. Never claim a script covers semantic analysis — correctness trumps token savings.

### Protocol
1.  Run 🔧 Script step (cheap, ~0 tokens).
2.  Review script output for atomic findings.
3.  Proceed to 🧠 Agent steps regardless of script result.
4.  Agent performs semantic analysis that scripts cannot.

**Location**: All governance scripts reside in `.agent/tools/`.

### Cache-Based Skip Pattern

For expensive agent-only workflows, a **hash-based cache** can avoid re-execution:

1.  **Pre-Check Script**: Computes content hash, compares to cache
2.  **If Match**: Skip workflow, return cached result (~0 tokens)
3.  **If Mismatch**: Full agent execution, then write new cache
4.  **Guarantee**: Script only checks hash — cannot make semantic decisions

This pattern applies to:
*   `/audit_tool_alignment` — Root workflow, costs ~27,000 tokens per run
*   Uses `check_tool_alignment_skip.exe` for hash comparison
*   Writes to `.agent/tools/.audit_cache` after successful audit

---

## Related Links
*   [Project Workflows](../project_workflows/SKILL.md)
*   [Environment Constraints](../environment_constraints/SKILL.md)
*   [Testing Philosophy](../testing_philosophy/SKILL.md)
*   [Agent Tools](../../tools/Cargo.toml) — Rust binaries for token-efficient audits
