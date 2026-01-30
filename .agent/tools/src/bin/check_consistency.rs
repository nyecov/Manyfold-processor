use agent_tools::prelude::*;

fn main() {
    let mut audit = AuditResult::new("Consistency Check");
    let targets = [".agent/skills", ".agent/workflows", "docs", "notes"];
    let patterns = ["TODO", "TBD", "FIXME", "[TBD]", "PLACEHOLDER"];

    for target in targets {
        let files = find_files(target, "md");
        for path in files {
            let content = read_to_string_lossy(&path);
            for (i, line) in content.lines().enumerate() {
                for pattern in &patterns {
                    if line.to_uppercase().contains(pattern) {
                         audit.fail(&format!("{}:{} -> Incomplete: '{}'", path.display(), i + 1, line.trim()));
                    }
                }
            }
        }
    }
    
    audit.print_and_exit();
}
