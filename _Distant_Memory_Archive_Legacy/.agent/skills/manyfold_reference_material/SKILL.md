---
name: Manyfold Reference Material
description: Guidelines for consulting the local Manyfold repository for internal logic and API specifics.
---

# Manyfold Reference Material

To aid development, a local clone of the Manyfold repository is available in `Manyfold_reference/`. This should be used **strictly for reference** to understand internal data structures, API behavior, and file handling logic.

## 📂 Location
*   **Path**: `Manyfold_reference/` (Git-ignored)
*   **Source**: `https://github.com/manyfold3d/manyfold`

## 🔍 How to Use

Use this local copy to answer questions that are not covered by the public documentation or our `manyfold_api_integration` skill.

### 1. API & Routing
*   **Routes**: Check `config/routes.rb` to find the exact path for endpoints.
*   **Controllers**: Inspect `app/controllers/` (especially `models_controller.rb` and `model_files_controller.rb`) to understand expected parameters and JSON payloads.
*   **Deserializers**: Check `app/deserializers/manyfold_api/v0/` to see the exact structure of JSON objects required for POST/PATCH requests.

### 2. File Handling
*   **Upload Protocol**: Refer to `config/initializers/tus.rb` (if present) or `config/routes.rb` to verify Tus settings.
*   **Supported Formats**: Check `config/initializers/mime_types.rb` to see which 3D file formats are officially supported.

### 3. Database Schema
*   **Schema**: View `db/schema.rb` to understand the relationships between Models, Files, Libraries, and Creators.

## ⚠️ Important Notes
*   **Do Not Edit**: Never modify files inside `Manyfold_reference/`.
*   **Do Not Commit**: This directory is git-ignored.
*   **Cross-Reference**: When implementing features, always cross-reference findings here with the official **[Manyfold API Endpoints](../manyfold_api_endpoints/SKILL.md)** skill to ensure you are using the public API contract, not internal implementations.
