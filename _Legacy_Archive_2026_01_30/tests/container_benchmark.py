import os
import time
import logging
import sys

# Paths inside container
sys.path.append("/app")
from utils.geometry_converter import GeometryConverter

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger("benchmark")

def run_benchmark():
    stl_path = "/tmp/sophia.stl"
    output_dir = "/tmp/benchmark_output"
    os.makedirs(output_dir, exist_ok=True)

    if not os.path.exists(stl_path):
        logger.error(f"Sample STL not found at {stl_path}")
        return

    logger.info("Starting Container Benchmark: STL to 3MF")
    logger.info(f"File: {os.path.basename(stl_path)} ({os.path.getsize(stl_path)/1024:.1f} KB)")
    print("-" * 40)

    # Method 1: Trimesh
    import trimesh
    logger.info("Method 1: Trimesh (Legacy)")
    def get_mem():
        with open("/proc/self/status") as f:
            for line in f:
                if "VmRSS" in line:
                    return int(line.split()[1])
        return 0

    start = time.time()
    mem_before = get_mem()
    mesh = trimesh.load(stl_path, file_type='stl')
    trimesh_out = os.path.join(output_dir, "trimesh_output.3mf")
    mesh.export(trimesh_out, file_type='3mf')
    trimesh_time = time.time() - start
    mem_after = get_mem()
    trimesh_mem = mem_after - mem_before
    logger.info(f"Trimesh Time: {trimesh_time:.4f}s")
    logger.info(f"Trimesh Memory Delta: {trimesh_mem} KB")
    print("-" * 40)


    # Method 3: Rust Optimizer
    logger.info("Method 3: Zero-Dependency Rust Optimizer")
    rust_bin = "/app/plugins/stl23mf/stl23mf"
    if os.path.exists(rust_bin):
        import subprocess
        start = time.time()
        rust_out = os.path.join(output_dir, "rust_output.3mf")
        
        # Run binary directly
        mem_before = get_mem()
        CMD = [rust_bin, "--input", stl_path, "--output", rust_out]
        p = subprocess.run(CMD, capture_output=True)
        if p.returncode == 0:
            rust_time = time.time() - start
            mem_after = get_mem()
            rust_mem = mem_after - mem_before
            
            logger.info(f"Rust Time: {rust_time:.4f}s")
            logger.info(f"Rust Memory Delta: {rust_mem} KB") # This tracks python mem, not subprocess. Subprocess is separate.
            logger.info(f"Speedup vs Trimesh: {trimesh_time / rust_time:.2f}x")
            logger.info(f"Speedup vs Python: {python_time / rust_time:.2f}x")
        else:
             logger.error(f"Rust failed: {p.stderr}")
    else:
        logger.error(f"Rust binary not found at {rust_bin}")

    print("-" * 40)
    logger.info("Benchmark Complete.")

if __name__ == "__main__":
    run_benchmark()
