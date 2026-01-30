# Project Skills

This directory contains specialized **Skills** that the AI agent uses to perform complex tasks.

## Skill Catalog (21 Skills)

### Core Philosophy
| Skill | Description |
|-------|-------------|
| [agentic_philosophy](agentic_philosophy/SKILL.md) | Core governing philosophy (Defensive Orchestration) |
| [project_details](project_details/SKILL.md) | Key project metadata, repository links |
| [project_workflows](project_workflows/SKILL.md) | Guide to available workflows |

### Architecture & Design
| Skill | Description |
|-------|-------------|
| [architectural_guidelines](architectural_guidelines/SKILL.md) | Core architectural principles (C4 model) |
| [c4_model](c4_model/SKILL.md) | Architectural visualization framework |
| [environment_constraints](environment_constraints/SKILL.md) | Resource limits and hardware constraints |

### Code & Testing
| Skill | Description |
|-------|-------------|
| [code_quality_standards](code_quality_standards/SKILL.md) | Rust formatting, linting, security |
| [testing_philosophy](testing_philosophy/SKILL.md) | BDD strategy and mandates |
| [gherkin_style_guide](gherkin_style_guide/SKILL.md) | Gherkin scenario standards |
| [cucumber_rust_reference](cucumber_rust_reference/SKILL.md) | Cucumber crate implementation |

### Domain & Specifications
| Skill | Description |
|-------|-------------|
| [geometry_governance](geometry_governance/SKILL.md) | 3D model processing governance |
| [3mf_specification](3mf_specification/SKILL.md) | 3MF Manufacturing Format details |
| [stl_specification](stl_specification/SKILL.md) | STL file parsing |

### Infrastructure & Deployment
| Skill | Description |
|-------|-------------|
| [deploy_on_radxa_rock5](deploy_on_radxa_rock5/SKILL.md) | Radxa Rock 5 ITX deployment |
| [deployment_operations](deployment_operations/SKILL.md) | Container deployment manual |
| [observability_standards](observability_standards/SKILL.md) | Logging and health checks |

### Integration & API
| Skill | Description |
|-------|-------------|
| [manyfold_api_endpoints](manyfold_api_endpoints/SKILL.md) | Manyfold API specification |
| [manyfold_reference_material](manyfold_reference_material/SKILL.md) | Local Manyfold repo reference |
| [api_resilience_strategy](api_resilience_strategy/SKILL.md) | Robust API communication |

### Documentation & Research
| Skill | Description |
|-------|-------------|
| [kb_linking](kb_linking/SKILL.md) | Internal documentation linking |
| [research_and_fallback_strategies](research_and_fallback_strategies/SKILL.md) | Web search fallback methods |

---

## Structure

Each skill is a subfolder containing a `SKILL.md` file:

```
.agent/skills/
└── [skill_name]/
    ├── SKILL.md          <-- Main instructions (Required)
    └── resources/        <-- Templates/Assets (Optional)
```

## SKILL.md Format

```yaml
---
name: [Skill Name]
description: [Short description]
requires: [list, of, dependencies]  # Optional
---
```
