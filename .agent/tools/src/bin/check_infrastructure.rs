use std::fs;
use std::path::Path;

/// Checks infrastructure alignment: compose.yml, Dockerfile, Cargo.toml
fn main() {
    let mut errors = 0;
    println!("[AUDIT] Checking Infrastructure Alignment...");

    // 1. Check compose.yml exists and has memory constraints
    let compose_path = "compose.yml";
    if Path::new(compose_path).exists() {
        if let Ok(content) = fs::read_to_string(compose_path) {
            // Check for memory reservation
            if !content.contains("reservation:") {
                println!("[XX] compose.yml -> Missing memory reservation");
                errors += 1;
            }
            // Check for memory limit
            if !content.contains("limits:") && !content.contains("limit:") {
                println!("[XX] compose.yml -> Missing memory limits");
                errors += 1;
            }
            println!("[OK] compose.yml found with memory config");
        }
    } else {
        println!("[XX] compose.yml -> File not found");
        errors += 1;
    }

    // 2. Check Dockerfile exists
    let dockerfile_path = "Dockerfile";
    if Path::new(dockerfile_path).exists() {
        if let Ok(content) = fs::read_to_string(dockerfile_path) {
            // Check for multi-stage build
            if content.matches("FROM").count() < 2 {
                println!("[WW] Dockerfile -> Consider multi-stage build");
            } else {
                println!("[OK] Dockerfile uses multi-stage build");
            }
        }
    } else {
        println!("[XX] Dockerfile -> File not found");
        errors += 1;
    }

    // 3. Check Cargo.toml exists
    let cargo_path = "Cargo.toml";
    if Path::new(cargo_path).exists() {
        println!("[OK] Cargo.toml found");
    } else {
        println!("[XX] Cargo.toml -> File not found");
        errors += 1;
    }

    // 4. Check for Python files (should be minimal)
    let python_count: usize = walkdir::WalkDir::new("src")
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "py"))
        .count();

    if python_count > 0 {
        println!("[WW] Found {} Python files in src/ (Rust-first mandate)", python_count);
    } else {
        println!("[OK] No Python files in src/");
    }

    if errors == 0 {
        println!("[OK] Infrastructure Alignment Valid");
        std::process::exit(0);
    } else {
        println!("[XX] Found {} infrastructure violations", errors);
        std::process::exit(1);
    }
}
