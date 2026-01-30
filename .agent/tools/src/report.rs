use std::process::exit;

pub enum AuditStatus {
    Pass,
    Fail,
    Warn,
}

pub struct AuditResult {
    pub tool_name: String,
    pub status: AuditStatus,
    pub findings: Vec<String>,
}

impl AuditResult {
    pub fn new(name: &str) -> Self {
        Self {
            tool_name: name.to_string(),
            status: AuditStatus::Pass,
            findings: Vec::new(),
        }
    }

    pub fn fail(&mut self, message: &str) {
        self.status = AuditStatus::Fail;
        self.findings.push(message.to_string());
    }

    pub fn warn(&mut self, message: &str) {
        // Only downgrade Pass -> Warn, never Fail -> Warn
        if let AuditStatus::Pass = self.status {
            self.status = AuditStatus::Warn;
        }
        self.findings.push(message.to_string());
    }
    
    pub fn print_and_exit(&self) -> ! {
        println!("=== {} Report ===\n", self.tool_name);
        
        if self.findings.is_empty() {
             // No findings
        } else {
             println!("Findings:");
             for finding in &self.findings {
                 println!("- {}", finding);
             }
        }
        
        match self.status {
            AuditStatus::Pass => {
                println!("\n✅ [PASS] No issues found.");
                exit(0);
            }
            AuditStatus::Warn => {
                println!("\n⚠️  [WARN] Issues found but execution can proceed.");
                exit(0);
            }
            AuditStatus::Fail => {
                println!("\n❌ [FAIL] Critical issues found.");
                exit(1);
            }
        }
    }
}
