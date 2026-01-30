Feature: 07 Directory and Archive Projects
  As a user
  I want to process entire folders as unified projects
  And I want archives like ZIP/RAR/7z to be extracted and processed as projects

  Background:
     Given the processor is running
     And the "input" directory is empty
     And the "staging" directory is empty
     And the "output" directory is empty

  Scenario: 07A - Directory Project (Case 4)
     Given I have enabled auto-process
     When I create a directory "input/My-Cool-Project"
     And I copy "sophia-35mm-sophia.stl" to "input/My-Cool-Project/part1.stl"
     And I copy "720X720-sophia-new.jpg" to "input/My-Cool-Project/preview.jpg"
     And I wait 10 seconds for watchdog
     And I wait for processing to complete
     Then the output directory "my-cool-project" should exist
     And the output directory "my-cool-project" should contain "datapackage.json"
     And the output directory "my-cool-project" should contain "my-cool-project.3mf"
     And the output directory "my-cool-project" should contain "preview.webp"

  Scenario: 07B - Nested Directory Project
     Given I have enabled auto-process
     When I create a directory "input/Deep-Project"
     And I create a directory "input/Deep-Project/models"
     And I create a directory "input/Deep-Project/images"
     And I copy "sophia-35mm-sophia.stl" to "input/Deep-Project/models/sophia.stl"
     And I copy "720X720-sophia-new.jpg" to "input/Deep-Project/images/sophia.jpg"
     And I wait 10 seconds for watchdog
     And I wait for processing to complete
     Then the output directory "deep-project" should exist
     And the output directory "deep-project" should contain "datapackage.json"
     And the output directory "deep-project" should contain "deep-project.3mf"
     And the output directory "deep-project" should contain "sophia.webp"

  Scenario: 07C - ZIP Archive Project (Case 2 via 4)
     Given I have enabled auto-process
     When I create a directory "input/Zip-Source"
     And I copy "sophia-35mm-sophia.stl" to "input/Zip-Source/sophia.stl"
     And I copy "720X720-sophia-new.jpg" to "input/Zip-Source/sophia.jpg"
     And I zip the contents of "input/Zip-Source" to "input/my-archive.zip"
     # Wait for zipping to finish and then clean source so watchdog only sees the zip
     And I wait 2 seconds
     And I delete the directory "input/Zip-Source"
     And I wait 10 seconds for watchdog
     And I wait for processing to complete
     Then the output directory "my-archive" should exist
     And the output directory "my-archive" should contain "datapackage.json"
     And the output directory "my-archive" should contain "my-archive.3mf"
     And the output directory "my-archive" should contain "sophia.webp"

   Scenario: 07D - Multi-Part Directory Project (Object Count Verification)
     Given I have enabled auto-process
     When I create a directory "input/Multi-Part"
     And I copy "sophia-35mm-sophia.stl" to "input/Multi-Part/sophia.stl"
     And I copy "Pauldron_plates.stl" to "input/Multi-Part/plates.stl"
     And I wait 10 seconds for watchdog
     And I wait for processing to complete
     Then the output directory "multi-part" should exist
     And the output directory "multi-part" should contain "multi-part.3mf"
     And the 3MF file "multi-part.3mf" inside "multi-part" should contain at least 2 objects
