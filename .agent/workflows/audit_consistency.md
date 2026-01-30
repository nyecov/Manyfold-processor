---
description: Logical audit of internal and cross-document mandate consistency.
---

# Atomic Audit: Document Consistency

This standalone workflow verifies the logical integrity of the documentation itself.

## 1. Internal Consistency
*   Verify that a document's conclusions follow from its premises.
*   Identify missing details or placeholders (e.g., TODOs, [TBD]).

## 2. Cross-Document Mandates
*   Scan for contradictions between documents (e.g., Doc A says "Use X", Doc B says "Do not use X").
*   Verify that version numbers, paths, and filenames are consistent across the entire knowledge base.

## 3. Risk Identification
*   Flag logical fallacies in architectural reasoning.
*   Identify outdated or orphaned guidelines that no longer align with the current project state.

## 4. Report
*   List all identifying specific conflicting statements with quotes and file paths.
