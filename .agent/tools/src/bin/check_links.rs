use agent_tools::prelude::*;

fn main() {
    let mut audit = AuditResult::new("Link Check (Absolute Paths)");
    let targets = [".agent", "docs", "notes", "tests"];

    for target in targets {
        // find_files returns empty vector if root doesn't exist
        let files = find_files(target, "md");
        
        for path in files {
            let content = read_to_string_lossy(&path);
            for (i, line) in content.lines().enumerate() {
                // Check for hardcoded absolute paths
                if line.contains("C:/") || line.replace("\\", "/").contains("Users/Furiosa") {
                    audit.fail(&format!("{}:{} -> {}", path.display(), i + 1, line.trim()));
                }
            }
        }
    }
    
    audit.print_and_exit();
}
