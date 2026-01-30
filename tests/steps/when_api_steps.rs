use super::world::DashboardWorld;
use cucumber::when;

#[when("_API I request the status from the API")]
async fn request_status_api(_world: &mut DashboardWorld) {
    // API layer: call /status endpoint
    // TODO: Add HTTP client call
}
