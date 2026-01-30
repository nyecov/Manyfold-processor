use std::fs;
use walkdir::WalkDir;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Constants {
    memory: MemoryConfig,
    network: NetworkConfig,
}

#[derive(Debug, Deserialize)]
struct MemoryConfig {
    reservation: String,
    limit: String,
    free_headroom: String,
}

#[derive(Debug, Deserialize)]
struct NetworkConfig {
    radxa_ip: String,
    ssh_user: String,
    ssh_port: u16,
    web_ui_port: u16,
}

/// Checks for hardcoded magic values that should use constants.yml
fn main() {
    let mut errors = 0;
    println!("[AUDIT] Checking for Hardcoded Constants...");

    // Magic values that should be in constants.yml
    let magic_values = [
        "192.168.2.2",  // radxa_ip
        "memory: 1G",   // reservation (in compose)
        "memory: 5G",   // limit
    ];

    let targets = [".agent/skills", "docs", "notes"];
    let exclude = ".agent/constants.yml";
    let exclude_env = ".agent/skills/environment_constraints";

    for target in targets {
        for entry in WalkDir::new(target)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map_or(false, |ext| ext == "md" || ext == "yml"))
        {
            let path = entry.path();
            let path_str = path.to_string_lossy();
            
            // Skip the constants.yml itself and environment_constraints
            if path_str.contains("constants.yml") || path_str.contains("environment_constraints") {
                continue;
            }
            
            if let Ok(content) = fs::read_to_string(path) {
                for (i, line) in content.lines().enumerate() {
                    for magic in &magic_values {
                        if line.contains(magic) {
                            println!("[XX] {}:{} -> Hardcoded: '{}'", 
                                path.display(), i + 1, magic);
                            errors += 1;
                        }
                    }
                }
            }
        }
    }

    if errors == 0 {
        println!("[OK] No Hardcoded Magic Values Found");
        std::process::exit(0);
    } else {
        println!("[XX] Found {} hardcoded constants (should use constants.yml)", errors);
        std::process::exit(1);
    }
}
