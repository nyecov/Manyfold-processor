# Project Development Guidelines

## 🧠 Philosophy & Workflow
1.  **Logic First, Code Last**:
    *   The primary focus must be on **high-level logic**, data flow, and **documentation**.
    *   **Do not write implementation code** until the architecture and logic are fully documented and approved.
    *   Writing actual Python/Shell scripts should be one of the *last* steps in the process.

2.  **Context Awareness**:
    *   Always consult the `/Informations` folder (e.g., `/Project Repositories/Manyfold-processor/Informations/`) before proposing changes.
    *   Use existing findings (like database schema analysis) to inform logical decisions.

3.  **Task Granularity**:
    *   Break tasks into small, manageable components.
    *   Avoid monolithic "solve everything" steps.

4.  **Use Case Dependency Hierarchy**:
    *   **Case 1 (STL)** is a logical subset of **Case 3 (Generic 3MF)**. 
        *   *Reasoning*: An STL must be converted to 3MF, at which point it becomes a "Generic 3MF" problem (needs metadata injection + JSON generation).
    *   **Case 4 (Mixed Archives)** is a superset of all others.
        *   *Constraint*: Do **NOT** attempt to solve Case 4 until Case 2 (MakerWorld) and Case 3 (Generic/Converted) are fully resolved and tested.

## 📉 Optimization Goals
*   **Storage Efficiency**: A key secondary goal is to reduce physical disk usage.
    *   Converting STLs to 3MF is not just for compatibility, but for **compression**.
    *   The system should prefer 3MF storage over raw STL wherever possible.
