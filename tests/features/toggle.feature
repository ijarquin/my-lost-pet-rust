Feature: Toggle Component Functionality
  As a user
  I want to interact with a toggle switch
  So that I can see its state change

  Scenario: Initial state of the toggle
    Given the toggle component is rendered
    Then the toggle should be "Closed"

  Scenario: Toggling the switch from Closed to Open
    Given the toggle component is rendered
    And the toggle is "Closed"
    When I click the toggle button
    Then the toggle should be "Open"

  Scenario: Toggling the switch from Open to Closed
    Given the toggle component is rendered
    And the toggle is "Closed"
    When I click the toggle button
    Then the toggle should be "Open"
    When I click the toggle button
    Then the toggle should be "Closed"