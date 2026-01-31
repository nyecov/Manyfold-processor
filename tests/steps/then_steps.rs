use super::world::DashboardWorld;
use cucumber::then;

#[then("I should receive a successful visual response on port 8080")]
async fn verify_success(_world: &mut DashboardWorld) {
    // UI layer: verify DOM elements visible (e.g., "Welcome to Manyfold")
    // TODO: Add browser automation
}

#[then("the Timeline should be empty")]
async fn verify_timeline_empty(_world: &mut DashboardWorld) {
    let client = reqwest::Client::new();
    let resp = client
        .get("http://localhost:8080/api/status")
        .send()
        .await
        .expect("Failed to get status");
    let json: serde_json::Value = resp.json().await.expect("Failed to parse JSON");

    if let Some(events) = json["timeline_events"].as_array() {
        if !events.is_empty() {
            panic!("❌ Timeline is NOT empty. Found: {:?}", events);
        } else {
            println!("✅ Timeline is empty.");
        }
    } else {
        panic!("❌ Response missing timeline_events");
    }
}
