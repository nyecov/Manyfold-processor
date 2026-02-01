use super::world::DashboardWorld;
use cucumber::when;

#[when(expr = "I request deletion of {string} via API")]
async fn click_delete_button(_world: &mut DashboardWorld, filename: String) {
    // API equivalent: DELETE /api/actions/delete-file/:filename
    let client = reqwest::Client::new();
    let url = format!("http://localhost:8080/api/actions/delete-file/{}", filename);
    let resp = client
        .delete(url)
        .send()
        .await
        .expect("Failed to send delete request");

    if !resp.status().is_success() {
        panic!(
            "❌ Failed to delete file via API. Status: {}",
            resp.status()
        );
    }
}

#[when("I request the dashboard home page")]
async fn request_dashboard(_world: &mut DashboardWorld) {
    // UI layer: navigate to dashboard via browser
    // TODO: Add browser automation
}

#[when(expr = "I copy {string} to {string}")]
async fn copy_files(_world: &mut DashboardWorld, src: String, dest_dir: String) {
    // 1. Try to resolve as Registry ID first
    // We assume Registry IDs don't contain path separators
    let src_path = if !src.contains('/') && !src.contains('\\') && !src.contains('*') {
        // It might be an ID. Try to resolve it.
        // We catch the panic/error if not found and fallback?
        // Or we just checking if it exists in manifest?
        // simple panic if it looks like an ID but fails is probably safer for strictness.
        // But to allow "filename.stl" (dummy) we might need to be careful.
        // Let's assume if it has no extension, it's an ID? Or check manifest.

        // Since we can't easily check manifest without loading it every time (which is fine for tests),
        // let's try to load it.
        let manifest = crate::support::resources::load_manifest();
        if let Some(res) = manifest.resources.iter().find(|r| r.id == src) {
            std::path::PathBuf::from("test_resources").join(&res.path)
        } else {
            // Not an ID, treat as path
            std::path::PathBuf::from(&src)
        }
    } else {
        std::path::PathBuf::from(&src)
    };

    // 2. Handle Copy
    // If it's a specific file (from registry or explicit path)
    if src_path.exists() && src_path.is_file() {
        let file_name = src_path.file_name().unwrap();
        let destination = if dest_dir.starts_with('/') {
            let base = std::path::Path::new(".").join(dest_dir.trim_start_matches('/'));
            if base.is_dir() || !dest_dir.contains('.') {
                base.join(file_name)
            } else {
                base
            }
        } else {
            let p = std::path::Path::new(&dest_dir);
            if p.extension().is_some() {
                p.to_path_buf()
            } else {
                p.join(file_name)
            }
        };

        // Ensure parent dir exists
        if let Some(parent) = destination.parent() {
            std::fs::create_dir_all(parent).expect("Failed to create destination parent directory");
        }

        std::fs::copy(&src_path, &destination).unwrap_or_else(|e| {
            panic!("Failed to copy {:?} to {:?}: {}", src_path, destination, e)
        });
        println!(
            "✅ Copied Registry File {:?} to {:?}",
            src_path, destination
        );
    } else {
        // Fallback to Glob for raw paths
        let src_pattern = src_path.to_string_lossy();
        let sources = glob::glob(&src_pattern).expect("Failed to read glob pattern");
        let mut count = 0;
        for path in sources.flatten() {
            let file_name = path.file_name().unwrap();
            let destination = if dest_dir.starts_with("/") {
                std::path::Path::new(".").join(dest_dir.trim_start_matches('/'))
            } else {
                std::path::Path::new(&dest_dir).to_path_buf()
            }
            .join(file_name);

            if let Some(parent) = destination.parent() {
                std::fs::create_dir_all(parent)
                    .expect("Failed to create destination parent directory");
            }

            std::fs::copy(&path, &destination)
                .unwrap_or_else(|_| panic!("Failed to copy {:?} to {:?}", path, destination));
            println!("Copied {:?} to {:?}", path, destination);
            count += 1;
        }
        if count == 0 {
            println!("⚠️ Warning: No files found for pattern/id '{}'", src);
        }
    }
}

#[when("I click the \"Clear Timeline\" button")]
async fn click_clear_timeline(_world: &mut DashboardWorld) {
    let client = reqwest::Client::new();

    // UI Anti-Masquerading: Verify button exists in UI before hitting API
    let ui_resp = client.get("http://localhost:8080/").send().await;
    match ui_resp {
        Ok(res) => {
            let body = res.text().await.unwrap_or_default();
            if !body.contains("id=\"clear-timeline-btn\"") {
                panic!("❌ CAUSE: UI ANTI-MASQUERADING DETECTED. The 'Clear Timeline' button is MISSING from the UI Dashboard. Step failed to avoid false positive via API bypass.");
            }
        }
        Err(e) => panic!("❌ Failed to reach UI for anti-masquerading check: {}", e),
    }

    // Proceed with action
    let resp = client
        .post("http://localhost:8080/api/actions/clear-timeline")
        .send()
        .await;

    match resp {
        Ok(res) => {
            if !res.status().is_success() {
                panic!("Failed to clear timeline: {}", res.status());
            }
        }
        Err(e) => panic!("Network error clearing timeline: {}", e),
    }
}

#[when("I click the \"Delete All\" button in the queue header")]
async fn click_delete_all(_world: &mut DashboardWorld) {
    let client = reqwest::Client::new();

    // UI Anti-Masquerading: Verify the button exists in UI via its JS function call in HTML
    let ui_resp = client.get("http://localhost:8080/").send().await;
    match ui_resp {
        Ok(res) => {
            let body = res.text().await.unwrap_or_default();
            if !body.contains("onclick=\"deleteAllFiles()\"") {
                panic!("❌ CAUSE: UI ANTI-MASQUERADING DETECTED. The 'Delete All' button is MISSING from the UI Dashboard. Step failed to avoid false positive via API bypass.");
            }
        }
        Err(e) => panic!("❌ Failed to reach UI for anti-masquerading check: {}", e),
    }

    // Proceed with action
    let resp = client
        .post("http://localhost:8080/api/actions/delete-all")
        .send()
        .await;

    match resp {
        Ok(res) => {
            if !res.status().is_success() {
                panic!("Failed to delete all files: {}", res.status());
            }
        }
        Err(e) => panic!("Network error deleting all files: {}", e),
    }
}

#[when(expr = "I wait {float} seconds")]
async fn wait_seconds(_world: &mut DashboardWorld, seconds: f64) {
    tokio::time::sleep(std::time::Duration::from_millis((seconds * 1000.0) as u64)).await;
}
