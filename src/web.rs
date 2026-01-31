use axum::{
    extract::Path as AxumPath,
    routing::{delete, get, post, put},
    Extension, Json, Router,
};
use image::AnimationDecoder;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
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
        .route("/api/actions/process/:filename", post(process_file))
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

async fn process_file(
    AxumPath(filename): AxumPath<String>,
    Extension(state): Extension<AppState>,
) -> impl axum::response::IntoResponse {
    let input_dir = std::env::var("INPUT_DIR").unwrap_or_else(|_| "input".to_string());
    let output_dir = std::env::var("OUTPUT_DIR").unwrap_or_else(|_| "output".to_string());
    let input_path = std::path::Path::new(&input_dir).join(&filename);

    // Create output dir if not exists
    let _ = std::fs::create_dir_all(&output_dir);

    // Validate connection/path
    if !input_path.exists() {
        return (
            axum::http::StatusCode::NOT_FOUND,
            format!("File not found: {}", filename),
        );
    }

    let ext = input_path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_lowercase();
    let timeline_msg;

    // Logic:
    // If GIF -> Check frames. If > 1, preserve (copy). Else convert.
    // Else -> Convert to WebP.

    if ext == "gif" {
        // Check for animation
        let file = File::open(&input_path).unwrap();
        let reader = BufReader::new(file);
        let decoder = image::codecs::gif::GifDecoder::new(reader).unwrap();

        // Count frames (safe heuristic)
        let frames = decoder.into_frames().collect_frames().unwrap_or_default();

        if frames.len() > 1 {
            // Animated GIF - Preserve
            let output_path = std::path::Path::new(&output_dir).join(&filename);
            match std::fs::copy(&input_path, &output_path) {
                Ok(_) => {
                    timeline_msg = format!(
                        "Processed: {} -> {} (Preserved Animation)",
                        filename, filename
                    );
                }
                Err(e) => {
                    return (
                        axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Failed to copy GIF: {}", e),
                    )
                }
            }
        } else {
            // Static GIF - Convert to WebP
            let img = image::open(&input_path)
                .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()));
            if let Ok(dynamic_image) = img {
                let new_filename = format!(
                    "{}.webp",
                    std::path::Path::new(&filename)
                        .file_stem()
                        .unwrap()
                        .to_str()
                        .unwrap()
                );
                let output_path = std::path::Path::new(&output_dir).join(&new_filename);

                // Save as WebP
                // Currently image crate doesn't support saving WebP directly in all versions or via save(), checking capabilities.
                // Actually image 0.25 supports webp via encoders.
                // Using save_with_format is easiest if supported.

                match dynamic_image.save_with_format(&output_path, image::ImageFormat::WebP) {
                    Ok(_) => {
                        timeline_msg = format!("Processed: {} -> {}", filename, new_filename);
                    }
                    Err(e) => {
                        return (
                            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Failed to save WebP: {}", e),
                        )
                    }
                }
            } else {
                return img.err().unwrap();
            }
        }
    } else {
        // Standard Conversion
        let img = image::open(&input_path);
        match img {
            Ok(dynamic_image) => {
                let new_filename = format!(
                    "{}.webp",
                    std::path::Path::new(&filename)
                        .file_stem()
                        .unwrap()
                        .to_str()
                        .unwrap()
                );
                let output_path = std::path::Path::new(&output_dir).join(&new_filename);

                match dynamic_image.save_with_format(&output_path, image::ImageFormat::WebP) {
                    Ok(_) => {
                        timeline_msg = format!("Processed: {} -> {}", filename, new_filename);
                    }
                    Err(e) => {
                        return (
                            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Failed to save WebP: {}", e),
                        )
                    }
                }
            }
            Err(e) => {
                return (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to open image: {}", e),
                )
            }
        }
    }

    // Update Timeline
    if !timeline_msg.is_empty() {
        let mut t = state.timeline.lock().unwrap();
        t.push(timeline_msg);
    }

    (axum::http::StatusCode::OK, "Processed".to_string())
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
