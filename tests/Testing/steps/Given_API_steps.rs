use cucumber::given;
use crate::steps::world::DashboardWorld;

#[given("Given_API the Manyfold Processor service is running")]
async fn service_is_running_api(_world: &mut DashboardWorld) {
    // Logic to verify service is up (direct API call)
}
