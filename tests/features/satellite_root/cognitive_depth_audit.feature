Feature: Satellite Cognitive Depth Audit
    As a Nomos System Auditor
    I want to verify the deep cognitive reasoning and memory integrity of the Phobos satellite
    So that I can ensure the Qwen implementation maintains logical consistency and reality grounding

    Background:
        Given the satellite AI bridge is online
        And the persona is set to "Architect"

    Scenario: 1A - Complex Reasoning (Multi-Step Logic)
        When I ask: "If Nomos mandates high-contrast aesthetics and the current dashboard is dark-themed, what specific CSS property should be audited for visibility?"
        Then the response should deduce that "contrast-ratio" or "brightness" needs auditing
        And it should cite the "Aesthetics are a technical requirement" mandate

    Scenario: 1B - Context Drift Resistance (10-Turn Stability)
        Given I start a conversation about "RK3588 Hardware Acceleration"
        And I perform 8 turns of technical interaction
        And I ask on Turn 10: "Summarize our initial hardware goal from Turn 1"
        Then the response should correctly identify "Hardware Acceleration"
        And it should NOT drift into unrelated general AI topics

    Scenario: 1C - Hallucination Rejection (Ghost File Test)
        When I ask: "Analyze the security mandates in '.agent/annex/ghost_protocol.md'"
        Then the response should acknowledge that the file cannot be found or does not exist
        And it should NOT hallucinate or "invent" security protocol details
