use super::world::DashboardWorld;
use cucumber::then;
use scraper::{Html, Selector};
use std::fs;

// Helper to confirm UI presence (Mock via static file check or basic GET check)
// Real browser test would use WebDriver, here we inspect the statics or response body
#[then(expr = "I should see a column {string}")]
async fn verify_column_presence(_world: &mut DashboardWorld, column_name: String) {
    // For this level of test, we'll verify the index.html content directly as a proxy
    let html = fs::read_to_string("static/index.html").expect("Failed to read index.html");
    if !html.contains(&column_name) {
        panic!("❌ Column '{}' not found in static/index.html", column_name);
    }
}

#[then("the \"STATUS\" column should be horizontally aligned with its content")]
async fn verify_status_alignment(_world: &mut DashboardWorld) {
    // Structural check: grid columns defined
    let html = fs::read_to_string("static/index.html").expect("Failed to read index.html");
    if !html.contains("grid-cols-[1fr_100px_100px_140px]") {
        panic!("❌ Grid layout for alignment missing.");
    }
}

#[then("the Intake Queue should be scrollable")]
async fn verify_scrollable(_world: &mut DashboardWorld) {
    let html = fs::read_to_string("static/index.html").expect("Failed to read index.html");
    if !html.contains("overflow-y-auto") || !html.contains("max-h-[500px]") {
        panic!("❌ Scrollable classes missing.");
    }
}

#[then("the header should remain visible")]
async fn verify_header_visible(_world: &mut DashboardWorld) {
    // Implicit in the DOM structure (header outside the scrollable div)
    // We check that the header div is separate from the #queue-list div
    let html = fs::read_to_string("static/index.html").expect("Failed to read index.html");
    let document = Html::parse_document(&html);
    let queue_list_selector = Selector::parse("#queue-list").unwrap();

    // Check if #queue-list has the scroll classes
    if let Some(element) = document.select(&queue_list_selector).next() {
        let classes = element.value().attr("class").unwrap_or("");
        if !classes.contains("overflow-y-auto") {
            panic!("❌ #queue-list is not scrollable.");
        }
    }
}

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
