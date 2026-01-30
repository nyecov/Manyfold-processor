# Hardware Acceleration Research: RGA & NPU on RK3588 (Rust)

This document outlines the strategy for utilizing the Rockchip RK3588's specialized hardware accelerators (NPU and RGA) within a Rust application.

## 1. NPU (Neural Processing Unit)

**Goal**: Run AI inference (e.g., classification, detection) on the 6 TOPS NPU instead of the CPU.

### 📚 Core Library: `rknpu2`
Rockchip provides the `rknpu2` C library for interfacing with the NPU. It requires models to be pre-converted to `.rknn` format.

### 🦀 Rust Integration
*   **Existing Crate**: `rknpu2-rs` (unofficial wrapper).
    *   Repo: `https://github.com/0079-overload/rknpu2-rs` (Use with caution, check for maintenance).
*   **Recommended Approach**: Use `bindgen` to generate safe Rust bindings from `rknn_api.h` distributed in the official [rknpu2 SDK](https://github.com/rockchip-linux/rknpu2).

### 🛠️ Implementation Strategy
1.  **Cross-Compilation**: Ensure `aarch64-unknown-linux-gnu` target.
2.  **Linking**: Requires `librknnrt.so` usually found in `/usr/lib/` or `/usr/lib/aarch64-linux-gnu/` on the Rock 5.
3.  **Workflow**:
    ```rust
    // Pseudo-code via bindgen mappings
    let ctx = rknn_init(...);
    let inputs = setup_inputs(...);
    rknn_run(ctx, inputs);
    let outputs = rknn_outputs_get(ctx, ...);
    ```

## 2. RGA (Raster Graphic Acceleration)

**Goal**: Perform 2D image operations (Resize, Crop, Format Conversion, Rotation) with zero CPU usage.

### 📚 Core Library: `librga`
The `librga` library is the standard user-space API for the RGA hardware.

### 🦀 Rust Integration
*   **Status**: No mature, "batteries-included" crate exists on Crates.io as of early 2026.
*   **Solution**: Generate bindings for `librga` using `bindgen`.

### 🛠️ Implementation Strategy
1.  **Headers**: Locate `RgaApi.h` or `RockchipRga.h` (from `librga-dev` package on Rockchip Linux distros).
2.  **Bindgen**: Create a `sys` crate (e.g., `librga-sys`) in your cargo workspace.
3.  **Usage**:
    *   Use `c_void` pointers for memory buffers.
    *   **Mapped Memory**: RGA works best with DRM (Direct Rendering Manager) buffers or dma-buf for zero-copy. Passing standard `Vec<u8>` requires the driver to map it, which has slight overhead but is still faster than CPU processing for large images.

## 3. Deployment Notes (Docker)

To utilize these in Docker, you must pass the hardware devices:
```yaml
devices:
  - /dev/rga:/dev/rga
  - /dev/dri:/dev/dri  # Render nodes for NPU/GPU
```
And mount the user-space libraries if not installed in the container:
```yaml
volumes:
  - /usr/lib/aarch64-linux-gnu/librga.so:/usr/lib/librga.so
  - /usr/lib/aarch64-linux-gnu/librknnrt.so:/usr/lib/librknnrt.so

---

## See Also
*   **Implementation Strategy**: [File_Format_Analysis.md](File_Format_Analysis.md) (Standard CPU-based processing rules).
*   **Deployment Rules**: [deploy_on_radxa_rock5](../.agent/skills/deploy_on_radxa_rock5/SKILL.md) (Mandates for using NPU/RGA).
```
