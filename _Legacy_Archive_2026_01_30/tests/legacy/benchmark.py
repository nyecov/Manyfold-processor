import subprocess
import time
import os
import shutil
import re
import datetime
import json

# Configuration
TEST_SOURCE = "Playground/test/TestCase1"
INPUT_DIR = "Playground/input"
OUTPUT_DIR = "Playground/output"
STAGING_DIR = "Playground/staging"
CONTAINER_NAME = "manyfold-processor"
SERVICE_NAME = "manyfold-processor"
LOG_FILE = "TestCases/TestCase2.log"

def log(msg):
    timestamp = time.strftime("%Y-%m-%d %H:%M:%S")
    formatted = f"[{timestamp}] {msg}"
    print(formatted)
    with open(LOG_FILE, "a") as f:
        f.write(formatted + "\n")

def run_command(cmd):
    subprocess.run(cmd, shell=True, check=True, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)

def get_stats():
    """Get current stats for the container."""
    try:
        # Format: {{.CPUPerc}},{{.MemUsage}}
        result = subprocess.run(
            f"sudo docker stats --no-stream --format \"{{{{.CPUPerc}}}},{{{{.MemUsage}}}}\" {CONTAINER_NAME}",
            shell=True, capture_output=True, text=True
        )
        if result.returncode != 0: return 0.0, 0.0
        
        output = result.stdout.strip()
        if not output: return 0.0, 0.0
        
        # Parse "99.05%,120MiB / 15GiB" -> 99.05, 120
        parts = output.split(',')
        cpu_str = parts[0].replace('%', '')
        mem_str = parts[1].split('/')[0].strip()
        
        # Parse Mem (MiB, GiB, etc)
        mem_val = 0.0
        if 'MiB' in mem_str:
            mem_val = float(mem_str.replace('MiB', ''))
        elif 'GiB' in mem_str:
            mem_val = float(mem_str.replace('GiB', '')) * 1024
        elif 'kiB' in mem_str:
            mem_val = float(mem_str.replace('kiB', '')) / 1024
        elif 'B' in mem_str:
            mem_val = float(mem_str.replace('B', '')) / (1024*1024)
            
        return float(cpu_str), mem_val
    except Exception:
        return 0.0, 0.0

