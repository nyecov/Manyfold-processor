//! Verifies Gherkin scenario quality
use agent_tools::prelude::*;

fn main() {
    let mut audit = AuditResult::new("Gherkin Quality Check");
    
    if !exists("tests") {
        audit.warn("No 'tests' directory found used for Gherkin features.");
        audit.print_and_exit(); 
    }

    // 1. Check Scenario Limits (Global)
    let files = find_files("tests", "feature");
    for path in &files {
        let content = read_to_string_lossy(path);
        let scenarios = content.matches("Scenario:").count();
        if scenarios > 10 { // Relaxed slightly, but generally should be low
             audit.warn(&format!("{}: High scenario count ({})", path.display(), scenarios));
        }
    }

    // 2. Check Twin-UI Linking (Logic Features)
    let logic_dir = std::path::Path::new("tests/features/processing_logic");
    if logic_dir.exists() {
        let logic_files = find_files("tests/features/processing_logic", "feature");
        for path in logic_files {
            let content = read_to_string_lossy(&path);
            let has_header = content.lines().any(|line| line.trim().starts_with("Twin-UI:") || line.trim().starts_with("# Twin-UI:"));
            
            if !has_header {
                audit.fail(&format!("{}: Missing 'Twin-UI: path/to/ui.feature' (or '# Twin-UI:' comment) in Logic test.", path.display()));
            } else {
                // Verify the link is valid
                if let Some(line) = content.lines().find(|l| l.trim().starts_with("Twin-UI:") || l.trim().starts_with("# Twin-UI:")) {
                    let link_path = line.trim()
                        .replace("Twin-UI:", "")
                        .replace("#", "")
                        .trim().to_string();
                    
                    let target = std::path::Path::new(&link_path);
                    if !target.exists() {
                        audit.fail(&format!("{}: Broken Twin-UI link. Target '{}' not found.", path.display(), link_path));
                    }
                }
            }
        }
    }
    
    audit.print_and_exit();
}
