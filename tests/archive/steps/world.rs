use cucumber::World;

#[derive(Debug, Default, World)]
pub struct DashboardWorld {
    pub response_code: u16,
    pub last_response_body: String,
    pub last_error: String,
}
