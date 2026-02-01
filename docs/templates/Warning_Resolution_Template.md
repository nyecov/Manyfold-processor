# Warning Resolution Template

A structured approach for analyzing and resolving audit warnings. Use this template when a warning requires human judgment rather than automatic fixing.

---

## 1. Warning Capture

```
[Paste exact warning output here]
```

**Source Tool**: _e.g., check_context, check_infrastructure_
**Severity**: ⚠️ WARN / ❌ FAIL
**Date Detected**: YYYY-MM-DD

---

## 2. Problem Analysis

### What is the tool checking?
> _Describe the rule or constraint being enforced_

### Why did it trigger?
> _Identify the specific condition that caused the warning_

### Content Breakdown
| Section | Lines | Classification |
|---------|-------|----------------|
| _section_ | X-Y | Strategy / Reference / Diagram / Other |

**Root Cause**: _Why does the content exceed the threshold?_

---

## 3. Resolution Analysis

### Option A: [Fix Approach]
| Pros | Cons |
|------|------|
| | |

**Verdict**: _Acceptable / Overkill / Required_

### Option B: [Alternative Approach]
| Pros | Cons |
|------|------|
| | |

**Verdict**: _Acceptable / Overkill / Required_

---

## 4. Self-Analysis (Decision Logic)

### Does the default pattern apply?
- [ ] Yes → Follow established pattern
- [ ] No → Explain why (content type, intent mismatch, etc.)

### Are there serious drawbacks to ignoring?
- [ ] Yes → Must fix
- [ ] No → Can suppress with documentation

### Is the warning form over function?
> _Would fixing satisfy the rule but degrade usability or coherence?_

### Decision Matrix
| Factor | Weight | Option A | Option B |
|--------|--------|----------|----------|
| Follows pattern | Low/Med/High | ✓/✗ | ✓/✗ |
| Content coherence | Low/Med/High | ✓/✗ | ✓/✗ |
| Serious drawbacks | Low/Med/High | ✓/✗ | ✓/✗ |

---

## 5. Corrective Action

**Selected Option**: _A / B / Hybrid_

**Reasoning Summary**:
> _1-2 sentences explaining why this option is correct_

**Action Taken**:
- [ ] Code/file modified
- [ ] Warning suppressed with inline comment
- [ ] Documentation added to changelogs

**Inline Comment (if suppressing)**:
```markdown
<!-- warning_reviewed: YYYY-MM-DD - [Brief justification] -->
```

---

## 6. Verification

- [ ] Re-run audit tool to confirm fix or confirm warning is documented
- [ ] Changelog entry created at `docs/changelogs/`

---

## Example: Context Warning

See [Context_Warning_Resolution.md](Context_Warning_Resolution.md) for a complete worked example applying this template.
