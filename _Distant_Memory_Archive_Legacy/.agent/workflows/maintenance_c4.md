---
description: Process for generating and updating C4 Model diagrams for the Manyfold Processor.
---

# Atomic Maintenance: Architecture Diagrams

This standalone workflow manages the maintenance of Mermaid-based C4 diagrams.

## 1. Context Gathering
*   Identify which C4 level (1-4) needs updating.
*   Read `architectural_guidelines` and `c4_model` for structural reference.

## 2. Diagram Drafting
*   Use Mermaid `graph TD` or `C4Context` syntax.
*   Follow project styling standards (boundaries vs. systems vs. components).

## 3. Insertion
*   Locate the relevant C4 section in the target documentation.
*   Insert or replace the Mermaid block.

## 4. Alignment Note
*   This tool only performs the **active update**. Verification of diagram consistency should be done via audit suites.
