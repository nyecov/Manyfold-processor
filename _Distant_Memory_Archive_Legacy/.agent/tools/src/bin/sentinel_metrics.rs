//! Phase 5: Metrics Sentinel
//! Reads .agent/metrics/audit_log.csv and reports "Context Cost" trends.
//!
//! Output: Avg Bytes per Audit.


use std::path::Path;

struct Metric {
    _timestamp: u64,
    bytes: u64,
    exit_code: i32,
}

fn main() {
    println!("[AUDIT] Sentinel: Metrics");
    
    let csv_path = ".agent/metrics/audit_log.csv";
    
    if !Path::new(csv_path).exists() {
        println!("[WW] No metrics log found yet.");
        return;
    }
    
    let content = match std::fs::read_to_string(csv_path) {
        Ok(c) => c,
        Err(_) => return,
    };
    
    let mut metrics = Vec::new();
    
    for line in content.lines() {
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() >= 3 {
             let _timestamp = parts[0].parse().unwrap_or(0);
             let bytes = parts[1].parse().unwrap_or(0);
             let exit = parts[2].parse().unwrap_or(0);
             metrics.push(Metric { _timestamp, bytes, exit_code: exit });
        }
    }
    
    // Analyze successful runs only
    let successful: Vec<&Metric> = metrics.iter().filter(|m| m.exit_code == 0).collect();
    if successful.is_empty() {
        println!("[WW] No successful audit runs recorded.");
        return;
    }
    
    let count = successful.len();
    let total_bytes: u64 = successful.iter().map(|m| m.bytes).sum();
    let avg = total_bytes as f64 / count as f64;
    
    // Baseline: 2000 bytes (approx 500 tokens)
    if avg > 2500.0 {
        println!("[WW] Avg Context Cost: {:.0} bytes (High > 2500)", avg);
    } else {
        println!("[OK] Avg Context Cost: {:.0} bytes (Optimal)", avg);
    }
}
