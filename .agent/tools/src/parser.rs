use regex::Regex;

/// Extract all Markdown links `[text](url)` from content
pub fn find_markdown_links(content: &str) -> Vec<String> {
    let re = Regex::new(r"\[.*?\]\((.*?)\)").unwrap();
    re.captures_iter(content)
        .map(|cap| cap[1].to_string())
        .collect()
}

/// Extract YAML frontmatter `requires: [foo, bar]`
/// Returns a list of required skills/workflows
pub fn parse_requires(content: &str) -> Vec<String> {
    let re = Regex::new(r"(?m)^requires:\s*\[(.*?)\]").unwrap();
    if let Some(cap) = re.captures(content) {
        cap[1].split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    } else {
        Vec::new()
    }
}
