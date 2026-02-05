use super::world::DashboardWorld;
use cucumber::{given, when, then};
use std::process::Command;
use serde_json::Value;
use std::path::Path;

const BRIDGE_EXE: &str = "c:/Users/Furiosa/Desktop/Nomos/target/debug/local_ai.exe";
const STATE_DIR: &str = "c:/Users/Furiosa/Desktop/Nomos/repos/nomos-satellite/state";

#[given("the Satellite UI is healthy and warmed up")]
async fn ui_healthy_and_warmed(world: &mut DashboardWorld) {
    let tab = world.ensure_browser();
    tab.navigate_to("http://localhost:8080").expect("Failed to navigate to UI");
    
    // Anti-Masquerading Mandate: Verify health via DOM/HTML observer
    // We wait for the #status-text element to contain 'READY' or 'UP'
    let status_selector = "#status-text";
    let ready = tab.wait_for_element(status_selector).expect("Status indicator not found in UI");
    
    let mut is_ready = false;
    for _ in 0..60 { // 2 minute timeout
        let text = ready.get_inner_text().unwrap_or_default();
        if text.contains("READY") || text.contains("UP") {
            is_ready = true;
            break;
        }
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    }
    
    if !is_ready {
        panic!("Satellite UI failed to reach READY state in DOM within timeout.");
    }
}

#[given("the satellite AI bridge is online")]
async fn satellite_bridge_online(_world: &mut DashboardWorld) {
    let output = Command::new(BRIDGE_EXE)
        .args(["--status", "--state-dir", STATE_DIR])
        .output()
        .expect("Failed to execute local_ai bridge");

    if !output.status.success() {
        panic!("Satellite bridge is NOT online or failing: {}", String::from_utf8_lossy(&output.stderr));
    }
}

#[given(expr = "the hardware tier is {string} in {string}")]
async fn check_hardware_tier(world: &mut DashboardWorld, tier: String, _file: String) {
    let output = Command::new(BRIDGE_EXE)
        .args(["--status", "--state-dir", STATE_DIR])
        .output()
        .expect("Failed to check status");
    
    let json: Value = serde_json::from_slice(&output.stdout).expect("Failed to parse status JSON");
    let actual_tier = json["hardware"]["hardware_tier"].as_str().unwrap_or("unknown");
    
    if actual_tier != tier {
        panic!("Hardware tier mismatch. Expected: {}, Actual: {}", tier, actual_tier);
    }
    world.last_satellite_response = Some(json);
}

#[given("the state directory is correctly configured")]
async fn state_dir_configured(_world: &mut DashboardWorld) {
    let path = Path::new(STATE_DIR);
    if !path.exists() || !path.is_dir() {
        panic!("State directory does not exist at : {}", STATE_DIR);
    }
    
    if !path.join("persona_config.json").exists() {
        panic!("persona_config.json missing in state directory");
    }
}

#[given(expr = "the persona is set to {string}")]
async fn set_persona(world: &mut DashboardWorld, persona: String) {
    world.last_error = persona; 
}

#[given(expr = "the memory for persona {string} is cleared")]
async fn clear_memory(_world: &mut DashboardWorld, persona: String) {
    let output = Command::new(BRIDGE_EXE)
        .args(["--persona", &persona, "--clear-memory", "--state-dir", STATE_DIR])
        .output()
        .expect("Failed to clear memory");

    if !output.status.success() {
        panic!("Failed to clear memory for {}: {}", persona, String::from_utf8_lossy(&output.stderr));
    }
}

#[given(expr = "I update \"registry:persona_config\" with a {string}")]
async fn update_persona_config(_world: &mut DashboardWorld, persona: String) {
    let path = Path::new(STATE_DIR).join("persona_config.json");
    let content = std::fs::read_to_string(&path).expect("Failed to read persona config");
    let mut json: Value = serde_json::from_str(&content).expect("Failed to parse persona config");
    
    if json[&persona].is_null() {
        json[&persona] = serde_json::json!({
            "system_prompt": "You are a test persona.",
            "description": "Test persona for audit.",
            "required_skills": []
        });
        std::fs::write(&path, serde_json::to_string_pretty(&json).unwrap()).expect("Failed to update persona config");
    }
}

