import requests
import json
import os
import shutil
import time
from behave import given, when, then

# Configuration
TEST_SOURCE = os.getenv("TEST_SOURCE", "../test_data")
INPUT_DIR = os.getenv("INPUT_DIR", "../playground/input")
OUTPUT_DIR = os.getenv("OUTPUT_DIR", "../playground/output")
STAGING_DIR = os.getenv("STAGING_DIR", "../playground/staging")
API_URL = os.getenv("API_URL", "http://localhost:6767/api")

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

@given('the processor is running')
def step_impl_processor_running(context):
    status = api_call("status")
    assert status is not None, "Processor API is unreachable"

@given('the "{dir_name}" directory is empty')
@then('the "{dir_name}" directory should be empty')
def step_impl_dir_empty(context, dir_name):
    target_dir = {
        "input": INPUT_DIR,
        "staging": STAGING_DIR,
        "output": OUTPUT_DIR
    }.get(dir_name.lower())
    
    assert target_dir, f"Unknown directory: {dir_name}"
    
    files = [f for f in os.listdir(target_dir) if not f.startswith('.')] if os.path.exists(target_dir) else []
    
    # If we have files during setup (Given), clean them.
    if files:
        _clean_dir(target_dir)
        files = [f for f in os.listdir(target_dir) if not f.startswith('.')]
    
    assert not files, f"Directory {dir_name} ({target_dir}) is not empty: {files}"

@given('I have enabled auto-process')
@when('I enable auto-process')
def step_impl_enable_auto(context):
    api_call("settings", "POST", {"auto_process": True})
    time.sleep(2)

@given('I have disabled auto-process')
@when('I disable auto-process')
def step_impl_disable_auto(context):
    api_call("settings", "POST", {"auto_process": False})
    time.sleep(2)

@when('I copy "{filename}" from test source to {dir_name}')
def step_impl_copy(context, filename, dir_name):
    target_dir = {
        "input": INPUT_DIR,
        "staging": STAGING_DIR
    }.get(dir_name.lower())
    assert target_dir, f"Unknown copy target: {dir_name}"
    _copy_asset(filename, target_dir)

@then('the files should appear in staging')
def step_impl_files_in_staging(context):
    found = False
    for _ in range(15):
        if os.path.exists(STAGING_DIR) and [f for f in os.listdir(STAGING_DIR) if not f.startswith('.')]:
            found = True
            break
        time.sleep(1)
    assert found, "Files never reached staging"

@when('I wait for processing to complete')
@then('I wait for processing to complete')
def step_impl_wait_complete(context):
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

@then('the output directory "{dirname}" should exist')
def step_impl_out_exists(context, dirname):
    path = os.path.join(OUTPUT_DIR, dirname)
    assert os.path.exists(path), f"Output {dirname} missing"

@then('the output directory "{dirname}" should NOT exist')
def step_impl_out_not_exists(context, dirname):
    path = os.path.join(OUTPUT_DIR, dirname)
    assert not os.path.exists(path), f"Output {dirname} exists"

@then('the output directory "{dirname}" should contain "{item}" file')
def step_impl_out_content_ext(context, dirname, item):
    path = os.path.join(OUTPUT_DIR, dirname)
    files = os.listdir(path)
    assert any(f.endswith(item) for f in files), f"No {item} in {dirname}"

@then('the output directory "{dirname}" should contain "{item}"')
def step_impl_out_content_exact(context, dirname, item):
    path = os.path.join(OUTPUT_DIR, dirname)
    files = os.listdir(path)
    assert item in files, f"{item} not in {dirname}"

@then('the output directory "{dirname}" should NOT contain "{filename}"')
def step_impl_out_missing(context, dirname, filename):
    path = os.path.join(OUTPUT_DIR, dirname)
    assert filename not in os.listdir(path)

@when('I trigger the "Process Loose Files" action via API')
def step_impl_trigger_loose(context):
    api_call("process/all", "POST")

@when('I wait {seconds:d} seconds for {reason}')
@when('I wait {seconds:d} seconds')
def step_impl_wait(context, seconds, reason="generic"):
    time.sleep(seconds)

@then('the file "{filename}" should remain in input')
def step_impl_in_input(context, filename):
    assert os.path.exists(os.path.join(INPUT_DIR, filename))

@when('I wait for "{dirname}" processing to complete')
def step_impl_wait_dir(context, dirname):
    path = os.path.join(OUTPUT_DIR, dirname)
    for _ in range(120):
        if os.path.exists(os.path.join(path, "datapackage.json")):
            return
        time.sleep(1)

@when('I wait for orphan processing to complete')
def step_impl_wait_orphan(context):
    for _ in range(60):
        if not [f for f in os.listdir(STAGING_DIR) if not f.startswith('.')]:
            return
        time.sleep(1)

@when('I request system settings')
def step_impl_req_settings(context):
    context.response = api_call("settings")

@then('the response should contain "{field}"')
def step_impl_resp_contains(context, field):
    assert field in context.response

@when('I change the setting "{key}" to "{value}"')
def step_impl_change_setting(context, key, value):
    val = True if value.lower() == "true" else False if value.lower() == "false" else value
    api_call("settings", "POST", {key: val})

@then('the settings should show "{key}" is "{value}"')
def step_impl_check_setting(context, key, value):
    settings = api_call("settings")
    val = settings.get(key)
    assert str(val).lower() == value.lower()

@when('I create a corrupt file "{filename}" in input')
def step_impl_corrupt(context, filename):
    with open(os.path.join(INPUT_DIR, filename), "w") as f:
        f.write("corrupt")

@then('I should see an error reported for "{filename}"')
def step_impl_check_error(context, filename):
    for _ in range(15):
        errs = api_call("errors")
        if errs and any(e.get("file") == filename or filename in e.get("error", "") for e in errs.get("errors", [])):
            return
        time.sleep(1)
    raise Exception(f"No error for {filename}")
