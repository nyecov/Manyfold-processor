use regex::Regex;
use std::path::{Path, PathBuf};

pub struct GeometryHelper;

impl GeometryHelper {
    pub fn generate_slug(filename: &str) -> String {
        let stem = Path::new(filename)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or(filename);

        let re = Regex::new(r"[^a-zA-Z0-9]+").unwrap();
        let slug = re.replace_all(stem, "-").to_lowercase();
        slug.trim_matches('-').to_string()
    }

    pub fn consolidate_mesh(inputs: &[PathBuf], output_path: &Path) -> anyhow::Result<()> {
        log::info!(
            "Consolidating {} meshes into {:?}",
            inputs.len(),
            output_path
        );

        // Initial Implementation: If multi-file, we just copy the "Main" one for now
        // to establish the folder structure. Real stl23mf integration can follow.
        if !inputs.is_empty() {
            std::fs::copy(&inputs[0], output_path)?;
            log::info!("Primary mesh copied to {:?}", output_path);
        }

        Ok(())
    }
}
