import subprocess
import time
import os
import shutil
import sys
import json
import urllib.request
import urllib.error

# Configuration
TEST_SOURCE = "Playground/test/TestCase1"
INPUT_DIR = "Playground/input"
OUTPUT_DIR = "Playground/output"
STAGING_DIR = "Playground/staging"
CONTAINER_NAME = "manyfold-processor"
API_URL = "http://localhost:6767/api/settings"
LOG_FILE = "TestCases/TestCase4.log"

def log(msg):
    timestamp = time.strftime("%Y-%m-%d %H:%M:%S")
    formatted = f"[{timestamp}] {msg}"
    print(formatted)
    with open(LOG_FILE, "a") as f:
        f.write(formatted + "\n")

def run_command(cmd):
    subprocess.run(cmd, shell=True, check=True, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)

def clean_IO():
    log("   [Clean] Clearing Input/Output/Staging...")
    run_command(f"rm -rf \"{INPUT_DIR}\"/* \"{OUTPUT_DIR}\"/* \"{STAGING_DIR}\"/*")

def set_auto_process(enabled: bool):
    data = json.dumps({"auto_process": enabled}).encode("utf-8")
    req = urllib.request.Request(API_URL, data=data, headers={'Content-Type': 'application/json'})
    try:
        with urllib.request.urlopen(req) as response:
            if response.status == 200:
                log(f"✅ [API] Auto-Process set to {enabled}")
                return True
            else:
                log(f"❌ [API] Failed to set Auto-Process: {response.status}")
                return False
    except Exception as e:
        log(f"❌ [API] Connection Error: {e}")
        return False

def get_auto_process_status():
    try:
        req = urllib.request.Request(API_URL.replace("/settings", "/settings"), method="POST") 
        # Wait, the endpoint is POST /api/settings to set, but how to GET? 
        # The GET / endpoint returns HTML but renders state.
        # Actually web/app.py update_settings returns {config}. 
        # So we can just set it to what we want and rely on return.
        # But to READ, we can use the "Fake Update" technique or just trust the setter return?
        # Let's verify by setting it again if needed.
        return True # Placeholder as we can't easily GET JSON config without parsing HTML or adding endpoint
    except:
        return False

def ensure_auto_process(target_state: bool):
    for i in range(5):
        if set_auto_process(target_state):
             # Verify consistency if possible, or just trust (since we claimed success)
             return True
        time.sleep(1)
    return False

def check_staging():
    # Verify files are in Staging and NOT in Output
    log("--- Verifying Queue (Staging) ---")
    
    # Check Staging
    files = os.listdir(STAGING_DIR)
    # Ignore dotfiles
    files = [f for f in files if not f.startswith('.')]
    
    if len(files) >= 2:
        log(f"✅ [Pass] Files found in Staging: {len(files)}")
    else:
        log(f"❌ [Fail] Staging empty or missing files. Found: {files}")
        return False
        
    # Check Output (Should be empty)
    if os.path.exists(OUTPUT_DIR):
        out_files = os.listdir(OUTPUT_DIR)
        if not out_files:
             log("✅ [Pass] Output directory is empty (Process held).")
        else:
             # Folder might exist if created earlier, but should be empty of target
             log(f"⚠️ [Check] Output files verified: {out_files}")
             if "sophia-35mm-sophia" in out_files:
                 log("❌ [Fail] Output processed despite toggle OFF!")
                 return False
                 
    return True

def run_test_4a():
    log("\n=== Scenario 4a: Auto-Process Toggle (Batch) ===")
    clean_IO()
    
    # 3. Disable Auto-Process
    log("   [Action] Disabling Auto-Process...")
    if not set_auto_process(False):
        sys.exit(1)
        
    # 4. Input Files
    log("   [Action] Copying files to Input...")
    run_command(f"cp \"{TEST_SOURCE}\"/* \"{INPUT_DIR}/\"")
    
    # 5. Wait for Staging move
    log("   [Wait] Waiting for Watchdog to move to Staging (10s)...")
    time.sleep(10)
    
    # 6. Verify Queue
    if not check_staging():
        log("🚫 CASE 4a FAILED: Queue verification failed.")
        return False
        
    # 7. Enable Auto-Process
    log("   [Action] Enabling Auto-Process...")
    if not set_auto_process(True):
        sys.exit(1)
    
    log("   [Wait] Waiting for processing (600s max)...")
    success = False
    for i in range(120): # 600s
        time.sleep(5)
        expected_folder = os.path.join(OUTPUT_DIR, "sophia-35mm-sophia")
        if os.path.exists(expected_folder):
             if os.path.exists(os.path.join(expected_folder, "datapackage.json")):
                 success = True
                 break
    if success:
        log("✅ [Pass] 4a: Output processed after toggle ON.")
        return True
    else:
        log("❌ [Fail] 4a: Output not processed (Timeout).")
        return False

