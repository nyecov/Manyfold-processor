Feature: Standalone Web Dashboard
  As a user
  I want a premium web interface
  So that I can manage the model intake queue without external dependencies.

  # [Testing Strategy: testing_philosophy]
  # [Architecture: architectural_guidelines]

  Scenario: Dashboard is accessible on port 8080
    Given Processor is running
    And the System is ready
    When I request the dashboard home page
    Then I should receive a successful visual response on port 8080
    When I click the "Clear Timeline" button
    Then the Timeline should be empty
