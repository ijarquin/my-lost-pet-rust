Feature: Image Carousel Navigation

  Scenario: Display next image when right arrow is clicked
    Given the carousel is displaying an image
    When the user clicks the right arrow
    Then the carousel should display the next image

  Scenario: Display previous image when left arrow is clicked
    Given the carousel is displaying an image
    When the user clicks the left arrow
    Then the carousel should display the previous image

  Scenario: Loop to first image when right arrow is clicked on the last image
    Given the carousel is displaying the last image
    When the user clicks the right arrow
    Then the carousel should display the first image

  Scenario: Loop to last image when left arrow is clicked on the first image
    Given the carousel is displaying the first image
    When the user clicks the left arrow
    Then the carousel should display the last image
