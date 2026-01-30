---
description: Meta-governance workflow to verify alignment between agent tools and documentation. AGENT-ONLY - No headless scripting allowed.
---

# Meta Audit: Tool-Documentation Alignment

> [!CAUTION]
> **ROOT WORKFLOW — EXECUTE FIRST**
> This workflow MUST be executed before any other audit or maintenance workflow.
> If `.agent/tools/` contains defects or misaligned scripts, all downstream workflows using "Headless First" will produce misleading results.

> [!CAUTION]
> **AGENT-ONLY WORKFLOW**: This workflow MUST be executed entirely by the AI agent. It must NEVER be replaced with a headless script. The analysis, correction, and verification require semantic understanding that only the agent can provide.

## Prerequisites

**None** — This is the root of the workflow dependency tree.

## Downstream Dependencies

All workflows with "Headless First" sections depend on this workflow passing:
*   `/audit_dependencies`
*   `/audit_gherkin`
*   `/audit_consistency`
*   `/audit_constants`
*   `/audit_context`
*   `/audit_infrastructure`
*   `/maintenance_links`
*   `/suite_full_audit`

---

This meta-workflow ensures that `.agent/tools/` scripts are properly integrated, documented, and referenced across all project documentation.

---

## 1. Inventory: Tool Catalog

### 1.1 List All Binaries
*   Read `.agent/tools/Cargo.toml` and extract all `[[bin]]` entries.
*   List all files in `.agent/tools/src/bin/`.
*   **Report**: Table of tool name, file path, and purpose (from code comments).

### 1.2 List All Shell Scripts
*   Scan `.agent/tools/scripts/` for `.sh`, `.py`, `.ps1` files.
*   **Report**: Table of script name and purpose.

---

## 2. Alignment Check: Tools ↔ Workflows

### 2.1 For Each Tool Binary
*   Search `.agent/workflows/*.md` for references to the tool.
*   **Pass**: Tool is referenced in at least one workflow's "Headless Execution" section.
*   **Fail**: Tool exists but no workflow uses it → Flag as "Orphan Tool".

### 2.2 For Each Workflow with Headless Section
*   Verify the referenced tool binary actually exists in `.agent/tools/target/release/`.
*   **Pass**: Binary exists and is executable.
*   **Fail**: Binary path is outdated or broken → Flag as "Broken Reference".

---

## 3. Alignment Check: Tools ↔ Skills

### 3.1 Agentic Philosophy Reference
*   Verify `.agent/skills/agentic_philosophy/SKILL.md` mentions `.agent/tools/` as the script location.
*   **Pass**: Location is correctly documented.
*   **Fail**: Location is missing or incorrect.

### 3.2 Project Workflows Reference
*   Verify `.agent/skills/project_workflows/SKILL.md` mentions the tool catalog.
*   **Pass**: All tools are represented in the workflow catalog.
*   **Fail**: New tools not reflected in catalog.

---

## 4. Alignment Check: Tools ↔ Docs & Notes

### 4.1 Token Efficiency Milestone
*   Verify `notes/token_efficiency_milestone.md` lists all current tools.
*   **Pass**: All tools documented.
*   **Fail**: New tools not reflected → Update required.

### 4.2 Improvement Plan
*   Verify `notes/token_efficiency_improvement_plan.md` reflects current Phase status.
*   **Pass**: Phase tasks match completed tools.
*   **Fail**: Plan is outdated.

---

## 5. Code-Workflow Semantic Alignment (CRITICAL)

> [!IMPORTANT]
> This step requires the agent to **read and understand the Rust source code** of each tool and compare it against the workflow requirements.

### 5.1 For Each Tool Binary

1.  **Read the source code** from `.agent/tools/src/bin/[tool_name].rs`.
2.  **Read the corresponding workflow** from `.agent/workflows/[workflow_name].md`.
3.  **Compare and verify**:

| Check | What to Verify |
|-------|----------------|
| **Purpose Match** | Does the script's `main()` logic match the workflow's stated goal? |
| **Patterns Checked** | Do the hardcoded patterns/values in the script match workflow requirements? |
| **Output Format** | Does the script's output (`[OK]`, `[XX]`) match workflow expectations? |
| **Exit Codes** | Does the script use `exit(0)` for pass and `exit(1)` for fail? |
| **Directories Scanned** | Does the script scan the same directories the workflow lists? |

### 5.2 Mapping Table

| Tool | Workflow | Key Logic to Verify |
|------|----------|---------------------|
| `audit_dependencies.rs` | `audit_dependencies.md` | Scans `.agent/skills/`, parses `requires:` YAML |
| `check_gherkin.rs` | `audit_gherkin.md` | Checks `tests/`, counts `Scenario:`, limit = 5 |
| `check_links.rs` | `maintenance_links.md` | Scans `.agent/`, `docs/`, `notes/`, `tests/`, flags `C:/` |
| `check_consistency.rs` | `audit_consistency.md` | Scans for `TODO`, `TBD`, `FIXME`, `PLACEHOLDER` |
| `check_constants.rs` | `audit_constants.md` | Checks against `constants.yml` magic values |
| `check_context.rs` | `audit_context.md` | Checks skill sizes, missing SKILL.md, annex README |
| `check_infrastructure.rs` | `audit_infrastructure.md` | Checks `compose.yml`, `Dockerfile`, `Cargo.toml` |

### 5.3 Drift Detection

For each tool-workflow pair:
*   **Pass**: Script logic implements all workflow requirements.
*   **Fail - Outdated Script**: Workflow requires checks the script doesn't perform.
*   **Fail - Outdated Workflow**: Script checks things not documented in workflow.
*   **Fail - Logic Mismatch**: Script uses different thresholds/patterns than workflow.

### 5.4 Correction Actions

| Failure Type | Action |
|--------------|--------|
| Outdated Script | Update Rust code to add missing checks |
| Outdated Workflow | Update workflow to document what script actually does |
| Logic Mismatch | Synchronize: prefer workflow as source of truth |

---

## 6. Correction Phase (AGENT REQUIRED)

For each failure detected:

### 5.1 Orphan Tool
*   Determine which workflow should use this tool.
*   Add "Headless First" section to that workflow.

### 5.2 Broken Reference
*   Fix the path in the workflow file.
*   Verify the binary compiles.

### 5.3 Missing Documentation
*   Update the relevant skill or note file.
*   Add tool to catalogs and milestones.

---

## 6. Verification

After corrections:
*   Re-run Steps 2-4 to confirm all checks pass.
*   **Final Report**: List of corrections made and verification status.

---

## Why No Headless Script?

| Aspect | Headless Script | Agent-Only |
|--------|-----------------|------------|
| **Semantic Analysis** | Cannot understand intent | Can infer purpose from code |
| **Correction** | Cannot edit files safely | Can make intelligent edits |
| **Context Awareness** | No knowledge of project structure | Full project context |
| **New Tool Discovery** | Must be pre-programmed | Can adapt to changes |

This workflow is the **governance layer** that ensures all other headless scripts remain valid.
