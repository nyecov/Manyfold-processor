Feature: 04 Archive Processing
  As a user
  I want to process 3MF/Zip archives from MakerWorld
  So that thumbnails and metadata are extracted automatically

  Background:
     Given the processor is running
     And the "input" directory is empty
     And the "staging" directory is empty
     And the "output" directory is empty

  Scenario: 04A - Valid 3MF Processing
     Given I have enabled auto-process
     When I copy "valid_makerworld.3mf" from test source to input
     And I wait 10 seconds for watchdog
     And I wait for processing to complete
     Then the output directory "valid-makerworld" should exist
     And the output directory "valid-makerworld" should contain "datapackage.json"
     And the output directory "valid-makerworld" should contain "valid_makerworld.3mf"

  Scenario: 04B - Multi-Model 3MF (Single Print Bed)
     Given I have enabled auto-process
     When I copy "Dragon+2+Big+and+Beautiful.3mf" from test source to input
     And I wait 10 seconds for watchdog
     And I wait for processing to complete
     Then the output directory "dragon-2-big-and-beautiful" should exist
     And the output directory "dragon-2-big-and-beautiful" should contain "datapackage.json"
     And the output directory "dragon-2-big-and-beautiful" should contain "Dragon+2+Big+and+Beautiful.3mf"

  Scenario: 04C - Multi-Model 3MF (Multiple Print Beds)
     Given I have enabled auto-process
     When I copy "Legendary+Supportless+dwarf+10+pack.3mf" from test source to input
     And I wait 10 seconds for watchdog
     And I wait for processing to complete
     Then the output directory "legendary-supportless-dwarf-10-pack" should exist
     And the output directory "legendary-supportless-dwarf-10-pack" should contain "datapackage.json"
     And the output directory "legendary-supportless-dwarf-10-pack" should contain "Legendary+Supportless+dwarf+10+pack.3mf"
