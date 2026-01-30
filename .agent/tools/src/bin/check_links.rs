use std::fs;
use walkdir::WalkDir;

fn main() {
    let mut errors = 0;
    println!("[AUDIT] Checking for Broken Links...");

    let targets = [".agent", "docs", "notes", "tests"];

    for target in targets {
        for entry in WalkDir::new(target)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map_or(false, |ext| ext == "md"))
        {
            let path = entry.path();
            if let Ok(content) = fs::read_to_string(path) {
                for (i, line) in content.lines().enumerate() {
                    if line.contains("C:/") || line.replace("\\", "/").contains("Users/Furiosa") {
                        println!("[XX] Absolute Path in {}:{} -> {}", path.display(), i + 1, line.trim());
                        errors += 1;
                    }
                }
            }
        }
    }

    if errors == 0 {
        println!("[OK] No Absolute Paths Found");
        std::process::exit(0);
    } else {
        println!("[XX] Found {} absolute path violations", errors);
        std::process::exit(1);
    }
}
