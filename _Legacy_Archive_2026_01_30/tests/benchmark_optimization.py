import os
import time
import logging
import sys

# Add src to path
sys.path.append(os.path.join(os.path.dirname(__file__), "..", "src"))
from utils.geometry_converter import GeometryConverter

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger("benchmark")

def run_benchmark():
    # Assets
    stl_path = os.path.join(os.path.dirname(__file__), "..", "test_data", "Loose Files STL_and_jpg", "sophia-35mm-sophia.stl")
    if not os.path.exists(stl_path):
        logger.error(f"Sample STL not found at {stl_path}")
        return

    output_dir = os.path.join(os.path.dirname(__file__), "..", "playground", "benchmark_output")
    os.makedirs(output_dir, exist_ok=True)

    logger.info("Starting Benchmark: STL to 3MF Conversion")
    logger.info(f"File: {os.path.basename(stl_path)} ({os.path.getsize(stl_path)/1024:.1f} KB)")
    print("-" * 40)

    # Trimesh Path
    import trimesh
    logger.info("Method 1: Trimesh (Legacy)")
    start = time.time()
    
    # Memory measurement helper
    # Memory measurement helper
    def get_mem():
        try:
            if os.name == 'posix':
                with open("/proc/self/status") as f:
                    for line in f:
                        if "VmRSS" in line:
                            return int(line.split()[1])
            # Windows/Other fallback (requires psutil or similar, skipping for now to avoid dependency)
            return 0
        except Exception:
            return 0

    mem_before = get_mem()
    mesh = trimesh.load(stl_path, file_type='stl')
    trimesh_out = os.path.join(output_dir, "trimesh_output.3mf")
    mesh.export(trimesh_out, file_type='3mf')
    trimesh_time = time.time() - start
    mem_after = get_mem()
    
    # Trimesh Memory
    trimesh_mem = mem_after - mem_before
    logger.info(f"Trimesh Time: {trimesh_time:.4f}s")
    logger.info(f"Trimesh Memory Delta: {trimesh_mem} KB")
    print("-" * 40)

    # 1.5 Pure Python Path
    logger.info("Method 1.5: Pure Python (Native Fallback)")
    start = time.time()
    mem_before = get_mem()
    python_out = os.path.join(output_dir, "python_output.3mf")
    success = GeometryConverter._stl_to_3mf_pure_python(stl_path, python_out)
    python_time = time.time() - start
    mem_after = get_mem()
    python_mem = mem_after - mem_before
    if success:
        logger.info(f"Pure Python Time: {python_time:.4f}s")
        logger.info(f"Pure Python Memory Delta: {python_mem} KB")
        logger.info(f"Pure Python Speedup vs Trimesh: {trimesh_time / python_time:.2f}x")
    else:
        logger.error("Pure Python conversion failed!")
    print("-" * 40)

    # 2. Rust Path (If available)
    from utils.geometry_converter import RUST_OPTIMIZER
    if os.path.exists(RUST_OPTIMIZER):
        logger.info("Method 2: Rust Optimizer (New)")
        start = time.time()
        rust_out = os.path.join(output_dir, "rust_output.3mf")
        # We'll use GeometryConverter.stl_to_3mf which prioritizes Rust
        # But we want to ensure it uses Rust.
        GeometryConverter.stl_to_3mf(stl_path, output_dir, "rust_output")
        rust_time = time.time() - start
        logger.info(f"Rust Time: {rust_time:.4f}s")
        
        speedup = trimesh_time / rust_time
        logger.info(f"Speedup Factor: {speedup:.2f}x")
    else:
        logger.warning(f"Rust Optimizer binary not found at {RUST_OPTIMIZER}. Build it first!")

    print("-" * 40)
    logger.info("Benchmark Complete.")

if __name__ == "__main__":
    run_benchmark()
