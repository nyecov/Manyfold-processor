from behave import when
import time
import os
import subprocess
import step_utils

@when('I rebuild the processor container without cache')
def step_rebuild_container(context):
    start = time.time()
    
    cmd = ["docker", "compose", "build", "--no-cache"]
    # Windows checks are handled inside run_manage_cmd, but here we run docker directly
    # Assuming docker is in PATH as verified by previous steps.
    
    cwd = os.path.dirname(step_utils.MANAGE_SCRIPT)
    subprocess.run(cmd, cwd=cwd, check=True, capture_output=True, text=True)
    
    context.build_time = time.time() - start
    print(f"DEBUG: Build took {context.build_time:.2f}s")

@when('the processor container should start successfully')
def step_start_container_when(context):
    step_utils.impl_start_container(context)

@when('the API should be ready within 30 seconds')
def step_check_api_ready_when(context):
    step_utils.impl_check_api_ready(context)

@when('I enable auto-process')
def step_impl_enable_auto_when(context):
    step_utils.impl_enable_auto(context)

@when('I disable auto-process')
def step_impl_disable_auto_when(context):
    step_utils.impl_disable_auto(context)

@when('I copy "{filename}" from test source to {dir_name}')
def step_impl_copy(context, filename, dir_name):
    # Need to reimplement or call util
    # step_utils._copy_asset is available but we need to map dir_name to target_dir first
    target_dir = {
        "input": step_utils.INPUT_DIR,
        "staging": step_utils.STAGING_DIR
    }.get(dir_name.lower())
    assert target_dir, f"Unknown copy target: {dir_name}"
    step_utils._copy_asset(filename, target_dir)

@when('I wait for processing to complete')
def step_impl_wait_complete_when(context):
    step_utils._wait_complete()

@when('I trigger the "Process Loose Files" action via API')
def step_impl_trigger_loose(context):
    step_utils.api_call("process/all", "POST")

@when('I wait {seconds:d} seconds for {reason}')
def step_impl_wait_reason(context, seconds, reason="generic"):
    time.sleep(seconds)

@when('I wait {seconds:d} seconds')
def step_impl_wait_simple(context, seconds):
    time.sleep(seconds)

@when('I wait for "{dirname}" processing to complete')
def step_impl_wait_dir(context, dirname):
    path = os.path.join(step_utils.OUTPUT_DIR, dirname)
    for _ in range(120):
        if os.path.exists(os.path.join(path, "datapackage.json")):
            return
        time.sleep(1)

@when('I wait for orphan processing to complete')
def step_impl_wait_orphan(context):
    for _ in range(60):
        staging_files = [f for f in os.listdir(step_utils.STAGING_DIR) if not f.startswith('.')]
        input_files = [f for f in os.listdir(step_utils.INPUT_DIR) if not f.startswith('.')]
        if not staging_files and not input_files:
            # Settle a bit to ensure API state catch up
            time.sleep(2)
            return
        time.sleep(1)

@when('I request system settings')
def step_impl_req_settings(context):
    context.response = step_utils.api_call("settings")

@when('I change the setting "{key}" to "{value}"')
def step_impl_change_setting(context, key, value):
    val = True if value.lower() == "true" else False if value.lower() == "false" else value
    step_utils.api_call("settings", "POST", {key: val})

@when('I create a corrupt file "{filename}" in input')
def step_impl_corrupt(context, filename):
    with open(os.path.join(step_utils.INPUT_DIR, filename), "w") as f:
        f.write("corrupt")

@when('I create a directory "{dir_path}"')
def step_create_directory(context, dir_path):
    # Adjust path if it's relative to root
    full_path = dir_path
    if dir_path.startswith("input/"):
        full_path = os.path.join(os.path.dirname(step_utils.INPUT_DIR), dir_path)
    os.makedirs(full_path, exist_ok=True)

@when('I copy "{filename}" to "{dest_path}"')
def step_copy_to_path(context, filename, dest_path):
    # Resolve source
    src = None
    # Check if it's a known asset or a real file
    dest_full = dest_path
    if dest_path.startswith("input/"):
        dest_full = os.path.join(os.path.dirname(step_utils.INPUT_DIR), dest_path)
    
    # We use step_utils._copy_asset logic but for a specific destination
    # _copy_asset(filename, target_dir) uses TEST_SOURCE and special logic
    target_dir = os.path.dirname(dest_full)
    target_filename = os.path.basename(dest_full)
    
    # Temporarily override dest inside _copy_asset or just reimplement
    step_utils._copy_asset(filename, target_dir)
    # If it was copied as filename, rename to target_filename
    if filename != target_filename:
        actual_copy = os.path.join(target_dir, filename)
        if os.path.exists(actual_copy):
            os.rename(actual_copy, dest_full)

@when('I zip the contents of "{source_dir}" to "{dest_zip}"')
def step_create_zip(context, source_dir, dest_zip):
    import zipfile
    
    # Resolve paths
    src_full = source_dir
    if source_dir.startswith("input/"):
        src_full = os.path.join(os.path.dirname(step_utils.INPUT_DIR), source_dir)
    
    dest_full = dest_zip
    if dest_zip.startswith("input/"):
        dest_full = os.path.join(os.path.dirname(step_utils.INPUT_DIR), dest_zip)
        
    with zipfile.ZipFile(dest_full, 'w') as zf:
        for root, dirs, files in os.walk(src_full):
            for f in files:
                abs_f = os.path.join(root, f)
                rel_f = os.path.relpath(abs_f, src_full)
                zf.write(abs_f, rel_f)
    
    # Optional: cleanup source dir?
    # shutil.rmtree(src_full)

@when('I delete the directory "{dir_path}"')
def step_delete_directory(context, dir_path):
    import shutil
    full_path = dir_path
    if dir_path.startswith("input/"):
        full_path = os.path.join(os.path.dirname(step_utils.INPUT_DIR), dir_path)
    if os.path.exists(full_path):
        shutil.rmtree(full_path)
