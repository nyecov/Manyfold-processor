use crate::config::SystemSettings;
use crate::geometry::GeometryHelper;
use crate::metadata::{MetadataHelper, Resource};
use axum::{
    extract::Path as AxumPath,
    routing::{delete, get, post},
    Extension, Json, Router,
};
use dashmap::DashMap;
use tokio::sync::watch;
use tracing::{info, warn, instrument};
use image::AnimationDecoder;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
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
    queue_items_with_size: Vec<(String, u64)>,
    processed_count: usize,
    system_load: f32,
    memory_usage: u64,
    auto_process_enabled: bool,
    timeline_events: Vec<String>,
    settle_status: HashMap<String, f32>,
    collisions: Vec<String>, // List of filenames that would collide
}

#[derive(Deserialize)]
struct SettingsUpdate {
    naming_penalties: Option<Vec<String>>,
    auto_process_enabled: Option<bool>,
    network_settle_seconds: Option<f32>,
}

#[derive(Clone)]
struct FileSettleInfo {
    last_size: u64,
    pulses_stable: u32,
    is_ready: bool,
}

#[derive(Clone)]
struct AppState {
    sys: Arc<Mutex<System>>,
    settings: Arc<Mutex<SystemSettings>>,
    queue_count: Arc<Mutex<usize>>,
    
    // Broadcast channels for state (Receiver side for consumers)
    queue_items: watch::Receiver<Vec<String>>,
    queue_items_with_size: watch::Receiver<Vec<(String, u64)>>,
    timeline: watch::Receiver<Vec<String>>,
    collisions: watch::Receiver<Vec<String>>,

    // Internal Senders for the watcher loop
    queue_items_tx: Arc<watch::Sender<Vec<String>>>,
    queue_items_with_size_tx: Arc<watch::Sender<Vec<(String, u64)>>>,
    timeline_tx: Arc<watch::Sender<Vec<String>>>,
    collisions_tx: Arc<watch::Sender<Vec<String>>>,

    file_settle_state: Arc<DashMap<String, FileSettleInfo>>,
}

