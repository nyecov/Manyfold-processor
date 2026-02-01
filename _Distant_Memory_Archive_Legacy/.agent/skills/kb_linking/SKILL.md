---
name: Knowledge Base Linking
description: Standards for internal documentation linking using relative paths and structured relationship types.
requires:
  - project_workflows  # Workflow orchestration context
---

# Knowledge Base Linking Standards

<!-- audited_by: .agent/workflows/audit_dependencies.md -->
<!-- audited_by: .agent/workflows/audit_consistency.md -->
<!-- audited_by: .agent/workflows/maintenance_links.md -->

To ensure the project's documentation remains robust, portable, and easily navigable across different environments and AI agent instances, all internal links must adhere to the following standards.

## 1. The Core Mandate: Relative Paths Only
*   **Absolute Paths are Forbidden**: Never use paths starting with `C:\`, `/home/`, or `file:///C:/`.
*   **Standard**: Use relative paths starting from the current file's location.
    *   *Correct*: `[Architectural Guidelines](../architectural_guidelines/SKILL.md)`
    *   *Incorrect*: `[Architectural Guidelines](file:///C:/Users/.../SKILL.md)`
*   **Rationale**: Absolute paths break when the project is moved or accessed from within a container.

## 2. Linking Logic: Vertical, Lateral, and Contextual

### Vertical Linking (Hierarchy)
Links between different levels of abstraction or abstraction types.
*   **Downward**: High-level concepts linking to detailed implementation or research.
    *   *Example*: `architectural_guidelines` linking to `geometry_converter.py`.
*   **Upward**: Detailed files linking back to their governing design documents or skills.
    *   *Example*: A test file linking to the `testing_and_verification` skill.

### Lateral Linking (Peer-to-Peer)
Links between documents at the same level of the hierarchy sharing functional or thematic overlaps.
*   **Skill-to-Skill**: Links between peer technical guides.
    *   *Example*: `stl_handling` <-> `3mf_handling`.
*   **Doc-to-Doc**: Links between research or analysis files.
    *   *Example*: `Language_Analysis.md` <-> `File_Format_Analysis.md`.

### Contextual Linking (High Relevance)
Links based on shared specific context that may not follow standard vertical or lateral paths.
*   **Usage**: When a document mentions a specific bug, hardware edge case, or specific decision that is detailed elsewhere.
*   **Constraint**: Use only when the connection is critical for understanding the current topic and not covered by standard roles.

## 3. Agent-Effective Patterns (v0.3+)

### The `requires:` Field
Skills should declare explicit dependencies in YAML frontmatter:
```yaml
---
name: My Skill
description: ...
requires:
  - environment_constraints  # Memory values
  - another_skill            # Why it's needed
---
```
**Agent Benefit**: Enables automated context loading.

### Canonical Constants Reference
For shared values (memory limits, IPs, ports), reference the canonical source:
```
.agent/constants.yml
```
**Pattern**: Don't duplicate values in skills—reference `constants.yml`.

---

## See Also
*   **Historical Context**: [annex](../../annex/README.md)
*   **Constants**: [constants.yml](../../constants.yml)
*   **Project Management**: [project_workflows](../project_workflows/SKILL.md)
