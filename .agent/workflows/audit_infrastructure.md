---
description: Systems audit of codebase and infrastructure alignment with design truth.
---

# Atomic Audit: Infrastructure & System

This standalone workflow verifies that the actual project implementation matches architectural mandates.

> [!NOTE]
> **Feedback Mandate**: After each check, provide a brief status update (✅/❌ + 1-line summary).

## 1. Codebase Alignment
*   Verify language usage (Rust primary, minimal Python).
*   Check for dependency compliance in `Cargo.toml`.
*   Verify standard error handling (No panics).
*   📢 **Report**: ✅/❌ Codebase compliance status

## 2. Infrastructure Alignment
*   Check `Dockerfile` for mandatory build dependencies and optimizations.
*   Check `compose.yml` for volume mounts and resource constraints (per `environment_constraints` skill).
*   📢 **Report**: ✅/❌ Infrastructure compliance status

## 3. Mandate Verification
*   Flag any direct database sharing or internal linking (Architecture break).
*   Verify that API integrations follow resilience standards (if detectable).
*   📢 **Report**: ✅/❌ Mandate compliance status

## 4. Report
*   List alignment violations with risk assessment (High/Low).
