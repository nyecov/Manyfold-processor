---
name: API Resilience Strategy
description: High-level policies for robust communication with the Manyfold API.
---

# API Resilience & Reliability

This skill defines the strategic policies to ensure the Manyfold Processor remains robust against network instability and Manyfold instance downtime.

## 1. Retry Policies (Exponential Backoff)
*   **Target**: All non-mutating (GET) and idempotent (PUT/DELETE) requests.
*   **Strategy**: Exponential Backoff + Jitter.
    *   Base Delay: 500ms
    *   Max Retries: 5
*   **Tus Resume**: Resumable uploads must prioritize chunk-level resumption over full file restarts.

## 2. Circuit Breakers
*   **Scenario**: Repeated 503 errors or connection timeouts.
*   **Policy**: Implement a cooldown period (e.g., 60s) where all outgoing requests are halted to prevent cascading failures.

## 3. Graceful Failure
*   **Action**: On critical API failure, local processing must be suspended and state saved. The system must not lose work due to API transient errors.

---

## See Also
*   **Technical Contract**: [manyfold_api_endpoints](../manyfold_api_endpoints/SKILL.md)
*   **Architecture**: [architectural_guidelines](../architectural_guidelines/SKILL.md)
