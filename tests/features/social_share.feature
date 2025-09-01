Feature: Image Carousel Navigation

  Scenario: Display social share icons when the social share button is clicked
    Given the social icons are hidden by default 
    When the user clicks the social share button 
    Then the social share icons should be visible

  Scenario: Hide displayed social share icons when the social share button is clicked
    Given the social icons are hidden by default 
    When the user clicks the social share button 
    Then the social share icons should be visible
    When the user clicks the social share button 
    Then the social share icons should be hidden 
    


