Feature: Intake Queue Management
    As a user
    I want a responsive and functional intake queue
    So that I can manage my staged models efficiently.

    Scenario: Queue UI is properly labeled and aligned
        Given Processor is running
        And the System is ready
        When I request the dashboard home page
        Then I should see a column "FILENAME"
        And the "STATUS" column should be horizontally aligned with its content

    Scenario: Queue supports large number of files with scrolling
        Given a large dataset is copied to the input directory
        When I request the dashboard home page
        Then the Intake Queue should be scrollable
        And the header should remain visible
        And _API the input directory is cleared

    Scenario: Deleting a file from the queue
        Given a file "accidental_upload.stl" is in the input directory
        When I request deletion of "accidental_upload.stl" via API
        Then the file "accidental_upload.stl" should be removed from the filesystem
        And the queue depth should decrease by 1
