@satellite @ui @diagnostics
Feature: Satellite UI Diagnostics
  As a Nomos Operator
  I want to run automated diagnostics on the Satellite WebUI
  To ensure the cognitive link and UI states are functioning correctly

  Background:
    Given the Satellite UI is healthy and warmed up

  Scenario: STATE_SYNC_STRESS - Verify Header Color Transitions
    When I trigger the "STATE_SYNC_STRESS" diagnostic scenario
    Then the terminal status indicator should cycle through "GENERATING", "ERROR", and "READY"
    And the diagnostic results table should report "PASS" for "STATE_SYNC_STRESS"
    And the memory bank should be verified as PRISTINE

  Scenario: PURGE_PERSISTENCE - Verify Clear View vs Backend State
    When I trigger the "PURGE_PERSISTENCE" diagnostic scenario
    Then the terminal display should be cleared
    And a system message "VIEW_PURGED // STATE_INTACT" should be visible
    And the diagnostic results table should report "PASS" for "PURGE_PERSISTENCE"
    And the memory bank should be verified as PRISTINE

  Scenario: SERIAL_QUEUING - Verify Multi-Persona Batch Resolution
    When I trigger the "SERIAL_QUEUING" diagnostic scenario
    Then the terminal should log mock responses for "Librarian", "Architect", and "Critic"
    And the diagnostic results table should report "PASS" for "SERIAL_QUEUING"
    And the memory bank should be verified as PRISTINE

  Scenario: READABILITY_VALIDATION - Verify Markdown and Noise Handling
    When I trigger the "READABILITY_VALIDATION" diagnostic scenario
    Then the terminal should log a structured markdown response
    And a system message "### Diagnostic Report" should be visible
    And the terminal log should NOT contain "{" or "}"
    And the diagnostic results table should report "PASS" for "READABILITY_VALIDATION"
    And the memory bank should be verified as PRISTINE
