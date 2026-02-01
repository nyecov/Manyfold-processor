use clap::Parser;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long, default_value = "test_resources")]
    base_dir: String,

    #[arg(long, default_value = "test_resources/manifest.yaml")]
    manifest: String,

    #[arg(long, action)]
    fix: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct Manifest {
    schema_version: String,
    resources: Vec<TestResource>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct TestResource {
    id: String,
    path: String,
    #[serde(rename = "type")]
    resource_type: String,
    tags: Vec<String>,
    description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    checksum: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    size_bytes: Option<u64>,
}

fn calculate_sha256(path: &Path) -> Result<String, io::Error> {
    let mut file = fs::File::open(path)?;
    let mut hasher = Sha256::new();
    io::copy(&mut file, &mut hasher)?;
    let hash = hasher.finalize();
    Ok(hex::encode(hash))
}

fn main() {
    let args = Args::parse();
    let base_path = PathBuf::from(&args.base_dir);
    let manifest_path = PathBuf::from(&args.manifest);

    // 1. Load Manifest
    if !manifest_path.exists() {
        eprintln!("❌ Manifest not found at {:?}", manifest_path);
        std::process::exit(1);
    }

    let content = fs::read_to_string(&manifest_path).expect("Failed to read manifest");
    let mut manifest: Manifest = serde_yaml::from_str(&content).expect("Failed to parse YAML");
    
    let mut registered_paths: HashSet<PathBuf> = HashSet::new();
    let mut ids: HashSet<String> = HashSet::new();
    let mut errors = 0;

    // 2. Validate Entries
    for resource in &mut manifest.resources {
        // ID Uniqueness
        if !ids.insert(resource.id.clone()) {
            eprintln!("❌ Duplicate ID found: {}", resource.id);
            errors += 1;
        }

        // Schema Validation
        if resource.description.trim().is_empty() {
            eprintln!("❌ Values missing: Resource '{}' has empty description.", resource.id);
            errors += 1;
        }
        if resource.tags.is_empty() {
             eprintln!("❌ Values missing: Resource '{}' has no tags.", resource.id);
             errors += 1;
        }

        // File Existence
        let file_path = base_path.join(&resource.path);
        if !file_path.exists() {
            eprintln!("❌ Broken Link: Resource '{}' points to missing file {:?}", resource.id, file_path);
            errors += 1;
            continue; 
        }

        registered_paths.insert(fs::canonicalize(&file_path).unwrap_or(file_path.clone()));

        // Integrity Check (Hash)
        let current_hash = calculate_sha256(&file_path).unwrap_or_default();
        if let Some(stored_hash) = &resource.checksum {
            if stored_hash != &current_hash {
                 eprintln!("❌ Integrity Failure: Resource '{}' content has changed! (Expected {}, Got {})", resource.id, stored_hash, current_hash);
                 errors += 1;
            }
        } else if args.fix {
            println!("🔧 Checksum added for '{}'", resource.id);
            resource.checksum = Some(current_hash);
        }

        // Integrity Check (Size)
        let metadata = fs::metadata(&file_path).expect("Failed to get metadata");
        let current_size = metadata.len();
        
        if let Some(stored_size) = resource.size_bytes {
            if stored_size != current_size {
                if args.fix {
                    println!("🔧 Size updated for '{}': {} -> {}", resource.id, stored_size, current_size);
                    resource.size_bytes = Some(current_size);
                } else {
                    eprintln!("❌ Integrity Failure: Resource '{}' size mismatch! (Expected {}, Got {})", resource.id, stored_size, current_size);
                    errors += 1;
                }
            }
        } else if args.fix {
            println!("🔧 Size added for '{}': {}", resource.id, current_size);
            resource.size_bytes = Some(current_size);
        } else {
             eprintln!("❌ Metadata Missing: Resource '{}' missing size_bytes.", resource.id);
             errors += 1;
        }
    }

    // 3. Find Orphans
    let mut orphans: Vec<PathBuf> = Vec::new();
    for entry in WalkDir::new(&base_path).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        
        // Skip directory itself and manifest
        if path.is_dir() || path.ends_with("manifest.yaml") {
            continue;
        }

        // Skip hidden files
        if path.file_name().and_then(|s| s.to_str()).map(|s| s.starts_with('.')).unwrap_or(false) {
            continue;
        }

        let abs_path = fs::canonicalize(path).unwrap_or(path.to_path_buf());
        if !registered_paths.contains(&abs_path) {
            orphans.push(path.to_path_buf());
        }
    }

    if !orphans.is_empty() {
        if args.fix {
            println!("🔍 Checking {} orphans...", orphans.len());
            for orphan in orphans {
                let relative_path = orphan.strip_prefix(&base_path).unwrap_or(&orphan);
                let path_str = relative_path.to_string_lossy().replace("\\", "/");
                let id = orphan.file_stem().unwrap().to_string_lossy().to_string().replace(" ", "_").to_lowercase();
                
                let hash = calculate_sha256(&orphan).unwrap_or_default();
                let size = fs::metadata(&orphan).map(|m| m.len()).unwrap_or(0);

                let new_resource = TestResource {
                    id: format!("auto_{}", id),
                    path: path_str,
                    resource_type: "detected".to_string(),
                    tags: vec!["needs-triage".to_string(), "auto-generated".to_string()],
                    description: "Automatically registered orphan. Agent review required.".to_string(),
                    checksum: Some(hash),
                    size_bytes: Some(size),
                };
                
                println!("✨ Auto-registered: {}", new_resource.id);
                manifest.resources.push(new_resource);
            }
            // Save manifest
            let yaml = serde_yaml::to_string(&manifest).unwrap();
            fs::write(&manifest_path, yaml).expect("Failed to write manifest");
        
        } else {
            for orphan in orphans {
                eprintln!("❌ Orphan found: {:?}", orphan);
            }
            eprintln!("💡 Tip: Run with --fix to auto-register orphans.");
            errors += 1;
        }
    }

    if errors > 0 {
        std::process::exit(1);
    }
    
    // Save manifest if modified (checksum updates)
    if args.fix {
        let yaml = serde_yaml::to_string(&manifest).unwrap();
        fs::write(&manifest_path, yaml).expect("Failed to write manifest");
    }

    println!("✅ Test Resource Registry is valid.");
}
