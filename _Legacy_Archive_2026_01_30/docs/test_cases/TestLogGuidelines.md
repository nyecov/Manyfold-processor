# Test Logging Guidelines

## 1. File Location & Naming
*   **Location**: The log file MUST be located in the same directory as the Test Case definition (`.md`) and script (`.py`).
*   **Naming**: It MUST verify the same base name.
    *   Example: `TestCase1.md` -> `TestCase1.py` -> `TestCase1.log`

## 2. Initialization (Rolling Log)
*   **Overwrite**: The log file MUST be cleared/overwritten at the start of each test run.
*   **Header**: Start with a clear header indicating the file name or test case.
    ```python
    with open(LOG_FILE, "w") as f:
        f.write(f"=== {LOG_FILE} ===\n")
    ```

## 3. Logging Format
*   **Timestamps**: Every line MUST be prefixed with a timestamp.
    *   Format: `[YYYY-MM-DD HH:MM:SS]`
*   **Dual Output**: Logs should be printed to `stdout` (for console) AND written to the file.
    ```python
    def log(msg):
        timestamp = time.strftime("%Y-%m-%d %H:%M:%S")
        formatted = f"[{timestamp}] {msg}"
        print(formatted)
        with open(LOG_FILE, "a") as f:
            f.write(formatted + "\n")
    ```

## 4. Content Requirements
### A. Execution Steps
Log major actions clearly:
*   `[Setup] Stopping Docker...`
*   `[Clean] Clearing Input/Output...`
*   `[Action] Copying files...`

### B. Success Criteria Verification
You MUST explicitly verify and log **each** success criterion defined in the `.md` file. Use Granular Checks.
*   **Pass**: Use `✅ [Pass]` prefix.
*   **Fail**: Use `❌ [Fail]` prefix.

**Example**:
```text
--- Verifying Success Criteria ---
✅ [Pass] Output Folder exists: output/sophia-35mm-sophia/
✅ [Pass] datapackage.json exists.
✅ [Pass] sophia-35mm-sophia.3mf exists.
❌ [Fail] WebP file missing.
```

### C. Metrics (If Applicable)
For benchmarks, log the raw numbers used for calculation:
*   Duration (seconds)
*   Input Size (MB)
*   Calculated Speed (sec/MB)
*   Resource usage (CPU/RAM)

## 5. Final Summary
End the log with a clear verdict:
*   `🎉 TEST SUCCESS: All criteria met.`
*   `🚫 TEST FAILED: One or more criteria failed.`
