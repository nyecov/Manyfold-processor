use std::fs;
use walkdir::WalkDir;

/// Checks for consistency issues: TODOs, TBDs, placeholders
fn main() {
    let mut errors = 0;
    println!("[AUDIT] Checking Document Consistency...");

    let targets = [".agent/skills", ".agent/workflows", "docs", "notes"];
    let patterns = ["TODO", "TBD", "FIXME", "[TBD]", "PLACEHOLDER"];

    for target in targets {
        for entry in WalkDir::new(target)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map_or(false, |ext| ext == "md"))
        {
            let path = entry.path();
            if let Ok(content) = fs::read_to_string(path) {
                for (i, line) in content.lines().enumerate() {
                    for pattern in &patterns {
                        if line.to_uppercase().contains(pattern) {
                            println!("[XX] {}:{} -> Incomplete: '{}'", 
                                path.display(), i + 1, line.trim());
                            errors += 1;
                        }
                    }
                }
            }
        }
    }

    if errors == 0 {
        println!("[OK] No Placeholders or TODOs Found");
        std::process::exit(0);
    } else {
        println!("[XX] Found {} consistency issues", errors);
        std::process::exit(1);
    }
}
