---
name: Agentic Philosophy
description: Core governing philosophy for AI-agent-driven development (Defensive Orchestration).
requires: [project_details, project_workflows]
---

# Agentic Engineering Philosophy

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

## 💰 Token Efficiency (Hybrid Model)

*   **Workflow (Prompt)**: Manager. Decides *what* to do.
*   **Script (Tool)**: Worker. Executes *how* to do it efficiently.

> **Mandate**: Substitute as many recurring processes as possible with locally-running atomic scripts to save tokens. The Agent orchestrates; the Script serves.

**Protocol**:
1.  Run script headless (cheap).
2.  If fail, re-run with `-v` (expensive but necessary).
3.  Fallback to manual analysis if scripts are broken.

**Location**: All governance scripts reside in `.agent/tools/`.

---

## Related Links
*   [Project Workflows](../project_workflows/SKILL.md)
*   [Environment Constraints](../environment_constraints/SKILL.md)
*   [Testing Philosophy](../testing_philosophy/SKILL.md)
