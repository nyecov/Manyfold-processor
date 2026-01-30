---
description: Analyze the documentation and skills for internal logical errors, contradictions, and information conflicts.
---

1.  **Ingest Knowledge Base**:
    *   Use `list_dir` and `view_file` to read ALL contents of `docs/`, `.agent/skills/`, and `notes/`.

2.  **Analyze Logic & Consistency**:
    *   **Goal**: Verify the *integrity of the documentation itself*, not the project implementation.
    *   **Do NOT** check the project code/files against the docs (e.g. do not check Dockerfile or Compose against the guide).
    *   **DO** check the documents against *each other* and *themselves*.

3.  **Search for Contradictions**:
    *   **Cross-Document**: Does Document A prohibit something that Document B mandates? (e.g. "Use Rust" in guidelines vs "Use Python" in deployment).
    *   **Internal Logic**: Does a document's conclusion follow from its premises? (e.g. "Hardware is RAM-constrained" -> "Use low-memory language").
    *   **Information Conflicts**: Are version numbers, paths, and filenames consistent across all docs?

4.  **Report Findings**:
    *   Identify specific conflicting statements (quote them).
    *   Identify logical fallacies.
    *   Report "No logical conflicts found" if the documentation offers a consistent worldview.
