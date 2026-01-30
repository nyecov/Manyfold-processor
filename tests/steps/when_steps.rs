use super::world::DashboardWorld;
use cucumber::when;

#[when("I request the dashboard home page")]
async fn request_dashboard(_world: &mut DashboardWorld) {
    // UI layer: navigate to dashboard via browser
    // TODO: Add browser automation
}
