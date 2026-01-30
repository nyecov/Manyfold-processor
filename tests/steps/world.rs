use cucumber::World;

#[derive(Debug, Default, World)]
pub struct DashboardWorld {
    pub response_code: u16,
}
