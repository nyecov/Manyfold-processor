# Test Case 3: Live Arrival & Sequential Copying

## Description
Verify that the Watchdog service correctly handles files arriving while the system is running ("Live"), including burst handling and sequential delays.

## Scenarios

### 3A: Simultaneous Arrival (Burst)
*   **Action**: Copy all files (`sophia.stl`, `sophia.jpg`) into `/input` at the same instant (wildcard copy).
*   **Expected Behavior**: The system should group them correctly into a single output package.

### 3B: Sequential Arrival (Lag)
*   **Action**: 
    1.  Copy `sophia.stl`.
    2.  Wait 2-5 seconds.
    3.  Copy `sophia.jpg`.
*   **Expected Behavior (Requirement)**: The Processor must "hold out" (debounce) until the incoming stream stops, ensuring the STL and JPG are processed together.
*   **Failure Mode**: If processed immediately, the STL might be processed alone, leaving the JPG orphaned.

## Setup
*   **Source**: `/Project Repositories/Manyfold-processor/Playground/test/TestCase1`
*   **Condition**: Container is ALREADY RUNNING before files are copied.

## Success Criteria
*   [ ] **3A**: Single folder `/output/sophia-35mm-sophia` containing `.3mf`, `.webp`, `.json`.
*   [ ] **3B**: Single folder `/output/sophia-35mm-sophia` containing `.3mf`, `.webp`, `.json`.
