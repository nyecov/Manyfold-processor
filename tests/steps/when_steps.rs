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
async fn copy_files(_world: &mut DashboardWorld, src_pattern: String, dest_dir: String) {
    // let dest_path = std::path::Path::new(&dest_dir);
    // Handle glob if present (basic * support)
    // For simplicity, we assume one level of globbing or direct path

    // We'll shell out to 'cp' or 'powershell' to handle globs easily platform-agnostic?
    // Actually, let's use glob crate if available, or just shell out for "copy resources".
    // Given the constraints, copying a directory is easier.

    // Simplification: Manual iteration
    let sources = glob::glob(&src_pattern).expect("Failed to read glob pattern");
    for path in sources.flatten() {
        let file_name = path.file_name().unwrap();
        let destination = if dest_dir.starts_with("/") {
            // If absolute/root relative, map to local (simplification for docker mapping)
            std::path::Path::new(".").join(dest_dir.trim_start_matches('/'))
        } else {
            std::path::Path::new(&dest_dir).to_path_buf()
        };

        let target = destination.join(file_name);
        std::fs::copy(&path, &target)
            .unwrap_or_else(|_| panic!("Failed to copy {:?} to {:?}", path, target));
        println!("Copied {:?} to {:?}", path, target);
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
