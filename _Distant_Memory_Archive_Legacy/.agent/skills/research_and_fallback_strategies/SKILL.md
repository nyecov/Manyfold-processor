---
name: Research and Fallback Strategies
description: Guidelines for handling web search failures and alternative research methods.
---

# Research and Fallback Strategies

## 1. When Web Searches Fail

Web search API failures (e.g., "no parts returned from GenerateContent") are intermittent infrastructure issues, not user errors.

### Immediate Response
*   **Do NOT retry endlessly** - After 2-3 failed attempts, pivot to alternatives.
*   **Communicate transparently** - Inform the user that searches failed and explain the fallback approach.
*   **Continue progress** - Don't block the entire task on search failures.

## 2. Alternative Research Methods

### A. Internal Knowledge Base
*   **When Sufficient**: Stable specifications (STL, 3MF, OBJ), well-known Rust crates, established algorithms.
*   **Confidence Check**: If the information is from a stable spec or widely-adopted standard, internal knowledge is reliable.

### B. Project Archive Analysis
*   **Legacy Code**: Examine `_Legacy_Archive_*/src/` for previous implementation patterns.
*   **Documentation**: Check `_Legacy_Archive_*/Informations/` for prior research notes.
*   **Example**: The token-matching sibling logic for STL files was derived from `stl_project_plugin.py`.

### C. Specification Documents
*   **Direct Sources**: For formats like 3MF, reference the official spec at [3mf.io](https://3mf.io/).
*   **Rust Crate Docs**: Use `docs.rs` links (can be constructed without search: `https://docs.rs/<crate-name>`).

## 3. Decision Matrix

| Research Need | Web Search Critical? | Fallback |
|---------------|---------------------|----------|
| File format specs (STL, 3MF) | No | Internal knowledge + spec docs |
| Rust crate capabilities | No | Construct docs.rs URL |
| Cutting-edge features (new 3MF extensions) | Yes | Defer or ask user for manual research |
| Hardware-specific APIs (RGA, NPU) | Maybe | Check legacy code first |
| Algorithm implementations | No | Standard CS knowledge |

## 4. Communication Protocol

### When Searches Fail
1.  **Acknowledge**: "Web search failed due to API error."
2.  **Explain Pivot**: "Using internal knowledge of [X] and legacy code analysis."
3.  **Confidence Statement**: "This approach is reliable because [reason]."
4.  **Offer Alternative**: "If you need cutting-edge info, I can retry later or you can provide sources."

### When to Ask User
*   If the research is **critical** (e.g., security vulnerability in a crate).
*   If **multiple approaches** exist and user preference is needed.
*   If the fallback method has **lower confidence** than desired.

## 5. Quality Assurance

### Validate Fallback Results
*   **Cross-reference**: Check multiple internal sources (e.g., crate name + algorithm name).
*   **Sanity Check**: Does the solution align with project constraints (Rust-only, low-memory)?
*   **Document Assumptions**: Note in the skill/doc if information is based on general knowledge vs. verified spec.

## 6. Example: STL/3MF Skills

**Search Failure**: All queries for "3mf error correction" and "rust stl repair" failed.

**Fallback Used**:
*   **3MF**: Internal knowledge of 3MF spec (ZIP structure, `mustHonor` attributes).
*   **STL**: Legacy `stl_project_plugin.py` showed token-matching logic.
*   **Rust Crates**: Known ecosystem (`stl_io`, `tri-mesh`, `quick-xml`).

**Result**: Comprehensive skills created without web search, validated against legacy implementation.
