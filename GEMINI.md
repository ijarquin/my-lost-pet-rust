Context:
There is a pet card component pet_card.rs this is a card component
that currently shows an image and a toggle that will display the pet details when is open.

New Feature:
Instead of displaying an static image I want to build an image carrusel that will show up to 4 images. The carrusel will have two arrows and as many dots as images available and it will behave as
1 - if we click on the right arrow we will move to the next image and dot corresponding with that image positon will stand out.
2 - if we click on the left hand side arrow we will move to the previous image

edcases: 
- if we are in the last image and user click on the right arrow will move to the very first image
- if the user is in the very first image and click on the left arrow we will move to the very last image.

Animation: when we move between images I am expecting and smoth transition from one image to the other. arrows and dots will display in white collor, exept for the dot from the image selected that will display in the same blue collor used for the application botton and header.

BDD Scenarios
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