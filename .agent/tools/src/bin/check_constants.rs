use agent_tools::prelude::*;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Constants {
    memory: MemoryConfig,
    network: NetworkConfig,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct MemoryConfig {
    reservation: String,
    limit: String,
    free_headroom: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct NetworkConfig {
    radxa_ip: String,
    ssh_user: String,
    ssh_port: u16,
    web_ui_port: u16,
}

fn main() {
    let mut audit = AuditResult::new("Constants Check");

    let magic_values = [
        "192.168.2.2",  // radxa_ip
        "memory: 1G",   // reservation (in compose)
        "memory: 5G",   // limit
    ];

    let targets = [".agent/skills", "docs", "notes"];

    for target in targets {
        // Need both md and yml files
        let mut files = find_files(target, "md");
        files.extend(find_files(target, "yml"));
        
        for path in files {
            let path_str = path.to_string_lossy();
            
            // Skip the constants.yml itself and environment_constraints
            if path_str.contains("constants.yml") || path_str.contains("environment_constraints") {
                continue;
            }
            
            // Files that document these values as examples
            let excludes = [
                "deploy_on_radxa_rock5", 
                "Documentation_Quality_Comparison", 
                "improvement_plan",
            ];
            
            let normalized_path = path_str.replace("\\", "/");
            if excludes.iter().any(|e| normalized_path.contains(e)) {
                continue;
            }
            
            let content = read_to_string_lossy(&path);
            for (i, line) in content.lines().enumerate() {
                for magic in &magic_values {
                    if line.contains(magic) {
                        audit.fail(&format!("{}:{} -> Hardcoded: '{}'", 
                            path.display(), i + 1, magic));
                    }
                }
            }
        }
    }
    
    audit.print_and_exit();
}
