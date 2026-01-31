use super::world::DashboardWorld;
use cucumber::when;

#[when("_API I request the status from the API")]
async fn request_status_api(_world: &mut DashboardWorld) {
    // API layer: call /status endpoint
    // TODO: Add HTTP client call
}

#[when(expr = "I request processing of {string} via API")]
async fn request_processing(_world: &mut DashboardWorld, filename: String) {
    let client = reqwest::Client::new();
    let url = format!("http://localhost:8080/api/actions/process/{}", filename);
    let resp = client
        .post(&url)
        .send()
        .await
        .expect("Failed to send process request");

    if !resp.status().is_success() {
        panic!("❌ Failed to process file: {}", resp.status());
    }
}
