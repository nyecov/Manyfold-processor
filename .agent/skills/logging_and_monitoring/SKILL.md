---
name: Logging and Monitoring
description: Standards for application logging, health checks, and observability.
---

# Logging & Monitoring Standards

## 1. Logging Strategy
To ensure debuggability on all Tiers (including the headless Tier 3), logs must be structured and persistent.

### Format
*   **Production**: JSON Lines (NDJSON). `{"timestamp": "...", "level": "INFO", "msg": "..."}`
    *   *Why*: Parsing by log aggregation tools (or simple `jq` filtering).
*   **Development**: Human-readable text (colored).

### Rust Implementation
Use `env_logger` or `tracing-subscriber`.
```rust
// Verify RUST_LOG env var controls verbosity
// Use 'json' feature flag for production builds if possible
```

### Retention
*   **StdOut/StdErr**: Docker daemon handles rotation. Configure `max-size: "10m"` in Compose.
*   **Panic Handling**:
    *   Use `panic::set_hook` to log fatal panics to JSON before exit.

## 2. Health Checks
The container must report its health to the orchestrator (Docker/OMV).

*   **Endpoint**: `GET /health`
*   **Response**: `200 OK` (JSON: `{"status": "healthy", "ram_free_mb": 850}`)
*   **Docker Healthcheck**:
    ```yaml
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8080/health"]
      interval: 30s
      retries: 3
    ```

## 3. Metrics (Optional for v0.3)
Expose simple metrics at `GET /metrics` (Prometheus format) for:
*   Files processed count.
*   Current RAM usage.
