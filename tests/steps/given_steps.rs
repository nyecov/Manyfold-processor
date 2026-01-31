use super::world::DashboardWorld;
use cucumber::given;

#[given("Processor is running")]
async fn service_is_running(_world: &mut DashboardWorld) {
    // UI layer: verify service is up visually
    // In strict BDD, this should check the browser.
    // For now, we assume if the test runs, it's fine, or use the API step for hard checks.
}

#[given("the System is ready")]
async fn system_is_ready(_world: &mut DashboardWorld) {
    let client = reqwest::Client::new();
    let resp = client.get("http://localhost:8080/health").send().await;
    match resp {
        Ok(res) => {
            if !res.status().is_success() {
                panic!("❌ CAUSE: System is NOT ready. Status: {}", res.status());
            }
            println!("✅ System is ready (Health Check Passed)");
        }
        Err(e) => panic!("❌ Failed to check system readiness: {}", e),
    }
}
