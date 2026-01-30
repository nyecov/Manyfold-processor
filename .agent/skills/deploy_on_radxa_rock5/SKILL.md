---
name: Deploy on Radxa Rock 5 ITX
description: Guidelines and checks for ensuring the application runs optimally on the Radxa Rock 5 ITX (RK3588) hardware.
---

# Radxa Rock 5 ITX Deployment Standards

This skill governs the deployment and optimization of the Manyfold Processor on the specific target hardware: **Radxa Rock 5 ITX**.

## 1. Hardware Context

> [!CRITICAL]
> **Hardware Preservation Strategy**
> **Scarcity & Cost Alert**: This deployment relies on consumer-grade hardware. Due to skyrocketing prices and scarcity, replacement is difficult.
> **Mandate**: All architectural decisions must prioritize **longevity** and **component health** over raw speed.
> *   **SSD**: Treat write endurance as a finite, precious resource. Avoid unnecessary I/O.
> *   **Thermals**: Respect the cooling capabilities of the ITX form factor.

*   **SoC**: Rockchip RK3588 (4x A76 + 4x A55)
*   **RAM**: 16 GB LPDDR5 (Shared with Video/NPU)
*   **GPU**: Mali-G610 MP4 (OpenCL/Vulkan)
*   **NPU**: rknpu2 (6 TOPS)
*   **Storage**: SATA SSDs + RAM Drive opportunities

## 2. Optimization Rules

### A. Memory Management (RAM Disk)
**Constraint**: To preserve the SSD lifespan (write endurance), all temporary processing MUST happen in RAM.
**Action**:
1.  Ensure `docker-compose.yml` mounts a `tmpfs` volume for `/app/temp`.
2.  Application logic must write intermediate files to `working_dir` (which points to this RAM drive).
3.  **Forbidden**: Writing temporary `.stl` or `.zip` extractions directly to the host filesystem.

### B. Image Processing (RGA)
**Constraint**: The CPU (A76) is powerful but should be reserved for logic and geometry.
**Action**:
1.  Prefer **Rockchip RGA (Raster Graphic Acceleration)** for any image resizing or format conversion (JPG -> WebP).
2.  If RGA is unavailable (e.g., inside standard container), use Pillow but ensure it runs on **Performance Cores**.

### C. AI Inference (NPU)
**Constraint**: Do NOT run AI models (PointNet, CLIP) on the CPU.
**Action**:
1.  Models must be converted to `.rknn` format (`rknn-toolkit2`).
2.  Inference must target the NPU device (`/dev/dri/renderD129` or specific NPU hook).

### D. Geometry Processing (Hybrid)
**Constraint**: Python is too slow for 100MB+ STL parsing on ARM.
**Action**:
1.  ALWAYS use the internal Rust application logic for geometry heavy lifting (e.g., STL to 3MF conversion).
2.  Do not use `trimesh` for core conversion tasks unless as a fallback.

### E. OpenMediaVault (OMV) Specific Rules
**Context**: This system uses OMV with strict UI-based workflow standards. These apply **ONLY** to this specific deployment environment.
**Action**:
1.  **Naming Convention**: Files must be named `[service_name].yml` and `[service_name].env` (e.g., `manyfold.yml`), NOT `docker-compose.yml`.
2.  **Standard Paths**:
    *   Configs: `/config`
    *   Compose Stacks: `/compose_files`
    *   Secrets: `/DockerSecrets`
    *   Databases: `/DB`

## 3. Capacity Planning & Constraints
**Golden Rule**: The Manyfold Processor is a system service. It requires guaranteed resources.

*   **Minimum RAM**: 2GB Total System RAM (Tier 3).
*   **Guaranteed Headroom**: **750MB FREE RAM** (Reserved).
    *   *Warning*: The container is configured to **exit immediately** if it cannot reserve this memory. Ensure you do not over-provision the host.
*   **Storage**: 10GB+ Free Disk Space (for temporary extraction and caching).

## 4. Pre-Deployment Checklist
When asked to "Deploy" or "Validate Compatibility":

1.  **Check Architecture**: ensure Docker image is built for `linux/arm64`.
2.  **Check Memory**: Verify host has >750MB RAM available for reservation.
3.  **Check Volumes**: Verify `tmpfs` configuration in Compose.
3.  **Check OMV compliance**: Ensure output is formatted for Copy/Paste into OMV UI.
4.  **Fan Control**: **IGNORE** this aspect. `rock5-fan-control.py` is an independent side-project managed externally. The Manyfold Processor must not attempt to manage, check, or interact with thermal control scripts.
