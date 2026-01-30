use super::world::DashboardWorld;
use cucumber::then;

#[then("_API I should receive a status code of 200")]
async fn verify_status_code_api(world: &mut DashboardWorld) {
    // API layer: assert response code
    assert_eq!(world.response_code, 200);
}