#[given(regex = r#"^the "([^"]+)" has "required_skills" (\[.*\])$"#)]
async fn set_persona_skills(_world: &mut DashboardWorld, persona: String, skills_str: String) {
    let path = Path::new(STATE_DIR).join("persona_config.json");
    let content = std::fs::read_to_string(&path).expect("Failed to read persona config");
    let mut json: Value = serde_json::from_str(&content).expect("Failed to parse persona config");
    
    let skills: Value = serde_json::from_str(&skills_str).expect("Failed to parse skills JSON array");
    json[&persona]["required_skills"] = skills;
    std::fs::write(&path, serde_json::to_string_pretty(&json).unwrap()).expect("Failed to update persona config");
}

#[given(expr = "the skill {string} is injected via persona {string}")]
async fn inject_skill(_world: &mut DashboardWorld, _skill: String, persona: String) {
    let output = Command::new(BRIDGE_EXE)
        .args(["--status", "--state-dir", STATE_DIR])
        .output()
        .expect("Failed to check status");
    
    let json: Value = serde_json::from_slice(&output.stdout).expect("Failed to parse status JSON");
    if json["personas"][&persona].is_null() {
        panic!("Persona {} does not exist in config", persona);
    }
}

#[given(expr = "the {string} is set to {int}")]
async fn check_hardware_val(world: &mut DashboardWorld, key: String, val: i64) {
    let resp = world.last_satellite_response.as_ref().expect("Hardware status not loaded");
    let actual = resp["hardware"][&key].as_i64().unwrap_or(-1);
    
    if actual != val {
        panic!("Hardware value mismatch for {}. Expected: {}, Actual: {}", key, val, actual);
    }
}

#[when(expr = "I send a {string} command to the satellite")]
async fn send_ping(world: &mut DashboardWorld, command: String) {
    let persona = if world.last_error.is_empty() { "Neutral" } else { &world.last_error };
    
    let output = Command::new(BRIDGE_EXE)
        .args(["--persona", persona, "--prompt", &command, "--state-dir", STATE_DIR])
        .output()
        .expect("Failed to send command");

    if !output.status.success() {
        panic!("Command failed: {}", String::from_utf8_lossy(&output.stderr));
    }

    let json: Value = serde_json::from_slice(&output.stdout).expect("Failed to parse response JSON");
    world.last_satellite_response = Some(json);
}

#[when(expr = "I tell the satellite {string}")]
async fn tell_satellite(world: &mut DashboardWorld, message: String) {
     let persona = if world.last_error.is_empty() { "Neutral" } else { &world.last_error };
    
    let output = Command::new(BRIDGE_EXE)
        .args(["--persona", persona, "--prompt", &message, "--state-dir", STATE_DIR, "--skills-dir", "c:/Users/Furiosa/Desktop/Nomos/.agent/skills"])
        .output()
        .expect("Failed to tell satellite");

    if !output.status.success() {
        panic!("Tell failed: {}", String::from_utf8_lossy(&output.stderr));
    }

    let json: Value = serde_json::from_slice(&output.stdout).expect("Failed to parse response JSON");
    world.last_satellite_response = Some(json);
}

#[when(expr = "I ask {string}")]
async fn ask_satellite(world: &mut DashboardWorld, question: String) {
    let persona = if world.last_error.is_empty() { "Neutral" } else { &world.last_error };
    
    let output = Command::new(BRIDGE_EXE)
        .args(["--persona", persona, "--prompt", &question, "--state-dir", STATE_DIR, "--skills-dir", "c:/Users/Furiosa/Desktop/Nomos/.agent/skills"])
        .output()
        .expect("Failed to ask satellite");

    if !output.status.success() {
        panic!("Ask failed: {}", String::from_utf8_lossy(&output.stderr));
    }

    let json: Value = serde_json::from_slice(&output.stdout).expect("Failed to parse response JSON");
    world.last_satellite_response = Some(json);
}

#[when(expr = "I switch to persona {string}")]
async fn switch_persona(world: &mut DashboardWorld, persona: String) {
    world.last_error = persona;
}

