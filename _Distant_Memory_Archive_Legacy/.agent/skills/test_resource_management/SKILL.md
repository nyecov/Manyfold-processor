---
name: Test Resource Management
description: Standards for managing binary test assets via the Test Resource Registry.
requires: [governance_integration]
---

# Test Resource Management

<!-- audited_by: .agent/workflows/audit_test_resources.md -->

Binary test assets (STLs, Images, etc.) must be managed via the **Test Resource Registry** (`test_resources/manifest.yaml`). This ensures they are discoverable, tagged, and semantically legible to AI Agents.

## The Rule of Enrollment
> "Every file in `test_resources` must be enrolled in `manifest.yaml`."

Unregistered files are considered **Orphans** and will cause governance checks to fail.

## Adding a New Resource
1.  Place the file in `test_resources/{category}/{filename}`.
2.  Run the auto-registration tool:
    ```bash
    cargo run --bin check_test_resources -- --fix
    ```
3.  Open `manifest.yaml` and:
    -   Update the `id` to be meaningful (e.g., `sophia_stl`).
    -   Add descriptive `tags` (e.g., `["valid", "high-poly"]`).
    -   Write a useful `description` explaining *why* this file exists.
    -   Remove the `needs-triage` tag.

## Schema
- **id**: Unique identifier (snake_case).
- **path**: Relative path from `test_resources/`.
- **type**: `stl`, `image`, `gcode`, etc.
- **tags**: List of capability keywords.
- **description**: Human-readable context.
- **checksum**: SHA256 hash (managed automatically).

## Self-Healing
The `audit_test_resources` workflow runs the validator. If orphans are found, they are auto-registered with `needs-triage` tags. The Agent is then responsible for filling in the metadata.
