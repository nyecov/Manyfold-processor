import os
import subprocess
import time
import requests
from behave import given, when, then

MANAGE_SCRIPT = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..", "..", "manage.sh"))
API_URL = os.getenv("API_URL", "http://localhost:6767/api")

@given('the processor docker container is stopped')
def step_stop_container(context):
    subprocess.run([MANAGE_SCRIPT, "stop"], check=True, capture_output=True)

@when('I rebuild the processor container without cache')
def step_rebuild_container(context):
    start = time.time()
    subprocess.run(["sudo", "docker", "compose", "build", "--no-cache"], 
                    cwd=os.path.dirname(MANAGE_SCRIPT),
                    check=True, capture_output=True, text=True)
    context.build_time = time.time() - start
    print(f"DEBUG: Build took {context.build_time:.2f}s")

@then('the build time should be recorded')
def step_record_build_time(context):
    assert context.build_time > 0
    project_root = os.path.dirname(MANAGE_SCRIPT)
    log_path = os.path.join(project_root, "docs", "benchmarks.log")
    os.makedirs(os.path.dirname(log_path), exist_ok=True)
    with open(log_path, "a") as f:
        f.write(f"{time.strftime('%Y-%m-%d %H:%M:%S')} - BUILD TIME: {context.build_time:.2f}s\n")

@when('the processor container should start successfully')
@then('the processor container should start successfully')
def step_start_container(context):
    start = time.time()
    subprocess.run([MANAGE_SCRIPT, "start"], check=True, capture_output=True)
    context.start_trigger_time = start

@when('the API should be ready within 30 seconds')
@then('the API should be ready within 30 seconds')
def step_check_api_ready(context):
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

@then('the startup time should be recorded')
def step_record_startup_time(context):
    assert context.startup_time > 0
    project_root = os.path.dirname(MANAGE_SCRIPT)
    log_path = os.path.join(project_root, "docs", "benchmarks.log")
    with open(log_path, "a") as f:
        f.write(f"{time.strftime('%Y-%m-%d %H:%M:%S')} - STARTUP TIME: {context.startup_time:.2f}s\n")

@then('the logs should contain the "{text}" banner')
def step_check_logs_banner(context, text):
    result = subprocess.run(["sudo", "docker", "logs", "manyfold-processor"], 
                            capture_output=True, text=True)
    assert text in result.stdout or text in result.stderr, f"Banner '{text}' not found in logs"
