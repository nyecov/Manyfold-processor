@satellite_audit
Feature: Satellite Structural & State Audit
    As a Nomos System Auditor
    I want to verify the structural and stateful integrity of the distribution bridge
    So that I can ensure it correctly handles API responses, persona configs, and memory lifecycles

    Background:
        Given the satellite AI bridge is online
        And the state directory is correctly configured

    # --- Stage 1: Structural Integrity ---

    Scenario: 1A - Agnostic Connectivity
        Given the persona is set to "Neutral"
        When I send a "ping" command to the satellite
        Then the response should be a valid JSON object
        And the "metadata" block must contain "timecode", "satellite_persona", and "satellite_model"

    Scenario: 1B - Persona Config Parsing
        Given I update "registry:persona_config" with a "Test-Persona"
        And the "Test-Persona" has "required_skills" ["test_dummy_happy"]
        When I invoke the bridge with persona "Test-Persona"
        Then the "metadata.satellite_persona" should be "Test-Persona"
        And the bridge should automatically inject the "Happy Path" skill text

    # --- Stage 2: Memory Lifecycle ---

    Scenario: 2A - Memory Persistence (Chat History)
        Given the persona is set to "Neutral"
        And the memory for persona "Neutral" is cleared
        When I tell the satellite "The secret code is 123456"
        And I ask "What is the secret code?"
        Then the response should contain "123456"

    Scenario: 2B - Memory Isolation (Persona Separation)
        Given the persona is set to "Neutral"
        And I tell the satellite "My name is Alice"
        When I switch to persona "Librarian"
        And I ask "What is my name?"
        Then the response should NOT contain "Alice"
        And it should identify me based on the Librarian's project context instead

    Scenario: 2C - Memory Clear Functionality
        Given the persona is set to "Neutral"
        And I tell the satellite "Remember this: Blue Rabbit"
        When I invoke the bridge with "--clear-memory"
        And I ask "What was the color of the rabbit?"
        Then the response should indicate it doesn't know or remember
