---
name: Manyfold API Endpoints
description: Technical specification for Manyfold instance API interactions.
---

# Manyfold API Endpoints

Technical reference for the REST and Tus interfaces provided by Manyfold.

## 1. Authentication
*   **Type**: OAuth2 / Personal Access Token.
*   **Header**: `Authorization: Bearer <token>`

## 2. Model Creation Workflow
1.  **POST `/upload`**: Initialize Tus session.
2.  **PATCH `/upload/:id`**: Stream file content.
3.  **POST `/models`**: Register model using the Tus ID.

## 3. Payload Schema
*   **Create Model**: `{"json": {"name": "...", "files": [{"id": "...", "name": "..."}]}}`

## 4. Rust Implementation
*   **Crates**: `reqwest` (HTTP), `serde` (JSON).
*   **Middleware**: Use `reqwest-middleware` for [Resilience Policies](../api_resilience_strategy/SKILL.md).

---

## See Also
*   **Policy**: [api_resilience_strategy](../api_resilience_strategy/SKILL.md)
*   **External Reference**: [manyfold_reference_material](../manyfold_reference_material/SKILL.md)
