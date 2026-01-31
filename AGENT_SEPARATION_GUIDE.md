# Agent Separation Guide

**Author**: Claude Opus 4.5  
**Date**: 2026-01-31

This document explains the separation of the **AI Agent** (orchestration framework) from the **Manyfold Processor** (application code).

---

## Philosophy of Separation

The AI Agent is a **reusable framework**. The Manyfold Processor is a **specific implementation**.

| Component | Repository | Purpose |
|-----------|------------|---------|
| **Agent Skeleton** | `Agent_Skeleton/` | Generic orchestration, skills, workflows |
| **Manyfold Processor** | `Manyfold-processor/` | 3D model processing application |

---

## What Was Extracted?

### ✅ Core (Included in Skeleton)

These are **generic** and apply to any project:

| Skill | Reason |
|-------|--------|
| `agentic_philosophy` | Core framework philosophy |
| `gherkin_style_guide` | BDD methodology (language-agnostic) |
| `kb_linking` | Documentation standards |
| `project_details` | **Template** for project metadata |
| `project_workflows` | Workflow organization guide |
| `research_and_fallback_strategies` | Agent behavior for web search |
| `testing_philosophy` | BDD strategy |

### ❌ Project-Specific (Excluded)

These are **Manyfold-specific** and remain in the original project:

| Skill | Reason |
|-------|--------|
| `3mf_specification` | 3D printing domain |
| `stl_specification` | 3D printing domain |
| `geometry_governance` | Manyfold business logic |
| `manyfold_api_endpoints` | Manyfold API specifics |
| `manyfold_reference_material` | Manyfold internal docs |
| `deploy_on_radxa_rock5` | Target hardware specific |
| `deployment_operations` | Manyfold container deployment |
| `environment_constraints` | Manyfold memory/hardware limits |

### ⚙️ Templated (Reset in Skeleton)

| File | Change |
|------|--------|
| `constants.yml` | Generic placeholder values |
| `project_details/SKILL.md` | `[Project Name]` placeholders |

---

## How to Use the Skeleton

1.  **Clone** the `Agent_Skeleton` repo.
2.  **Copy** the `.agent/` directory into your new project.
3.  **Customize**:
    - `constants.yml` → Your memory limits, network config, versions.
    - `project_details/SKILL.md` → Your repo URL, maintainer, version.
4.  **Add** new skills as needed in `.agent/skills/[skill_name]/SKILL.md`.
5.  **Add** new workflows in `.agent/workflows/[workflow_name].md`.

---

## Rust Defaults

The skeleton includes Rust-oriented workflows (e.g., `cargo fmt`, `cargo clippy`). If your project uses a different language:

1.  Edit `.agent/workflows/audit_code_quality.md`.
2.  Replace Rust commands with your language's equivalents.
3.  Edit `.agent/skills/code_quality_standards/SKILL.md` if included.

---

## Maintaining Both Repositories

| Task | Agent Skeleton | Manyfold Processor |
|------|----------------|-------------------|
| Add core workflow | ✅ Add here, then sync | Sync from skeleton |
| Add domain skill | — | ✅ Add here only |
| Update philosophy | ✅ Update here | Sync from skeleton |

---

## Related Files

- [Agent Skeleton README](./Agent_Skeleton/README.md)
- [Original Agentic Philosophy](./`.agent/skills/agentic_philosophy/SKILL.md`)
