use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Clone)]
pub struct SystemSettings {
    pub naming_penalties: Vec<String>,
    pub auto_process_enabled: bool,
    pub network_settle_seconds: f32,
}

impl Default for SystemSettings {
    fn default() -> Self {
        Self {
            naming_penalties: vec![
                "base".to_string(),
                "stand".to_string(),
                "support".to_string(),
                "vignette".to_string(),
            ],
            auto_process_enabled: false,
            network_settle_seconds: 2.0,
        }
    }
}

impl SystemSettings {
    pub fn load() -> Self {
        let path = Path::new("config.json");
        if path.exists() {
            if let Ok(content) = fs::read_to_string(path) {
                if let Ok(settings) = serde_json::from_str(&content) {
                    return settings;
                }
            }
        }
        let default = Self::default();
        let _ = default.save();
        default
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let content = serde_json::to_string_pretty(self)?;
        fs::write("config.json", content)?;
        Ok(())
    }
}
