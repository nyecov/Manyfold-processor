---
description: detailed audit of hardcoded values versus canonical constants.
---

# Atomic Audit: Canonical Data (Constants)

This standalone workflow ensures that shared project values use the canonical `constants.yml` source of truth rather than being hardcoded in skills or code.

## 1. Inventory
*   Read `.agent/constants.yml` to load the canonical values (Memory, Network, Version, etc.).

## 2. Scan for Violations
Search all `SKILL.md` and `docs/*.md` files for hardcoded duplication of canonical values.

### Common Violations to Flag
*   **Memory Values**: Occurrences of `1G`, `5G`, `750M` (outside of `constants.yml` itself).
*   **Network Config**: Occurrences of `192.168.2.2`, `8080`, `dev` (SSH user).
*   **Hardware Tiers**: Definitions of "Tier 1", "Tier 2", "Tier 3" that deviate or don't link to `environment_constraints`.

## 3. Verification Criteria
*   **Rule**: If a skill needs a value, it should reference `constants.yml` or the `environment_constraints` skill.
*   **Exception**: `environment_constraints` skill *is* the documentation for these values, so it may contain them (but should ideally pull from or mirror `constants.yml`).

## 4. Report
*   List files with hardcoded "Magic Numbers/Strings".
*   Recommend replacing with: `See [constants.yml](../../constants.yml)`.
