use super::world::DashboardWorld;
use cucumber::{given, when};

#[given("_API Processor is running")]
async fn service_is_running_api(_world: &mut DashboardWorld) {
    let client = reqwest::Client::new();
    let resp = client.get("http://localhost:8080/api/status").send().await;

    match resp {
        Ok(res) => {
            if !res.status().is_success() {
                 panic!("❌ CAUSE: API returned status {}\n   ENDPOINT: /api/status", res.status());
            }
            let body = res.text().await.unwrap_or_default();
            if !body.contains("\"engine_status\":\"online\"") && !body.contains("\"engine_status\": \"online\"") {
                 panic!("❌ CAUSE: Processor is running but NOT READY.\n   RESPONSE: {}", body);
            }
            println!("✅ Processor is online and ready");
        },
        Err(e) => panic!("❌ CAUSE: Cannot connect to Manyfold Processor at localhost:8080.\n   HINT: Is Docker running? (Run 'docker compose up -d')\n   ERROR: {}", e),
    }
}

#[given("Auto-Processing is disabled")]
async fn disable_auto_processing(_world: &mut DashboardWorld) {
    let client = reqwest::Client::new();
    let resp = client
        .post("http://localhost:8080/api/config/settings/update")
        .json(&serde_json::json!({ "auto_process_enabled": false }))
        .send()
        .await;

    match resp {
        Ok(res) => {
            if !res.status().is_success() {
                panic!("Failed to disable auto-process: {}", res.status());
            }
        }
        Err(e) => panic!("Failed to call API: {}", e),
    }
}

#[given(expr = "{string} is in the system naming penalties")]
#[when(expr = "{string} is in the system naming penalties")]
async fn set_naming_penalty(_world: &mut DashboardWorld, penalty: String) {
    let client = reqwest::Client::new();
    let resp = client
        .post("http://localhost:8080/api/config/settings/update")
        .json(&serde_json::json!({ "naming_penalties": vec![penalty] }))
        .send()
        .await;

    if !resp.unwrap().status().is_success() {
        panic!("Failed to set naming penalty");
    }
}
