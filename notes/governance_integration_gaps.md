# Governance Integration Gap Fixes

Future improvements to make governance integration more automatic.

---

## Current State (2026-01-31)
- `/feature_governance_integration` is a **manual workflow**
- Relies on agent recognition or user invocation
- No automatic trigger when new skills/workflows are created

---

## Planned Improvements

### 1. Sentinel Pattern (High Complexity)
Auto-detect new skill/workflow files and prompt for governance check.

**Implementation**:
- Create `sentinel_new_artifacts.rs`
- Watch for new files in `.agent/skills/` and `.agent/workflows/`
- Output: `[NEW] skill_name - Run /feature_governance_integration`

### 2. Catalog Entry Validation (Medium Complexity)
Tool to verify skills/workflows are listed in `project_workflows/SKILL.md`.

**Implementation**:
- Create `check_catalog_entries.rs`
- Parse `project_workflows/SKILL.md` for listed items
- Compare against actual files in skills/workflows directories
- Flag unlisted items

### 3. Companion Pairing Check (Medium Complexity)
Verify skills have companion workflows and vice versa.

**Implementation**:
- Extend `audit_dependencies.exe`
- Check for `<!-- depends: ... -->` comments pointing to companion
- Warn if skill has no workflow reference or workflow has no skill reference

---

## Current Workaround
Added **New Artifact Gate** rule to `agentic_philosophy/SKILL.md`:
> "After creating any skill/workflow, invoke `/feature_governance_integration`."

This makes it a documented mandate rather than automated enforcement.

---

## Priority
1. **Catalog Entry Validation** — Most impactful, catches the gap we hit today
2. **Sentinel Pattern** — Nice to have, reduces reliance on agent memory
3. **Companion Pairing** — Edge case, low priority
