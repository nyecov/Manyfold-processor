# Workshop Notes & Hardware Context

## 🛠️ Available Hardware (User Lab)
1.  **Raspberry Pi 2**:
    *   **Architecture**: ARMv7 (32-bit).
    *   **Role**: Tier 3 Target (Legacy/Low-Power).
    *   **Constraint**: Extremely limited RAM (1GB) and CPU.
2.  **Raspberry Pi 5**:
    *   **Architecture**: ARM64 (Cortex-A76).
    *   **Role**: Tier 2 Target (High-Power Generic).
    *   **Notes**: Powerful, but lacks the specific RK3588 NPU/RGA accelerators found in the Rock 5.
3.  **HP T610 Thin Client**:
    *   **Architecture**: AMD64 (AMD G-T56N APU).
    *   **Role**: Tier 2 Target (x86_64).
    *   **Notes**: Validation for generic PC support.

## 📝 Workshop Items
### 1. Logging & Monitoring
*   **Decision**: Require structured logging (JSON in production) and Health Checks.
*   **Action**: Created [logging_and_monitoring](../.agent/skills/logging_and_monitoring/SKILL.md).

### 2. Memory Managment
*   **Constraint**: 750MB specific requirement.
*   **Decision**: Must be enforced via **Docker Resource Limits** (reservation) + Application Check.
*   **Bounds**:
    *   **Lower Bound (Guarantee)**: 750MB (`--memory-reservation`).
    *   **Upper Bound (Limit)**: Required to prevent OOM kills destabilizing the host.

### 3. Testing Strategy
*   **Constraint**: Hardware verification on User Lab devices (RPi 2/5, HP T610) is deferred until **v1.0 (Release)**.
*   **Context**: Early development (v0.x) focuses on code stability in the standard Docker environment. Specialized hardware testing is reserved for the Release Candidate phase.

---

## See Also
*   **Deployment Rules**: [deploy_on_radxa_rock5](../.agent/skills/deploy_on_radxa_rock5/SKILL.md) (RK3588 optimizations).
*   **Hardware Context**: [developer_hardware_specs](../.agent/skills/developer_hardware_specs/SKILL.md) (Development environment specs).
