import json
import time
import urllib.request
import os
import shutil

# Config
BASE_URL = "http://localhost:6767/api"
LOG_FILE = "TestCases/TestCase5.log"
INPUT_DIR = "Playground/input"
TEST_SOURCE = "Playground/test/TestCase1"

def log(msg):
    timestamp = time.strftime("%Y-%m-%d %H:%M:%S")
    formatted = f"[{timestamp}] {msg}"
    print(formatted)
    with open(LOG_FILE, "a") as f:
        f.write(formatted + "\n")

def api_get(endpoint):
    try:
        with urllib.request.urlopen(f"{BASE_URL}/{endpoint}") as response:
            return json.loads(response.read().decode())
    except Exception as e:
        log(f"❌ API GET Error ({endpoint}): {e}")
        return None

def api_post(endpoint, data):
    try:
        payload = json.dumps(data).encode("utf-8")
        req = urllib.request.Request(f"{BASE_URL}/{endpoint}", data=payload, headers={'Content-Type': 'application/json'})
        with urllib.request.urlopen(req) as response:
            return json.loads(response.read().decode())
    except Exception as e:
        log(f"❌ API POST Error ({endpoint}): {e}")
        return None

def main():
    with open(LOG_FILE, "w") as f:
        f.write(f"=== {LOG_FILE} ===\n")

    log("=== TestCase 5: Advanced API Verification ===")

    # 1. Verify Status (Waiting)
    status = api_get("status")
    if status and status.get("state") == "waiting":
        log("✅ [Pass] Initial state is 'waiting'")
    else:
        log(f"❌ [Fail] Unexpected initial state: {status}")

    # 2. Verify Settings GET
    settings = api_get("settings")
    if settings and "auto_process" in settings:
        log(f"✅ [Pass] Settings GET successful: {settings}")
    else:
        log("❌ [Fail] Settings GET failed")

    # 3. Verify Settings POST (Toggle)
    current_auto = settings.get("auto_process")
    new_auto = not current_auto
    updated = api_post("settings", {"auto_process": new_auto})
    if updated and updated.get("auto_process") == new_auto:
        log(f"✅ [Pass] Settings POST successful. Toggle: {new_auto}")
    else:
        log(f"❌ [Fail] Settings POST failed: {updated}")

    # 4. Verify Processing State
    api_post("settings", {"auto_process": True}) # Ensure it's ON
    log("   [Action] Creating small dummy STL to trigger 'processing' state...")
    # Minimal 84-byte binary STL header
    dummy_stl = b'\x00' * 80 + b'\x00' * 4
    with open(os.path.join(INPUT_DIR, "dummy.stl"), "wb") as f:
        f.write(dummy_stl)
    
    # Check status quickly
    found_processing = False
    for i in range(15):
        s = api_get("status")
        if s and s.get("state") == "processing":
            found_processing = True
            log("✅ [Pass] State transitioned to 'processing'")
            break
        time.sleep(1)
    
    if not found_processing:
        log("❌ [Fail] State never transitioned to 'processing' (or was too fast)")

    # 5. Verify Error Reporting
    log("   [Action] Triggering an error (Invalid Zip)...")
    with open(os.path.join(INPUT_DIR, "corrupt.zip"), "w") as f:
        f.write("This is not a zip file")
    
    time.sleep(10) # Wait for handler to fail (Get past debounce)
    errs = api_get("errors")
    if errs and len(errs.get("errors", [])) > 0:
        latest = errs["errors"][-1]
        log(f"✅ [Pass] Error reported: {latest['error']} for file {latest['file']}")
    else:
        log(f"❌ [Fail] Error buffer is empty after failure: {errs}")

    log("=== TestCase 5 FINISHED ===")

if __name__ == "__main__":
    main()
