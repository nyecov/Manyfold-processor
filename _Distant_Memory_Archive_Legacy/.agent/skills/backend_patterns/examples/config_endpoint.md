# Config Endpoint: Code Example

## Rust/Axum Implementation

### Request Payload
```rust
#[derive(Deserialize)]
struct ConfigPayload {
    enabled: bool,
}
```

### GET Handler
```rust
async fn get_config(Extension(state): Extension<AppState>) -> Json<Value> {
    let enabled = *state.setting.lock().unwrap();
    Json(json!({ "enabled": enabled }))
}
```

### PUT Handler
```rust
async fn set_config(
    Extension(state): Extension<AppState>,
    Json(payload): Json<ConfigPayload>,
) -> Json<Value> {
    let mut setting = state.setting.lock().unwrap();
    *setting = payload.enabled;
    log::info!("Config set to: {}", *setting);
    Json(json!({ "status": "success", "enabled": *setting }))
}
```

### Router Setup
```rust
let app = Router::new()
    .route("/api/config/my-setting", get(get_config))
    .route("/api/config/my-setting", put(set_config))
    .layer(Extension(state));
```

---

## Project Reference
- Implementation: [web.rs](../../../../src/web.rs)
