//! Checks git status for workflow dependencies
use agent_tools::prelude::*;
use std::process::Command;
use std::collections::HashMap;

/// Parses workflow files for dependency comments and checks git status.
fn main() {
    println!("[AUDIT] Checking Workflow Skip Conditions...");
    
    let workflows_dir = ".agent/workflows";
    let mut results: HashMap<String, Vec<String>> = HashMap::new();
    
    if exists(workflows_dir) {
        let files = find_files(workflows_dir, "md");
        
        for path in files {
            let workflow_name = path.file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown");
            
            let content = read_to_string_lossy(&path);
            let mut dependencies: Vec<String> = Vec::new();
            
            // Extract <!-- depends: path --> comments
            for line in content.lines() {
                if let Some(start) = line.find("<!-- depends:") {
                    if let Some(end) = line[start..].find("-->") {
                        let dep = line[start + 13..start + end].trim();
                        if !dep.is_empty() {
                            dependencies.push(dep.to_string());
                        }
                    }
                }
            }
            
            if !dependencies.is_empty() {
                results.insert(workflow_name.to_string(), dependencies);
            }
        }
    }
    
    if results.is_empty() {
        println!("[WW] No workflows have dependency comments (<!-- depends: path -->)");
        println!("[WW] Add comments to enable skip detection");
        std::process::exit(0);
    }
    
    // Check git status for each workflow's dependencies
    let mut skip_count = 0;
    let mut run_count = 0;
    
    for (workflow, deps) in &results {
        let mut modified: Vec<String> = Vec::new();
        
        for dep in deps {
            // Check if file/directory has uncommitted changes
            let status = Command::new("git")
                .args(["status", "--porcelain", dep])
                .output();
            
            if let Ok(output) = status {
                let stdout = String::from_utf8_lossy(&output.stdout);
                if !stdout.trim().is_empty() {
                    modified.push(dep.clone());
                    continue;
                }
            }
            
            // Check if file/directory changed since last commit
            let diff = Command::new("git")
                .args(["diff", "HEAD~1", "--name-only", "--", dep])
                .output();
            
            if let Ok(output) = diff {
                let stdout = String::from_utf8_lossy(&output.stdout);
                if !stdout.trim().is_empty() {
                    modified.push(dep.clone());
                }
            }
        }
        
        if modified.is_empty() {
            println!("[SKIP] {} - No dependencies changed", workflow);
            skip_count += 1;
        } else {
            println!("[RUN]  {} - Modified: {}", workflow, modified.join(", "));
            run_count += 1;
        }
    }
    
    println!("---");
    println!("Summary: {} workflows to SKIP, {} workflows need to RUN", skip_count, run_count);
    
    if run_count > 0 {
        std::process::exit(1);
    } else {
        std::process::exit(0);
    }
}
