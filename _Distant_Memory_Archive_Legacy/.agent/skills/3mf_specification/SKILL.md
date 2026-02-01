---
name: 3MF Specification
description: Technical details for the 3MF Manufacturing Format.
---

# 3MF Specification

Technical implementation details for the ZIP/XML-based 3MF format.

## 1. Structure
*   **Container**: Standard ZIP archive.
*   **Core Model**: `3D/3dmodel.model` (XML).
*   **Metadata**: Embedded in the model XML or in the `Metadata/` subfolder.

## 2. Rust Implementation
*   **ZIP**: `zip` crate for random access.
*   **XML**: `quick-xml` for low-memory streaming parsing.

## 3. Critical Constraints
*   **MustHonor**: Any extension with `mustHonor="true"` that is not recognized MUST cause a hard failure.
*   **Resumable Uploads**: Reference the [Tus protocol](../manyfold_api_endpoints/SKILL.md) for 3MF registration.

---

## See Also
*   **Governance**: [geometry_governance](../geometry_governance/SKILL.md)
*   **Source**: [stl_specification](../stl_specification/SKILL.md)
