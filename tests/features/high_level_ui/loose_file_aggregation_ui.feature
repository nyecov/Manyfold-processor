Feature: Loose File Aggregation (UI)

    Background:
        Given _API Processor is running
        And _API the input directory is cleared
        And Auto-Processing is disabled

    Scenario: Golden Path Aggregation Display
        When I copy "sophia_stl" to "input/big-model.stl"
        And I request processing of "big-model.stl" via API
        Then within 30 seconds, the WebUI Timeline should show "Processed: big-model"
