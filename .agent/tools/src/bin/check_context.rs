//! Evaluates semantic organization (Strategy vs Reference)
use agent_tools::prelude::*;

fn main() {
    let mut audit = AuditResult::new("Context Check");
    let skills_dir = ".agent/skills";
    let annex_dir = ".agent/annex";
    
    // 1. Check skill sizes (SKILL.md only)
    let files = find_files(skills_dir, "md");
    for path in files {
        if path.file_name().map_or(false, |n| n == "SKILL.md") {
             let content = read_to_string_lossy(&path);
             let line_count = content.lines().count();
             if line_count > 200 {
                 audit.fail(&format!("{} -> Too broad ({} lines, max 200)", path.display(), line_count));
             } else if line_count > 150 {
                 audit.warn(&format!("{} -> Consider splitting ({} lines)", path.display(), line_count));
             }
        }
    }
    
    // 2. Check for skills without descriptions (Missing SKILL.md)
    let dirs = list_immediate_subdirs(skills_dir);
    for dir in dirs {
        let skill_file = dir.join("SKILL.md");
        if !skill_file.exists() {
            audit.fail(&format!("{} -> Missing SKILL.md", dir.display()));
        }
    }
    
    // 3. Check annex has README
    let annex_readme = std::path::Path::new(annex_dir).join("README.md");
    if !annex_readme.exists() {
        audit.fail(&format!("{} -> Missing README.md", annex_dir));
    }
    
    audit.print_and_exit();
}
