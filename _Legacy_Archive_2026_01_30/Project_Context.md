# Project Context: Manyfold Processor

## 🎯 Goal
To create a processing pipeline that prepares 3D model files for insertion into a self-hosted **Manyfold** service.

*   **Repo**: `https://github.com/nyecov/Manyfold-processor`
*   **Target Service**: [Manyfold](https://github.com/manyfold3d/manyfold)

## 🔄 Workflow Profile
1.  **Input Scenarios** (The processor must handle these 4 cases):
    *   **Case 1: Raw STL Files** (Subset of Case 3)
        *   Input: One or more `.stl` files.
        *   Action: Convert to `.3mf` (compression benefit), then treat as **Case 3**.
    *   **Case 2: MakerWorld 3MF**
        *   Input: `.3mf` files sourced from MakerWorld.
        *   Action: Extract rich metadata (Title, Description, Licensing, Creator, Links), extract embedded images/thumbnails.
    *   **Case 3: Generic 3MF**
        *   Input: Standard `.3mf` files (e.g., from PrusaSlicer/Generic).
        *   Action: Attempt metadata extraction (likely partial/limited). Fallback to filename parsing.
    *   **Case 4: Mixed Archives** (Superset)
        *   Input: Compressed folders (`.zip`, `.7z`, `.rar`) containing any combination of Cases 1-3.
        *   Action: Recursively unpack, identify content type, and process accordingly.

## 📉 Secondary Goal: Storage Optimization
*   **Objective**: Reduce physical disk usage.
*   **Method**: Convert voluminous STL files to the more efficient `.3mf` format (which uses ZIP compression) during processing.

2.  **Process**:
    *   Convert/Validate geometry.
    *   Generate metadata.
    *   Render previews.
3.  **Output** (Paired artifacts):
    *   **Model**: `.3mf` (Standardized 3D format)
    *   **Metadata**: `datapackage.json` (Manyfold compatible)
    *   **Preview**: `.webp` (Optimized image)

## 💡 Key Design Constraint
The output must represent a valid "Object" structure that can be directly ingested by a Manyfold instance without further manual tagging.

## ⚙️ Manyfold Service Configuration
*   **Database Path**: `\\Rock-5-itx\db\manyfold` (Local: `/DB/manyfold`)
*   **Environment Variables**:
    *   `DB_USER`: `manyfold`
    *   `DB_PASS`: `swungpresoakunmaskedgravitysloped8punch`
    *   `DB_NAME`: `manyfold`
    *   `REDIS_PASS`: `siberianprognosislevershastiness5macesemicolon`
    *   `SECRET_KEY_BASE`: `QKYgk66bUzt9evzk9SKPz8e4UAEmD7JRMxkgW68aPCD97M4ZJ5g9yxs5Z8LbE9Jp`

