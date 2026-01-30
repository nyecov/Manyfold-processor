# Hardware Acceleration Analysis (Radxa Rock 5 ITX)

The Radxa Rock 5 ITX is powered by the **Rockchip RK3588** SoC, which features robust accelerators. This document analyzes their potential impact on the Manyfold Processor workload.

## 1. Hardware Overview
*   **CPU**: 4x Cortex-A76 + 4x Cortex-A55 (Current bottleneck for Python/Geometry).
*   **GPU**: ARM Mali-G610 MP4 (Supports OpenCL, Vulkan).
*   **NPU**: 6 TOPS (INT8/INT16/FP16) - Optimized for AI inference.
*   **VPU/RGA**: Video Processing Unit & Raster Graphic Acceleration (2D Hardware Blit).

## 2. Workload Analysis

### A. Image Processing (High Potential)
*   **Current**: Python `Pillow` (CPU).
*   **Bottleneck**: Resizing and converting large 4K textures/photos to WebP takes significant CPU time (~200-500ms per image).
*   **Solution: Rockchip RGA (Raster Graphic Acceleration)**.
    *   The RK3588 has a dedicated 2D hardware engine for resize/crop/format-conversion.
    *   **Performance**: Can process 4K images in milliseconds with **~0% CPU usage**.
    *   **Utilization**: Use `librga` (C++ library) or Python bindings if available. This is the "Low Hanging Fruit" for optimization.

### B. Geometry Processing (Low Potential)
*   **Current**: Rust `stl23mf` (CPU).
*   **Bottleneck**: Vertex processing, float math.
*   **GPU Potential**:
    *   **Compute Shaders (OpenCL/Vulkan)**: Could parallelize massive vertex operations (e.g., simplification of a 10M poly mesh).
    *   **Constraint**: The overhead of transferring mesh data from RAM to GPU memory often outweighs the speedup for "simple" file format conversions. Only beneficial for extremely complex algorithms (e.g., mesh repair, boolean operations).
*   **NPU Potential**: ❌ **None**. The NPU is designed for Tensor operations (Matrix Multiplication) for Neural Networks, not general purpose floating point geometry math.

### C. AI Categorization (NPU Potential)
*   **Concept**: Automatically tagging models (e.g., "Dragon", "Sword", "Sci-Fi").
*   **Hardware**: The **NPU (6 TOPS)** is widespread for running models like CLIP or MobileNet.
*   **Utilization**:
    *   Load a quantized (`rknn`) Image Classification model.
    *   Feed the rendered thumbnail to the NPU.
    *   **Result**: Automatic classification in <20ms with low power usage.
    *   **Status**: A great "Phase 2" feature.

## 3. Implementation Roadmap

### Phase 1: RGA for Images (Easy Wins)
Replace `Pillow` with a CLI wrapper around `librga` or `ffmpeg-rkmpp` for image conversion.
*   **Impact**: Faster previews, lower CPU load.

### Phase 2: NPU for Intelligence
Implement a "Smart Tagging" plugin using `rknn-toolkit2`.
*   **Impact**: New feature capability (Auto-Tagging).

### Verdict
*   **GPU**: Not worth the effort for current geometry tasks.
*   **NPU**: Excellent for future AI features.
*   **RGA (2D)**: Immediate benefit for image handling.

## 4. Training Feasibility (Can I train on the Rock 5?)
**Short Answer: No.**

### Why?
1.  **NPU is for Inference Only**: The RK3588 NPU (6 TOPS) is highly optimized for running *existing* models (Inference) using INT8 quantization. It lacks the floating-point precision (FP32/BF16) and backward propagation hardware required for *training*.
2.  **GPU Limitations**: While the Mali G610 is decent, major training frameworks (PyTorch/TensorFlow) rely on NVIDIA CUDA or AMD ROCm. Methods to train on Mali (via OpenCL) are experimental, slow, and lack memory.
3.  **3D Training Complexity**: Training *massive* foundation models requires 24GB+ VRAM. **However**, your specific PC **(16GB VRAM / 32GB RAM)** is **perfectly capable** of training the efficient, lightweight models (e.g., MobileNet, YOLO, PointNet) that effectively run on the Rock 5's NPU.

### Recommended Workflow (Green Light ✅)
1.  **Train on your PC**: Your 16GB VRAM is ideal for training models optimized for Edge deployment.
    *   *2D approach*: Render thumbnails -> Train YOLOv8/MobileNet (Fastest, easiest).
    *   *3D approach*: Voxelize/Sample points -> Train PointNet (Feasible with 16GB VRAM).
2.  **Export**: Save the model in ONNX format.
3.  **Convert**: Use `rknn-toolkit2` to convert ONNX -> RKNN.
4.  **Run on Rock 5**: Deploy the `.rknn` file to the NPU.
