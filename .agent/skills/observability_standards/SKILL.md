---
name: Observability Standards
description: Standards for application logging, health checks, and global observability.
---

# Observability Standards: Strategy & Governance

This skill defines the high-level strategy for monitoring the health and behavior of the Manyfold Processor across all deployment tiers.

## 1. Logging Strategy
To ensure debuggability on headless or remote devices, logs must be structured and meaningful.
*   **Production**: Mandatory **JSON Lines (NDJSON)** format.
    *   *Rationale*: Allows automated parsing by `jq` and log collectors (OMV, Grafana).
*   **Development**: Human-readable colored text.
*   **Policy**: No sensitive data (API keys, passwords) must ever appear in logs.

## 2. Health & Vitality
*   **Mandatory Endpoint**: `GET /health` must return a `200 OK` with JSON state (e.g., RAM usage).
*   **Vitality Check**: The container must report its current resource consumption relative to its hardware tier limits.

## 3. Panics & Fatal Errors
*   **Governance**: All fatal panics MUST be intercepted and logged as a structured "FATAL" event before the process terminates.

---

## See Also
*   **Operational Manual**: [deployment_operations](../deployment_operations/SKILL.md)
*   **Hardware Baseline**: [deploy_on_radxa_rock5](../deploy_on_radxa_rock5/SKILL.md)
