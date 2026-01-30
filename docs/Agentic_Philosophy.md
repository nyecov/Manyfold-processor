# Agentic Engineering Philosophy

This document provides a high-level overview of the **Defensive Orchestration** paradigm used to govern AI agent interactions with this codebase.

## Quick Reference

| Concept | Description |
|---------|-------------|
| **Ghost in the Machine** | `.agent/` is an OS, not config. `constants.yml` is law. |
| **Annex Strategy** | History in `.agent/annex/`, active in `.agent/skills/`. |
| **Meta-Governance** | Workflows audit workflows. |
| **Hybrid Model** | Workflow = Manager (Prompt). Script = Worker (Tool). |

## Core Documents
*   **Full Skill**: [Agentic Philosophy](../.agent/skills/agentic_philosophy/SKILL.md)
*   **Workflows**: [Project Workflows](../.agent/skills/project_workflows/SKILL.md)

## Self-Healing Protocol
1.  **Reactive**: Run `/suite_full_audit` → Fix drift.
2.  **Proactive**: Sentinels auto-update catalogs.
3.  **Pre-Emptive**: Block commits that degrade health.

---
*See the full skill for design rules, token efficiency, and implementation details.*
