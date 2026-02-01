# High-Level Design: Loose File Processing (Case 1)

This document summarizes the design for processing "Loose Files" (individual 3D models and images) as implemented in the Manyfold Processor.

## 1. Overview
Loose File Processing is the fallback mechanism for content not already packaged in a 3MF container. The system treats individual `.stl` files as the catalyst for creating a new Manyfold project, automatically aggregating related assets (sibling STLs and images) into a consistent output structure.

## 2. Core Logic: The "Project" Concept
The processor do not treat files in isolation. When an STL is detected, the system defines a **Logical Project** consisting of:
- **Primary Model**: The triggering `.stl` file.
- **Aggregated Models**: All sibling `.stl` files in the same directory.
- **Associated Media**: Relevant image files (`.jpg`, `.png`, etc.) found in the vicinity.

## 3. Processing Pipeline

### 3.1 Identification & Aggregation
- **Trigger**: A watchdog event on a `.stl` file in the `/input` directory.
- **Sibling Search**: The `StlProjectPlugin` scans the parent directory for all other `.stl` files.
- **Clustering**: All discovered STLs are bundled together. This allows multipart models distributed as loose files to be merged into a single logical entity.

### 3.2 Intelligent Asset Association (Image Matching)
The system uses a two-tier approach to find the "preview" for the model:
1.  **Token-Based Matching**: Filenames are tokenized (split by spaces, underscores, dashes). If a model and an image share significant tokens (e.g., `dragon_v1.stl` and `dragon_render.jpg`), they are associated.
2.  **Single Image Rule**: If no token match is found but exactly **one** image exists in the folder, the system implicitly associates that image with the project.

### 3.3 Transformation Layer
- **Geometry Consolidation**:
    - **Case A (Single STL)**: Converted directly to a `.3mf` file using the Rust `stl23mf` backend.
    - **Case B (Multi STL)**: Merged into a single, multi-object `.3mf` file to preserve the spatial relationship or logical grouping.
- **Image Optimization**: All associated images are converted to **WebP** for high-performance web delivery.
- **Metadata Generation**: A `datapackage.json` is automatically generated. The project **Title** is derived from the primary filename (e.g., `scary-dragon.stl` $\to$ `Scary Dragon`).

## 4. Output & Persistence
The final output is an "Atomic Project Folder" moved to the `/output` directory:
```text
/output/[Category]/[Project-Slug]/
├── [Project-Slug].3mf      <-- Consolidated Mesh
├── datapackage.json        <-- Manyfold Metadata
└── preview.webp            <-- Optimized Image
```

## 5. Summary Table
| Phase | Action | Outcome |
| :--- | :--- | :--- |
| **Ingest** | Watchdog detects `.stl` | Process starts. |
| **Logic** | Aggregation Check | Scans for other STLs and Images. |
| **Transform** | `stl23mf` Call | Merged 3MF created. |
| **Transform** | Image Conversion | Sibling images become WebP. |
| **Commit** | Directory Move | Clean output folder; input cleanup. |
