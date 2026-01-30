Feature: 02 API Robustness and Error Reporting
  As a developer
  I want to ensure the API correctly reports status, settings, and errors
  So that I can rely on the system state programmatically

  Background:
     Given the processor is running
     And the "input" directory is empty
     And the "output" directory is empty
     And I have disabled auto-process

  Scenario: 02A - Verify Settings API
     When I request system settings
     Then the response should contain "auto_process"
     When I change the setting "auto_process" to "false"
     Then the settings should show "auto_process" is "false"
     When I change the setting "auto_process" to "true"
     Then the settings should show "auto_process" is "true"

  Scenario: 02B - Error Reporting Handling
     Given I have enabled auto-process
     When I create a corrupt file "corrupt.zip" in input
     And I wait 10 seconds for watchdog
     Then I should see an error reported for "corrupt.zip"
