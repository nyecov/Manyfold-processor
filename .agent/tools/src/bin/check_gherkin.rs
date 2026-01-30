//! Verifies Gherkin scenario quality
use agent_tools::prelude::*;

fn main() {
    let mut audit = AuditResult::new("Gherkin Quality Check");
    
    // Check if tests directory exists
    if !exists("tests") {
        audit.warn("No 'tests' directory found used for Gherkin features.");
        audit.print_and_exit(); 
    }

    let files = find_files("tests", "feature");
    
    for path in files {
        let content = read_to_string_lossy(&path);
        let scenarios = content.matches("Scenario:").count();
        
        if scenarios > 5 {
            audit.fail(&format!("{}: Too many scenarios ({} > 5)", path.display(), scenarios));
        }
    }
    
    audit.print_and_exit();
}