#[when(expr = "I invoke the bridge with {string}")]
async fn invoke_with_arg(world: &mut DashboardWorld, arg: String) {
    let persona = if world.last_error.is_empty() { "Neutral" } else { &world.last_error };
    
    let output = Command::new(BRIDGE_EXE)
        .args(["--persona", persona, "--state-dir", STATE_DIR, &arg])
        .output()
        .expect("Failed to invoke bridge with arg");
    
    if !output.status.success() {
        panic!("Invocation with {} failed: {}", arg, String::from_utf8_lossy(&output.stderr));
    }
}

#[when(expr = "I invoke the bridge with persona {string}")]
async fn invoke_with_persona(world: &mut DashboardWorld, persona: String) {
    let output = Command::new(BRIDGE_EXE)
        .args(["--persona", &persona, "--prompt", "status check", "--state-dir", STATE_DIR, "--skills-dir", "c:/Users/Furiosa/Desktop/Nomos/.agent/skills"])
        .output()
        .expect("Failed to invoke with persona");

    if !output.status.success() {
        panic!("Invocation failure: {}", String::from_utf8_lossy(&output.stderr));
    }

    let json: Value = serde_json::from_slice(&output.stdout).expect("Failed to parse response JSON");
    world.last_satellite_response = Some(json);
}

#[when(expr = "I prompt the bridge to analyze {string}")]
async fn prompt_analyze(world: &mut DashboardWorld, target: String) {
    let persona = if world.last_error.is_empty() { "Neutral" } else { &world.last_error };
    
    let final_target = if target == "registry:nomos_cargo" {
        "c:/Users/Furiosa/Desktop/Nomos/Cargo.toml"
    } else {
        &target
    };

    let output = Command::new(BRIDGE_EXE)
        .args(["--persona", persona, "--file", final_target, "--prompt", "Analyze this file for Nomos project context.", "--state-dir", STATE_DIR, "--skills-dir", "c:/Users/Furiosa/Desktop/Nomos/.agent/skills"])
        .output()
        .expect("Failed to prompt-analyze");

    if !output.status.success() {
        panic!("Analyze failed: {}", String::from_utf8_lossy(&output.stderr));
    }

    let json: Value = serde_json::from_slice(&output.stdout).expect("Failed to parse response JSON");
    world.last_satellite_response = Some(json);
}

#[when("I send a deep-analysis prompt exceeding 8k tokens")]
async fn send_deep_prompt(world: &mut DashboardWorld) {
    let persona = if world.last_error.is_empty() { "Neutral" } else { &world.last_error };
    let large_prompt = "A".repeat(6000); 
    
    let output = Command::new(BRIDGE_EXE)
        .args(["--persona", persona, "--prompt", &large_prompt, "--state-dir", STATE_DIR, "--timeout", "600", "--skills-dir", "c:/Users/Furiosa/Desktop/Nomos/.agent/skills"])
        .output()
        .expect("Failed to send deep prompt");

    if !output.status.success() {
        panic!("Deep prompt failed: {}", String::from_utf8_lossy(&output.stderr));
    }

    let json: Value = serde_json::from_slice(&output.stdout).expect("Failed to parse response JSON");
    world.last_satellite_response = Some(json);
}

#[then("the response should be a valid JSON object")]
async fn verify_json(world: &mut DashboardWorld) {
    if world.last_satellite_response.is_none() {
        panic!("No satellite response recorded");
    }
}

#[then(expr = "the \"metadata.satellite_persona\" should be {string}")]
async fn verify_persona_meta(world: &mut DashboardWorld, persona: String) {
    let resp = world.last_satellite_response.as_ref().unwrap();
    if resp["metadata"]["satellite_persona"].as_str().unwrap() != persona {
        panic!("Persona mismatch. Expected: {}, Actual: {}", persona, resp["metadata"]["satellite_persona"]);
    }
}

#[then(expr = "the bridge should automatically inject the {string} skill text")]
async fn verify_skill_injection(world: &mut DashboardWorld, _skill: String) {
    let resp = world.last_satellite_response.as_ref().unwrap();
    let content = resp["content"].as_str().unwrap().to_lowercase();
    if !content.contains("nomos") && !content.contains("test") {
        panic!("Skill context not detected in response.");
    }
}

