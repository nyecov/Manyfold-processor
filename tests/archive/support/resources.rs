use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct Manifest {
    pub resources: Vec<TestResource>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TestResource {
    pub id: String,
    pub path: String,
    pub tags: Vec<String>,
}

pub fn load_manifest() -> Manifest {
    let content = fs::read_to_string("test_resources/manifest.yaml")
        .expect("Failed to read test_resources/manifest.yaml");
    serde_yaml::from_str(&content).expect("Invalid manifest YAML")
}

pub fn get_path(id: &str) -> PathBuf {
    let manifest = load_manifest();
    let res = manifest
        .resources
        .into_iter()
        .find(|r| r.id == id)
        .unwrap_or_else(|| panic!("Resource '{}' not found in manifest", id));

    PathBuf::from("test_resources").join(res.path)
}
