---
name: Manyfold API Integration
description: Guide for implementing API communications between the Rust Processor and the Manyfold instance.
---

# Manyfold API Integration

This guide outlines how to interact with the Manyfold instance from the Rust-based Processor.

## 1. Authentication

Manyfold uses **Doorkeeper** (OAuth2) or Personal Access Tokens.
*   **Header**: `Authorization: Bearer <YOUR_ACCESS_TOKEN>`
*   **Generation**: Users generates API tokens in the Manyfold WebUI (User Settings -> API Tokens).

## 2. File Upload Protocol: Tus

Manyfold uses the **Tus.io** resumable upload protocol. You **cannot** upload files directly via the `/models` endpoint. You must first upload to the Tus server, then link the resulting ID when creating the model.

*   **Tus Endpoint**: `/upload` (e.g., `http://localhost:3214/upload`)
*   **Rust Crate**: `tus_client` (or implement raw HTTP per Tus spec).

### Upload Workflow
1.  **POST** to `/upload` to create a session.
2.  **PATCH** content to the returned location.
3.  **Result**: You get a final URL/ID for the uploaded file.

## 3. Creating a Model

Once files are uploaded via Tus, you create the model by referencing them.

*   **Endpoint**: `POST /models`
*   **Header**: `Content-Type: application/json`
*   **Payload Schema** (`ManyfoldApi::V0::UploadedModelDeserializer`):

```json
{
  "json": {
    "name": "My Processed Model",
    "files": [
      {
        "id": "http://localhost:3214/upload/12345abcd...", 
        "name": "model_geometry.3mf"
      }
    ],
    "keywords": ["processed", "auto"],
    "sensitive": false
  }
}
```

> [!NOTE]
> The `id` in the `files` array must be the **full URL** or unique identifier returned by the Tus server.

## 4. Rust Implementation Guide

### A. Dependencies
Add to `Cargo.toml`:
```toml
[dependencies]
reqwest = { version = "0.11", features = ["json", "multipart"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
# standard http for headers
```

### B. Example Workflow

```rust
use reqwest::Client;
use serde_json::json;

async fn upload_and_create(base_url: &str, token: &str, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    // 1. Upload via Tus (Simplified pseudo-code, distinct from standard POST)
    // In reality, use a Tus client library to handle the Creation and Offset steps.
    let tus_url = format!("{}/upload", base_url);
    let upload_url = perform_tus_upload(&tus_url, file_path).await?; 

    // 2. Create Model
    let payload = json!({
        "json": {
            "name": "New Import",
            "files": [
                {
                    "id": upload_url, // URL returned by Tus
                    "name": "geometry.3mf"
                }
            ],
            "keywords": ["tag1"]
        }
    });

    let res = client.post(format!("{}/models", base_url))
        .header("Authorization", format!("Bearer {}", token))
        .header("Content-Type", "application/json") // Crucial
        .json(&payload)
        .send()
        .await?;
        
    if !res.status().is_success() {
        eprintln!("Error: {:?}", res.text().await?);
    }
    
    Ok(())
}
```

## 5. Other Endpoints

*   **Trigger Scan**: `POST /models/:id/scan` - Queues a background analysis job.
*   **Accept**: Manyfold endpoints respond to `json` or `manyfold_api_v0` format. Ensuring `Accept: application/json` is usually sufficient.

## 6. Development Tips

*   **Routes**: To see all routes in development, clone Manyfold and run `bin/rails routes`.
## 7. Resilience & Reliability

The API client must be robust against network glitches and downtime of the Manyfold instance.

### A. Retry Policy (Exponential Backoff)
*   **Requirement**: All non-mutating (GET) and idempotent (PUT/DELETE) requests MUST retry on failures (5xx or Timeout).
*   **Strategy**: Exponential Backoff + Jitter.
    *   Base delay: 500ms
    *   Max retries: 5
*   **Uploads**: Large file uploads (Tus) must use the `tus_client`'s built-in resume capability. If that fails, restart the *chunk*, not the whole file.

### B. Circuit Breaker
*   **Scenario**: If Manyfold returns `503 Service Unavailable` or connection refused repeatedly.
*   **Action**: Stop all outgoing requests for a cooldown period (e.g., 60 seconds) to prevent cascading failures.
*   **Implementation**: Use `reqwest-middleware` with retry middleware.

### C. Example Configuration
```rust
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::{RetryTransientMiddleware, policies::ExponentialBackoff};

pub fn build_resilient_client() -> ClientWithMiddleware {
    let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);
    ClientBuilder::new(reqwest::Client::new())
        .with(RetryTransientMiddleware::new_with_policy(retry_policy))
        .build()
}
```

