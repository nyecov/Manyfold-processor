# Technical Specifications

## 🏗️ Architecture: Dockerized Watchdog
The application runs as a background service within a Docker container, utilizing an event-driven "Watchdog" pattern to minimize resource usage.

### Lifecycle
1.  **Idle State**: The service sleeps.
2.  **Trigger**: New file/folder detected.
3.  **Processing**:
    *   File is locked/claimed.
    *   **Global Auto-Process Check**:
        *   **ON (Default)**: Proceed to auto-detection (Step 4).
        *   **OFF**: Immediately move to `STAGING` for manual review.
4.  **Auto-Logic**:
    *   **Auto-Process**: If confidence is high, process and move to `OUTPUT`.
    *   **Quarantine**: If ambiguous, move to `STAGING`.

### Web Interface Features
*   **Dashboard**: List items in `STAGING`.
*   **Control Panel**:
    *   **Auto-Watchdog Toggle**: Enable/Disable automatic processing.
    *   **Theming**: Must support **Night Mode** (Priority). Future: Theme Editor.
*   **Responsiveness**:
    *   **Current Status**: Optimized for **Desktop / Computer Screens**.
    *   **Roadmap**: Mobile View / Small Screen optimization is planned for a later phase (Phase 3+).

### Reliability & Efficiency Strategy
*   **CPU Efficiency**:
    *   **Idle**: Event-driven (Watchdog/Inotify) ensures **0% CPU usage** when no files are present.
    *   **Validation**: Files are checked for integrity (ZIP structure, 3MF headers) before processing.
    *   *Note on MD5*: **Disabled** by default. We use the internal `CRC32` check of the `.3mf/zip` format.
*   **Staging Logic (Strict)**:
    *   **Lock Rule**: The processor will **NOT** move new files into Staging if Staging is not empty. This prevents mixing files from different batches.
    *   **Folder Grouping**: For STL packs, we treat the *Directory* as the unit of work. If files are in a subfolder, we assume they are related.
*   **Crash Recovery**:


    *   **Startup Scan**: On container restart, the service checks `/input` for any leftovers (files that arrived during downtime or crash) and processes them.
    *   **Atomic Output**: Files are built in a temp buffer and atomically moved (`shutil.move`) to `/output` only when fully complete. This prevents "half-copied" corrupt files.
    *   **Safety Net**: Any unhandled exception during processing triggers an automatic move to `/staging` for human review, so no data is ever lost.
*   **Large File Strategy (1GB+)**:
    *   **3MF/ZIP**: We use **Random Access** reading. We do *not* load the entire 1GB file into RAM. We only seek and read the specific `metadata` and `thumbnail` headers (typically <50KB). This allows processing 2GB+ files with <50MB RAM usage.
    *   **Direct Management**: For all files (including large ones), the processor focuses on **File Management** (moving, renaming, organizing) and **Metadata Extraction**. It explicitly **does not** attempt mesh repair, geometry analysis, or deep file modifications to ensure maximum performance and stability on low-power hardware.

### Roadmap: Phase 2b (Deferred Conversions)
Features originally planned but deferred to prioritize stability and speed for the MVP:
1.  **Format Standardization**:
    *   **STL -> 3MF**: Converting legacy binary STLs into modern 3MF containers.
        *   *Performance Note*: Uses `trimesh`. Conversion of 100MB STL takes ~2 minutes on low-power hardware and requires ~1.2GB RAM.
    *   **Image -> WebP**: Converting sibling JPG/PNGs to optimized WebP thumbnails to save space.
2.  **Implementation Note**: These should be implemented as optional "Plugins" or "Workers" that can be enabled if CPU resources allow, rather than core blockers.





### Persistence & State Strategy
*   **Settings (`auto_process`, `theme`)**: Stored in `/config/config.json`.
    *   *Efficiency*: Zero RAM overhead (vs Redis). Simple backup/restore.
    *   *User Benefit*: Settings survive container restarts.
*   **Procession Stages**: Mapped 1:1 to filesystem folders (`/staging`, `/output`).
    *   *Benefit*: "Source of Truth" is always the disk. No de-sync possible.
*   **UI State (Menus/Views)**: Handled via Browser `localStorage` and URL routing (Future).

