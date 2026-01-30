---
description: Verify that the project codebase aligns with the design documentation, flagging contradictions but ignoring missing features.
---

1.  **Ingest Design Truth (Documentation)**:
    *   Use `list_dir` and `view_file` to read ALL contents of `docs/`, `.agent/skills/`, and `notes/`.
    *   Identify key constraints (e.g., "Use Rust", "750MB Limit", "Use librga", "No panics").

2.  **Ingest Implementation Reality (Code)**:
    *   Use `list_dir` and `view_file` to examine:
        *   `Cargo.toml` (Dependencies).
        *   `Dockerfile` & `docker-compose.yml` (Infrastructure).
        *   `src/**/*.rs` (Application Logic).
        *   `tests/` (Verification Logic).

3.  **Analyze Alignment**:
    *   **Goal**: Find *Contradictions*, not *Gaps*.
    *   **Violation Examples (Flag these)**:
        *   Docs say "Use `reqwest-middleware`" but Code uses raw `reqwest`.
        *   Docs say "750MB RAM Reservation" but Dockerfile/Compose misses the flag.
        *   Docs say "No Python" but a `.py` file performs core logic.
    *   **Non-Violations (Ignore these)**:
        *   Docs mention "File Monitoring" but `src/monitor.rs` doesn't exist yet (Feature not built).

4.  **Report Findings**:
    *   **Alignment Violations**: List specific code/config that contradicts a design mandate.
    *   **Risk Assessment**: High (Architecture break) vs Low (Minor config drift).
    *   Report "Code implementation aligns with known design constraints" if no contradictions found.
