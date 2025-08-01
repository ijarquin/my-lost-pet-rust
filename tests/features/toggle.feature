Feature: Toggle Component Functionality
  As a user
  I want to interact with a toggle switch
  So that I can see its state change

  Scenario: Initial state of the toggle
    Given the toggle component is rendered
    Then the toggle should be "Off"
    And the toggle button should have class "off"

#   Scenario: Toggling the switch from Off to On
#     Given the toggle component is rendered
#     And the toggle is "Off"
#     When I click the toggle button
#     Then the toggle should be "On"
#     And the toggle button should have class "on"

#   Scenario: Toggling the switch from On to Off
#     Given the toggle component is rendered
#     And the toggle is "On"
#     When I click the toggle button
#     Then the toggle should be "Off"
#     And the toggle button should have class "off"