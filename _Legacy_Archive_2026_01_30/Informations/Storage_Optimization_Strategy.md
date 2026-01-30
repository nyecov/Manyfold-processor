# Storage Optimization Strategy (I/O Reduction)

Minimizing write operations is critical for preserving consumer SSDs and SMR HDDs, especially in specific automation pipelines. This document outlines strategies to achieve **Minimal Write Amplification**.

## 1. The Problem: Write Amplification
In the current "Naive" pipeline:
1.  **Extract**: A 1GB ZIP is extracted to `/temp` (1GB Writes).
2.  **Process**: `stl23mf` reads meshes, writes a new `.3mf` (500MB Writes).
3.  **Move**: Final files are moved to `/output` (500MB Writes).
4.  **Cleanup**: `/temp` is deleted.
*   **Total Written**: ~2GB for a 500MB result.

## 2. Solution: RAM Drive (tmpfs)
The Radxa Rock 5 ITX has **16GB RAM**. We can leverage this to absorb all intermediate writes.

### A. Docker Configuration
Mount a `tmpfs` volume for the working directory / staging area.
```yaml
services:
  processor:
    tmpfs:
      - /app/temp:size=4G,mode=1777
```

### B. Impact Analysis
*   **Extraction**: ZIP extraction happens in RAM. (0 Disk Writes).
*   **Intermediate Processing**: `stl23mf` generates the `.3mf` in RAM. (0 Disk Writes).
*   **Final Output**: Only the *final, compressed* artifact is written to the physical storage (SSD/HDD).
*   **Result**: 
    *   **Writes Reduced by**: **~75%** (Only the final file hits the disk).
    *   **Speed**: IOPS becomes infinite (DDR4 speed vs SATA/NVMe).

## 3. Advanced Optimization: Streaming Pipeline

### A. In-Memory Image Conversion
*   **Current**: Extract `image.jpg` -> Read -> Convert -> Write `image.webp`.
*   **Optimized**:
    1.  Read `image.jpg` bytes directly from ZIP stream (Python `zipfile`).
    2.  Decode in Memory (Pillow/RGA).
    3.  Encode to WebP in Memory buffer.
    4.  Write only `image.webp` to RAM-Buffer or Output.
*   **Status**: Requires code changes in `ImageProcessor`.

### B. Streaming Geometry (Z-Pipe)
*   **Concept**: Pipe the output of the STL converter directly into the 3MF ZIP stream without creating a temporary `.3mf` file on disk first.
*   **Complexity**: High. `stl23mf` currently outputs a file.
*   **Verdict**: Stick to **RAM Drive** (Solution 2) as it achieves the same benefit without code complexity.

## 4. Implementation Checklist

1.  **Modify `compose.yml`**: Add `tmpfs` mount to the container.
2.  **Update Config**: Ensure the application uses the RAM-mounted path for `working_dir`.
3.  **Log Reduction**:
    *   Set Log Level to `WARNING` in production to stop writing "Processing..." logs to disk.
    *   Direct logs to `stdout` (Docker logging driver handles rotation) rather than a file inside the container.

## 5. Recommendation
**Implement Solution 2 (RAM Drive) immediately.**
It requires **zero code changes**—only a configuration change in `docker-compose.yml`—and instantly eliminates 100% of temporary file wear on your SSD.
