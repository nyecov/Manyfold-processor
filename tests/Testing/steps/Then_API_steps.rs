use cucumber::then;
use crate::steps::world::DashboardWorld;

#[then("Then_API I should receive a status code of 200")]
async fn verify_success_api(world: &mut DashboardWorld) {
    assert_eq!(world.response_code, 200);
}
