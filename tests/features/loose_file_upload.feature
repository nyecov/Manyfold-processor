Feature: Loose File Upload

  Scenario: Sophia Loose Ingestion (Auto-Process OFF)
    Given _API Processor is running
    And Auto-Processing is disabled
    When I copy "test_resources/sophia/*" to "/input"
    Then within 5 seconds, the WebUI Queue Depth should be 2
    And the WebUI Timeline should show "Incoming: sophia-35mm-sophia.stl (Type: Loose)"
    And the WebUI Timeline should show "Incoming: 720X720-sophia-new.jpg (Type: Loose)"
    And System Memory Usage should be less than 50 percent
