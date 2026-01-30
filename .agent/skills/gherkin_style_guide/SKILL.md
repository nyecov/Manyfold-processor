---
name: Gherkin Style Guide
description: Methodological standards for writing clear, unique, and maintainable Gherkin scenarios.
---

# Gherkin Style Guide: Standards & Methodology

<!-- audited_by: .agent/workflows/audit_gherkin.md -->

This guide defines the "how" of writing executable specifications to ensure logical clarity and AI readability.

## 1. Writing Quality Scenarios
*   **Behavior, Not Implementation**: Describe *what* happens, not *how* (e.g., "When I save the model" vs "When I click the green button").
*   **Business Language**: Use a shared vocabulary that stakeholders can understand.
*   **Scenario Limits**: Maximum **5 scenarios** per `.feature` file.
*   **Step Count**: Aim for 3–7 steps per scenario.

## 2. Layering & Prefixes
To distinguish interaction layers while maintaining logical grouping:
*   **UI Actions**: Use standard Gherkin keywords (`Given`, `When`, `Then`).
*   **API Actions**: Use prefixed Gherkin keywords (`Given_API`, `When_API`, `Then_API`).

## 3. Uniqueness & Reuse
*   **Unique Steps**: `Given/When/Then` sentences must be unique for different logical intents.
*   **Sentence Reuse**: A UI step and an API step MAY share the exact same sentence (e.g., `Given the service is running` and `Given_API the service is running`) to maintain layer independence.

## 4. Twin Linking (AI Traceability)
Corresponding UI and API steps MUST be cross-linked for AI agent recognition.
*   **Format**: Use `# [twin: Step Name]` in the Gherkin step.
*   **Example**: `Given the service is running # [twin: Given_API the service is running]`

---

## See Also
*   **Governance**: [testing_philosophy](../testing_philosophy/SKILL.md)
*   **Reference**: [cucumber_rust_reference](../cucumber_rust_reference/SKILL.md)
