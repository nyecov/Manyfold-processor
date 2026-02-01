---
description: Audit skill examples for existence, format, and proper linking from SKILL.md files.
---

# Atomic Audit: Skill Examples

This workflow validates that skill examples are properly structured and linked.

<!-- depends: .agent/skills -->

---

## Execution Protocol

> [!NOTE]
> **Hybrid Mode**: This workflow uses both headless scripts (🔧) and agent analysis (🧠).

<!-- depends: .agent/tools/src/bin/check_skill_examples.rs -->

### 🔧 Step 1: Headless Example Validation
```powershell
.agent\tools\target\release\check_skill_examples.exe
```
**Covers**: Missing examples, empty files, orphan detection

*   **If `[OK]`**: Proceed to Agent step for format validation.
*   **If `[XX]`**: Fix issues, then re-run.

---

### 🧠 Step 2: Validate Example Format (AGENT-ONLY)
> Script cannot do this — requires semantic understanding.

Each example file should contain:
- [ ] Header with pattern name
- [ ] Code blocks with language tags
- [ ] (Optional) Project Reference section linking to actual implementation

**Action**: Report malformed examples.

---

---

## Report Template

| Skill | Issue | Source |
|-------|-------|--------|
| `frontend_patterns` | Missing example | 🔧 Script |
| `backend_patterns` | Bad format | 🧠 Agent |
