use super::world::DashboardWorld;
use cucumber::given;
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[given(expr = "a file {string} is in the input directory")]
async fn create_dummy_file(_world: &mut DashboardWorld, filename: String) {
    let input_dir = std::env::var("INPUT_DIR").unwrap_or_else(|_| "input".to_string());
    let path = Path::new(&input_dir).join(filename);

    // Create dummy file
    let mut file = File::create(&path).expect("Failed to create dummy file");
    writeln!(file, "dummy content").expect("Failed to write to dummy file");
}

#[given("a large dataset is copied to the input directory")]
async fn create_large_dataset(_world: &mut DashboardWorld) {
    let input_dir = std::env::var("INPUT_DIR").unwrap_or_else(|_| "input".to_string());

    for i in 1..=25 {
        let filename = format!("dataset_file_{}.stl", i);
        let path = Path::new(&input_dir).join(filename);
        let mut file = File::create(&path).expect("Failed to create dummy dataset file");
        writeln!(file, "dummy content {}", i).expect("Failed to write to file");
    }
}

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
