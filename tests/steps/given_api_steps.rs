use super::world::DashboardWorld;
use cucumber::given;

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
        .put("http://localhost:8080/api/config/auto-process")
        .json(&serde_json::json!({ "enabled": false }))
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
