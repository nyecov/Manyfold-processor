---
description: Semantic audit of skill organization, chopping, and Strategy vs. Reference separation.
---

# Atomic Audit: Skill Context

This standalone workflow evaluates the semantic organization of the project's knowledge base.

## 1. Skills Inventory
*   List all files in `.agent/skills/`.
*   Identify primary functional domains (e.g., Geometry, API, Testing, Operations).

## 2. Cluster Analysis
*   Identify skills that share the same functional domain.
*   Verify the **Separation of Concerns**:
    *   **Strategy**: High-level mandates and "Whys".
    *   **Reference**: Technical specifications and "Hows".
*   Flag if implementation details are polluting strategic skills or vice-versa.

## 3. Chopping & Granularity
*   Evaluate if a skill is too broad (serving too many purposes).
*   Check if information is scattered across too many small files (fragmentation).

## 4. Logical Alignment
*   Ensure that the "Source of Truth" for a specific topic is clear.
*   Flag contradictions between peer skills.

## 5. Report
*   Suggest specific content migrations or skill merges/splits for better clarity.
