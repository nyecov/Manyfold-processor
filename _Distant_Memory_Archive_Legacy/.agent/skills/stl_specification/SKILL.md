---
name: STL Specification
description: Technical details for parsing and processing STL files.
---

# STL (Stereolithography) Specification

Technical implementation details for the STL file format.

## 1. Format Details
*   **Binary**: 80-byte header + triangle count + 50 bytes per triangle.
*   **ASCII**: "Triangle Soup" - large and inefficient.
*   **Detection**: Binary is assumed if the file doesn't start with the literal "solid" tag.

## 2. Rust Implementation
*   **Crate**: `stl_io` for robust streaming I/O.
*   **Triangle Loop**: Use `BufReader` to iterate through triads without full buffering.

## 3. Conversion to 3MF
*   **Vertex Deduplication**: Mandatory mapping of duplicate coordinates to unique IDs.
*   **Precision**: Use `OrderedFloat<f32>` to handle float comparisons safely in maps.

---

## See Also
*   **Governance**: [geometry_governance](../geometry_governance/SKILL.md)
*   **Target**: [3mf_specification](../3mf_specification/SKILL.md)
