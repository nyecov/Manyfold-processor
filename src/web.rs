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
    timeline_events: Vec<String>, // Added timeline
}

#[derive(Deserialize)]
struct AutoProcessConfig {
    enabled: bool,
}

#[derive(Clone)]
struct AppState {
    sys: Arc<Mutex<System>>,
    auto_process_enabled: Arc<Mutex<bool>>,
    queue_count: Arc<Mutex<usize>>,
    timeline: Arc<Mutex<Vec<String>>>,
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
        auto_process_enabled: Arc::new(Mutex::new(false)),
        queue_count: Arc::new(Mutex::new(0)),
        timeline: Arc::new(Mutex::new(Vec::new())),
    };

    // Spawn File Watcher (Mock/Simple)
    let watcher_state = state.clone();
    let input_dir = std::env::var("INPUT_DIR").unwrap_or_else(|_| "input".to_string());
    tokio::spawn(async move {
        loop {
            // Count files in input_dir
            let mut count = 0;
            let mut new_files = Vec::new();
            if let Ok(entries) = std::fs::read_dir(&input_dir) {
                for entry in entries.flatten() {
                    if entry.path().is_file() {
                        count += 1;
                        if let Some(name) = entry.file_name().to_str() {
                            new_files.push(format!("Incoming: {} (Type: Loose)", name));
                        }
                    }
                }
            }

            // Update State
            {
                let mut q = watcher_state.queue_count.lock().unwrap();
                *q = count;

                let mut t = watcher_state.timeline.lock().unwrap();
                // Naive: just replace timeline with current files for this test
                // Logic: If file exists, it's "Incoming"
                *t = new_files;
            }

            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        }
    });

    // Serve static files from the "static" directory
    let static_files = ServeDir::new("static");

    // Define the router
    let app = Router::new()
        .nest_service("/", static_files)
        .route("/api/status", get(get_status))
        .route("/api/process/all", post(process_all))
        .route("/api/config/auto-process", get(get_auto_process))
        .route("/api/config/auto-process", put(set_auto_process))
        .route("/api/actions/clear-timeline", post(clear_timeline))
        .route("/health", get(health_check))
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
    let queue = *state.queue_count.lock().unwrap();
    let timeline = state.timeline.lock().unwrap().clone();

    Json(Status {
        engine_status: "online".to_string(),
        queue_count: queue,
        processed_count: 0,
        system_load: load,
        memory_usage: memory,
        auto_process_enabled: auto,
        timeline_events: timeline,
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

async fn clear_timeline(Extension(state): Extension<AppState>) -> Json<serde_json::Value> {
    let mut t = state.timeline.lock().unwrap();
    t.clear();
    log::info!("Timeline cleared via API");
    Json(serde_json::json!({ "status": "success" }))
}

async fn health_check() -> axum::http::StatusCode {
    axum::http::StatusCode::OK
}
