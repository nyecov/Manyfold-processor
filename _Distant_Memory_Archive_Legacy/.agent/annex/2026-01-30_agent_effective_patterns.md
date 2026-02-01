# Design Document: Agent-Effective Documentation Patterns

**Date**: 2026-01-30
**Version**: 0.3
**Status**: Implemented

---

## Overview

This document describes the architecture of the **Agent-Effective Documentation Patterns** introduced in v0.3 to improve AI agent navigation and context loading.

## Problem Statement

Previous documentation linking relied on implicit relationships. While relative path links existed, the AI agent had to:
1. Manually discover related skills
2. Infer dependencies from prose
3. Duplicate values across multiple files

## Solution Architecture

### 1. Explicit Dependency Declaration (`requires:` Field)

**Location**: YAML frontmatter of `SKILL.md` files

```yaml
---
name: Deployment Operations
description: ...
requires:
  - environment_constraints  # Memory/resource values
  - deploy_on_radxa_rock5    # Target hardware specs
---
```

**Benefits**:
*   Declarative dependency graph
*   Enables future tooling for auto-context loading
*   Self-documenting skill relationships

### 2. Canonical Constants (`constants.yml`)

**Location**: `.agent/constants.yml`

**Contents**:
*   Memory limits (reservation, limit, headroom)
*   Network configuration (IP, SSH, ports)
*   Version info (project, Rust, Docker)
*   Tier definitions (Tier 1/2/3)

**Benefits**:
*   Single Source of Truth
*   Eliminates Double-Source risk
*   Machine-readable format

### 3. Updated `kb_linking` Skill

Added Section 3: "Agent-Effective Patterns (v0.3+)" documenting:
*   `requires:` field usage
*   `constants.yml` reference pattern

---

## Implementation Status

| Component | Status | Files |
| :-- | :--: | :-- |
| `requires:` field | ✅ | 4 priority skills updated |
| `constants.yml` | ✅ | `.agent/constants.yml` created |
| `kb_linking` update | ✅ | Section 3 added |

---

## See Also
*   [kb_linking](../skills/kb_linking/SKILL.md)
*   [constants.yml](../constants.yml)
*   [skills_workflows_improvement_plan.md](../../notes/skills_workflows_improvement_plan.md)
