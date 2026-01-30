---
name: Project Workflows
description: Guide to available Antigravity Workflows and maintenance procedures for the project.
---

# Project Workflows

This project utilizes **Antigravity Workflows** (stored in `.agent/workflows/`) to automate routine maintenance and verification tasks.

## 🚀 Available Workflows

### 1. Check Documentation Consistency
*   **Command**: `/check_docs_consistency`
*   **File**: `.agent/workflows/check_docs_consistency.md`
*   **Purpose**: Scans `docs/` and `.agent/skills/` to identify:
    *   Logical conflicts (e.g., conflicting language mandates).
    *   Missing details or placeholders.
    *   Inconsistencies in hardware/deployment rules.
*   **When to use**: Run this before finalizing any major documentation update or architectural shift.

### 2. Check Implementation Alignment
*   **Command**: `/check_implementation_alignment`
*   **File**: `.agent/workflows/check_implementation_alignment.md`
*   **Purpose**: Verifies that the actual codebase matches architectural mandates while ignoring missing features.
*   **When to use**: Run this when validating the project's health or after significant code changes.

## 🛠️ Maintenance Procedures

### Adding New Workflows
1.  Create a `.md` file in `.agent/workflows/`.
2.  Add a YAML frontmatter with a `description`.
3.  Define the step-by-step instructions.
4.  Update this skill (`project_workflows`) to include the new workflow.

### Updating Skills
*   Always ensure new findings (e.g., from Language Analysis) are reflected in the relevant skills (`architectural_guidelines`, `deploy_on_radxa_rock5`).
*   Use `/check_docs_consistency` to verify changes.