def run_test_4b():
    log("\n=== Scenario 4b: Late Sibling (Staging Isolation) ===")
    clean_IO()
    
    # Ensure Auto-Process is ON (Retry logic to handle previous Auto-Off race)
    log("   [Setup] Ensuring Auto-Process is ON...")
    if not ensure_auto_process(True):
        log("❌ [Setup] Failed to force Auto-Process ON.")
        return False
    
    # Validation: Wait 2s to make sure it sticks (in case background Watchdog flips it)
    time.sleep(2)
    # Re-apply to be paranoid
    ensure_auto_process(True)
    
    # 1. Copy STL
    log("   [Action] Copying STL (Primary)...")
    stl_source = os.path.join(TEST_SOURCE, "sophia-35mm-sophia.stl")
    shutil.copy(stl_source, INPUT_DIR)
    
    # 2. Wait for it to start (15s to get past debounce + start)
    log("   [Wait] Waiting 15s for STL to enter processing...")
    time.sleep(15)
    
    # 3. Copy JPG
    log("   [Action] Copying JPG (Late Sibling)...")
    img_source = os.path.join(TEST_SOURCE, "720X720-sophia-new.jpg")
    shutil.copy(img_source, INPUT_DIR)
    
    # 4. Verification 1: JPG should remain in Input (Lock active)
    log("   [Verify] Checking if JPG is locked in Input...")
    time.sleep(5) # Give it chance to move if lock is broken
    
    if os.path.exists(os.path.join(INPUT_DIR, "720X720-sophia-new.jpg")):
        log("✅ [Pass] 4b: JPG remained in Input (Staging Locked).")
    else:
        log("❌ [Fail] 4b: JPG moved to Staging prematurely! Lock failed.")
        return False
        
    # 5. Wait for STL finish
    log("   [Wait] Waiting for STL completion (and Staging clear)...")
    stl_done = False
    for i in range(120):
        time.sleep(5)
        expected_stl_out = os.path.join(OUTPUT_DIR, "sophia-35mm-sophia")
        # Check if STL output exists
        if os.path.exists(os.path.join(expected_stl_out, "datapackage.json")):
             # Strict Check: Ensure WebP is NOT here (orphaned logic)
             files_in_stl = os.listdir(expected_stl_out)
             if any(f.endswith(".webp") for f in files_in_stl):
                 log("❌ [Fail] 4b: Strict Check Failed. WebP found in STL output folder (Should be orphaned).")
                 return False
             else:
                 stl_done = True
                 log("✅ [Pass] 4b: STL processed (Isolated).")
                 break
    
    if not stl_done:
        log("❌ [Fail] 4b: STL timeout.")
        return False
        
    # 6. Wait for JPG finish (it should move now that staging is empty)
    log("   [Wait] Waiting for JPG to process (auto-move)...")
    jpg_done = False
    for i in range(60): # 5 mins max
        time.sleep(5)
        # The filename is 720X720-sophia-new.jpg -> Output folder '720x720-sophia-new' (slugified)
        orphan_folder = os.path.join(OUTPUT_DIR, "720x720-sophia-new")
        if os.path.exists(orphan_folder):
             # Success Criterion: Orphan should NOT have a datapackage.json
             if not os.path.exists(os.path.join(orphan_folder, "datapackage.json")):
                 jpg_done = True
                 log("✅ [Pass] 4b: JPG processed (Orphaned, No JSON).")
                 break
             else:
                 log("❌ [Fail] 4b: Orphan folder contains datapackage.json (Should be skipped).")
                 return False
             
    if jpg_done:
        return True
    else:
         log("❌ [Fail] 4b: JPG did not process.")
         return False

def main():
    # Reset log file
    with open(LOG_FILE, "w") as f:
        f.write(f"=== {LOG_FILE} ===\n")

    if not os.path.exists(INPUT_DIR): os.makedirs(INPUT_DIR)
    
    log("=== Running TestCase 4: Combo ===")
    
    # 1. Start Docker
    log("   [Setup] ensuring Docker is running...")
    run_command("sudo docker compose up -d")
    time.sleep(5) # Warmup
    
    passes = 0
    if run_test_4a(): passes += 1
    if run_test_4b(): passes += 1
    
    log(f"\nSummary: {passes}/2 Scenarios Passed")
    if passes == 2:
        log("🎉 TestCase 4 (a+b) SUCCESS")
    else:
        log("🚫 TestCase 4 FAILED")
        sys.exit(1)

if __name__ == "__main__":
    main()
