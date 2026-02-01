# Test Resources Guide

## Overview
We use a **Test Resource Registry** (`test_resources/manifest.yaml`) to manage binary assets like STLs and Images. This allows AI Agents to intelligently select files based on capabilities (e.g., "valid", "corrupt") rather than guessing filenames.

## How to Add a New File

1.  **Drop the file**: Place your file in `test_resources/{category}/{filename}`.
2.  **Auto-Register**: Run the following command locally to generate a template entry:
    ```bash
    cd .agent/tools
    cargo run --bin check_test_resources -- --fix
    ```
3.  **Fill Details**: Open `test_resources/manifest.yaml` and find your new entry (tagged `needs-triage`).
    -   **Tags**: Add relevant semantic tags (e.g., `["high-poly", "manifold"]`).
    -   **Description**: Explain *why* this file exists.
    -   **Cleanup**: Remove the `needs-triage` tag.

## Schema
- **id**: Unique identifier (snake_case).
- **path**: Relative path from `test_resources/`.
- **type**: `stl`, `image`, `gcode`, etc.
- **tags**: List of capability keywords.
- **description**: Human-readable context.
- **checksum**: SHA256 hash (managed automatically).
- **size_bytes**: File size in bytes (managed automatically).

## Standard Tags
- `valid` / `invalid`: General validity.
- `corrupt`: Specifically engineered corruption.
- `large`: >50MB.
- `organic`: Smooth curves (good for decimation testing).
- `geometric`: Sharp edges.
- `manifold` / `non-manifold`: Geometry quality.

## Troubleshooting
If `run_full_audit.sh` fails on "Test Data Audit":
1.  Check if you have orphan files (files not in manifest).
2.  Check if you have broken links (manifest entries without files).
3.  Run the `--fix` command to attempt automatic repair.
