## Scenario 1A: Auto-Process (Startup/Watchdog)
1.  **Condition**: `auto_process: true`.
2.  **Action**: Copy `sophia.stl` + `sophia.jpg` to `/input`.
3.  **Expected**: Watchdog detects and processes immediately to `/output/sophia/`.

## Scenario 1B: Manual Process (Manual Queue)
1.  **Condition**: `auto_process: false`.
2.  **Action**: Copy `sophia.stl` + `sophia.jpg` to `/input`.
3.  **Expected**: 
    - Watchdog moves files to `/staging`.
    - User clicks **"Process Loose Files"** in UI.
    - Files process to `/output/sophia/`.

## Input Data
1.  `sophia-35mm-sophia.stl` (Model)
2.  `720X720-sophia-new.jpg` (Sibling Image)

## Success Criteria
*   [ ] **1A**: STL+JPG group and process automatically when toggle is ON.
*   [ ] **1B**: STL+JPG group and process only after manual click when toggle is OFF.
*   [ ] Output folder `sophia-35mm-sophia` contains `.3mf`, `.webp`, and `datapackage.json`.

## Execution Command
```bash
# 1. Clean
rm -rf Playground/input/* Playground/staging/* Playground/output/*

# 2. Stage
cp Playground/test/TestCase1/* Playground/input/

# 3. Run
docker compose up -d --build
```
