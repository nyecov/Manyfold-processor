use cucumber::given;
use crate::steps::world::DashboardWorld;

// [twin: Given_API the Manyfold Processor service is running]
#[given("the Manyfold Processor service is running")]
async fn service_is_running_ui(_world: &mut DashboardWorld) {
    // Logic to verify service is up (mocked for now)
}
