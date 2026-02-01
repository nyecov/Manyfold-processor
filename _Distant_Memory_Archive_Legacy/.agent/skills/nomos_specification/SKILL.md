---
name: Nomos Specification
description: The origin, purpose, and separation strategy for Nomos (the Generalized AI Agent).
requires:
  - agentic_philosophy
  - project_details
---

# Nomos Specification

**Nomos** is the generalized AI agent framework that evolved from the `Manyfold-processor` project.

*   **Repository**: [https://github.com/nyecov/Nomos](https://github.com/nyecov/Nomos)

## 1. Origin Story
*   **Genesis**: Originally, the AI agent tooling was built specifically to develop the [Manyfold-processor](https://github.com/nyecov/Manyfold-processor).
*   **Evolution**: As the processor grew, the agentic tooling (Governance, Self-Healing, BDD automation) proved to be powerful independent of the specific business logic.
*   **Schism**: In February 2026, the specific processor logic was decoupled from the generalized agentic core. The agentic core became **Nomos**.

## 2. Mission & Identity
*   **Name**: Nomos (Greek: *Law, Custom, Convention*).
*   **Purpose**: To provide a generalized, self-governing, and self-healing framework for AI-assisted software development.
*   **Key Capabilities**:
    *   **Defensive Orchestration**: Enforcing strict rules to prevent "Agentic drift".
    *   **Self-Healing**: Automated audits and corrections (The "Self-Healing Cycle").
    *   **Dual-Track Verification**: Integrating BDD (Cucumber) with low-level logic.
    *   **Hybrid Execution**: Combining cheap Rust binaries with semantic LLM intelligence.

## 3. Separation Strategy
The goal of this repository (`nyecov/Nomos`) is to decouple the **Manyfold-processor** specific parts from the **Nomos** core.

| Component | Status | Action |
| :--- | :--- | :--- |
| **Agent Core** | ✅ **Keep & Enhance** | The `.agent/` directory, workflows, and Rust tools. |
| **Processor Logic** | ⏳ **Decouple** | `src/` code specific to 3D processing (STL, Geometry). |
| **Tests** | 🔄 **Refactor** | Keep the BDD *framework* (`cucumber_runner.rs`, `gherkin_style_guide`), but genericize the *content*. |

## 4. Usage in Other Projects
Nomos is designed to be a "plug-in" supervisor for other repositories.
*   **Role**: Quality Assurance, Process Enforcement, and Automated Refactoring.
*   **Maintenance**: This repository (`nyecov/Nomos`) is the upstream source. Improvements here should flow to downstream projects.

## 5. Roadmap
1.  **Extract Core**: Isolate the `.agent` system and Rust tools into a standalone package.
2.  **Generalize Tests**: Replace specific 3D model tests with generic examples (or move 3D tests to an "Example" folder).
3.  **Governance API**: Define a clean interface for adding Nomos to a new project (e.g., `nomos init`).

## See Also
*   **Philosophy**: [agentic_philosophy](../agentic_philosophy/SKILL.md)
*   **Original Project**: [project_details](../project_details/SKILL.md)