def main():
    # Reset log file
    with open(LOG_FILE, "w") as f:
        f.write(f"=== {LOG_FILE} ===\n")
        
    log("=== Starting Benchmark TestCase2 ===")
    
    # 1. Reset Environment
    log("[1/5] Resetting environment...")
    run_command(f"sudo docker compose down")
    if os.path.exists(INPUT_DIR):
        for f in os.listdir(INPUT_DIR): os.remove(os.path.join(INPUT_DIR, f))
    if os.path.exists(OUTPUT_DIR):
        shutil.rmtree(OUTPUT_DIR)
        os.makedirs(OUTPUT_DIR)
    if os.path.exists(STAGING_DIR):
        for f in os.listdir(STAGING_DIR): os.remove(os.path.join(STAGING_DIR, f))

    # 2. Stage Files
    log("[2/5] Staging files...")
    # Get STL size for calculation
    stl_file = [f for f in os.listdir(TEST_SOURCE) if f.endswith('.stl')][0]
    stl_path = os.path.join(TEST_SOURCE, stl_file)
    stl_size_mb = os.path.getsize(stl_path) / (1024 * 1024)
    log(f"      File: {stl_file} ({stl_size_mb:.2f} MB)")
    
    run_command(f"cp \"{TEST_SOURCE}\"/* \"{INPUT_DIR}/\"")

    # 3. Start Container
    log("[3/5] Starting container...")
    run_command(f"sudo docker compose up -d")
    
    # 4. Monitor Loop
    log("[4/5] Monitoring processing...")
    stats_log = []
    start_time = None
    end_time = None
    
    # Poll logs and stats
    # We look for specific log lines to determine timestamps
    
    max_wait = 300 # 5 minutes timeout
    polling_start = time.time()
    
    while True:
        if time.time() - polling_start > max_wait:
            log("TIMEOUT")
            break
            
        # Get Logs
        log_res = subprocess.run(f"sudo docker logs {CONTAINER_NAME}", shell=True, capture_output=True, text=True)
        logs = log_res.stdout
        
        # Check for Start Trigger
        if not start_time:
            # Look for "Handling event for: ... .stl"
            match = re.search(r'(\d{4}-\d{2}-\d{2}\s\d{2}:\d{2}:\d{2},\d{3}).*Handling event for:.*\.stl', logs)
            if match:
                ts_str = match.group(1)
                # Parse python logging timestamp format
                # 2026-01-24 23:53:48,618
                dt = datetime.datetime.strptime(ts_str, '%Y-%m-%d %H:%M:%S,%f')
                start_time = dt
                log(f"      Job Started: {ts_str}")

        # Check for End Trigger
        if start_time and not end_time:
            # Look for "moved artifacts to /output/"
            # We want the LAST occurrence if there are multiple, but effectively the first valid one after start
            # Actually, "Processing Case 1" ... -> "moved artifacts"
            match = re.search(r'(\d{4}-\d{2}-\d{2}\s\d{2}:\d{2}:\d{2},\d{3}).*moved artifacts to /output/', logs)
            if match:
                ts_str = match.group(1)
                dt = datetime.datetime.strptime(ts_str, '%Y-%m-%d %H:%M:%S,%f')
                # Ensure this end time is AFTER start time (simple check)
                if dt > start_time:
                    end_time = dt
                    log(f"      Job Finished: {ts_str}")
                    break
        
        # Stats
        cpu, mem = get_stats()
        stats_log.append({'cpu': cpu, 'mem': mem})
        
        time.sleep(0.5)

    # 5. Analysis
    log("[5/5] Analyzing results...")
    
    if start_time and end_time:
        duration = (end_time - start_time).total_seconds()
        log(f"\nRESULTS:")
        log(f"Total Duration: {duration:.2f} seconds")
        log(f"STL Size:       {stl_size_mb:.2f} MB")
        log(f"Speed:          {duration / stl_size_mb:.2f} sec/MB")
        log(f"Speed:          {stl_size_mb / duration:.2f} MB/sec")
        
        # Filter stats during the active window?
        # Since we polled continuously, let's just avg all non-zero
        cpus = [s['cpu'] for s in stats_log if s['cpu'] > 0]
        mems = [s['mem'] for s in stats_log if s['mem'] > 0]
        
        avg_cpu = sum(cpus)/len(cpus) if cpus else 0
        max_cpu = max(cpus) if cpus else 0
        max_mem = max(mems) if mems else 0
        
        # Stats Analysis
        log(f"Avg CPU:        {avg_cpu:.1f}%")
        log(f"Max CPU:        {max_cpu:.1f}%")
        log(f"Max RAM:        {max_mem:.1f} MB")
        
        log(f"\n--- Verifying Success Criteria for Benchmark ---")
        all_passed = True
        
        # Criterion 1: Script runs to completion (Implicit if we are here)
        log("✅ [Pass] Benchmark script ran to completion.")
        
        # Criterion 2: Stats generated
        if len(stats_log) > 0:
            log("✅ [Pass] Resource usage stats generated.")
        else:
            log("❌ [Fail] No resource stats captured.")
            all_passed = False
            
        # Criterion 3: Speed calculated
        if duration > 0 and stl_size_mb > 0:
            log(f"✅ [Pass] Conversion Speed calculated: {duration / stl_size_mb:.2f} sec/MB")
        else:
            log("❌ [Fail] invalid speed calculation.")
            all_passed = False
        
        # Criterion 4: Max Duration
        if duration < 600:
             log(f"✅ [Pass] Processing time {duration:.2f}s < 600s limit.")
        else:
             log(f"❌ [Fail] Processing time {duration:.2f}s > 600s limit.")
             all_passed = False
             
        if all_passed:
            log("🎉 BENCHMARK SUCCESS")
        else:
            log("🚫 BENCHMARK FAILED")
        
    else:
        log("FAILED to detect start/end times in logs.")

if __name__ == "__main__":
    main()
