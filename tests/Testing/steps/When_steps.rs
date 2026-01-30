use cucumber::when;
use crate::steps::world::DashboardWorld;

// [twin: When_API I request the status from the API]
#[when("I request the dashboard home page")]
async fn request_dashboard_ui(_world: &mut DashboardWorld) {
    // Logic to navigate to URL via WebDriver/Playwright
}
