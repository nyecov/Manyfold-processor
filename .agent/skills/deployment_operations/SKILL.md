---
name: Deployment Operations
description: Technical manual for deploying and managing the Manyfold Processor container.
---

# Deployment Operations

Technical reference for Docker orchestration and container lifecycle management.

## 1. Docker Basics
### Volume Mounts
*   `/input`: Target watch directory.
*   `/output`: Processed assets destination.
*   `/config`: Persistent configurations.
*   `/app/temp`: RAM Drive (`tmpfs`) mount point.

### Environment Variables
*   `RUST_LOG`: Controls verbosity (e.g., `info`, `debug`).
*   `WORKING_DIR`: Sets the internal temp path.

## 2. Operations Manual
*   **Start**: `docker-compose up -d`
*   **Logs**: `docker logs manyfold-processor -f`
*   **Health Check**:
    ```yaml
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8080/health"]
      interval: 30s
    ```

## 3. Maintenance
*   **Updates**: Pull latest image and restart stack.
*   **Clean Up**: Periodic removal of orphan volumes.

---

## See Also
*   **Governance**: [observability_standards](../observability_standards/SKILL.md)
*   **Hardware Tiering**: [deploy_on_radxa_rock5](../deploy_on_radxa_rock5/SKILL.md)
