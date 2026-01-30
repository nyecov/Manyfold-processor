from behave import then
import os
import time
import subprocess
import step_utils
import zipfile
import re

@then('the build time should be recorded')
def step_record_build_time(context):
    assert context.build_time > 0
    project_root = os.path.dirname(step_utils.MANAGE_SCRIPT)
    log_path = os.path.join(project_root, "docs", "benchmarks.log")
    os.makedirs(os.path.dirname(log_path), exist_ok=True)
    with open(log_path, "a") as f:
        f.write(f"{time.strftime('%Y-%m-%d %H:%M:%S')} - BUILD TIME: {context.build_time:.2f}s\n")

@then('the processor container should start successfully')
def step_start_container_then(context):
    step_utils.impl_start_container(context)

@then('the API should be ready within 30 seconds')
def step_check_api_ready_then(context):
    step_utils.impl_check_api_ready(context)

@then('the startup time should be recorded')
def step_record_startup_time(context):
    assert context.startup_time > 0
    project_root = os.path.dirname(step_utils.MANAGE_SCRIPT)
    log_path = os.path.join(project_root, "docs", "benchmarks.log")
    with open(log_path, "a") as f:
        f.write(f"{time.strftime('%Y-%m-%d %H:%M:%S')} - STARTUP TIME: {context.startup_time:.2f}s\n")

@then('the logs should contain the "{text}" banner')
def step_check_logs_banner(context, text):
    cmd = ["docker", "logs", "manyfold-processor"]
    result = subprocess.run(cmd, capture_output=True, text=True)
    assert text in result.stdout or text in result.stderr, f"Banner '{text}' not found in logs"

@then('the "{dir_name}" directory should be empty')
def step_impl_dir_empty_then(context, dir_name):
    step_utils.impl_dir_empty(context, dir_name)

@then('the output directory should be empty')
def step_impl_output_empty(context):
    files = [f for f in os.listdir(step_utils.OUTPUT_DIR) if not f.startswith('.')]
    assert not files, f"Output directory is not empty: {files}"

@then('the files should appear in staging')
def step_impl_files_in_staging(context):
    found = False
    for _ in range(15):
        if os.path.exists(step_utils.STAGING_DIR) and [f for f in os.listdir(step_utils.STAGING_DIR) if not f.startswith('.')]:
            found = True
            break
        time.sleep(1)
    assert found, "Files never reached staging"

@then('I wait for processing to complete')
def step_impl_wait_complete_then(context):
    step_utils._wait_complete()

@then('the output directory "{dirname}" should exist')
def step_impl_out_exists(context, dirname):
    path = os.path.join(step_utils.OUTPUT_DIR, dirname)
    assert os.path.exists(path), f"Output {dirname} missing"

@then('the output directory "{dirname}" should NOT exist')
def step_impl_out_not_exists(context, dirname):
    path = os.path.join(step_utils.OUTPUT_DIR, dirname)
    assert not os.path.exists(path), f"Output {dirname} exists"

@then('the output directory "{dirname}" should contain "{item}" file')
def step_impl_out_content_ext(context, dirname, item):
    path = os.path.join(step_utils.OUTPUT_DIR, dirname)
    files = os.listdir(path)
    assert any(f.endswith(item) for f in files), f"No {item} in {dirname}"

@then('the output directory "{dirname}" should contain "{item}"')
def step_impl_out_content_exact(context, dirname, item):
    path = os.path.join(step_utils.OUTPUT_DIR, dirname)
    files = os.listdir(path)
    assert item in files, f"{item} not in {dirname}"

@then('the output directory "{dirname}" should NOT contain "{filename}"')
def step_impl_out_missing(context, dirname, filename):
    path = os.path.join(step_utils.OUTPUT_DIR, dirname)
    assert filename not in os.listdir(path)

@then('the file "{filename}" should remain in input')
def step_impl_in_input(context, filename):
    assert os.path.exists(os.path.join(step_utils.INPUT_DIR, filename))

@then('the file "{filename}" should NOT exist in input')
def step_impl_not_in_input(context, filename):
    assert not os.path.exists(os.path.join(step_utils.INPUT_DIR, filename))

@then('the response should contain "{field}"')
def step_impl_resp_contains(context, field):
    assert field in context.response

@then('the settings should show "{key}" is "{value}"')
def step_impl_check_setting(context, key, value):
    settings = step_utils.api_call("settings")
    val = settings.get(key)
    assert str(val).lower() == value.lower()

@then('I should see an error reported for "{filename}"')
def step_impl_check_error(context, filename):
    for _ in range(15):
        errs = step_utils.api_call("errors")
        if errs and any(e.get("file") == filename or filename in e.get("error", "") for e in errs.get("errors", [])):
            return
        time.sleep(1)
    raise Exception(f"No error for {filename}")

@then('the 3MF file "{filename}" inside "{project_name}" should contain at least {count} objects')
def step_check_3mf_object_count(context, filename, project_name, count):
    path = os.path.join(step_utils.OUTPUT_DIR, project_name, filename)
    assert os.path.exists(path), f"3MF file {path} not found"
    
    found_obj_count = 0
    try:
        with zipfile.ZipFile(path, 'r') as z:
            # Method A: Check Metadata/model_settings.config (Bambu/Orca style)
            if "Metadata/model_settings.config" in z.namelist():
                with z.open("Metadata/model_settings.config") as f:
                    content = f.read().decode('utf-8')
                    found_obj_count = content.count("<object ")
            
            # Method B: Check 3D/3dmodel.model (Standard 3MF)
            if found_obj_count == 0 and "3D/3dmodel.model" in z.namelist():
                with z.open("3D/3dmodel.model") as f:
                    content = f.read().decode('utf-8')
                    found_obj_count = content.count("<object ")
    except Exception as e:
        assert False, f"Failed to inspect 3MF: {e}"

    assert found_obj_count >= int(count), f"Expected at least {count} objects in 3MF, found {found_obj_count}"