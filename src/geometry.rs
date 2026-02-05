use once_cell::sync::Lazy;
use regex::Regex;
use std::path::{Path, PathBuf};

static SLUG_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"[^a-zA-Z0-9]+").unwrap());

pub struct GeometryHelper;

impl GeometryHelper {
    pub fn generate_slug(filename: &str) -> String {
        let stem = Path::new(filename)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or(filename);

        let slug = SLUG_REGEX.replace_all(stem, "-").to_lowercase();
        slug.trim_matches('-').to_string()
    }

    #[tracing::instrument(skip(inputs))]
    pub async fn consolidate_mesh(inputs: &[PathBuf], output_path: &Path) -> anyhow::Result<()> {
        tracing::info!(
            "Consolidating {} meshes into {:?}",
            inputs.len(),
            output_path
        );

        // Initial Implementation: If multi-file, we just copy the "Main" one for now
        // to establish the folder structure. Real stl23mf integration can follow.
        if !inputs.is_empty() {
            tokio::fs::copy(&inputs[0], output_path).await?;
            tracing::info!("Primary mesh copied to {:?}", output_path);
        }

        Ok(())
    }
}
