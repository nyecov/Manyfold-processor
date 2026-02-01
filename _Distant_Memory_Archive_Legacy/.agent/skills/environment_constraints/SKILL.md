---
name: Environment Constraints
description: Single source of truth for all resource limits, memory reservations, and hardware constraints.
---

# Environment Constraints

<!-- audited_by: .agent/workflows/audit_constants.md -->
<!-- audited_by: .agent/workflows/audit_infrastructure.md -->

This skill is the **canonical source of truth** for all resource limits and environmental parameters. Other skills MUST reference this document rather than duplicating values.

## 1. Memory Management

> [!NOTE]
> Actual memory limits are determined by **Docker Compose** configuration at deployment time. The values below are development defaults.

### Container Resources
| Parameter | Development Default | Docker Flag |
| :-- | :-- | :-- |
| **Memory Reservation** | 1 GB | `--memory-reservation=1g` |
| **Memory Hard Limit (Max)** | 5 GB | `--memory=5g` |
| **Minimum Startup RAM** | 500 MB | (Application check) |

### Working Directory
| Parameter | Value | Notes |
| :-- | :-- | :-- |
| **RAM Drive Mount** | `/app/temp` | `tmpfs` volume in compose |
| **Environment Variable** | `WORKING_DIR` | Must point to RAM drive |

## 2. Hardware Tier Thresholds

| Tier | RAM Threshold | Features |
| :-- | :-- | :-- |
| **Tier 1 (Radxa)** | ≥ 16 GB | Full NPU/RGA, RAM Drive |
| **Tier 2 (Generic)** | 2-16 GB | CPU Fallback, Standard I/O |
| **Tier 3 (Legacy)** | < 2 GB | Streaming-only, No ML |

## 3. Storage Constraints

| Parameter | Value | Notes |
| :-- | :-- | :-- |
| **Minimum Free Disk** | 10 GB | For temp extraction |
| **Max Single File** | 1 GB | Streaming required above |

---

## See Also
*   **Architecture**: [architectural_guidelines](../architectural_guidelines/SKILL.md)
*   **Target Hardware**: [deploy_on_radxa_rock5](../deploy_on_radxa_rock5/SKILL.md)
