use std::collections::HashSet;
use serde::Deserialize;
use agent_tools::prelude::*;

#[derive(Debug, Deserialize)]
struct SkillFrontmatter {
    requires: Option<Vec<String>>,
}

fn main() {
    let mut audit = AuditResult::new("Dependency Audit");
    let skills_dir = ".agent/skills";
    let mut skill_names = HashSet::new();

    // 1. Index all skill names (immediate subdirectories)
    let dirs = list_immediate_subdirs(skills_dir);
    for path in dirs {
        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
             skill_names.insert(name.to_string());
        }
    }

    // 2. Scan each SKILL.md for 'requires'
    for skill in &skill_names {
        let path = std::path::Path::new(skills_dir).join(skill).join("SKILL.md");
        
        if !path.exists() {
            audit.fail(&format!("[{}] SKILL.md missing", skill));
            continue;
        }
        
        let content = read_to_string_lossy(&path);
        
        // Parse frontmatter (between first two ---)
        let parts: Vec<&str> = content.split("---").collect();
        if parts.len() < 3 {
             audit.fail(&format!("[{}] Invalid frontmatter", skill));
             continue;
        }
        
        let frontmatter_str = parts[1];
        let fm: Result<SkillFrontmatter, _> = serde_yaml::from_str(frontmatter_str);
        
        match fm {
             Ok(data) => {
                 if let Some(reqs) = data.requires {
                     for req in reqs {
                         if !skill_names.contains(&req) {
                             audit.fail(&format!("[{}] Invalid requirement: '{}'", skill, req));
                         }
                     }
                 }
             },
             Err(e) => {
                 audit.fail(&format!("[{}] YAML Error: {}", skill, e));
             }
        }
    }
    
    audit.print_and_exit();
}
