# Project Skills

This directory contains specialized "Skills" that the AI agent can use to perform complex tasks.

## Structure

Each skill is a subfolder containing a `SKILL.md` file and any necessary resources.

```
.agent/skills/
└── [skill_name]/
    ├── SKILL.md          <-- Main instructions (Required)
    ├── scripts/          <-- Helper scripts (Optional)
    └── resources/        <-- Templates/Assets (Optional)
```

## SKILL.md Format

The `SKILL.md` file must start with YAML frontmatter:

```markdown
---
name: [Skill Name]
description: [Short description of what this skill does]
---

# Instructions

1. Step 1...
2. Step 2...
```

## Usage
When the agent needs to perform a task covered by a specific skill (e.g., "Deploy to Production"), it will read the corresponding `SKILL.md` and follow the specialized procedure.
