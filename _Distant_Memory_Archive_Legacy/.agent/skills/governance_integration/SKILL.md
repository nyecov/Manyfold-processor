---
name: Governance Integration
description: Decision framework for ensuring new features are covered by the self-healing audit system.
requires: [agentic_philosophy, project_workflows]
---

# Governance Integration

<!-- audited_by: .agent/workflows/audit_dependencies.md -->

When new features, patterns, or structural changes are introduced, they must be integrated into the project's self-healing governance system. This skill defines the decision framework for that process.

See [examples/skill_examples_case.md](examples/skill_examples_case.md) for a real-world application of this framework.

## Core Principle

> "Every new feature is a potential governance gap until proven otherwise."

New additions can introduce:
- **Unvalidated file structures** (examples, configs, new directories)
- **Unchecked linking patterns** (new cross-references)
- **Unmonitored conventions** (naming, format standards)

Without audit coverage, these drift silently.

---

## Decision Framework

### 1. Gap Detection Prompt
The canonical trigger for governance analysis:

> "*[Feature X]* is a new addition. Can the current workflows and tools deal with it?"

### 2. Coverage Audit
For each new feature, check existing coverage:

| Audit Type | Example Tools |
|------------|---------------|
| Structural | `check_links`, `check_context` |
| Semantic | `check_consistency`, `audit_dependencies` |
| Format | `check_gherkin`, `check_infrastructure` |

### 3. Token Efficiency Classification
Determine what can be automated:

| Check Type | Scriptable? | Reasoning |
|------------|-------------|-----------|
| File exists | ✅ Yes | Deterministic FS operation |
| Link valid | ✅ Yes | Path resolution |
| Format correct | ⚠️ Partial | Regex for structure, agent for semantics |
| Content quality | ❌ No | Requires understanding |

**Rule**: Maximize headless coverage. Agent-only for true semantic analysis.

---

## Integration Requirements

When a gap is identified, create:

1. **Headless Tool** (if scriptable) → `.agent/tools/src/bin/check_{feature}.rs`
2. **Audit Workflow** → `.agent/workflows/audit_{feature}.md`
3. **Documentation** → `docs/changelogs/{Feature}_Process.md`

### Checklist
- [ ] Tool registered in `Cargo.toml`
- [ ] Tool added to `run_full_audit.sh`
- [ ] Workflow added to `project_workflows/SKILL.md`
- [ ] Process documented

---

## Warning Resolution

When audits produce warnings (not failures), a structured analysis determines the correct response.

### Decision Logic
1. **Does the default pattern apply?** — If yes, follow it
2. **Are there serious drawbacks to ignoring?** — If yes, must fix
3. **Is the warning form over function?** — Would fixing satisfy the rule but degrade usability?

See [examples/warning_resolution_template.md](examples/warning_resolution_template.md) for the full template.

**Real-world example**: [Context Warning Resolution](../../../docs/changelogs/Context_Warning_Resolution.md) — architectural_guidelines.md at 152 lines.

---

## Foundation

This skill implements the **Defensive Orchestration** principle from [Agentic Philosophy](../agentic_philosophy/SKILL.md):

> "Agentic 'vibecoding' can introduce broken logic, missed concepts, and bloat at every step."

Governance integration is the mechanism that prevents drift from accumulating.

---

## Examples
- [examples/skill_examples_case.md](examples/skill_examples_case.md) — Feature integration case study
- [examples/warning_resolution_template.md](examples/warning_resolution_template.md) — Warning analysis template

---

## See Also
- [Agentic Philosophy](../agentic_philosophy/SKILL.md) – Why we govern
- [Project Workflows](../project_workflows/SKILL.md) – Available audits
- [/feature_governance_integration](../../workflows/feature_governance_integration.md) – Step-by-step procedure
