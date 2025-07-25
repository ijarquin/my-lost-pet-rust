Feature: Site Header
  As a user, I would like to navigate through the site header

  Scenario: link to lost pets page
    Given I am on the home page
    When I click on the "Missing pets" link in the header
    Then I should be redirected to the missing pets page
    And I should see the missing pets page title "Missing Pets"
