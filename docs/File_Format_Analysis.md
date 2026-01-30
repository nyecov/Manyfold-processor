# File Format Analysis & handling Strategy

This document details the internal structure and processing strategy for supported file formats.

## 1. Archive Formats (`.zip`, `.rar`, `.7z`)

**Legacy Logic**: Used `patoolib` to spawn external CLIs (`unrar`, `7z`).
**Rust Strategy**: Use native libraries where possible for performance and safety, falling back to bindings only when necessary.

### A. ZIP
*   **Structure**: Central Directory at end of file. Random access possible.
*   **Rust Crate**: `zip` (Native Rust).
*   **Strategy**:
    *   **Large Files**: Open with `ZipArchive` (streaming/seekable). Do NOT read into RAM.
    *   **Extraction**: Stream individual files to `working_dir` (RAM Drive).
    *   **Metadata**: Read `3D/3dmodel.model` (if 3MF) directly from the archive without full extraction.

### B. RAR / 7Zip
*   **Structure**: Solid compression often prevents random access. Sequential extraction usually required.
*   **Rust Crate**: `compress-tools` (Bindings to `libarchive`).
    *   *Why*: `sevenz-rust` exists but `libarchive` covers RAR, 7Z, TAR, etc., with a single API.
*   **Strategy**:
    *   **Extraction**: Sequential stream to `working_dir`.
    *   **Password**: Fail fast if password protected (Exception: Allow user to configure a global "keyring" in future).

---

## 2. 3D Model Formats

### A. 3MF (3D Manufacturing Format)
*   **Structure**: Data-heavy ZIP container.
    *   `3D/3dmodel.model`: XML Geometry & Metadata.
    *   `Metadata/`: Thumbnail images (png/jpg).
*   **Rust Strategy**:
    *   Treat as **ZIP**.
    *   **Parsing**: Use `quick-xml` (fast, streaming) to find `<metadata name="Title">` tags in `3dmodel.model`.
    *   **Thumbnails**: Extract directly from `Metadata/` folder. Do not re-encode if already WebP/JPG.

### B. STL (Stereolithography)
*   **Structure**: Binary or ASCII list of triangles. No metadata.
*   **Legacy Logic**:
    *   **Aggregation**: Merged multiple STLs into one 3MF.
    *   **Siblings**: Matched images via filename tokens (e.g., `car_v1.stl` <-> `car_preview.jpg`).
*   **Rust Strategy**:
    *   **Parsing**: Use `stl_io` crate.
    *   **Conversion**: Stream triangles into a new 3MF container (using `xml-writer` or similar). structure.
    *   **Sibling Request**: Re-implement the "Token Overlap" logic in Rust to find preview images.

### C. OBJ (Wavefront)
*   **Structure**: ASCII (`v`, `vn`, `f` lines).
*   **Challenge**: Optimization. ASCII parsing is slow in Python, fast in Rust.
*   **Strategy**:
    *   Parse with `tobj` (Tiny OBJ Loader).
    *   Convert to 3MF mesh structure immediately.

---

## 3. Large File Strategy (1GB+)

*   **Constraint**: Tier 3 devices (1GB RAM) cannot load the full file.
*   **Implementation**:
    1.  **Archives**: Never call `.extract_all()`. Iterate entries and stream `std::io::copy` to disk.
    2.  **3MF**: Peek at `3dmodel.model` header (first 50KB) for metadata. Stop reading if title found.
    3.  **IO Buffer**: Use `BufReader` with 8KB capacity for efficient SSD usage.
