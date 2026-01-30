use agent_tools::prelude::*;

fn main() {
    let mut audit = AuditResult::new("Consistency Check");
    let targets = [".agent/skills", ".agent/workflows", "docs", "notes"];
    let patterns = ["TODO", "TBD", "FIXME", "[TBD]", "PLACEHOLDER"];
    
    // Files that legitimately discuss these terms (rules & history)
    let excludes = [
        "code_quality_standards/SKILL.md",
        "project_workflows/SKILL.md", 
        "audit_consistency.md",
        "audit_tool_alignment.md",
        "token_efficiency_analysis",
        "alignment_report.md",
    ];

    for target in targets {
        let files = find_files(target, "md");
        for path in files {
            // Check excludes
            let path_str = path.to_string_lossy().replace("\\", "/");
            if excludes.iter().any(|e| path_str.contains(e)) {
                continue;
            }

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
