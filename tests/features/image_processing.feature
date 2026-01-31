Feature: Image Processing
    As a user
    I want to process images on demand
    So that they are optimized for web usage

    Scenario: Processing a static image to WebP
        Given Processor is running
        And a file "test_image.jpg" is in the input directory
        When I request processing of "test_image.jpg" via API
        Then a file "test_image.webp" should exist in the output directory
        And the WebUI Timeline should show "Processed: test_image.jpg -> test_image.webp"

    Scenario: Preserving an animated GIF
        Given Processor is running
        And a file "animated.gif" is in the input directory
        When I request processing of "animated.gif" via API
        Then a file "animated.gif" should exist in the output directory
        And the WebUI Timeline should show "Processed: animated.gif -> animated.gif (Preserved Animation)"
