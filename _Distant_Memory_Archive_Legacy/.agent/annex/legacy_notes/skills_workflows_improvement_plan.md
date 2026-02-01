# Skills & Workflows Improvement Plan

> **Philosophy**: This plan follows the [Agentic Philosophy](../.agent/skills/agentic_philosophy/SKILL.md) (Defensive Orchestration).

**Created**: 2026-01-30
**Status**: Active backlog for v0.4+

---

## Observations: AI Agent Effectiveness

Based on practical experience with the current skill/workflow architecture, here's what helps the AI agent navigate and use the documentation effectively.

### ✅ What Works Well

| Pattern | Why It Helps |
| :-- | :-- |
| **YAML Frontmatter** (`name:`, `description:`) | Read by system at session start; agent *knows* skills exist |
| **Relative Path Links** | Agent can programmatically follow with `view_file` |
| **"See Also" Sections** | Clear pointers to related context |
| **Skills in `/skills/` directory** | Auto-discovered and listed in agent context |

### ⚠️ Limited Effectiveness

| Pattern | Issue |
| :-- | :-- |
| **Inline `# [twin:]` Annotations** | Agent doesn't auto-parse; mainly for human reviewers/tooling |
| **Dense Link Webs** | Noise if links aren't actively followed |
| **Implicit Dependencies** | Agent must infer relationships (e.g., "this skill uses values from...") |

### 💡 Proposed Improvements

#### 1. Explicit Dependency Declarations
Add a `requires:` field in YAML frontmatter:
```yaml
---
name: Deployment Operations
description: ...
requires:
  - environment_constraints  # Memory/resource values
  - deploy_on_radxa_rock5    # Target hardware specifics
---
```
**Benefit**: Agent can auto-load dependencies when entering a skill context.

#### 2. Canonical Constants File
Create a single source of truth for shared values:
```
.agent/constants.yml
```
```yaml
memory:
  reservation: 1G
  limit: 5G
ports:
  web_ui: 8080
target_hardware:
  ip: 192.168.2.2
  ssh_user: dev
```
**Benefit**: Skills reference this file; eliminates Double-Source-of-Truth risk.

#### 3. Skill Context Loading Directive
Add a `context:` field for skills that should be auto-loaded together:
```yaml
context:
  - environment_constraints  # Always load this with me
```

#### 4. Structured Twin Annotations for Tooling
Move `[twin:]` from inline comments to a structured format:
```yaml
# In feature file header or separate manifest
twins:
  - ui_step: "Given the Manyfold Processor service is running"
    api_step: "Given_API the Manyfold Processor service is running"
```

---

## Priority Actions

| Priority | Action | Effort |
| :--: | :-- | :--: |
| 🔴 | Add `requires:` field to skills | Low |
| 🟠 | Create `constants.yml` for shared values | Medium |
| 🟡 | Formalize twin-link manifest for features | Low |
| 🟢 | Document pattern in `kb_linking` skill | Low |

---

## Related Files
*   [kb_linking](/.agent/skills/kb_linking/SKILL.md) - Current linking standards
*   [project_workflows](/.agent/skills/project_workflows/SKILL.md) - Workflow architecture
*   [testing_philosophy](/.agent/skills/testing_philosophy/SKILL.md) - Twin-linking context
