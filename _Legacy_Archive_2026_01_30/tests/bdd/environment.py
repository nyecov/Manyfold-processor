import requests
import time
import os
import shutil

API_URL = os.getenv("API_URL", "http://localhost:6767/api")

def before_all(context):
    # --- Logging Setup ---
    # Define log directory relative to this file (tests/bdd/environment.py -> tests/logs)
    context.log_dir = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "logs"))
    
    # Overwrite/Clean logs
    if os.path.exists(context.log_dir):
        shutil.rmtree(context.log_dir)
    os.makedirs(context.log_dir, exist_ok=True)
    print(f"Test Logs Directory: {context.log_dir}")
    
    # --- API Check ---
    if os.getenv("SKIP_STARTUP_WAIT"):
        print("Skipping Global API Wait (Assuming Cold Start Test)...")
        return

    # Wait for API to be ready
    print(f"Waiting for Processor API at {API_URL}...")
    for i in range(30):
        try:
            resp = requests.get(f"{API_URL}/status", timeout=2)
            if resp.status_code == 200:
                print("API Validated.")
                return
        except:
            pass
        time.sleep(1)
    raise Exception("API Startup Timeout")

def before_feature(context, feature):
    # Sanitize feature name for filename
    safe_name = "".join([c if c.isalnum() or c in (' ', '-', '_') else '_' for c in feature.name]).strip()
    safe_name = safe_name.replace(' ', '_')
    
    log_file = os.path.join(context.log_dir, f"{safe_name}.log")
    context.feature_log_file = open(log_file, "w", encoding="utf-8")
    
    context.feature_log_file.write(f"FEATURE: {feature.name}\n")
    context.feature_log_file.write("=" * 40 + "\n")

def before_scenario(context, scenario):
    if hasattr(context, 'feature_log_file'):
        context.feature_log_file.write(f"\nSCENARIO: {scenario.name}\n")
        context.feature_log_file.write("-" * 20 + "\n")

def after_step(context, step):
    if hasattr(context, 'feature_log_file'):
        status = step.status.name.upper()
        context.feature_log_file.write(f"[{status}] {step.keyword} {step.name}\n")
        if status == 'FAILED':
             context.feature_log_file.write(f"  ERROR: {step.error_message}\n")

def after_feature(context, feature):
    if hasattr(context, 'feature_log_file'):
        context.feature_log_file.write("\n" + "=" * 40 + "\n")
        context.feature_log_file.write("END OF FEATURE\n")
        context.feature_log_file.close()
        delattr(context, 'feature_log_file')

def after_all(context):
    pass
