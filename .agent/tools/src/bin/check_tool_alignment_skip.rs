//! Hash-based skip detection for /audit_tool_alignment
//!
//! This script computes a SHA-256 hash of all tool-related content and compares
//! it against a cached value. If unchanged, the expensive agent audit can be skipped.
//!
//! Exit Codes:
//!   0 = Skip audit (cached PASS, hash matches)
//!   1 = Skip audit but warn (cached FAIL, hash matches)  
//!   2 = Run full audit (hash mismatch or no cache)

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{fs, path::Path, process::exit};
use walkdir::WalkDir;

const CACHE_PATH: &str = ".agent/tools/.audit_cache";

#[derive(Debug, Serialize, Deserialize)]
struct AuditCache {
    version: u32,
    timestamp: String,
    hash: String,
    result: String,
    #[serde(default)]
    findings: Vec<String>,
}

fn main() {
    println!("=== Tool Alignment Skip Check ===\n");

    let current_hash = compute_content_hash();
    println!("Current content hash: {}", &current_hash);

    match read_cache() {
        Some(cache) => {
            println!("Cached hash:          {}...", &cache.hash[..16.min(cache.hash.len())]);
            println!("Last audit:           {}", cache.timestamp);
            println!("Cached result:        {}", cache.result);

            if cache.hash == current_hash {
                println!("\n[SKIP] Content unchanged since last audit");
                if cache.result == "PASS" {
                    println!("✅ Returning cached PASS - no audit needed");
                    exit(0);
                } else {
                    println!("⚠️  Returning cached FAIL - review previous findings:");
                    for finding in &cache.findings {
                        println!("  - {}", finding);
                    }
                    exit(1);
                }
            } else {
                println!("\n[RUN] Content changed - full audit required");
                println!("Hash mismatch detected");
                exit(2);
            }
        }
        None => {
            println!("No cache found at {}", CACHE_PATH);
            println!("\n[RUN] First run - full audit required");
            exit(2);
        }
    }
}

fn compute_content_hash() -> String {
    let mut hasher = Sha256::new();
    let mut files_hashed = 0;

    // 1. Hash all tool source files (sorted for determinism)
    let bin_path = Path::new(".agent/tools/src/bin");
    if bin_path.exists() {
        let mut rs_files: Vec<_> = WalkDir::new(bin_path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map_or(false, |ext| ext == "rs"))
            .map(|e| e.path().to_path_buf())
            .collect();
        rs_files.sort();

        for file in rs_files {
            if let Ok(content) = fs::read_to_string(&file) {
                hasher.update(content.as_bytes());
                files_hashed += 1;
            }
        }
    }
    println!("Hashed {} Rust source files", files_hashed);

    // 2. Hash Cargo.toml (dependency changes matter)
    let cargo_path = Path::new(".agent/tools/Cargo.toml");
    if let Ok(content) = fs::read_to_string(cargo_path) {
        hasher.update(content.as_bytes());
        println!("Hashed Cargo.toml");
    }

    // 3. Hash workflows that reference tools (sorted for determinism)
    let workflows_path = Path::new(".agent/workflows");
    let mut workflow_count = 0;
    if workflows_path.exists() {
        let mut workflow_files: Vec<_> = WalkDir::new(workflows_path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map_or(false, |ext| ext == "md"))
            .filter_map(|e| {
                let content = fs::read_to_string(e.path()).ok()?;
                if content.contains(".agent\\tools\\")
                    || content.contains(".agent/tools/")
                    || content.contains("Headless")
                {
                    Some((e.path().to_path_buf(), content))
                } else {
                    None
                }
            })
            .collect();
        workflow_files.sort_by(|a, b| a.0.cmp(&b.0));

        for (_, content) in workflow_files {
            hasher.update(content.as_bytes());
            workflow_count += 1;
        }
    }
    println!("Hashed {} tool-referencing workflows", workflow_count);

    hex::encode(hasher.finalize())
}

fn read_cache() -> Option<AuditCache> {
    let content = fs::read_to_string(CACHE_PATH).ok()?;
    serde_yaml::from_str(&content).ok()
}

/// Write cache after a successful audit (called by agent, not this binary)
/// This function is provided for reference but not used by the skip checker
#[allow(dead_code)]
fn write_cache(result: &str, findings: Vec<String>) -> std::io::Result<()> {
    let hash = compute_content_hash();
    let cache = AuditCache {
        version: 1,
        timestamp: chrono::Utc::now().to_rfc3339(),
        hash,
        result: result.to_string(),
        findings,
    };
    let yaml = serde_yaml::to_string(&cache).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    fs::write(CACHE_PATH, yaml)
}
