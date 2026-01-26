import requests
import time
import os

API_URL = os.getenv("API_URL", "http://localhost:6767/api")

def before_all(context):
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

def after_all(context):
    pass
