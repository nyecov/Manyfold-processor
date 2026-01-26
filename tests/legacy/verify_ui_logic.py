import subprocess
import time
import os
import shutil
import sys
import json
import urllib.request
import urllib.error

API_BASE = "http://localhost:6767/api"

def log(msg):
    print(f"[UI-TEST] {msg}")

def call_api(path, method="POST"):
    url = f"{API_BASE}{path}"
    req = urllib.request.Request(url, method=method)
    try:
        with urllib.request.urlopen(req) as response:
            return response.status, json.loads(response.read().decode())
    except urllib.error.HTTPError as e:
        return e.code, None
    except Exception as e:
        print(e)
        return 0, None

def main():
    log("Verifying UI API Endpoints...")
    
    # 1. Test Process All (Empty)
    status, data = call_api("/process/all", "POST")
    if status == 200:
        log("✅ POST /process/all returned 200 OK")
    else:
        log(f"❌ POST /process/all failed: {status}")
        sys.exit(1)

    # 2. Test Process Single File (Mock file)
    # create dummy in staging
    with open("Playground/staging/test_dummy.stl", "w") as f: f.write("dummy")
    
    status, data = call_api("/process/test_dummy.stl", "POST")
    if status == 200 and data.get("status") == "success":
        log("✅ POST /process/test_dummy.stl returned Success")
    else:
        log(f"❌ POST /process/test_dummy.stl failed: {status} {data}")

    # Cleanup
    if os.path.exists("Playground/staging/test_dummy.stl"):
        os.remove("Playground/staging/test_dummy.stl")

if __name__ == "__main__":
    main()
