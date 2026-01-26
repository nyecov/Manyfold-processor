Feature: 05 Isolation and Manual Controls
  As a system administrator
  I want to control processing via the auto-process toggle and ensure late files are isolated
  So that batch operations are safe and complete

  Background:
     Given the processor is running
     And the "input" directory is empty
     And the "staging" directory is empty
     And the "output" directory is empty

  Scenario: 05A - Batch Processing Toggle (Staging Check)
     Given I have disabled auto-process
     When I copy "sophia-35mm-sophia.stl" from test source to input
     And I copy "720X720-sophia-new.jpg" from test source to input
     And I wait 10 seconds for watchdog
     Then the files should appear in staging
     And the output directory should be empty
     When I enable auto-process
     And I wait for processing to complete
     Then the output directory "sophia-35mm-sophia" should exist
     And the output directory "sophia-35mm-sophia" should contain "datapackage.json"

  Scenario: 05B - Late Sibling Isolation (Orphan Logic)
     Given I have enabled auto-process
     When I copy "sophia-35mm-sophia.stl" from test source to input
     And I wait 10 seconds for processing to start
     And I copy "720X720-sophia-new.jpg" from test source to input
     And I wait 5 seconds
     Then the file "720X720-sophia-new.jpg" should remain in input
     When I wait for "sophia-35mm-sophia" processing to complete
     And I wait for orphan processing to complete
     Then the output directory "720x720-sophia-new" should exist
     And the output directory "720x720-sophia-new" should NOT contain "datapackage.json"
