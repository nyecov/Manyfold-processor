Feature: 01 System Startup Benchmark
  As a developer
  I want to verify that the Manyfold Processor container builds and starts without errors
  So that I can ensure system reliability and track performance.

  Scenario: 01A - Cold Build and Startup Benchmark
    Given the processor docker container is stopped
    When I rebuild the processor container without cache
    Then the build time should be recorded
    And the processor container should start successfully
    And the API should be ready within 30 seconds
    And the startup time should be recorded
    And the logs should contain the "MANYFOLD PROCESSOR - SYSTEM ONLINE" banner
