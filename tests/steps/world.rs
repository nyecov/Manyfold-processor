use cucumber::World;
use headless_chrome::{Browser, LaunchOptions, Tab};
use std::sync::Arc;
use std::fmt;

// Wrapper to hide headless_chrome types from derive macros or provide missing traits
pub struct BrowserWrapper {
    pub browser: Arc<Browser>,
    pub tab: Arc<Tab>,
}

impl fmt::Debug for BrowserWrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BrowserWrapper(Active)")
    }
}

#[derive(Debug, Default, World)]
pub struct DashboardWorld {
    pub response_code: u16,
    pub last_response_body: String,
    pub last_error: String,
    // Wrapped Browser
    pub browser_wrapper: Option<BrowserWrapper>,
    // Satellite Context
    pub last_satellite_response: Option<serde_json::Value>,
    // Refinement Orbit (v1.1)
    pub exclusion_registry: Vec<String>,
    pub consensus_stratum: u8,
}

impl DashboardWorld {
    pub fn ensure_browser(&mut self) -> Arc<Tab> {
        if self.browser_wrapper.is_none() {
             let options = LaunchOptions {
                headless: false, 
                ..Default::default()
            };
            
            let browser = Browser::new(options).expect("Failed to launch Headless Chrome");
            let tab = browser.wait_for_initial_tab().expect("Failed to get initial tab");
            
            self.browser_wrapper = Some(BrowserWrapper {
                browser: Arc::new(browser),
                tab: tab,
            });
        }
        self.browser_wrapper.as_ref().unwrap().tab.clone()
    }
}
