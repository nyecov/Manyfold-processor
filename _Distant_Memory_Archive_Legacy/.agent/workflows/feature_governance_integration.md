---
description: Process for analyzing new features against existing governance and integrating audit coverage.
---

# Feature Governance Integration Workflow

When a new feature, pattern, or structural change is introduced, use this workflow to ensure it's covered by the self-healing governance system.

<!-- depends: .agent/skills/governance_integration/SKILL.md -->

---

## Phase 1: Gap Analysis

### Trigger Prompt
> "**[Feature X]** is a new addition. Can the current workflows and tools deal with it?"

### Step 1: Identify Affected Components
- [ ] Which directories/files are affected?
- [ ] Does this change file structure conventions?
- [ ] Does this introduce new linking patterns?

### Step 2: Audit Current Coverage
| Component | Covers This? | Gap? |
|-----------|--------------|------|
| `/maintenance_links` | | |
| `/audit_dependencies` | | |
| `/audit_consistency` | | |
| `/audit_context` | | |
| `run_full_audit.sh` | | |

### Step 3: Determine Gap Severity
- **No Gap**: Existing tools already cover → Document and close
- **Minor Gap**: Existing tool can be extended → Update tool
- **Major Gap**: New workflow/tool needed → Proceed to Phase 2

---

## Phase 2: Token Efficiency Analysis

### For Each Required Check, Ask:

| Check Description | Scriptable? | Reasoning |
|-------------------|-------------|-----------|
| _e.g., "File exists"_ | ✅ Yes | File system operation |
| _e.g., "Format is correct"_ | ❌ No | Requires semantic analysis |

### Decision Matrix
| Scriptable Checks | Agent Checks | Mode |
|-------------------|--------------|------|
| All | None | 🔧 Headless Only |
| Some | Some | 🔧+🧠 Hybrid |
| None | All | 🧠 Agent Only |

---

## Phase 3: Implementation

### If Headless Tool Needed:
1. Create tool in `.agent/tools/src/bin/check_{feature}.rs`
2. Add binary to `Cargo.toml`
3. Build: `cargo build --release --bin check_{feature}`
4. Test: `.agent\tools\target\release\check_{feature}.exe`

### If Workflow Needed:
1. Create `.agent/workflows/audit_{feature}.md`
2. Use hybrid mode template if applicable
3. Add dependency comment: `<!-- depends: .agent/tools/... -->`

### Integration Checklist:
- [ ] Tool added to `Cargo.toml`
- [ ] Tool added to `run_full_audit.sh`
- [ ] Workflow registered in `project_workflows/SKILL.md`
- [ ] Process documented in `docs/changelogs/`

---

## Phase 4: Verification

```powershell
# Test the new tool
.agent\tools\target\release\check_{feature}.exe

# Run full audit to confirm integration
.agent\tools\scripts\run_full_audit.sh
```

### Success Criteria
- [ ] New tool passes on current codebase
- [ ] Full audit completes without errors
- [ ] Documentation is linked and discoverable

---

## Template: Hybrid Workflow

```markdown
---
description: [Brief description of what this audits]
---

# Atomic Audit: [Feature Name]

<!-- depends: .agent/tools/src/bin/check_{feature}.rs -->

## Execution Protocol

> [!NOTE]
> **Hybrid Mode**: This workflow uses both scripts (🔧) and agent analysis (🧠).

### 🔧 Step 1: Headless [Check Type]
\```powershell
.agent\tools\target\release\check_{feature}.exe
\```
**Covers**: [List deterministic checks]

*   **If `[OK]`**: Proceed to Agent step.
*   **If `[XX]`**: Fix issues, then re-run.

---

### 🧠 Step 2: [Semantic Analysis] (AGENT-ONLY)
> Script cannot do this — requires [reason].

**Action**: [What agent should do]
```

---

## Phase 5: Self-Application (Recursive Check)

> [!CAUTION]
> **Mandatory**: Every new skill/workflow must pass its own governance checks before completion.

### Run on New Artifacts
```powershell
# Verify dependencies declared
.agent\tools\target\release\audit_dependencies.exe

# Verify examples linked (if skill has examples)
.agent\tools\target\release\check_skill_examples.exe
```

### Self-Check Questions
- [ ] Does the new skill have `requires:` frontmatter?
- [ ] Is the new workflow/skill listed in `project_workflows/SKILL.md`?
- [ ] Does the skill link to its companion workflow (and vice versa)?
- [ ] Is there a parent skill that should reference this?

### If Any Fail
Fix the gaps before marking the integration complete. This step prevents the recursive prompting problem where users must ask "is it integrated?" multiple times.

---

## See Also
- [Governance Integration Skill](../skills/governance_integration/SKILL.md) — Decision framework
- [Agentic Philosophy](../skills/agentic_philosophy/SKILL.md) — Foundation principles
- [Project Workflows](../skills/project_workflows/SKILL.md) — Workflow catalog
