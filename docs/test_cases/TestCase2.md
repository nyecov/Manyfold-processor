# Test Case 2: Performance Benchmark (Phase 2b Conversion)

## Description
Measure the system resources (CPU, RAM) and time required to process a standard dataset (STL + Image) using the Phase 2b conversion pipeline (STL->3MF, JPG->WebP).

## Setup
*   **Source**: `/Project Repositories/Manyfold-processor/Playground/test/TestCase1` (Standard "Sophia" dataset)
*   **Destination**: `/Project Repositories/Manyfold-processor/Playground/input`
*   **Condition**: Files are pre-seeded before startup.

## Metrics to Measure
1.  **Total Processing Time**: From "Watchdog Detection" to "Artifacts Moved".
2.  **Conversion Speed**: Seconds per MB of input STL.
3.  **Peak CPU**: usage % during conversion.
4.  **Peak RAM**: usage (MB) during conversion.

## Input Data Profile
*   **Main File**: `sophia-35mm-sophia.stl` (~100 MB)
*   **Sibling**: `720X720-sophia-new.jpg` (~150 KB)

## Execution Method
Run the automated benchmark script:
```bash
python3 Playground/benchmark.py
```

## Success Criteria
*   [ ] Benchmark script runs to completion.
*   [ ] JSON Report generated with processing time and resource usage stats.
*   [ ] Conversion Speed (sec/MB) is calculated.
*   [ ] **Max Duration**: Processing time for ~100MB input must be **< 10 minutes** (600 seconds).