### Future Optimization: Redis
*   *Role*: Can be introduced in Phase 2 for **Job Locking** or **Progress Tracking** if multiple workers are added.
*   *Verdict*: Not required for Phase 1 MVP (Single Worker).

### Metadata Enrichment Strategy
*   **Local Extraction**: Primary method. Parsing `.3mf` XML (MakerWorld) or filename conventions (Case 1/3).
*   **External APIs**: Future extensibility for Thingiverse/Cults.
    *   *Constraint*: No "live scraping" in Phase 1 to ensure stability. API integrations will be triggered manually via Web UI or config plugins.


4.  **User Intervention (Web UI)**:
    *   User accesses local webpage.
    *   Reviews queued items, fixes paths/metadata.
    *   Triggers "Approve" -> Move to `OUTPUT`.

---

## 💻 System Constraints & Optimization
*   **Target Hardware**: Radxa ROCK 5 ITX (RK3588 ARM SoC).
*   **Web Framework**: **FastAPI** (Python).
    *   *Reason*: Extremely lightweight, asynchronous (doesn't block watchdog), and provides automatic API docs.

---

## 🐍 Language Choice: Python 3.11+
Selected for the following efficiency and capability reasons:

1.  **I/O Efficiency**: The primary workload is file manipulation (ZIP handling, JSON, File IO). Python's `zipfile` and `shutil` libraries are wrappers around performant C implementations.
2.  **Web & Event Capable**: **FastAPI** + `watchdog` allows a single process to handle both background monitoring and serving a reactive UI efficiently.
3.  **Ecosystem**:
    *   `trimesh` / `numpy`: Highly optimized C/Fortran backends for any 3D geometry processing.
    *   `Pillow`: Optimized C library for image conversion (WebP).
4.  **Maintenance**: High readability and ease of modification.

### Alternatives Considered: Rust
*   **Pros**:
    *   **Efficiency**: Extremely low memory footprint (~5MB vs Python's ~50MB) and near-zero CPU overhead.
    *   **Single Binary**: Simplifies deployment without Python runtime dependencies.
*   **Cons (Why Python was chosen)**:
    *   **3D Ecosystem**: Python has `trimesh` and `numpy`, the industry standards for geometry processing. Equivalent libraries in Rust are less mature for high-level mesh repair/analysis.
    *   **Development Velocity**: Rapid iteration for logic changes (e.g., parsing new metadata formats) is faster in Python.
*   **Verdict**: Python is chosen for Phase 1 to leverage the extensive 3D library ecosystem. The I/O-bound nature of the task means the performance gap is negligible on modern hardware (RK3588), though Rust would be better for extremely constrained limits (Pi Zero).
    *   *Optimization Note*: A future rewrite of the geometry processing module in a compiled language (Rust/C++) could reduce STL->3MF conversion time from ~2 minutes to <10 seconds and significantly lower RAM usage.


---

## 🌍 Cross-Platform Compatibility
*   **Architectures**: Must support **linux/amd64** (Standard PC, LattePanda, AMD G-T56N), **linux/arm64** (Rock 5, Pi 3/4/5, Orange Pi), and **linux/arm/v7** (Pi 2, Pi Zero).
    *   *Base Image*: `python:3.11-slim-bookworm` (Official Multi-Arch support including arm32v7).
    *   *Constraint*: Performance on constrained devices (Pi Zero, G-T56N) will be limited; rendering features may need to automatically disable if OpenGL is unavailable.
*   **Path Sanitization**:
    *   **Rule**: Input files may contain Windows-style backslashes (`\`) in metadata (e.g., from legacy tools).
    *   **Action**: Internal logic **MUST** normalize all paths to POSIX forward slashes (`/`).

## 🐳 Container Specifications
*   **Base Image**: `python:3.11-slim-bookworm`.
*   **Compatibility**: Verified for Windows (Docker Desktop w/ WSL2) and Linux Host (Armbian).
*   **Ports**: `6767` (Web UI).
*   **Volumes**:
    *   `/input`: Watch folder.
    *   `/output`: Destination folder (Manyfold Library).
    *   `/staging`: Intermediate area for items awaiting review.
    *   `/config`: Configuration files.
*   **Privileges**: Standard user (`PUID`/`PGID` environment variables for permission mapping).
