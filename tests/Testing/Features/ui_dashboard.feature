Feature: Standalone Web Dashboard
  As a user
  I want a premium web interface
  So that I can manage the model intake queue without external dependencies.

  # (Testing Strategy: [testing_philosophy](../../../.agent/skills/testing_philosophy/SKILL.md))
  # (Architecture: [architectural_guidelines](../../../.agent/skills/architectural_guidelines/SKILL.md))

  Scenario: Dashboard is accessible on port 8080
    Given the Manyfold Processor service is running # [twin: Given_API the Manyfold Processor service is running]
    When I request the dashboard home page # [twin: When_API I request the status from the API]
    Then I should receive a successful visual response on port 8080 # [twin: Then_API I should receive a status code of 200]
