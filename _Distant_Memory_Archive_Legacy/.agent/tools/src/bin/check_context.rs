//! Evaluates semantic organization (Strategy vs Reference)
use agent_tools::prelude::*;
use sha2::{Sha256, Digest};
use regex::Regex;

fn main() {
    let mut audit = AuditResult::new("Context Check");
    let skills_dir = ".agent/skills";
    let annex_dir = ".agent/annex";
    
    // Regex for suppression comment: <!-- context_warning_reviewed: <HASH> -->
    // Consumes optional trailing newline to avoid hash drift
    let suppression_re = Regex::new(r"<!--\s*context_warning_reviewed:\s*([a-f0-9]{64})\s*-->(\r?\n)?").unwrap();

    // 1. Check skill sizes (SKILL.md only)
    let files = find_files(skills_dir, "md");
    for path in files {
        if path.file_name().map_or(false, |n| n == "SKILL.md") {
             let content = read_to_string_lossy(&path);
             let line_count = content.lines().count();
             
             if line_count > 200 {
                 audit.fail(&format!("{} -> Too broad ({} lines, max 200)", path.display(), line_count));
             } else if line_count > 150 {
                 // Check for valid suppression
                 let mut is_suppressed = false;
                 
                 if let Some(cap) = suppression_re.captures(&content) {
                     let stored_hash = &cap[1];
                     
                     // Calculate hash of content WITHOUT the suppression line
                     // We act as if that line is empty to avoid circular dependency, or remove it
                     let clean_content = suppression_re.replace(&content, "");
                     let mut hasher = Sha256::new();
                     hasher.update(clean_content.as_bytes());
                     let calculated_hash = hex::encode(hasher.finalize());
                     
                     if stored_hash == calculated_hash {
                         is_suppressed = true;
                     } else {
                         audit.warn(&format!("{} -> Hash mismatch! Content changed since review. Expected: {}", path.display(), calculated_hash));
                     }
                 }

                 if !is_suppressed {
                     // Calculate hash for user convenience
                     let clean_content = suppression_re.replace(&content, "");
                     let mut hasher = Sha256::new();
                     hasher.update(clean_content.as_bytes());
                     let current_hash = hex::encode(hasher.finalize());
                     
                     audit.warn(&format!("{} -> Consider splitting ({} lines). To suppress, add: <!-- context_warning_reviewed: {} -->", path.display(), line_count, current_hash));
                 }
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
