Feature: 03 Loose Files Processing
  As a user
  I want loose STL and image files to be grouped and processed into a 3MF project
  So that I can import them into the Manyfold database cleanly

  Background:
     Given the processor is running
     And the "input" directory is empty
     And the "staging" directory is empty
     And the "output" directory is empty

  Scenario: 03A - Auto-Process On (Happy Path)
     Given I have enabled auto-process
     When I copy "sophia-35mm-sophia.stl" from test source to input
     And I copy "720X720-sophia-new.jpg" from test source to input
     And I wait 10 seconds for watchdog
     And I wait for processing to complete
     Then the output directory "sophia-35mm-sophia" should exist
     And the output directory "sophia-35mm-sophia" should contain "datapackage.json"
     And the output directory "sophia-35mm-sophia" should contain ".3mf" file
     And the output directory "sophia-35mm-sophia" should contain ".webp" file

  Scenario: 03B - Manual API Trigger
     Given I have disabled auto-process
     When I copy "sophia-35mm-sophia.stl" from test source to input
     And I copy "720X720-sophia-new.jpg" from test source to input
     And I wait 10 seconds for watchdog
     Then the files should appear in staging
     And the output directory "sophia-35mm-sophia" should NOT exist
     When I trigger the "Process Loose Files" action via API
     And I wait for processing to complete
     Then the output directory "sophia-35mm-sophia" should exist
     And the output directory "sophia-35mm-sophia" should contain "datapackage.json"

  Scenario: 03C - Multi-STL Input Aggregation
     Given I have enabled auto-process
     When I copy "sophia-35mm-sophia.stl" from test source to input
     And I copy "Pauldron_plates.stl" from test source to input
     And I copy "720X720-sophia-new.jpg" from test source to input
     And I wait 10 seconds for watchdog
     And I wait for processing to complete
     # Both STLs should be aggregated into 'pauldron-plates'
     Then the output directory "pauldron-plates" should exist
     And the output directory "pauldron-plates" should contain "datapackage.json"
     And the output directory "pauldron-plates" should contain ".3mf" file
     And the output directory "pauldron-plates" should contain ".webp" file
     And the output directory "sophia-35mm-sophia" should NOT exist
