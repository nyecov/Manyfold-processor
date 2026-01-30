---
name: Testing Philosophy
description: High-level strategy and governing mandates for Behavior Driven Development (BDD).
---

# Testing Philosophy: Strategy & Governance

The software behavior is governed by a **Dual-Track Verification** strategy, ensuring reliability across both user-facing and internal interfaces.

## 1. Core Mandates
*   **Dual-Track Verification**: Every feature MUST be verified through both **UI Testing** (End-to-End) and **API Testing** (Logic/Data).
*   **Tests are Specifications**: The "truth" of what the software does is defined in `.feature` files.
*   **Hardware Tier Strategy**:
    *   **Tier 2 (Standard)**: Primary target for automated verification (Docker/CI).
    *   **Tier 1/3 (Hardware)**: Specific verification is deferred until v1.0.

## 2. Speed vs. Reliability
*   **API Substitution Policy**: Once a feature is verified by a foundational UI test, subsequent tests may use **API-based steps** to bypass slow UI interactions for setup or verification.
*   **Twin Linking Intent**: UI and API layers must remain logically connected to allow for automated test optimization and high-speed fallback.

## 3. Directory Structure
```
tests/
└── Testing/
    ├── Features/       <-- Executable specifications
    └── steps/          <-- Rust implementation
```

---

## See Also
*   **Methodology**: [gherkin_style_guide](../gherkin_style_guide/SKILL.md) (How to write features).
*   **Implementation**: [cucumber_rust_reference](../cucumber_rust_reference/SKILL.md) (Rust technical lookup).
