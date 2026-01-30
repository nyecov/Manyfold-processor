---
name: Project Workflows
description: Guide to available Antigravity Workflows (Atomics and Suites) for the project.
---

# Project Workflows

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

## 🚀 Atomic Standalones (Basics)

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
