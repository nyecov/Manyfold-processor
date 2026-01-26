Feature: 06 Auto-Process Safety Controls
  As a system administrator
  I want the processor to follow strict safety rules for auto-processing
  So that files are not processed unintentionally.

  Scenario: 06A - System Always Starts with Auto-Process OFF
    Given the processor docker container is stopped
    When I rebuild the processor container without cache
    Then the processor container should start successfully
    And the API should be ready within 30 seconds
    And the settings should show "auto_process" is "false"

  Scenario: 06B - Auto-Process Resets to OFF after completion
    Given the processor is running
    And the "input" directory is empty
    And the "staging" directory is empty
    And I have disabled auto-process
    When I copy "sophia-35mm-sophia.stl" from test source to staging
    When I enable auto-process
    And I wait for processing to complete
    Then the settings should show "auto_process" is "false"
    And the "input" directory should be empty
    And the "staging" directory should be empty