pub async fn start_web_server() -> anyhow::Result<()> {
    // Initialize system collector (CPU + RAM only)
    let sys = System::new_with_specifics(
        RefreshKind::new()
            .with_cpu(CpuRefreshKind::everything())
            .with_memory(),
    );

    // Initialize watch channels
    let (queue_items_tx, queue_items_rx) = watch::channel(Vec::new());
    let (queue_items_with_size_tx, queue_items_with_size_rx) = watch::channel(Vec::new());
    let (timeline_tx, timeline_rx) = watch::channel(Vec::new());
    let (collisions_tx, collisions_rx) = watch::channel(Vec::new());

    let state = AppState {
        sys: Arc::new(Mutex::new(sys)),
        settings: Arc::new(Mutex::new(SystemSettings::load())),
        queue_count: Arc::new(Mutex::new(0)),
        queue_items: queue_items_rx,
        queue_items_with_size: queue_items_with_size_rx,
        timeline: timeline_rx,
        collisions: collisions_rx,
        queue_items_tx: Arc::new(queue_items_tx),
        queue_items_with_size_tx: Arc::new(queue_items_with_size_tx),
        timeline_tx: Arc::new(timeline_tx),
        collisions_tx: Arc::new(collisions_tx),
        file_settle_state: Arc::new(DashMap::new()),
    };

    // Spawn File Watcher (Refined with Settle Logic)
    let watcher_state = state.clone();
    let input_dir = std::env::var("INPUT_DIR").unwrap_or_else(|_| "input".to_string());
    let output_dir = std::env::var("OUTPUT_DIR").unwrap_or_else(|_| "output".to_string());
    
    tokio::spawn(async move {
        let mut seen_files: HashSet<String> = HashSet::new();
        let mut last_timeline = Vec::new();

        loop {
            let mut current_files = Vec::new();
            if let Ok(mut entries) = tokio::fs::read_dir(&input_dir).await {
                while let Ok(Some(entry)) = entries.next_entry().await {
                    if let Ok(meta) = entry.metadata().await {
                        if meta.is_file() {
                            if let Some(name) = entry.file_name().to_str() {
                                current_files.push(name.to_string());
                            }
                        }
                    }
                }
            }

            // Sync Settle State & Collision Logic
            let mut current_collisions = Vec::new();
            let mut current_items_with_size = Vec::new();
            
            {
                let settings = watcher_state.settings.lock().unwrap().clone();
                let target_pulses = (settings.network_settle_seconds / 0.5).ceil() as u32;

                // Cleanup deleted files from DashMap (Periodic State Consistency)
                watcher_state.file_settle_state.retain(|f, _| current_files.contains(f));

                for f in &current_files {
                    let path = std::path::Path::new(&input_dir).join(f);
                    
                    if let Ok(m) = std::fs::metadata(&path) {
                        let current_size = m.len();
                        current_items_with_size.push((f.clone(), current_size));

                        let mut info = watcher_state.file_settle_state.entry(f.clone()).or_insert(FileSettleInfo {
                            last_size: current_size,
                            pulses_stable: 0,
                            is_ready: false,
                        });

                        if !info.is_ready {
                            if info.last_size == current_size {
                                info.pulses_stable += 1;
                                // Fast non-blocking lock check is hard globally, 
                                // so we stick to the stable pulse count for now
                                if info.pulses_stable >= target_pulses {
                                    info.is_ready = true;
                                    log::info!("File settled and ready: {}", f);
                                }
                            } else {
                                info.last_size = current_size;
                                info.pulses_stable = 0;
                            }
                        }

                        // Cached Collision Check
                        if f.to_lowercase().ends_with(".stl") {
                            let slug = GeometryHelper::generate_slug(f);
                            if std::path::Path::new(&output_dir).join(&slug).exists() {
                                current_collisions.push(f.clone());
                            }
                        }
                    }
                }
            }

            // Update State for UI (Broadcast via watch channels)
            {
                let _ = watcher_state.queue_items_tx.send(current_files.clone());
                let _ = watcher_state.queue_items_with_size_tx.send(current_items_with_size);
                let _ = watcher_state.collisions_tx.send(current_collisions);
                
                let mut q = watcher_state.queue_count.lock().unwrap();
                *q = current_files.len();

                // Timeline updates
                let mut timeline_changed = false;
                for f in &current_files {
                    if !seen_files.contains(f) {
                        last_timeline.push(format!("Incoming: {} (Awaiting Settle)", f));
                        seen_files.insert(f.clone());
                        timeline_changed = true;
                    }
                }
                if seen_files.len() != current_files.len() {
                    seen_files.retain(|f| current_files.contains(f));
                }

                if timeline_changed {
                    let _ = watcher_state.timeline_tx.send(last_timeline.clone());
                }
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
        .route("/api/config/settings", get(get_settings))
        .route("/api/config/settings/update", post(set_settings))
        .route("/api/actions/clear-timeline", post(clear_timeline))
        .route(
            "/api/actions/process/:filename",
            post(process_file_with_hint),
        )
        .route("/api/actions/delete-file/:filename", delete(delete_file))
        .route("/api/actions/delete-all", post(delete_all))
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
    let (load, memory) = {
        let mut sys = state.sys.lock().unwrap();
        sys.refresh_cpu();
        sys.refresh_memory();
        (sys.global_cpu_info().cpu_usage(), sys.used_memory() / 1024 / 1024)
    };

    let settings = state.settings.lock().unwrap().clone();
    let queue = *state.queue_count.lock().unwrap();
    
    let queue_items = state.queue_items.borrow().clone();
    let queue_items_with_size = state.queue_items_with_size.borrow().clone();
    let timeline = state.timeline.borrow().clone();
    let collisions = state.collisions.borrow().clone();

    let target_pulses = (settings.network_settle_seconds / 0.5).ceil() as u32;
    let mut settle_status = HashMap::new();

    for r in state.file_settle_state.iter() {
        let f = r.key();
        let info = r.value();
        let progress = if info.is_ready {
            1.0
        } else {
            (info.pulses_stable as f32 / target_pulses as f32).min(0.99)
        };
        settle_status.insert(f.clone(), progress);
    }
    Json(Status {
        engine_status: "online".to_string(),
        queue_count: queue,
        queue_items,
        queue_items_with_size,
        processed_count: 0,
        system_load: load,
        memory_usage: memory,
        auto_process_enabled: settings.auto_process_enabled,
        timeline_events: timeline,
        settle_status,
        collisions,
    })
}

async fn get_settings(Extension(state): Extension<AppState>) -> Json<SystemSettings> {
    let settings = state.settings.lock().unwrap().clone();
    Json(settings)
}

async fn set_settings(
    Extension(state): Extension<AppState>,
    Json(payload): Json<SettingsUpdate>,
) -> Json<serde_json::Value> {
    let mut settings = state.settings.lock().unwrap();

    if let Some(auto) = payload.auto_process_enabled {
        settings.auto_process_enabled = auto;
    }
    if let Some(penalties) = payload.naming_penalties {
        settings.naming_penalties = penalties;
    }
    if let Some(buffer) = payload.network_settle_seconds {
        settings.network_settle_seconds = buffer;
    }

    let _ = settings.save();
    log::info!("System settings updated via API");
    Json(serde_json::json!({ "status": "success", "settings": *settings }))
}

async fn process_all() -> Json<serde_json::Value> {
    log::info!("Triggering manual process-all from UI");
    Json(serde_json::json!({ "status": "success", "message": "Batch processing initiated" }))
}

#[derive(Deserialize)]
struct ProcessPayload {
    thumbnail_hint: Option<String>,
}

async fn process_file_with_hint(
    AxumPath(filename): AxumPath<String>,
    Extension(state): Extension<AppState>,
    Json(payload): Json<ProcessPayload>,
) -> Json<serde_json::Value> {
    log::info!(
        "Processing request for: {} (Hint: {:?})",
        filename,
        payload.thumbnail_hint
    );

    // Check if ready
    {
        if let Some(info) = state.file_settle_state.get(&filename) {
            if !info.is_ready {
                return Json(
                    serde_json::json!({ "status": "error", "message": "File is still settling" }),
                );
            }
        }
    }

    if filename.to_lowercase().ends_with(".stl") {
        match handle_loose_stl_project(&filename, &state, payload.thumbnail_hint).await {
            Ok(_) => {
                Json(serde_json::json!({ "status": "success", "message": "Project processed" }))
            }
            Err(e) => Json(
                serde_json::json!({ "status": "error", "message": format!("Processing failed: {}", e) }),
            ),
        }
    } else {
        // Fallback for other files (simple delete or handle as zip)
        Json(
            serde_json::json!({ "status": "error", "message": "Only STL files trigger project creation currently" }),
        )
    }
}

#[instrument(skip(state))]
async fn handle_loose_stl_project(
    primary: &str,
    state: &AppState,
    thumbnail_hint: Option<String>,
) -> anyhow::Result<()> {
    let input_dir = std::env::var("INPUT_DIR").unwrap_or_else(|_| "input".to_string());
    let output_dir = std::env::var("OUTPUT_DIR").unwrap_or_else(|_| "output".to_string());
    let settings = state.settings.lock().unwrap().clone();

    // 1. Aggregation (Flat Grab) - Async
    let mut stl_files = Vec::new();
    let mut image_files = Vec::new();

    let mut entries = tokio::fs::read_dir(&input_dir).await?;
    while let Some(entry) = entries.next_entry().await? {
        let name = entry.file_name().to_string_lossy().to_string();
        let lower = name.to_lowercase();
        if lower.ends_with(".stl") {
            stl_files.push(name);
        } else if lower.ends_with(".jpg")
            || lower.ends_with(".png")
            || lower.ends_with(".webp")
            || lower.ends_with(".gif")
        {
            image_files.push(name);
        }
    }

    if stl_files.is_empty() {
        warn!("Process triggered but no STL files found in {}", input_dir);
        return Err(anyhow::anyhow!("No STL files found for processing"));
    }

    // 2. Identify the "Main" Model
    let mut main_stl = primary.to_string();
    let mut max_score = -1.0;

    for f in &stl_files {
        let path = std::path::Path::new(&input_dir).join(f);
        let meta = tokio::fs::metadata(&path).await?;
        let size = meta.len() as f32;
        let mut score = size;

        // Apply penalties
        for penalty in &settings.naming_penalties {
            if f.to_lowercase().contains(&penalty.to_lowercase()) {
                score *= 0.1; // Reduce score by 90% for penalized keywords
            }
        }

        if score > max_score {
            max_score = score;
            main_stl = f.clone();
        }
    }

    let slug = GeometryHelper::generate_slug(&main_stl);
    let project_path = std::path::Path::new(&output_dir).join(&slug);

    // 3. Collision Handling (Overwrite)
    if project_path.exists() {
        warn!("Collision detected for {}. Overwriting.", slug);
        tokio::fs::remove_dir_all(&project_path).await?;
    }
    tokio::fs::create_dir_all(&project_path).await?;

    // 4. Transform Geometry (Awaiting Async)
    let mut stl_paths = Vec::new();
    for f in &stl_files {
        stl_paths.push(std::path::Path::new(&input_dir).join(f));
    }
    let output_mesh = project_path.join(format!("{}.3mf", slug));
    GeometryHelper::consolidate_mesh(&stl_paths, &output_mesh).await?;

    // 5. Intelligent Thumbnail Selection
    let mut winner_image: Option<String> = None;
    let mut best_priority = 0; // 0 = none, 1 = size, 2 = keyword, 3 = name match, 4 = manual hint

    for img in &image_files {
        let mut current_priority = 1;

        if let Some(ref hint) = thumbnail_hint {
            if img == hint {
                current_priority = 4;
            }
        }

        if current_priority < 4 {
            let img_stem = std::path::Path::new(img)
                .file_stem()
                .unwrap_or_default()
                .to_string_lossy();
            let stl_stem = std::path::Path::new(&main_stl)
                .file_stem()
                .unwrap_or_default()
                .to_string_lossy();

            if img_stem == stl_stem {
                current_priority = 3;
            } else if img.to_lowercase().contains("thumbnail") {
                current_priority = 2;
            }
        }

        if current_priority > best_priority {
            best_priority = current_priority;
            winner_image = Some(img.clone());
        } else if current_priority == best_priority && current_priority == 1 {
            // Size fallback
            let p1 = std::path::Path::new(&input_dir).join(img);
            let p2 = winner_image
                .as_ref()
                .map(|w| std::path::Path::new(&input_dir).join(w));
            let s1 = std::fs::metadata(p1)?.len();
            let s2 = if let Some(p) = p2 {
                std::fs::metadata(p)?.len()
            } else {
                0
            };
            if s1 > s2 {
                winner_image = Some(img.clone());
            }
        }
    }

    // 6. Media Transformation
    let mut resources = Vec::new();
    resources.push(Resource {
        name: "Main Model".to_string(),
        path: format!("{}.3mf", slug),
        media_type: "model/3mf".to_string(),
    });

    for img in image_files {
        let src = std::path::Path::new(&input_dir).join(&img);
        let is_winner = winner_image.as_ref().map(|w| w == &img).unwrap_or(false);
        let dest_name = if is_winner {
            format!("{}_thumbnail", slug)
        } else {
            std::path::Path::new(&img)
                .file_stem()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string()
        };

        // Established Image Logic
        let (processed_name, media_type) =
            process_image_to_project(&src, &project_path, &dest_name).await?;
        resources.push(Resource {
            name: dest_name,
            path: processed_name,
            media_type,
        });
    }

    // 7. Manifest Creation
    MetadataHelper::create_datapackage(&project_path, &slug, &slug.replace('-', " "), resources)?;

    // 8. Cleanup
    for f in stl_files {
        let _ = tokio::fs::remove_file(std::path::Path::new(&input_dir).join(f)).await;
    }

    let mut t = state.timeline.borrow().clone();
    t.push(format!("Project '{}' created successfully", slug));
    t.push(format!("Processed: {} -> {}.3mf", primary, slug));
    let _ = state.timeline_tx.send(t);

    Ok(())
}

async fn process_image_to_project(
    src: &std::path::Path,
    dest_dir: &std::path::Path,
    name: &str,
) -> anyhow::Result<(String, String)> {
    let lower_src = src.to_string_lossy().to_lowercase();

    if lower_src.ends_with(".gif") {
        // Animation Check
        let file = File::open(src)?;
        let reader = std::io::BufReader::new(file);
        let decoder = image::codecs::gif::GifDecoder::new(reader)?;
        let frames: Vec<_> = decoder.into_frames().collect_frames()?;

        if frames.len() > 1 {
            let dest_file = format!("{}.gif", name);
            std::fs::copy(src, dest_dir.join(&dest_file))?;
            return Ok((dest_file, "image/gif".to_string()));
        }
    }

    // Fallback to WebP for static images
    let img = image::open(src)?;
    let dest_file = format!("{}.webp", name);
    img.save(dest_dir.join(&dest_file))?;
    Ok((dest_file, "image/webp".to_string()))
}

async fn clear_timeline(Extension(state): Extension<AppState>) -> Json<serde_json::Value> {
    let _ = state.timeline_tx.send(Vec::new());
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
            let mut t = state.timeline.borrow().clone();
            t.push(format!("Deleted: {} (Manually)", filename));
            let _ = state.timeline_tx.send(t);
            // Explicit Cache Invalidation
            state.file_settle_state.remove(&filename);
        }
        Ok(Json(serde_json::json!({ "status": "success" })))
    } else {
        Err(axum::http::StatusCode::NOT_FOUND)
    }
}

async fn delete_all(Extension(state): Extension<AppState>) -> Json<serde_json::Value> {
    let input_dir = std::env::var("INPUT_DIR").unwrap_or_else(|_| "input".to_string());
    let mut deleted_count = 0;

    if let Ok(entries) = std::fs::read_dir(&input_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() && std::fs::remove_file(path).is_ok() {
                deleted_count += 1;
            }
        }
    }

    if deleted_count > 0 {
        info!("Batch delete via API: {} files removed", deleted_count);
        let mut t = state.timeline.borrow().clone();
        t.push(format!("Batch Deleted: {} files from input", deleted_count));
        let _ = state.timeline_tx.send(t);
        // Explicit Batch Cache Invalidation
        state.file_settle_state.clear();
    }

    Json(serde_json::json!({ "status": "success", "count": deleted_count }))
}

async fn health_check() -> axum::http::StatusCode {
    axum::http::StatusCode::OK
}
