---
description: Combined audit of documentation context and consistency.
---

# Orchestrated Suite: Documentation Audit

This suite performs a comprehensive audit of the project's documentation by invoking atomic standalones.

> [!NOTE]
> **Feedback Mandate**: After each step, provide a brief status update (✅/❌ + 1-line summary).

## 1. Semantic Audit
*   Invoke `/audit_context`
*   Verify that the documentation is correctly segmented into Strategy and Reference roles.
*   📢 **Report**: Context audit status

## 2. Logical Audit
*   Invoke `/audit_consistency`
*   Verify that there are no internal contradictions or conflicting mandates between peer documents.
*   📢 **Report**: Consistency audit status

## 3. Findings Consolidation
*   Synthesize reports from both atomics into a unified documentation health overview.