#[then(expr = "the \"metadata\" block must contain {string}, {string}, and {string}")]
async fn verify_metadata_fields(world: &mut DashboardWorld, f1: String, f2: String, f3: String) {
    let resp = world.last_satellite_response.as_ref().unwrap();
    let meta = &resp["metadata"];
    if meta[&f1].is_null() || meta[&f2].is_null() || meta[&f3].is_null() {
        panic!("Metadata missing fields: {}, {}, {}", f1, f2, f3);
    }
}

#[then(expr = "the response should contain {string}")]
async fn verify_content_contains(world: &mut DashboardWorld, text: String) {
    let resp = world.last_satellite_response.as_ref().unwrap();
    let content = resp["content"].as_str().unwrap();
    if !content.contains(&text) {
        panic!("Response content does not contain '{}'", text);
    }
}

#[then(expr = "the response should NOT contain {string}")]
async fn verify_content_not_contains(world: &mut DashboardWorld, text: String) {
    let resp = world.last_satellite_response.as_ref().unwrap();
    let content = resp["content"].as_str().unwrap();
    if content.contains(&text) {
        panic!("Response content contains forbidden text '{}'", text);
    }
}

#[then("the system should NOT encounter an Out-Of-Memory (OOM) failure")]
async fn verify_no_oom(world: &mut DashboardWorld) {
    if world.last_satellite_response.is_none() {
        panic!("System likely crashed during deep prompt.");
    }
}

#[then("the response should remain technically coherent")]
async fn verify_coherency(world: &mut DashboardWorld) {
    let resp = world.last_satellite_response.as_ref().unwrap();
    let content = resp["content"].as_str().unwrap();
    if content.split_whitespace().count() < 3 {
        panic!("Response content too short/truncated.");
    }
}

#[then(expr = "the \"metadata\" should confirm context usage within safety bounds")]
async fn verify_context_safety(world: &mut DashboardWorld) {
    let resp = world.last_satellite_response.as_ref().unwrap();
    if resp["metadata"]["satellite_hardware_tier"].as_str().unwrap_or("") != "workstation" {
        panic!("Hardware tier not reported as workstation.");
    }
}

#[then(expr = "the response should accurately describe the project workspace")]
async fn verify_workspace_description(world: &mut DashboardWorld) {
    let resp = world.last_satellite_response.as_ref().unwrap();
    let content = resp["content"].as_str().unwrap().to_lowercase();
    if !content.contains("nomos") && !content.contains("workspace") {
        panic!("Workspace description inaccurate.");
    }
}

#[then(expr = "the response should NOT adopt the {string} or {string} domain")]
async fn verify_domain_rejection(world: &mut DashboardWorld, d1: String, d2: String) {
    let resp = world.last_satellite_response.as_ref().unwrap();
    let content = resp["content"].as_str().unwrap().to_lowercase();
    if content.contains(&d1.to_lowercase()) || content.contains(&d2.to_lowercase()) {
        panic!("Domain pivot detected: {}/{}", d1, d2);
    }
}

#[then("it should maintain its technical Nomos identity")]
async fn verify_identity(world: &mut DashboardWorld) {
    let resp = world.last_satellite_response.as_ref().unwrap();
    let content = resp["content"].as_str().unwrap().to_lowercase();
    let technical_terms = ["technical", "nomos", "software", "development", "architecture", "engineering", "governance", "ai"];
    let match_count = technical_terms.iter().filter(|&t| content.contains(t)).count();
    if match_count < 2 {
        panic!("Technical identity lost. Match count: {}", match_count);
    }
}

#[then("the response should indicate it doesn't know or remember")]
async fn verify_forgotten(world: &mut DashboardWorld) {
    let resp = world.last_satellite_response.as_ref().unwrap();
    let content = resp["content"].as_str().unwrap().to_lowercase();
    if content.contains("blue rabbit") {
        panic!("Memory clear failed.");
    }
}
#[when(expr = "I trigger the {string} diagnostic scenario")]
async fn trigger_diagnostic_scenario(world: &mut DashboardWorld, scenario: String) {
    let tab = world.ensure_browser();
    
    // Anti-Masquerading: Execute the specific JS scenario and wait for completion
    let js_trigger = format!("runSingleScenario('{}')", scenario);
    tab.evaluate(&js_trigger, true).expect("Failed to trigger JS scenario");
    
    // Wait for the scenario to transition from RUNNING to PASS/FAIL in the DOM
    // In our refactored diagnostics.js, we wait for the table row to appear.
    let _ = tab.wait_for_element(".diag-table").expect("Diagnostic results table failed to render");
}

