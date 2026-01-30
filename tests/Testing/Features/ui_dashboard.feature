Feature: Standalone Web Dashboard
  As a user
  I want a premium web interface
  So that I can manage the model intake queue without external dependencies.

  # [Testing Strategy: testing_philosophy]
  # [Architecture: architectural_guidelines]

  Scenario: Dashboard is accessible on port 8080
    Given the Manyfold Processor service is running
    When I request the dashboard home page
    Then I should receive a successful visual response on port 8080
