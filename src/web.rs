use axum::{
    routing::{get, post, put},
    Extension, Json, Router,
};
use serde::{Deserialize, Serialize};
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use sysinfo::{CpuExt, CpuRefreshKind, RefreshKind, System, SystemExt};
use tower_http::services::ServeDir;

#[derive(Serialize)]
struct Status {
    engine_status: String,
    queue_count: usize,
    processed_count: usize,
    system_load: f32,
    memory_usage: u64,
    auto_process_enabled: bool,
}

#[derive(Deserialize)]
struct AutoProcessConfig {
    enabled: bool,
}

#[derive(Clone)]
struct AppState {
    sys: Arc<Mutex<System>>,
    auto_process_enabled: Arc<Mutex<bool>>,
}

pub async fn start_web_server() -> anyhow::Result<()> {
    // Initialize system collector (CPU + RAM only)
    let sys = System::new_with_specifics(
        RefreshKind::new()
            .with_cpu(CpuRefreshKind::everything())
            .with_memory(),
    );

    let state = AppState {
        sys: Arc::new(Mutex::new(sys)),
        auto_process_enabled: Arc::new(Mutex::new(false)), // Default OFF
    };

    // Serve static files from the "static" directory
    let static_files = ServeDir::new("static");

    // Define the router
    let app = Router::new()
        .nest_service("/", static_files)
        .route("/api/status", get(get_status))
        .route("/api/process/all", post(process_all))
        .route("/api/config/auto-process", get(get_auto_process))
        .route("/api/config/auto-process", put(set_auto_process))
        .layer(Extension(state));

    // Define the address
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    log::info!("Web server listening on http://{}", addr);

    // Start the server
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .map_err(|e| anyhow::anyhow!("Axum server error: {}", e))?;

    Ok(())
}

async fn get_status(Extension(state): Extension<AppState>) -> Json<Status> {
    let mut sys = state.sys.lock().unwrap();
    sys.refresh_cpu();
    sys.refresh_memory();

    let load = sys.global_cpu_info().cpu_usage();
    let memory = sys.used_memory() / 1024 / 1024; // MB
    let auto = *state.auto_process_enabled.lock().unwrap();

    Json(Status {
        engine_status: "online".to_string(),
        queue_count: 0,
        processed_count: 0,
        system_load: load,
        memory_usage: memory,
        auto_process_enabled: auto,
    })
}

async fn get_auto_process(Extension(state): Extension<AppState>) -> Json<serde_json::Value> {
    let enabled = *state.auto_process_enabled.lock().unwrap();
    Json(serde_json::json!({ "enabled": enabled }))
}

async fn set_auto_process(
    Extension(state): Extension<AppState>,
    Json(payload): Json<AutoProcessConfig>,
) -> Json<serde_json::Value> {
    let mut auto = state.auto_process_enabled.lock().unwrap();
    *auto = payload.enabled;
    log::info!("Auto-Processor set to: {}", *auto);
    Json(serde_json::json!({ "status": "success", "enabled": *auto }))
}

async fn process_all() -> Json<serde_json::Value> {
    log::info!("Triggering manual process-all from UI");
    Json(serde_json::json!({ "status": "success", "message": "Batch processing initiated" }))
}
