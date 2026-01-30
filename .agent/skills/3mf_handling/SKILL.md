---
name: 3MF Handling
description: Specialized guide for processing 3MF files, covering parsing, validation, and metadata extraction.
---

# 3MF (3D Manufacturing Format) Handling

## 1. Core Specifications

*   **Container**: ZIP-based archive.
*   **Geometry**: XML file at `3D/3dmodel.model`.
*   **Philosophy**: "Unambiguous" - designed to solve STL's ambiguities.
*   **Spec**: [3MF Consortium](https://3mf.io/)

## 2. File Structure

```
model.3mf (ZIP)
├── 3D/
│   └── 3dmodel.model (XML - Vertices, Triangles, Metadata)
├── Metadata/
│   ├── thumbnail.png
│   └── preview.jpg
└── [Rels]/ (Relationships - optional)
```

## 3. Rust Implementation

### Parsing Strategy
*   **Treat as ZIP**: Use `zip` crate for random access.
*   **XML Parsing**: Use `quick-xml` (streaming, fast).

### Example
```rust
use zip::ZipArchive;
use quick_xml::Reader;
use std::fs::File;

let file = File::open("model.3mf")?;
let mut archive = ZipArchive::new(file)?;

// Extract metadata
let model_file = archive.by_name("3D/3dmodel.model")?;
let mut reader = Reader::from_reader(model_file);
// Parse XML for <metadata name="Title">
```

## 4. Validation & Error Handling

### XML Integrity
*   **Well-Formed**: Ensure all tags are properly closed.
*   **Namespace Handling**: 3MF uses XML namespaces. Parse with namespace awareness.

### MustHonor Attribute
*   **Critical**: If `mustHonor="true"` on an extension the processor doesn't understand, **MUST FAIL**.
*   **Example**: `<model xmlns:custom="..." mustHonor="true">` - if `custom` is unknown, reject the file.

### Project Scope Limitation
> **CRITICAL**: Mesh validation, watertightness checking, and geometry repair are **OUT OF SCOPE** for the Manyfold Processor.

*   **What We Parse**: XML structure, metadata fields, file integrity.
*   **What We Don't Validate**: Mesh manifoldness, triangle topology, geometric correctness.
*   **Rationale**: Manyfold handles model analysis. This processor focuses on **file management and metadata extraction**.

## 5. Metadata Extraction

### Common Metadata Fields
*   `<metadata name="Title">` - Model name.
*   `<metadata name="Description">` - Model description.
*   `<metadata name="Designer">` - Creator name.

### Thumbnails
*   Located in `Metadata/` folder.
*   Extract directly from ZIP without full decompression.
*   **Optimization**: If already WebP/JPG, do NOT re-encode.

## 6. Large File Strategy

*   **Random Access**: 3MF (ZIP) allows seeking to specific files.
*   **Metadata-Only Read**: Extract `3dmodel.model` header (first 50KB) for metadata without loading full mesh.
*   **Memory**: Use `BufReader` with 8KB buffer for efficient SSD usage.

## 7. Writing 3MF

### Structure
1.  Create ZIP archive.
2.  Write `3D/3dmodel.model` with:
    *   Vertex list: `<vertices><vertex x="..." y="..." z="..."/></vertices>`
    *   Triangle list: `<triangles><triangle v1="0" v2="1" v3="2"/></triangles>`
3.  Add thumbnails to `Metadata/` folder.
4.  Write `[Content_Types].xml` (required by ZIP spec).

### Example (Simplified)
```rust
use zip::write::FileOptions;
use std::io::Write;

let file = File::create("output.3mf")?;
let mut zip = zip::ZipWriter::new(file);

zip.start_file("3D/3dmodel.model", FileOptions::default())?;
zip.write_all(b"<model>...</model>")?;

zip.finish()?;
```
