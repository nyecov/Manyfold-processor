use std::fs;
use walkdir::WalkDir;

fn main() {
    let mut errors = 0;
    println!("[AUDIT] Checking Gherkin Quality...");

    for entry in WalkDir::new("tests")
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "feature"))
    {
        let path = entry.path();
        if let Ok(content) = fs::read_to_string(path) {
            let scenarios = content.matches("Scenario:").count();
            
            if scenarios > 5 {
                println!("[XX] {}: Too many scenarios ({} > 5)", path.display(), scenarios);
                errors += 1;
            }
        }
    }

    if errors == 0 {
        println!("[OK] Gherkin Limits Compliant");
        std::process::exit(0);
    } else {
        println!("[XX] Found {} violations", errors);
        std::process::exit(1);
    }
}
