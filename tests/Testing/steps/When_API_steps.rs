use cucumber::when;
use crate::steps::world::DashboardWorld;

#[when("When_API I request the status from the API")]
async fn request_dashboard_api(world: &mut DashboardWorld) {
    // Logic to perform HTTP request
    world.response_code = 200;
}
