use axum::{
    extract::Path as AxumPath,
    routing::{delete, get, post, put},
    Extension, Json, Router,
};
use image::AnimationDecoder;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::{
    collections::HashSet,
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use sysinfo::{CpuExt, CpuRefreshKind, RefreshKind, System, SystemExt};
use tower_http::services::ServeDir;
use std::collections::HashMap;
use crate::config::SystemSettings;
use crate::geometry::GeometryHelper;
use crate::metadata::{MetadataHelper, Resource};

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
    queue_items: Arc<Mutex<Vec<String>>>,
    queue_items_with_size: Arc<Mutex<Vec<(String, u64)>>>,
    timeline: Arc<Mutex<Vec<String>>>,
    file_settle_state: Arc<Mutex<HashMap<String, FileSettleInfo>>>,
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
        settings: Arc::new(Mutex::new(SystemSettings::load())),
        queue_count: Arc::new(Mutex::new(0)),
        queue_items: Arc::new(Mutex::new(Vec::new())),
        queue_items_with_size: Arc::new(Mutex::new(Vec::new())),
        timeline: Arc::new(Mutex::new(Vec::new())),
        file_settle_state: Arc::new(Mutex::new(HashMap::new())),
    };

    // Spawn File Watcher (Refined with Settle Logic)
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

            // Sync Settle State
            {
                let mut settle_map = watcher_state.file_settle_state.lock().unwrap();
                let settings = watcher_state.settings.lock().unwrap().clone();
                let target_pulses = (settings.network_settle_seconds / 0.5).ceil() as u32;

                // Cleanup deleted files
                settle_map.retain(|f, _| current_files.contains(f));

                for f in &current_files {
                    let path = std::path::Path::new(&input_dir).join(f);
                    let metadata = std::fs::metadata(&path);
                    
                    if let Ok(m) = metadata {
                        let current_size = m.len();
                        let info = settle_map.entry(f.clone()).or_insert(FileSettleInfo {
                            last_size: current_size,
                            pulses_stable: 0,
                            is_ready: false,
                        });

                        if !info.is_ready {
                            if info.last_size == current_size {
                                info.pulses_stable += 1;
                                
                                // Hard Safety: Exclusive Lock Test
                                let can_lock = std::fs::OpenOptions::new()
                                    .write(true)
                                    .open(&path)
                                    .is_ok();

                                if info.pulses_stable >= target_pulses && can_lock {
                                    info.is_ready = true;
                                    log::info!("File settled and ready: {}", f);
                                }
                            } else {
                                info.last_size = current_size;
                                info.pulses_stable = 0;
                            }
                        }
                    }
                }
            }

            // Update State for UI
            {
                let mut q = watcher_state.queue_count.lock().unwrap();
                *q = current_files.len();

                let mut items = watcher_state.queue_items.lock().unwrap();
                *items = current_files.clone();

                let mut items_with_size = watcher_state.queue_items_with_size.lock().unwrap();
                let mut new_items_with_size = Vec::new();

                for f in &current_files {
                    let path = std::path::Path::new(&input_dir).join(f);
                    let size = std::fs::metadata(path).map(|m| m.len()).unwrap_or(0);
                    new_items_with_size.push((f.clone(), size));
                }
                *items_with_size = new_items_with_size;

                let mut t = watcher_state.timeline.lock().unwrap();
                for f in &current_files {
                    if !seen_files.contains(f) {
                        t.push(format!("Incoming: {} (Awaiting Settle)", f));
                        seen_files.insert(f.clone());
                    }
                }
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
        .route("/api/config/settings", get(get_settings))
        .route("/api/config/settings/update", post(set_settings))
        .route("/api/actions/clear-timeline", post(clear_timeline))
        .route("/api/actions/process/:filename", post(process_file_with_hint))
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
    let mut sys = state.sys.lock().unwrap();
    sys.refresh_cpu();
    sys.refresh_memory();

    let load = sys.global_cpu_info().cpu_usage();
    let memory = sys.used_memory() / 1024 / 1024; // MB
    let settings = state.settings.lock().unwrap().clone();
    let queue = *state.queue_count.lock().unwrap();
    let queue_items = state.queue_items.lock().unwrap().clone();
    let queue_items_with_size = state.queue_items_with_size.lock().unwrap().clone();
    let timeline = state.timeline.lock().unwrap().clone();
    
    let settle_state = state.file_settle_state.lock().unwrap();
    let mut settle_status = HashMap::new();
    let target_pulses = (settings.network_settle_seconds / 0.5).ceil() as u32;

    let output_dir = std::env::var("OUTPUT_DIR").unwrap_or_else(|_| "output".to_string());
    let mut collisions = Vec::new();

    for (f, info) in settle_state.iter() {
        let progress = if info.is_ready {
            1.0
        } else {
            (info.pulses_stable as f32 / target_pulses as f32).min(0.99)
        };
        settle_status.insert(f.clone(), progress);

        // Collision Check
        if f.to_lowercase().ends_with(".stl") {
             let slug = GeometryHelper::generate_slug(f);
             if std::path::Path::new(&output_dir).join(&slug).exists() {
                 collisions.push(f.clone());
             }
        }
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
    log::info!("Processing request for: {} (Hint: {:?})", filename, payload.thumbnail_hint);
    
    // Check if ready
    {
        let settle_map = state.file_settle_state.lock().unwrap();
        if let Some(info) = settle_map.get(&filename) {
            if !info.is_ready {
                return Json(serde_json::json!({ "status": "error", "message": "File is still settling" }));
            }
        }
    }

    if filename.to_lowercase().ends_with(".stl") {
        match handle_loose_stl_project(&filename, &state, payload.thumbnail_hint).await {
            Ok(_) => Json(serde_json::json!({ "status": "success", "message": "Project processed" })),
            Err(e) => Json(serde_json::json!({ "status": "error", "message": format!("Processing failed: {}", e) })),
        }
    } else {
        // Fallback for other files (simple delete or handle as zip)
        Json(serde_json::json!({ "status": "error", "message": "Only STL files trigger project creation currently" }))
    }
}

async fn handle_loose_stl_project(
    primary: &str,
    state: &AppState,
    thumbnail_hint: Option<String>,
) -> anyhow::Result<()> {
    let input_dir = std::env::var("INPUT_DIR").unwrap_or_else(|_| "input".to_string());
    let output_dir = std::env::var("OUTPUT_DIR").unwrap_or_else(|_| "output".to_string());
    let settings = state.settings.lock().unwrap().clone();

    // 1. Aggregation (Flat Grab)
    let mut stl_files = Vec::new();
    let mut image_files = Vec::new();
    
    for entry in std::fs::read_dir(&input_dir)? {
        let entry = entry?;
        let name = entry.file_name().to_string_lossy().to_string();
        let lower = name.to_lowercase();
        if lower.ends_with(".stl") {
            stl_files.push(name);
        } else if lower.ends_with(".jpg") || lower.ends_with(".png") || lower.ends_with(".webp") || lower.ends_with(".gif") {
            image_files.push(name);
        }
    }

    if stl_files.is_empty() {
        return Err(anyhow::anyhow!("No STL files found for processing"));
    }

    // 2. Identify the "Main" Model
    let mut main_stl = primary.to_string();
    let mut max_score = -1.0;

    for f in &stl_files {
        let path = std::path::Path::new(&input_dir).join(f);
        let size = std::fs::metadata(path)?.len() as f32;
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
        log::warn!("Collision detected for {}. Overwriting.", slug);
        std::fs::remove_dir_all(&project_path)?;
    }
    std::fs::create_dir_all(&project_path)?;

    // 4. Transform Geometry
    let mut stl_paths = Vec::new();
    for f in &stl_files {
        stl_paths.push(std::path::Path::new(&input_dir).join(f));
    }
    let output_mesh = project_path.join(format!("{}.3mf", slug));
    GeometryHelper::consolidate_mesh(&stl_paths, &output_mesh)?;

    // 5. Intelligent Thumbnail Selection
    let mut winner_image: Option<String> = None;
    let mut best_priority = 0; // 0 = none, 1 = size, 2 = keyword, 3 = name match, 4 = manual hint

    for img in &image_files {
        let mut current_priority = 1;
        
        if let Some(ref hint) = thumbnail_hint {
            if img == hint { current_priority = 4; }
        }
        
        if current_priority < 4 {
            let img_stem = std::path::Path::new(img).file_stem().unwrap_or_default().to_string_lossy();
            let stl_stem = std::path::Path::new(&main_stl).file_stem().unwrap_or_default().to_string_lossy();
            
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
            let p2 = winner_image.as_ref().map(|w| std::path::Path::new(&input_dir).join(w));
            let s1 = std::fs::metadata(p1)?.len();
            let s2 = if let Some(p) = p2 { std::fs::metadata(p)?.len() } else { 0 };
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
             std::path::Path::new(&img).file_stem().unwrap_or_default().to_string_lossy().to_string()
        };

        // Established Image Logic
        let (processed_name, media_type) = process_image_to_project(&src, &project_path, &dest_name).await?;
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
        let _ = std::fs::remove_file(std::path::Path::new(&input_dir).join(f));
    }
    // Cleanup images too as they've been copied/transferred
    // (Note: imaging logic can be refined to move or copy)
    
    let mut t = state.timeline.lock().unwrap();
    t.push(format!("Project '{}' created successfully", slug));
    t.push(format!("Processed: {} -> {}.3mf", primary, slug));

    Ok(())
}

async fn process_image_to_project(src: &std::path::Path, dest_dir: &std::path::Path, name: &str) -> anyhow::Result<(String, String)> {
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
        log::info!("Batch delete via API: {} files removed", deleted_count);
        let mut t = state.timeline.lock().unwrap();
        t.push(format!("Batch Deleted: {} files from input", deleted_count));
    }

    Json(serde_json::json!({ "status": "success", "count": deleted_count }))
}

async fn health_check() -> axum::http::StatusCode {
    axum::http::StatusCode::OK
}
