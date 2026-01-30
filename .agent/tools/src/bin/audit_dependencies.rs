use std::fs;
use std::collections::HashSet;
use walkdir::WalkDir;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct SkillFrontmatter {
    requires: Option<Vec<String>>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let skills_dir = ".agent/skills";
    let mut skill_names = HashSet::new();
    let mut errors = 0;

    println!("[AUDIT] Starting Dependency Audit...");

    // 1. Index all skill names
    for entry in WalkDir::new(skills_dir)
        .min_depth(1)
        .max_depth(1)
        .into_iter()
        .filter_map(|e| e.ok()) {
        if entry.file_type().is_dir() {
            if let Some(name) = entry.file_name().to_str() {
                skill_names.insert(name.to_string());
            }
        }
    }

    // 2. Scan each SKILL.md for 'requires'
    for skill in &skill_names {
        let path = format!("{}/{}/SKILL.md", skills_dir, skill);
        let content = match fs::read_to_string(&path) {
            Ok(c) => c,
            Err(_) => {
                println!("[XX] [{}] SKILL.md missing", skill);
                errors += 1;
                continue;
            }
        };

        // Parse frontmatter (between first two ---)
        let parts: Vec<&str> = content.split("---").collect();
        if parts.len() < 3 {
             println!("[XX] [{}] Invalid frontmatter", skill);
             errors += 1;
             continue;
        }

        let frontmatter_str = parts[1];
        let fm: Result<SkillFrontmatter, _> = serde_yaml::from_str(frontmatter_str);

        match fm {
            Ok(data) => {
                if let Some(reqs) = data.requires {
                    for req in reqs {
                        if !skill_names.contains(&req) {
                             println!("[XX] [{}] Invalid requirement: '{}'", skill, req);
                             errors += 1;
                        }
                    }
                }
            },
            Err(e) => {
                println!("[XX] [{}] YAML Error: {}", skill, e);
                errors += 1;
            }
        }
    }

    if errors == 0 {
        println!("[OK] Dependency Graph Valid");
        std::process::exit(0);
    } else {
        println!("[XX] Found {} errors", errors);
        std::process::exit(1);
    }
}
