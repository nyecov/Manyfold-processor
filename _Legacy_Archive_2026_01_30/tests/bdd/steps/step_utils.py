import os
import sys
import shutil
import time
import requests
import subprocess
import json

# Configuration and Constants
IS_WINDOWS = sys.platform.startswith('win')
SCRIPT_NAME = "manage.ps1" if IS_WINDOWS else "manage.sh"
# Adjust path to find manage script relative to this utils file (tests/bdd/steps/step_utils.py)
# .../tests/bdd/steps/../../.. -> project root
MANAGE_SCRIPT = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..", "..", SCRIPT_NAME))

TEST_SOURCE = os.getenv("TEST_SOURCE", "../test_data")
INPUT_DIR = os.getenv("INPUT_DIR", "../playground/input")
OUTPUT_DIR = os.getenv("OUTPUT_DIR", "../playground/output")
STAGING_DIR = os.getenv("STAGING_DIR", "../playground/staging")
API_URL = os.getenv("API_URL", "http://localhost:6767/api")

# --- Helper Functions ---

def api_call(endpoint, method="GET", data=None):
    """Unified API call helper using requests"""
    url = f"{API_URL}/{endpoint}"
    try:
        if method == "GET":
            resp = requests.get(url, timeout=10)
        elif method == "POST":
            resp = requests.post(url, json=data, timeout=10)
        else:
            return None
        
        if resp.status_code == 200:
            return resp.json()
        return None
    except Exception as e:
        print(f"API Error ({endpoint}): {e}")
        return None

def run_manage_cmd(cmd_list, cwd=None):
    """Executes the manage script with the given command list."""
    if cwd is None:
        cwd = os.getcwd()
        
    cmd = []
    if IS_WINDOWS:
        cmd = ["powershell", "-ExecutionPolicy", "Bypass", "-File", MANAGE_SCRIPT] + cmd_list
    else:
        cmd = [MANAGE_SCRIPT] + cmd_list
        
    return subprocess.run(cmd, check=True, capture_output=True, cwd=cwd, text=True)

def _clean_dir(directory):
    """Helper to safely empty a directory"""
    if not os.path.exists(directory):
        os.makedirs(directory, exist_ok=True)
        return
    
    for f in os.listdir(directory):
        path = os.path.join(directory, f)
        try:
            if os.path.isfile(path) or os.path.islink(path):
                os.unlink(path)
            elif os.path.isdir(path):
                shutil.rmtree(path)
        except Exception as e:
            print(f"Warning: Failed to delete {path}: {e}")

def _create_dummy_stl(dest):
    """Creates a minimal valid 1-triangle STL (134 bytes)"""
    header = b'\x00' * 80
    count = b'\x01\x00\x00\x00' # 1 triangle
    normal = b'\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x80\x3f'
    v1 = b'\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00'
    v2 = b'\x00\x00\x80\x3f\x00\x00\x00\x00\x00\x00\x00\x00'
    v3 = b'\x00\x00\x00\x00\x00\x00\x80\x3f\x00\x00\x00\x00'
    attr = b'\x00\x00'
    with open(dest, "wb") as f:
        f.write(header + count + normal + v1 + v2 + v3 + attr)

def _copy_asset(filename, target_dir):
    """Helper to copy or generate test assets"""
    dest = os.path.join(target_dir, filename)
    os.makedirs(target_dir, exist_ok=True)
    
    if filename == "sophia-35mm-sophia.stl":
        _create_dummy_stl(dest)
    else:
        src = os.path.join(TEST_SOURCE, filename)
        if not os.path.exists(src):
            with open(dest, "w") as f:
                f.write("dummy content")
        else:
            shutil.copy(src, dest)

def _wait_complete():
    TIMEOUT = 300
    for i in range(TIMEOUT):
        status = api_call("status")
        if status:
            state = status.get("state")
            queue = status.get("queue", {})
            if state == "waiting" and queue.get("staging", 0) == 0 and queue.get("input", 0) == 0:
                time.sleep(2)
                return
        time.sleep(1)
    raise Exception(f"Processing timeout at {TIMEOUT}s")


# --- Implementation Logic (Decoupled from Decorators) ---

def impl_dir_empty(context, dir_name):
    target_dir = {
        "input": INPUT_DIR,
        "staging": STAGING_DIR,
        "output": OUTPUT_DIR
    }.get(dir_name.lower())
    
    assert target_dir, f"Unknown directory: {dir_name}"
    
    files = [f for f in os.listdir(target_dir) if not f.startswith('.')] if os.path.exists(target_dir) else []
    
    # If we have files and we are calling this logic (usually used in Given/Then to assert empty), 
    # for Given we might want to clean, for Then we expect empty.
    # The original common_steps allowed cleaning. We'll preserve that behavior.
    if files:
        _clean_dir(target_dir)
        files = [f for f in os.listdir(target_dir) if not f.startswith('.')]
    
    assert not files, f"Directory {dir_name} ({target_dir}) is not empty: {files}"

def impl_enable_auto(context):
    api_call("settings", "POST", {"auto_process": True})
    time.sleep(2)

def impl_disable_auto(context):
    api_call("settings", "POST", {"auto_process": False})
    time.sleep(2)

def impl_start_container(context):
    start = time.time()
    run_manage_cmd(["start"])
    context.start_trigger_time = start

def impl_check_api_ready(context):
    url = f"{API_URL}/status"
    ready = False
    for _ in range(30):
        try:
            resp = requests.get(url, timeout=2)
            if resp.status_code == 200:
                ready = True
                break
        except:
            pass
        time.sleep(1)
    
    assert ready, "API failed to become ready within 30s"
    context.startup_time = time.time() - context.start_trigger_time
    print(f"DEBUG: Startup took {context.startup_time:.2f}s")
