//! Checks skill examples for existence and orphans
//! Workflow: /audit_skill_examples
use agent_tools::prelude::*;
use regex::Regex;
use std::collections::HashSet;

fn main() {
    let mut audit = AuditResult::new("Skill Examples Check");
    let skills_dir = ".agent/skills";
    let mut skills_with_examples = 0;

    // Regex to find markdown links to examples/
    let example_link_re = Regex::new(r"\[.*?\]\((examples/[^)]+)\)").unwrap();

    // Get all skill directories
    let skill_dirs: Vec<_> = std::fs::read_dir(skills_dir)
        .into_iter()
        .flatten()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_dir())
        .collect();

    for skill_entry in skill_dirs {
        let skill_path = skill_entry.path();
        let skill_name = skill_path.file_name().unwrap().to_string_lossy();
        let skill_md = skill_path.join("SKILL.md");

        if !skill_md.exists() {
            continue;
        }

        let content = read_to_string_lossy(&skill_md);
        
        // Find all example links in SKILL.md
        let mut linked_examples: HashSet<String> = HashSet::new();
        for cap in example_link_re.captures_iter(&content) {
            linked_examples.insert(cap[1].to_string());
        }

        if linked_examples.is_empty() {
            continue;
        }

        skills_with_examples += 1;

        // Check linked examples exist
        for example_link in &linked_examples {
            let example_path = skill_path.join(example_link);
            if !example_path.exists() {
                audit.fail(&format!("{}: Missing example -> {}", skill_name, example_link));
            } else {
                // Check if file is empty
                let example_content = read_to_string_lossy(&example_path);
                if example_content.trim().is_empty() {
                    audit.fail(&format!("{}: Empty example -> {}", skill_name, example_link));
                }
            }
        }

        // Check for orphan examples (exist but not linked)
        let examples_dir = skill_path.join("examples");
        if examples_dir.exists() {
            if let Ok(entries) = std::fs::read_dir(&examples_dir) {
                for entry in entries.filter_map(|e| e.ok()) {
                    let file_name = entry.file_name().to_string_lossy().to_string();
                    let relative_path = format!("examples/{}", file_name);
                    
                    if !linked_examples.contains(&relative_path) {
                        audit.warn(&format!("{}: Orphan example -> {}", skill_name, relative_path));
                    }
                }
            }
        }
    }

    // Report summary
    if skills_with_examples == 0 {
        println!("Note: No skills with examples found");
    } else {
        println!("Checked {} skill(s) with examples", skills_with_examples);
    }

    audit.print_and_exit();
}
