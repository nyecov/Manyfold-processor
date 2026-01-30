use super::world::DashboardWorld;
use cucumber::given;

#[given("_API the Manyfold Processor service is running")]
async fn service_is_running_api(_world: &mut DashboardWorld) {
    // API layer: check /health endpoint
    // TODO: Add HTTP client call
}
