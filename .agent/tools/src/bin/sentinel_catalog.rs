//! Metadata Sentinel: Tool Catalog Synchronizer
//!
//! Scans `.agent/tools/src/bin/*.rs` and updates the "Binary Registry" 
//! in `.agent/skills/project_workflows/SKILL.md`.
//!
//! Exit Codes:
//!   0 = Catalog aligned (no changes)
//!   1 = Catalog updated (user must stage changes)
//!   2 = Error

use agent_tools::prelude::*;
use std::path::Path;

struct ToolMetadata {
    name: String,
    description: String,
    workflow: String,
}

fn main() {
    let mut audit = AuditResult::new("Sentinel: Catalog");


    // 1. Scan Tools
    let tools_dir = ".agent/tools/src/bin";
    let tools = scan_tools(tools_dir);
    
    if tools.is_empty() {
        audit.fail("No tools found in source directory");
        audit.print_and_exit();
    }

    // 2. Read SKILL.md
    let skill_path = ".agent/skills/project_workflows/SKILL.md";
    if !exists(skill_path) {
        audit.fail("SKILL.md not found");
        audit.print_and_exit();
    }
    let content = read_to_string_lossy(Path::new(skill_path));

    // 3. Update Table
    let (new_content, updated) = update_registry_table(&content, &tools);
    
    if updated {
        if let Err(e) = std::fs::write(skill_path, new_content) {
            audit.fail(&format!("Failed to write SKILL.md: {}", e));
        } else {
            // We consciously exit with 1 (Fail) to block the commit
            // and force the user to review/stage the update.
            println!("[WW] Catalog updated. Please stage changes and retry commit.");
            std::process::exit(1);
        }
    } else {
        println!("[OK] Catalog aligned.");
        std::process::exit(0);
    }
}

fn scan_tools(dir: &str) -> Vec<ToolMetadata> {
    let mut metadata = Vec::new();
    let files = find_files(dir, "rs");

    for path in files {
        let name = path.file_stem().unwrap().to_string_lossy().to_string();
        let content = read_to_string_lossy(&path);
        
        // Parse doc comments
        let mut description = "Undocumented".to_string();
        let mut workflow = "(Meta)".to_string(); // Default

        for line in content.lines() {
            if line.starts_with("//!") {
                let clean = line.trim_start_matches("//!").trim();
                // Simple heuristic: First line is description
                if description == "Undocumented" && !clean.is_empty() {
                    description = clean.to_string();
                }
                // Try to find workflow reference? 
                // For now, simple description is enough.
            }
        }
        
        // Infer workflow from name if possible
        if name.starts_with("check_") {
            workflow = format!("/audit_{}", &name[6..].replace("_skip", ""));
        } else if name.starts_with("audit_") {
            workflow = format!("/{}", name);
        }

        metadata.push(ToolMetadata {
            name: format!("{}.exe", name), // Windows focus
            description,
            workflow, 
        });
    }
    
    metadata.sort_by(|a, b| a.name.cmp(&b.name));
    metadata
}

fn update_registry_table(content: &str, tools: &[ToolMetadata]) -> (String, bool) {
    let table_marker = "### Binary Registry";
    let start_idx = match content.find(table_marker) {
        Some(i) => i,
        None => return (content.to_string(), false), // Table not found, abort safely
    };

    // Find table start (header)
    let header_start = match content[start_idx..].find("| Tool |") {
        Some(i) => start_idx + i,
        None => return (content.to_string(), false),
    };

    // Find table end (first empty line after header)
    // Handle CRLF (\r\n\r\n) or LF (\n\n)
    let table_end_rel = if let Some(i) = content[header_start..].find("\r\n\r\n") {
        i
    } else if let Some(i) = content[header_start..].find("\n\n") {
        i
    } else {
        content.len() - header_start
    };
    let table_end = header_start + table_end_rel;

    // Construct new table
    let mut new_table = String::new();
    new_table.push_str("| Tool | Workflow | 🔧 Covers |\n");
    new_table.push_str("|------|----------|-----------|\n");
    
    for tool in tools {
        // preserve pipes format
        new_table.push_str(&format!("| `{}` | `{}` | {} |\n", 
            tool.name, tool.workflow, tool.description));
    }

    let original_table = &content[header_start..table_end];
    
    // Normalize newlines for comparison
    if original_table.trim() == new_table.trim() {
        return (content.to_string(), false);
    }

    // Replace
    let new_content = format!("{}{}{}", 
        &content[..header_start],
        new_table.trim(),
        &content[table_end..]
    );

    (new_content, true)
}
