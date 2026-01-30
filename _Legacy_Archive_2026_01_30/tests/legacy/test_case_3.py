import subprocess
import time
import os
import shutil
import sys

# Configuration
TEST_SOURCE = "Playground/test/TestCase1"
INPUT_DIR = "Playground/input"
OUTPUT_DIR = "Playground/output"
CONTAINER_NAME = "manyfold-processor"
STAGING_DIR = "Playground/staging"
LOG_FILE = "TestCases/TestCase3.log"

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
    else:
         log("❌ [Fail] datapackage.json missing.")
         all_passed = False

    # Criterion 3: 3MF exists
    has_3mf = any(f.endswith(".3mf") for f in files)
    if has_3mf:
        log("✅ [Pass] sophia-35mm-sophia.3mf exists.")
    else:
        log("❌ [Fail] 3MF file missing.")
        all_passed = False
        
    # Criterion 4: WebP exists 
    has_webp = any(f.endswith(".webp") for f in files)
    if has_webp:
        log("✅ [Pass] 720X720-sophia-new.webp exists.")
    else:
        log("❌ [Fail] WebP file missing.")
        all_passed = False

    if all_passed:
        log(f"🎉 {case_name} SUCCESS: All criteria met.")
        return True
    else:
        log(f"🚫 {case_name} FAILED: One or more criteria failed.")
        return False

def run_test_3a():
    log("\n=== Running TestCase 3A: Simultaneous Copy ===")
    clean_IO()
    
    log("   [Action] Copying all files at once...")
    subprocess.run(f"cp \"{TEST_SOURCE}\"/* \"{INPUT_DIR}/\"", shell=True)
    
    log("   [Wait] Waiting for processing (600s max for conversion)...")
    # Wait loop
    for i in range(120): # 120 * 5s = 600s
        time.sleep(5)
        if os.path.exists(os.path.join(OUTPUT_DIR, "sophia-35mm-sophia")):
            # Wait a bit more for files to finish writing
            time.sleep(5)
            break
            
    return check_output("TestCase 3A")

def run_test_3b():
    log("\n=== Running TestCase 3B: Sequential Copy (Lag) ===")
    clean_IO()
    
    stl_source = os.path.join(TEST_SOURCE, "sophia-35mm-sophia.stl")
    img_source = os.path.join(TEST_SOURCE, "720X720-sophia-new.jpg")
    
    log("   [Action] Copying STL...")
    shutil.copy(stl_source, INPUT_DIR)
    
    log("   [Lag] Waiting 3 seconds...")
    time.sleep(3)
    
    log("   [Action] Copying JPG...")
    shutil.copy(img_source, INPUT_DIR)
    
    log("   [Wait] Waiting for processing (600s max)...")
    for i in range(120):
        time.sleep(5)
        expected_folder = os.path.join(OUTPUT_DIR, "sophia-35mm-sophia")
        if os.path.exists(expected_folder):
            # Check if JSON exists, which implies finish
            if os.path.exists(os.path.join(expected_folder, "datapackage.json")):
                 break
    
    return check_output("TestCase 3B")

def main():
    # Reset log file
    with open(LOG_FILE, "w") as f:
        f.write(f"=== {LOG_FILE} ===\n")

    if not os.path.exists(INPUT_DIR): os.makedirs(INPUT_DIR)
    
    log("Ensure Docker is running...")
    run_command("sudo docker compose up -d")
    time.sleep(5) # Warmup
    
    passes = 0
    if run_test_3a(): passes += 1
    if run_test_3b(): passes += 1
    
    log(f"\nSummary: {passes}/2 Tests Passed")
    if passes != 2:
        sys.exit(1)

if __name__ == "__main__":
    main()
