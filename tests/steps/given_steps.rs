use super::world::DashboardWorld;
use cucumber::given;

#[given("the Manyfold Processor service is running")]
async fn service_is_running(_world: &mut DashboardWorld) {
    // UI layer: verify service is up visually
    // TODO: Add browser automation to check page loads
}
