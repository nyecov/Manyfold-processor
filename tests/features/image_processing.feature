Feature: Image Processing
    This feature verifies that images are correctly converted and associated with projects.

    Background:
        Given Processor is running
        And _API the input directory is cleared

    Scenario: Processing a static image to WebP
        When I copy "sophia_jpg" to "input/test_image.jpg"
        And I copy "sophia_stl" to "input/trigger.stl"
        And I request processing of "trigger.stl" via API
        Then a file "trigger/trigger_thumbnail.webp" should exist in the output directory
        And the WebUI Timeline should show "Processed: trigger"

    Scenario: Preserving an animated GIF
        When I copy "animated_gif_dave" to "input/animated.gif"
        And I copy "sophia_stl" to "input/trigger_anim.stl"
        And I request processing of "trigger_anim.stl" via API
        Then a file "trigger-anim/trigger-anim_thumbnail.gif" should exist in the output directory
        And the WebUI Timeline should show "Processed: trigger-anim"
