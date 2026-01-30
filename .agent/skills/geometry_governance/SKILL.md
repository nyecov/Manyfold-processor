---
name: Geometry Governance
description: Core principles and project boundaries for 3D model processing.
---

# Geometry Governance

This skill defines the architectural boundaries and strategic constraints for all 3D geometry processing within the Manyfold Processor.

## 1. Project Scope & Boundaries 
> [!CRITICAL]
> The Manyfold Processor is a **File Management Utility**, not a 3D modeling or repair tool.

### In Scope
*   **Parsing**: Reading triangle data from STL/3MF.
*   **Conversion**: STL to 3MF (Vertex deduplication).
*   **Aggregation**: Merging multiple STLs into a single 3MF object.
*   **Metadata**: Extracting and linking embedded metadata.

### Out of Scope
*   **Mesh Repair**: No hole filling, normal flipping, or intersection fixes.
*   **Validation**: No manifoldness or watertightness checks.
*   **Optimization**: No mesh simplification or decimation.

## 2. Technical Mandates 
*   **Language**: Heavy lifting MUST be done in **Rust**. Python falls back only for orchestration.
*   **Memory Management**:
    *   **Streaming**: Process large files in chunks. Never load 1GB+ files into memory entirely.
    *   **Tier 3 Strategy**: On devices with < 2GB RAM, use approximate deduplication or limited maps to prevent OOM.
*   **Efficiency**: Vertex deduplication is mandatory for STL -> 3MF to prevent file bloat.

---

## See Also
*   **STL Technicals**: [stl_specification](../stl_specification/SKILL.md)
*   **3MF Technicals**: [3mf_specification](../3mf_specification/SKILL.md)
*   **Hardware Constraints**: [deploy_on_radxa_rock5](../deploy_on_radxa_rock5/SKILL.md)