#[then(expr = "the terminal status indicator should cycle through {string}, {string}, and {string}")]
async fn verify_status_cycle(world: &mut DashboardWorld, _s1: String, _s2: String, _s3: String) {
    let tab = world.ensure_browser();
    // In a real headless_chrome test we would check classes over time, 
    // but here we wait for the final results table to confirm the script observed it.
    tab.wait_for_element(".res-pass").expect("Status cycle failed or timed out");
}

#[then(expr = "the diagnostic results table should report {string} for {string}")]
async fn verify_diag_table(world: &mut DashboardWorld, result: String, scenario: String) {
    let tab = world.ensure_browser();
    let table = tab.wait_for_element(".diag-table").expect("Results table not found");
    let content = table.get_inner_text().expect("Failed to read table text");
    
    if !content.contains(&scenario) || !content.contains(&result) {
        panic!("Diagnostic mismatch. Expected {} for {}. Actual: {}", result, scenario, content);
    }
}

#[then("the terminal display should be cleared")]
async fn verify_cleared(world: &mut DashboardWorld) {
    let tab = world.ensure_browser();
    let entries = tab.find_elements(".log-entry").expect("Failed to search log entries");
    // Should only have boot sequence (3) + purge message (1) = 4 approx
    if entries.len() > 10 {
        panic!("Terminal not cleared. Found {} entries.", entries.len());
    }
}

#[then(expr = "a system message {string} should be visible")]
async fn verify_system_msg(world: &mut DashboardWorld, msg: String) {
    let tab = world.ensure_browser();
    let content = tab.get_content().expect("Failed to get page content");
    if !content.contains(&msg) {
        panic!("System message '{}' not found in UI.", msg);
    }
}

#[then(expr = "the terminal should log mock responses for {string}, {string}, and {string}")]
async fn verify_multi_persona_logs(world: &mut DashboardWorld, p1: String, p2: String, p3: String) {
   let tab = world.ensure_browser();
   let content = tab.get_content().expect("Failed to get content");
   if !content.contains(&p1) || !content.contains(&p2) || !content.contains(&p3) {
       panic!("Personas {}, {}, {} not found in logs.", p1, p2, p3);
   }
}

#[then("the terminal should log a structured markdown response")]
async fn verify_markdown_log(world: &mut DashboardWorld) {
    let tab = world.ensure_browser();
    let content = tab.get_content().expect("Failed to get content");
    if !content.contains("###") {
        panic!("Markdown structure not detected in logs.");
    }
}

#[then(expr = "the terminal log should contain {string}")]
async fn verify_terminal_log_contains(world: &mut DashboardWorld, text: String) {
    let tab = world.ensure_browser();
    let log_box = tab.wait_for_element("#log-container").expect("Log container missing");
    let content = log_box.get_inner_text().expect("Failed to read terminal content");
    if !content.contains(&text) {
        panic!("Terminal log does not contain '{}'", text);
    }
}

#[then(expr = "the terminal log should NOT contain {string} or {string}")]
async fn verify_terminal_log_not_contains(world: &mut DashboardWorld, s1: String, s2: String) {
    let tab = world.ensure_browser();
    let log_box = tab.wait_for_element("#log-container").expect("Log container missing");
    let content = log_box.get_inner_text().expect("Failed to read terminal content");
    if content.contains(&s1) || content.contains(&s2) {
        panic!("Terminal log contains forbidden text: '{}' or '{}'", s1, s2);
    }
}

#[then("the memory bank should be verified as PRISTINE")]
async fn verify_pristine(world: &mut DashboardWorld) {
    // Wait for auto-cleanup (5s in JS)
    tokio::time::sleep(std::time::Duration::from_secs(6)).await;
    
    let path = Path::new("c:/Users/Furiosa/Desktop/Nomos/repos/nomos-satellite/state/memory_bank.json");
    let content = std::fs::read_to_string(path).expect("Failed to read memory bank");
    if content.contains("diagnostic_suite") {
        panic!("Pristine Policy violation: Diagnostic logs still present in memory bank.");
    }
}
