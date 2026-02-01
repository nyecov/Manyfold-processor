use super::world::DashboardWorld;
use cucumber::when;

#[when("_API I request the status from the API")]
async fn request_status_api(_world: &mut DashboardWorld) {
    // API layer: call /status endpoint
    // TODO: Add HTTP client call
}

#[when(expr = "I immediately request processing of {string} via API")]
#[when(expr = "I request processing of {string} via API")]
async fn request_processing(world: &mut DashboardWorld, filename: String) {
    let client = reqwest::Client::new();
    let url = format!("http://localhost:8080/api/actions/process/{}", filename);
    let payload = serde_json::json!({ "thumbnail_hint": null });

    let start = std::time::Instant::now();
    let timeout = std::time::Duration::from_secs(10); // Wait up to 10s for settle

    loop {
        let resp = client
            .post(&url)
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&payload).unwrap())
            .send()
            .await
            .expect("Failed to send process request");

        world.response_code = resp.status().as_u16();
        world.last_response_body = resp.text().await.unwrap_or_default();

        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&world.last_response_body) {
            world.last_error = json["message"].as_str().unwrap_or("").to_string();
            
            // If it's still settling, wait and retry
            if world.last_error == "File is still settling" && start.elapsed() < timeout {
                tokio::time::sleep(std::time::Duration::from_millis(500)).await;
                continue;
            }
        }
        break;
    }
}

#[when(expr = "I request processing of {string} with {string} as the thumbnail")]
async fn request_processing_with_hint(world: &mut DashboardWorld, filename: String, hint: String) {
    let client = reqwest::Client::new();
    let url = format!("http://localhost:8080/api/actions/process/{}", filename);
    let resp = client
        .post(&url)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&serde_json::json!({ "thumbnail_hint": Some(hint) })).unwrap())
        .send()
        .await
        .expect("Failed to send process request");

    world.response_code = resp.status().as_u16();
    world.last_response_body = resp.text().await.unwrap_or_default();
}
