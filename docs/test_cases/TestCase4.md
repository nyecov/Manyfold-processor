# Test Case 4: Auto-Process Toggle & Queue Verification

## Description
Verify that the "Auto-Process" feature can be disabled to hold files in a staging queue, and then enabled to process them in bulk.

## Setup
*   **Source**: `/Project Repositories/Manyfold-processor/Playground/test/TestCase1` (Standard "Sophia" dataset)
*   **Destination**: `/Project Repositories/Manyfold-processor/Playground/input`
*   **Condition**: Container running.

## Scenario 4a: Auto-Process Toggle (Batch)
1.  **Disable Auto-Process**: valid API call to set `auto_process: false`.
2.  **Input Files**: Copy files to `/input`.
3.  **Verify Queue**: Files held in `/staging`.
4.  **Enable Auto-Process**: valid API call to set `auto_process: true`.
5.  **Verify Output**: Batch processed.

## Scenario 4b: Late Sibling (Busy Staging Isolation)
1.  **Setup**: Auto-Process ON. Staging Empty.
2.  **Action 1**: Copy `.stl` (Model) to `/input`.
    *   *Check*: Moves to `.staging` and starts processing.
3.  **Action 2 (Lag)**: While STL is processing (wait 5-10s), copy `.jpg` (Sibling) to `/input`.
    *   *Check*: JPG **remains in `/input`** (Staging Lock active).
4.  **Wait**: Wait for STL completion.
    *   *Check*: Output folder `sophia-35mm-sophia` contains `.3mf` + `.json` (NO `.webp`).
5.  **Finalize**: Once Staging clears, JPG moves to Staging -> Output.
    *   *Check*: Output folder `720X720-sophia-new` contains `.webp` (NO `datapackage.json`).

## Success Criteria
*   [ ] 4a: API successfully toggles settings and flushes queue.
*   [ ] 4b: JPG is locked in Input while STL processes.
*   [ ] 4b: STL processes alone (missing sibling).
*   [ ] 4b: JPG processes alone after delay.

## Execution Command
```bash
python3 TestCases/test_case_4.py
```
