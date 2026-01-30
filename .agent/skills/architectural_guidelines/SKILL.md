---
name: Architectural Guidelines
description: Core architectural principles and system design using the C4 model for the Manyfold Processor.
---

# Instructions

When creating new features, refactoring code, or analyzing the system, use this document to understand the architectural boundaries and responsibilities.

1.  **Language**: **Rust** is the primary language for all application logic. Python may be used only for legacy wrapping or prototyping but should be phased out.
2.  **Deployment**: The application is **container-native**. It must be designed to run as a Docker container from the start.
3.  **Independence**: This is a standalone **Utility** for Manyfold, NOT part of the Manyfold core project.
4.  **Interaction**: All interaction with the Manyfold instance must occur via **API calls**. No direct database sharing or internal linking unless explicitly required by the utility nature (e.g., shared volumes for file processing).

---

# System Architecture (C4 Model)

## Level 1: System Context
**Goal**: Show how the software fits into the world.

The **Manyfold Processor** is an external utility that automates file processing for a Manyfold instance.

*   **Users**: Interact via the **WebUI** or drop files into the Input Directory.
*   **Manyfold Processor**: The standalone utility (Rust + Docker).
*   **Manyfold Instance**: The target application where processed files are destined.
    *   *Interaction*: API-based.
*   **File System**: Shared volumes for Input/Output.

## Level 2: Containers
**Goal**: Show the high-level technical building blocks (deployable units).

The system is delivered as a single cohesive unit, optimized for Docker on Radxa Rock 5 ITX.

1.  **Main Application Container (Rust)**:
    *   **Core Logic**: Handles file monitoring, geometry processing, and metadata generation.
    *   **Web Server**: Embedded (e.g., Actix/Axum) to serve the WebUI and API endpoints.
2.  **Manyfold Instance (External)**: The separate service we interact with via API.

## Level 3: Components (Rust Application)
**Goal**: Show the internal structure of the Rust application.

1.  **File Monitor**: Watches the `/input` directory for changes.
2.  **Processing Pipeline**:
    *   **Router**: Determines method (File vs Directory vs Archive).
    *   **Geometry Engine**: `stl23mf` logic (integrated or internal module) for mesh conversion and merging.
    *   **Asset Processor**: Handling images and other assets.
3.  **API Client**: Manages authentication and communication with the Manyfold API.
4.  **WebUI Backend**: Serves the frontend interface and handles user control.

## Level 4: Code
**Goal**: Highlight key implementation details.

*   **Language**: Rust (Edition 2021+).
*   **Concurrency**: Async/Await (Tokio) for I/O-bound tasks (API calls, file watching).
*   **Performance**: Zero-cost abstractions where possible. Minimal memory footprint.
*   **Error Handling**: Robust `Result` handling. No panics in production.

## Technical Goals & Constraints
*   **Primary Language**: Rust.
*   **Core Platform**: Docker on Radxa Rock 5 ITX (Linux/Arm64).
*   **Separation of Concerns**: The Processor sees Manyfold as a black box accessed via API.
