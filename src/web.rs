use axum::{
    extract::Path as AxumPath,
    routing::{delete, get, post, put},
    Extension, Json, Router,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashSet,
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use sysinfo::{CpuExt, CpuRefreshKind, RefreshKind, System, SystemExt};
use tower_http::services::ServeDir;

#[derive(Serialize)]
struct Status {
    engine_status: String,
    queue_count: usize,
    queue_items: Vec<String>,
    processed_count: usize,
    system_load: f32,
    memory_usage: u64,
    auto_process_enabled: bool,
    timeline_events: Vec<String>,
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
    queue_items: Arc<Mutex<Vec<String>>>,
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
        queue_items: Arc::new(Mutex::new(Vec::new())),
        timeline: Arc::new(Mutex::new(Vec::new())),
    };

    // Spawn File Watcher (Mock/Simple)
    let watcher_state = state.clone();
    let input_dir = std::env::var("INPUT_DIR").unwrap_or_else(|_| "input".to_string());
    tokio::spawn(async move {
        let mut seen_files: HashSet<String> = HashSet::new();
        loop {
            let mut current_files = Vec::new();
            if let Ok(entries) = std::fs::read_dir(&input_dir) {
                for entry in entries.flatten() {
                    if entry.path().is_file() {
                        if let Some(name) = entry.file_name().to_str() {
                            current_files.push(name.to_string());
                        }
                    }
                }
            }

            // Update State
            {
                let mut q = watcher_state.queue_count.lock().unwrap();
                *q = current_files.len();

                let mut items = watcher_state.queue_items.lock().unwrap();
                *items = current_files.clone();

                let mut t = watcher_state.timeline.lock().unwrap();
                for f in &current_files {
                    if !seen_files.contains(f) {
                        t.push(format!("Incoming: {} (Type: Loose)", f));
                        seen_files.insert(f.clone());
                    }
                }

                // Optional: cleanup seen_files if they are removed from current_files
                // so they can be re-detected if added again.
                seen_files.retain(|f| current_files.contains(f));
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
        .route("/api/actions/delete-file/:filename", delete(delete_file))
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
    let queue_items = state.queue_items.lock().unwrap().clone();
    let timeline = state.timeline.lock().unwrap().clone();

    Json(Status {
        engine_status: "online".to_string(),
        queue_count: queue,
        queue_items,
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

async fn delete_file(
    Extension(state): Extension<AppState>,
    AxumPath(filename): AxumPath<String>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    // Basic security: prevent path traversal
    if filename.contains('/') || filename.contains('\\') || filename.contains("..") {
        return Err(axum::http::StatusCode::BAD_REQUEST);
    }

    let input_dir = std::env::var("INPUT_DIR").unwrap_or_else(|_| "input".to_string());
    let path = std::path::Path::new(&input_dir).join(&filename);

    if std::fs::remove_file(path).is_ok() {
        log::info!("File deleted via API: {}", filename);
        {
            let mut t = state.timeline.lock().unwrap();
            t.push(format!("Deleted: {} (Manually)", filename));
        }
        Ok(Json(serde_json::json!({ "status": "success" })))
    } else {
        Err(axum::http::StatusCode::NOT_FOUND)
    }
}

async fn health_check() -> axum::http::StatusCode {
    axum::http::StatusCode::OK
}
