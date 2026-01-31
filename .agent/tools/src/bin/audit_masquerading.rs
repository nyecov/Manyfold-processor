use regex::Regex;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

const UI_KEYWORDS: &[&str] = &["click", "dashboard", "visual", "page", "see", "show", "ui"];
const API_KEYWORDS: &[&str] = &["client.post", "client.get", "/api/", "reqwest"];
const UI_VERIFICATION_PATTERNS: &[&str] = &[
    ".contains(\"id=\")",
    ".contains(\"<\")",
    ".contains(\"class=\")",
    "body.contains(",
];

struct Finding {
    file: String,
    fn_name: String,
    step_text: String,
    reason: String,
}

fn audit_file(path: &Path) -> Vec<Finding> {
    let content = fs::read_to_string(path).unwrap_or_default();
    let mut findings = Vec::new();

    // Combined regex to capture step type, text, function name, and body
    // Note: Rust's regex crate doesn't support backreferences or complex lookarounds,
    // but this multi-line regex should work for standard cucumber-rs patterns.
    let re = Regex::new(r#"(?s)#\[(given|when|then)\((?:expr = )?"(.*?)"\)\]\s*async fn (.*?)\(.*?\{([\s\S]*?)\n\}"#).unwrap();

    for cap in re.captures_iter(&content) {
        let step_text = cap.get(2).map_or("", |m| m.as_str()).to_lowercase();
        let fn_name = cap.get(3).map_or("", |m| m.as_str());
        let body = cap.get(4).map_or("", |m| m.as_str());

        let is_ui_intent = UI_KEYWORDS.iter().any(|k| step_text.contains(k));
        if is_ui_intent {
            let has_api_call = API_KEYWORDS.iter().any(|k| body.contains(k));
            if has_api_call {
                let has_ui_verification = UI_VERIFICATION_PATTERNS.iter().any(|p| body.contains(p));
                if !has_ui_verification {
                    findings.append(&mut vec![Finding {
                        file: path.to_string_lossy().to_string(),
                        fn_name: fn_name.to_string(),
                        step_text: step_text.to_string(),
                        reason: "UI-named step hits API without DOM/HTML verification (e.g., body.contains('id=') is missing)".to_string(),
                    }]);
                }
            }
        }
    }

    findings
}

fn main() {
    let steps_dir = "tests/steps";
    let mut all_findings = Vec::new();

    if !Path::new(steps_dir).exists() {
        eprintln!(
            "Error: Directory {} not found. Run from project root.",
            steps_dir
        );
        std::process::exit(1);
    }

    for entry in WalkDir::new(steps_dir).into_iter().filter_map(|e| e.ok()) {
        if entry.path().is_file()
            && entry.path().extension().is_some_and(|ext| ext == "rs")
            && entry.file_name() != "mod.rs"
        {
            all_findings.extend(audit_file(entry.path()));
        }
    }

    if !all_findings.is_empty() {
        println!("\n[!] ANTI-MASQUERADING AUDIT FAILED");
        println!("====================================");
        for finding in all_findings {
            println!("[*] File: {}", finding.file);
            println!("   Func: {}", finding.fn_name);
            println!("   Step: \"{}\"", finding.step_text);
            println!("   Note: {}", finding.reason);
            println!("{}", "-".repeat(40));
        }
        std::process::exit(1);
    } else {
        println!("[+] ANTI-MASQUERADING AUDIT PASSED: All UI steps verified.");
        std::process::exit(0);
    }
}
