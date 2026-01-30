---
name: Project Workflows
description: Guide to available Antigravity Workflows (Atomics and Suites) for the project.
---

# Project Workflows

<!-- audited_by: .agent/workflows/audit_dependencies.md -->
<!-- audited_by: .agent/workflows/audit_context.md -->

This project utilizes a hierarchical suite of **Atomics** (independent standalones) and **Suites** (orchestrators) to automate maintenance and verification.

## 🏗️ Workflow Architecture
*   **Atomic Standalones**: Specialized tools with **zero dependencies**. They never call other workflows.
*   **Orchestrated Suites**: High-level tools that invoke multiple Atomics for comprehensive audits.

## 🧠 Philosophy: Defensive Orchestration
> "Agentic 'vibecoding' can introduce broken logic, missed concepts, and bloat at every step—even with human supervision."

To counteract the risks of high-velocity agentic changes (drift, hallucination, complexity):
*   **Meta-Governance**: We build workflows that govern other workflows (e.g., `/audit_workflows`).
*   **Deep Self-Checking**: Every layer (Docs, Tests, Logic, Infra) has a dedicated auditor.
*   **Skepticism as Default**: "Human supervision is necessary but insufficient."

> See [Agentic Philosophy](../agentic_philosophy/SKILL.md) for the complete paradigm.

---

## � Root Workflow (Execute First)

> [!CAUTION]
> This workflow MUST be executed before any audit or maintenance workflow that uses "Headless First".

*   **`/audit_tool_alignment`**: **[AGENT-ONLY]** Verifies `.agent/tools/` scripts align with workflows. **No downstream workflow is valid until this passes.**

---

## �🚀 Atomic Standalones (Basics)

### Audit Tools (Passive)
*   **`/audit_context`**: Evaluates semantic organization (Strategy vs. Reference).
*   **`/audit_code_quality`**: Performs code formatting, linting, and security checks.
*   **`/audit_consistency`**: Checks logical integrity across the knowledge base.
*   **`/audit_gherkin`**: Verifies BDD scenario quality and layering.
*   **`/audit_step_glue`**: Verifies alignment between Gherkin and Rust.
*   **`/audit_infrastructure`**: Checks codebase/Docker against design mandates.
*   **`/audit_constants`**: Checks for hardcoded values vs `constants.yml`.
*   **`/audit_dependencies`**: Validates `requires:` fields in skills.
*   **`/audit_workflows`**: Governs workflow library integrity and redundancy.
*   **`/audit_tool_alignment`**: **[AGENT-ONLY]** Meta-audit ensuring tools align with docs/workflows.

### Maintenance Tools (Active)
*   **`/maintenance_links`**: Synchronizes relative links across the project.
*   **`/maintenance_c4`**: Updates Mermaid-based C4 architecture diagrams.
*   **`/maintenance_annex_migration`**: Audits skills/docs/notes for content to archive.
*   **`/maintenance_annex_review`**: Review annex content for candidate retrieval (move to active) or staleness updates.

---

## 🏛️ Orchestrated Suites (Compounded)

*   **`/suite_docs`**: Performs a full documentation audit (Context + Consistency).
*   **`/suite_tests`**: Performs a full testing stack audit (Gherkin + Glue).
*   **`/suite_full_audit`**: The ultimate project health check (Docs + Tests + System).

---

## 🛠️ Maintenance Procedures

### Adding Workflows
1.  **Atomic**: Must be self-contained; no `/command` calls.
2.  **Suite**: Can orchestrate Atomics via `/command` calls.
3.  Update this skill catalog accordingly.

### Standards
*   Always use relative paths in documentation.
*   Follow the "Strategy vs. Reference" semantic model.

---

## ⚡ Hybrid Mode Tools

Workflows use scripts for **atomic checks** while agents handle **semantic analysis**.

### Binary Registry
| Tool | Workflow | 🔧 Covers |
|------|----------|-----------|
| `audit_dependencies.exe` | `/audit_dependencies` | Broken `requires:` links |
| `check_gherkin.exe` | `/audit_gherkin` | Scenario count >5 |
| `check_links.exe` | `/maintenance_links` | Absolute paths |
| `check_consistency.exe` | `/audit_consistency` | TODOs/TBDs/FIXMEs |
| `check_constants.exe` | `/audit_constants` | Known magic values |
| `check_context.exe` | `/audit_context` | File sizes, structure |
| `check_infrastructure.exe` | `/audit_infrastructure` | compose/Dockerfile presence |
| `check_workflow_skip.exe` | (Meta) | Git change detection |

### Workflow Format
```markdown
### 🔧 Step 1: Headless [Check]
  → Script output

### 🧠 Step 2: [Analysis] (AGENT-ONLY)
  → Agent performs semantic analysis
```

### Key Principle
> Scripts cover **atomic** steps only. Agent-Only steps are **never skipped** — correctness over efficiency.

### Feedback Requirements
> [!IMPORTANT]
> **Mandatory Feedback**: After completing each atomic step, the agent MUST provide a brief status update to the user:
> *   ✅ **PASS** or ❌ **FAIL** indicator
> *   Summary of findings (1-2 lines max)
> *   Any blocking issues requiring attention
>
> This ensures visibility into workflow progress and allows early intervention if issues arise.

---

## See Also
*   **Historical Context**: [annex](../../annex/README.md)
*   **Linking Standards**: [kb_linking](../kb_linking/SKILL.md)
*   **Agent Tools**: [tools](../../tools/Cargo.toml) — Headless First binaries
*   **Philosophy**: [agentic_philosophy](../agentic_philosophy/SKILL.md)
