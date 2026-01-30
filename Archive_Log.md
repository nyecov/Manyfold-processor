# Manyfold Processor Archive Log (2026-01-30)

## Overview
This document logs the state of the project immediately before a comprehensive overhaul. All previous code, documentation, and configuration have been moved to `/_Legacy_Archive_2026_01_30`.

## Archived Contents
The following directories and files have been archived:
- `src/`: Source code (Python).
- `tests/`: BDD tests (`behave`).
- `Informations/`: Documentation and Analysis.
- `.agent/`: Agent-specific workflows and guidelines.
- `docs/`: Old documentation.
- `playground/`: Experiments.
- `test_data/`: Sample inputs.
- `Dockerfile`, `docker-compose.yml`: Container config.
- `manage.sh`, `manage.ps1`: Utility scripts.
- `README.md`, `requirements.txt`: Project definitions.

## Explored Ideas & Analysis
The following concepts were analyzed and documented during the "Phase 1" development.

### 1. Hardware Acceleration (Rock 5 ITX / RK3588)
*   **Source**: `Informations/Hardware_Acceleration_Analysis.md`
*   **Guidance**:
    *   **Images**: Use **RGA (2D Engine)** for resizing/conversion (Zero CPU usage).
    *   **AI Inference**: Use **NPU (6 TOPS)** for classification (Auto-tagging).
    *   **Geometry**: GPU/NPU is **not** suitable. Use CPU/Rust.
    *   **Training**: Do **not** train on the device. Train on PC (16GB VRAM is sufficient for PointNet/YOLO) and export `rknn` models to the device.

### 2. Performance & Language (Rust vs Python)
*   **Source**: `Informations/Performance_Projections.md`
*   **Guidance**:
    *   **Current Hybrid**: Python Orchestration + Rust Binary (`stl23mf`) is the optimal balance of speed and development velocity.
    *   **Full Rust Rewrite**: Would reduce Docker image size (~160MB -> ~15MB) and Idle RAM (~50MB -> ~5MB), but offer negligible processing speed gains.
    *   **Verdict**: Stick to Hybrid unless memory constraints are extreme (<512MB RAM).

### 3. Storage Optimization (SSD Lifespan)
*   **Source**: `Informations/Storage_Optimization_Strategy.md`
*   **Guidance**:
    *   **Problem**: Write Amplification from extracting ZIPs and intermediate steps (~2GB write for 500MB data).
    *   **Solution**: Use **RAM Drive (tmpfs)** for `/temp` and `/staging`.
    *   **Implementation**: Add `tmpfs: - /app/temp` to `docker-compose.yml`.

### 4. Architecture (C4 Model)
*   **Source**: `Informations/Architecture.md`
*   **Structure**:
    *   **Container**: Dockerized Python Service + Rust Binary.
    *   **Components**: Watchdog Listener -> Router -> Plugins (Loose/Archive/Directory).

### 5. Technical Specifications
*   **Source**: `Informations/Technical_Specs.md`
*   **Key Constraints**:
    *   Target: Radxa Rock 5 ITX.
    *   Input: Watchdog on `/input`.
    *   Output: Atomic move to `/output`.
    *   Stack: Python 3.11, FastAPI, Rust (`stl23mf`).

## Feature Status (at time of Archive)
*   **Archive Processing (Case 2)**: Implemented but basic content extraction.
*   **Directory Projects (Case 4)**: Implemented logic to treat folders as units.
*   **Loose Files (Case 1)**: Basic aggregation implementation.
*   **Workflows**: Agent workflows were set up in `.agent/workflows`.

---
*End of Log*
