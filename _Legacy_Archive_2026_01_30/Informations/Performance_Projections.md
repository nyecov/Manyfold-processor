# Performance Projections: Hybrid vs. Native Rewrite

This document projects the performance characteristics of the Manyfold Processor if it were rewritten entirely in Go or Rust, compared to the current Hybrid (Python + Rust) architecture.

## 1. Executive Summary

| Metric | Current (Python + Rust) | Full Go | Full Rust |
| :--- | :--- | :--- | :--- |
| **Idle Memory** | ~45-60 MB | ~10-15 MB | ~2-5 MB |
| **Peak Memory (Processing)** | ~1.2 GB+ (large Stl) | ~800 MB | ~600 MB |
| **Docker Image Size** | ~150-200 MB | ~20-30 MB | ~10-15 MB |
| **Boot Time** | ~1-2 seconds | < 100ms | < 50ms |
| **Throughput (1 Core)** | Low (GIL limited) | High | Very High |

**Verdict**:
*   **Current Architecture**: Optimized for *Development Velocity*. Using `stl23mf` (Rust) for the heavy lifting mitigates the major CPU/Memory bottlenecks of Python for geometry tasks.
*   **Full Rewrite**: Would primarily benefit **Idle Memory** (useful for running 24/7 on low-end hardware like Pi Zero) and **Image Size**. Processing speed gains would be marginal since the heavy lifting is *already* done in Rust.

---

## 2. Detailed Analysis

### A. Memory Footprint

#### Idle State (Watchdog Listening)
*   **Python**: The Python runtime (CPython) has a significant baseline overhead. Loading standard libraries (`os`, `shutil`, `threading`) and frameworks (`FastAPI`, `uvicorn`) consumes ~50MB immediately.
*   **Go**: Garbage collected, but compiles to native binaries. The runtime overhead is minimal. A blocking watcher/webserver would sit around 10MB.
*   **Rust**: No Garbage Collector. Zero-cost abstractions. A simple file watcher and Actix-web server can run in <5MB RAM.

#### Processing State (Geometry Conversion)
*   **Python**: Historically, `trimesh` (Python) loads the entire mesh into standard Python objects/Numpy arrays, which is memory inefficient (overhead per object).
    *   *Mitigation*: We currently call out to `stl23mf`.
*   **Rust/Go**: Native structs are packed efficiently in memory.
    *   *Note*: Since we already use `stl23mf` for the heaviest task, a full rewrite would only save the memory overhead of the *orchestration* logic (moving files, parsing JSON), which is negligible compared to the geometry data.

### B. Docker Image Size

*   **Python**: Requires `python:3.11-slim` base image (~120MB uncompressed) + dependencies.
*   **Go/Rust**:
    *   **Go**: Can compile to a static binary. `FROM scratch` deployment is possible. Binary size ~15MB.
    *   **Rust**: Same. Binary size ~5-10MB (stripped).
    *   *Impact*: Faster deployments, less disk usage on the host.

### C. Concurrency & Throughput

*   **Python**:
    *   **GIL (Global Interpreter Lock)**: Only one thread executes Python bytecode at a time. CPU-bound tasks in Python block the event loop.
    *   *Workaround*: We use `subprocess` to spawn `stl23mf`. This bypasses the GIL but adds process-spawning overhead (~50ms).
*   **Go**:
    *   **Goroutines**: Lightweight threads. Can handle thousands of concurrent file processing jobs efficiently across all cores.
*   **Rust**:
    *   **Async/Await**: Extremely efficient. Similar to Go but with more manual control over memory safety.

### D. Development trade-offs

| Feature | Python | Go | Rust |
| :--- | :--- | :--- | :--- |
| **Code Speed** | Very Fast | Fast | Moderate (Borrow Checker) |
| **Ecosystem** | Huge (Data/AI/Web) | Strong (Cloud/Network) | Strong (Systems/WASM) |
| **Safety** | Low (Runtime Errors) | High (Type Safe) | Very High (Memory Safe) |

## 3. Recommendation

**Stay with Hybrid (Python + Rust) for Phase 1.**

The **Bottleneck** in this application is **Disk I/O** (reading zip headers, moving files) and **Geometry Math**.
1.  **Disk I/O**: Python wraps C functions for OS calls; the slowdown is negligible.
2.  **Geometry Math**: We have already solved this by delegating to `stl23mf` (Rust).

**When to Rewrite?**
Consider a full rewrite to **Rust** only if:
1.  You need to run on hardware with **<512MB RAM** (e.g., Raspberry Pi Zero 2W running many other services).
2.  You scale to processing **thousands of files per minute** (where Python's process-spawning overhead becomes measurable).
