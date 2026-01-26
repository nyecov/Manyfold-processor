# External Platform Integrations

## Overview
This document outlines the feasibility of integrating with major 3D model platforms to enrich metadata (description, tags, license, images).

## 📊 Platform Status Matrix

| Platform | Official API | Status | Feasibility | notes |
| :--- | :--- | :--- | :--- | :--- |
| **Thingiverse** | ✅ Open (REST) | Active | High | Requires API Key. Good for legacy content. |
| **Cults3D** | ✅ GraphQL | Active | Medium | Focused on user data/sales, file access restricted. Good for metadata. |
| **MyMiniFactory**| ✅ Open (REST) | Active | High | OAuth2 based. Good for store integration. |
| **Printables** | ❌ None | Unofficial | Low/Risky | "Unofficial" GraphQL exists but may break. No official support. |
| **MakerWorld** | ❌ None | Closed | Low | No public API. Metadata must be extracted from `.3mf` files (Case 2). |

## 🔗 Integration Strategies

### 1. MakerWorld (Case 2)
*   **Strategy**: Since there is no API, we strictly rely on the **Case 2** logic: parsing the internal `3D/3dmodel.model` XML within the declared 3MF file. This is reliable and does not require network access.

### 2. Thingiverse & MyMiniFactory
*   **Strategy**: Plugin-based architecture.
*   **Trigger**: If a user provides a URL (e.g., in a text file or UI input) or if a legacy ID is found in the filename (e.g., `thingiverse_12345.stl`).
*   **Action**: Fetch metadata -> Populate `datapackage.json`.

### 3. Printables / Cults
*   **Strategy**: Low priority due to access restrictions.
*   **Fallback**: User manual entry via Web UI.

## ⚠️ Recommendation
For the **Phase 1 MVP**, we should **NOT** implement live API scraping to avoid scope creep and rate-limiting issues.
*   **Focus**: Extracting metadata *already present* in the files (MakerWorld 3MFs are rich in data).
*   **Future**: Add a "Fetch Metadata from URL" button in the Web UI that triggers specific handlers.
