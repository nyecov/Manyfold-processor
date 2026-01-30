---
name: STL Handling
description: Specialized guide for processing STL files, covering parsing, error correction, and format conversions.
---

# STL (Stereolithography) Handling

## 1. Core Specifications

*   **Format**: "Triangle Soup" - no connectivity information, just raw triangles.
*   **Binary vs ASCII**: 
    *   **Binary** (Standard): 80-byte header + triangle count (4 bytes) + 50 bytes per triangle.
    *   **ASCII**: Human-readable but extremely large and slow to parse.

## 2. Rust Implementation

### Parsing
*   **Crate**: `stl_io` (Read/Write).
*   **Strategy**: Stream triangles. **Never** load 1GB+ files into `Vec<Triangle>` if only converting.
*   **Binary Detection**: Check first 5 bytes for "solid" (ASCII) vs binary header.

### Example
```rust
use stl_io::read_stl;
use std::fs::File;
use std::io::BufReader;

let mut file = BufReader::new(File::open("model.stl")?);
let mesh = read_stl(&mut file)?;
// mesh.vertices, mesh.faces
```

## 3. Project Scope: File Processing Only

> **CRITICAL**: Mesh repair, watertightness checking, and geometry optimization are **OUT OF SCOPE** for the Manyfold Processor.

### What This Project Does
*   **Parse** STL files (read triangle data).
*   **Convert** STL to 3MF format (vertex deduplication).
*   **Extract** metadata and associate sibling files (images).

### What This Project Does NOT Do
*   **Repair** broken geometry (holes, intersections, flipped normals).
*   **Validate** mesh manifoldness or watertightness.
*   **Optimize** mesh topology or reduce triangle count.

**Rationale**: The Manyfold application itself handles model analysis and validation. This processor is a **file management utility**, not a 3D modeling tool.

## 4. Conversion to 3MF

### Pipeline
1.  **Read STL**: Stream binary triangles.
2.  **Deduplicate Vertices**: STL repeats vertices for every face. 3MF uses a shared Vertex List.
    *   **Algorithm**: Use `HashMap<OrderedFloat<f32>, Index>` to map XYZ coords to a single ID.
3.  **Write 3MF**: Write unique Vertex list + Triangle Index list to `3dmodel.model`.

### Memory Considerations
*   **Streaming**: Use `BufReader` for large files.
*   **Deduplication Map**: Can grow large on complex models. Consider `DashMap` for concurrent access or probabilistic checks if RAM < 1GB.

## 5. Large Asset Handling
*   **Never Buffer Entire Mesh**: Process in chunks.
*   **RAM Constraint**: On Tier 3 devices (< 2GB RAM), limit deduplication map size or use approximate methods.
