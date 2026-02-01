---
description: detailed audit of hardcoded values versus canonical constants.
---

# Atomic Audit: Canonical Data (Constants)

This standalone workflow ensures that shared project values use the canonical `constants.yml` source of truth rather than being hardcoded in skills or code.

<!-- depends: .agent/constants.yml -->
<!-- depends: .agent/skills/environment_constraints/SKILL.md -->
<!-- depends: .agent/skills/deploy_on_radxa_rock5/SKILL.md -->
<!-- depends: docs/Documentation_Quality_Comparison.md -->
<!-- depends: .agent/tools/src/bin/check_constants.rs -->

---

## Execution Protocol

> [!NOTE]
> **Hybrid Mode**: This workflow uses both headless scripts (🔧) and agent analysis (🧠).

### 🔧 Step 1: Headless Magic Value Detection
```powershell
.agent\tools\target\release\check_constants.exe
```
**Covers**: Known magic values (IP addresses, memory sizes)

*   **If `[OK]`**: Proceed to Agent steps.
*   **If `[XX]`**: Replace hardcoded values with `constants.yml` references, then proceed.

---

### 🧠 Step 2: Unknown Constant Discovery (AGENT-ONLY)
> Script cannot do this — script only checks known patterns.

*   Scan for other hardcoded values that might need canonicalization.
*   Look for port numbers, timeouts, paths, version numbers.
*   **Action**: Add new values to `constants.yml`.

---

### 🧠 Step 3: constants.yml Completeness (AGENT-ONLY)
> Script cannot do this — requires understanding of what should be centralized.

*   Review `constants.yml` structure.
*   Verify all environment-specific values are captured.
*   **Action**: Add missing constants.

---

### 🧠 Step 4: Reference Verification (AGENT-ONLY)
> Script cannot do this — requires checking if skills properly reference constants.

*   Verify skills use `environment_constraints` skill for values.
*   Check that code loads from `constants.yml` at runtime.
*   **Action**: Add missing references.

---

## Report

| Finding | Source |
|---------|--------|
| Known Magic Values | 🔧 Script |
| Unknown Constants | 🧠 Agent |
| Missing from constants.yml | 🧠 Agent |
| Missing References | 🧠 Agent |
