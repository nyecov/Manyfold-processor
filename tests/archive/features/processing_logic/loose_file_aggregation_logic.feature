Feature: Loose File Aggregation (Logic)
    This feature handles the backend logic for loose file aggregation.

    Background:
        Given _API Processor is running
        And _API the input directory is cleared
        And Auto-Processing is disabled

    Scenario: Automatic Aggregation Logic
        When I copy "sophia_stl" to "input/big-model.stl"
        And I copy "sophia_stl" to "input/small-accessory.stl"
        And I copy "sophia_jpg" to "input/small-accessory.jpg"
        And I request processing of "big-model.stl" via API
        Then a project folder "big-model" should be created in output
        And _API project "big-model" metadata "name" should be "big-model"

    Scenario: Naming Penalty Logic
        When I copy "sophia_stl" to "input/v1-latest-part.stl"
        And I copy "sophia_stl" to "input/base_stand.stl"
        And "base" is in the system naming penalties
        And I request processing of "v1-latest-part.stl" via API
        Then the project folder name should be "v1-latest-part"

    Scenario: Image Association Logic
        When I copy "sophia_stl" to "input/model.stl"
        And I copy "animated_gif_dave" to "input/animated_instruction.gif"
        And I request processing of "model.stl" via API
        Then _API project "model" metadata "/resources/1/name" should be "model_thumbnail"

# Twin-UI: tests/features/high_level_ui/loose_file_aggregation_ui.feature
