use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct Resource {
    pub name: String,
    pub path: String,
    #[serde(rename = "mediaType")]
    pub media_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct Datapackage {
    pub name: String,
    pub title: String,
    pub resources: Vec<Resource>,
}

pub struct MetadataHelper;

impl MetadataHelper {
    pub fn create_datapackage(
        project_dir: &Path,
        slug: &str,
        title: &str,
        resources: Vec<Resource>,
    ) -> anyhow::Result<()> {
        let dp = Datapackage {
            name: slug.to_string(),
            title: title.to_string(),
            resources,
        };

        let path = project_dir.join("datapackage.json");
        let content = serde_json::to_string_pretty(&dp)?;
        fs::write(path, content)?;
        
        log::info!("Generated datapackage.json for {}", slug);
        Ok(())
    }
}
