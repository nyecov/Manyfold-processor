use std::fs;
use std::collections::HashMap;
use walkdir::WalkDir;

/// Checks skill organization: large files, missing docs, orphaned skills
fn main() {
    let mut errors = 0;
    let mut warnings = 0;
    println!("[AUDIT] Checking Skill Context Organization...");

    let skills_dir = ".agent/skills";
    let annex_dir = ".agent/annex";
    
    // 1. Check skill sizes (flag if >200 lines = too broad)
    for entry in WalkDir::new(skills_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_name().to_str() == Some("SKILL.md"))
    {
        let path = entry.path();
        if let Ok(content) = fs::read_to_string(path) {
            let line_count = content.lines().count();
            if line_count > 200 {
                println!("[XX] {} -> Too broad ({} lines, max 200)", 
                    path.display(), line_count);
                errors += 1;
            } else if line_count > 150 {
                println!("[WW] {} -> Consider splitting ({} lines)", 
                    path.display(), line_count);
                warnings += 1;
            }
        }
    }

    // 2. Check for skills without descriptions
    for entry in WalkDir::new(skills_dir)
        .min_depth(1)
        .max_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_dir())
    {
        let skill_file = entry.path().join("SKILL.md");
        if !skill_file.exists() {
            println!("[XX] {} -> Missing SKILL.md", entry.path().display());
            errors += 1;
        }
    }

    // 3. Check annex has README
    let annex_readme = format!("{}/README.md", annex_dir);
    if !std::path::Path::new(&annex_readme).exists() {
        println!("[XX] {} -> Missing README.md", annex_dir);
        errors += 1;
    }

    if errors == 0 && warnings == 0 {
        println!("[OK] Skill Context Organization Valid");
        std::process::exit(0);
    } else if errors == 0 {
        println!("[WW] {} warnings (no errors)", warnings);
        std::process::exit(0);
    } else {
        println!("[XX] Found {} errors, {} warnings", errors, warnings);
        std::process::exit(1);
    }
}
