use agent_tools::prelude::*;

fn main() {
    let mut audit = AuditResult::new("Infrastructure Check");
    
    // 1. compose.yml
    let compose_path = "compose.yml";
    if exists(compose_path) {
        let content = read_to_string_lossy(std::path::Path::new(compose_path));
        if !content.contains("reservation:") && !content.contains("reservations:") {
            audit.fail("compose.yml -> Missing memory reservation");
        }
        if !content.contains("limits:") && !content.contains("limit:") {
            audit.fail("compose.yml -> Missing memory limits");
        }
    } else {
        audit.fail("compose.yml -> File not found");
    }
    
    // 2. Dockerfile
    let dockerfile_path = "Dockerfile";
    if exists(dockerfile_path) {
        let content = read_to_string_lossy(std::path::Path::new(dockerfile_path));
        if content.matches("FROM").count() < 2 {
            audit.warn("Dockerfile -> Consider multi-stage build");
        }
    } else {
        audit.fail("Dockerfile -> File not found");
    }
    
    // 3. Cargo.toml
    if !exists("Cargo.toml") {
        audit.fail("Cargo.toml -> File not found");
    }
    
    // 4. Python files check
    if exists("src") {
        let python_files = find_files("src", "py");
        if !python_files.is_empty() {
             audit.warn(&format!("Found {} Python files in src/ (Rust-first mandate)", python_files.len()));
        }
    }
    
    audit.print_and_exit();
}
