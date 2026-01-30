import subprocess
import time
import os
import shutil
import sys

# Configuration
TEST_SOURCE = "Playground/test/TestCase1"
INPUT_DIR = "Playground/input"
OUTPUT_DIR = "Playground/output"
STAGING_DIR = "Playground/staging"
CONTAINER_NAME = "manyfold-processor"
LOG_FILE = "TestCases/TestCase1.log"

def log(msg):
    timestamp = time.strftime("%Y-%m-%d %H:%M:%S")
    formatted = f"[{timestamp}] {msg}"
    print(formatted)
    with open(LOG_FILE, "a") as f:
        f.write(formatted + "\n")

def run_command(cmd):
    subprocess.run(cmd, shell=True, check=True, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)

def clean_IO():
    print("   [Clean] Clearing Input/Output/Staging...")
    # Explicitly remove contents, but keep directories
    if os.path.exists(INPUT_DIR):
        for f in os.listdir(INPUT_DIR):
            path = os.path.join(INPUT_DIR, f)
            if os.path.isfile(path) or os.path.islink(path): os.unlink(path)
            elif os.path.isdir(path): shutil.rmtree(path)
            
    if os.path.exists(OUTPUT_DIR):
        shutil.rmtree(OUTPUT_DIR)
        os.makedirs(OUTPUT_DIR)
        
    if os.path.exists(STAGING_DIR):
        for f in os.listdir(STAGING_DIR):
            path = os.path.join(STAGING_DIR, f)
            if os.path.isfile(path) or os.path.islink(path): os.unlink(path)
            elif os.path.isdir(path): shutil.rmtree(path)

def check_output(case_name):
    log(f"--- Verifying Success Criteria for {case_name} ---")
    all_passed = True
    
    # Criterion 1: Output Folder exists
    expected_folder = os.path.join(OUTPUT_DIR, "sophia-35mm-sophia")
    if os.path.exists(expected_folder):
        log("✅ [Pass] Output Folder exists: output/sophia-35mm-sophia/")
    else:
        log("❌ [Fail] Output Folder not found.")
        all_passed = False
        return False # block further checks if folder missing

    files = os.listdir(expected_folder)
    
    # Criterion 2: datapackage.json exists
    if "datapackage.json" in files:
         log("✅ [Pass] datapackage.json exists.")
         # ideally check contents, but existence is the main check here
    else:
         log("❌ [Fail] datapackage.json missing.")
         all_passed = False

    # Criterion 3: 3MF exists (Original STL deleted)
    has_3mf = any(f.endswith(".3mf") for f in files)
    stl_gone = not os.path.exists(os.path.join(INPUT_DIR, "sophia-35mm-sophia.stl"))
    
    if has_3mf:
        log("✅ [Pass] sophia-35mm-sophia.3mf exists.")
    else:
        log("❌ [Fail] 3MF file missing.")
        all_passed = False
        
    if stl_gone:
        log("✅ [Pass] Original STL deleted from Input.")
    else:
        log("❌ [Fail] Original STL still present in Input.")
        all_passed = False

    # Criterion 4: WebP exists (Original JPG deleted)
    has_webp = any(f.endswith(".webp") for f in files)
    jpg_gone = not os.path.exists(os.path.join(INPUT_DIR, "720X720-sophia-new.jpg"))
    
    if has_webp:
        log("✅ [Pass] 720X720-sophia-new.webp exists.")
    else:
        log("❌ [Fail] WebP file missing.")
        all_passed = False
        
    if jpg_gone:
        log("✅ [Pass] Original JPG deleted from Input.")
    else:
        log("❌ [Fail] Original JPG still present in Input.")
        all_passed = False

    if all_passed:
        log(f"🎉 {case_name} SUCCESS: All criteria met.")
        return True
    else:
        log(f"🚫 {case_name} FAILED: One or more criteria failed.")
        return False

def main():
    # Reset log file
    with open(LOG_FILE, "w") as f:
        f.write(f"=== {LOG_FILE} ===\n")

    log("=== Running TestCase 1: Startup Scan ===")
    
    # 1. Stop Docker
    log("   [Setup] Stopping Docker...")
    run_command("sudo docker compose down")
    
    # 2. Clean
    clean_IO()
    
    # 3. Stage Files (Pre-boot)
    log("   [Setup] Pre-staging files...")
    run_command(f"cp \"{TEST_SOURCE}\"/* \"{INPUT_DIR}/\"")
    
    # 4. Start Docker
    log("   [Action] Starting Docker...")
    run_command("sudo docker compose up -d")
    
    # 5. Wait
    log("   [Wait] Waiting for processing (120s max for conversion)...")
    success = False
    for i in range(24): # 120s
        time.sleep(5)
        if os.path.exists(os.path.join(OUTPUT_DIR, "sophia-35mm-sophia")):
            # Found folder, give it a moment to finish writing
            time.sleep(5)
            success = check_output("TestCase 1")
            break
            
    if not success:
        log("❌ TestCase 1 FAILED: Timeout waiting for output.")
        sys.exit(1)

if __name__ == "__main__":
    main()
