use cucumber::{given, then, when, World};
use thirtyfour::prelude::*;
use pretty_assertions::assert_ne;

const BASE_URL: &str = "http://localhost:8080/missing-pets";

#[derive(Default, World)]
pub struct CarouselWorld {
    driver: Option<WebDriver>,
}

impl std::fmt::Debug for CarouselWorld {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CarouselWorld")
            .field("driver", &"WebDriver connection (not printable)")
            .finish()
    }
}

async fn get_current_image_src(driver: &WebDriver) -> Result<String, WebDriverError> {
    let image_element = driver.find(By::Css("div.tw-opacity-100 > img")).await?;
    image_element.attr("src").await.map(|s| s.unwrap_or_default())
}

async fn click_right_arrow(driver: &WebDriver) -> Result<(), WebDriverError> {
    let next_button = driver.find(By::Css(r"button.tw-absolute.tw-top-1\/2.tw-right-2")).await?;
    next_button.click().await
}

async fn click_left_arrow(driver: &WebDriver) -> Result<(), WebDriverError> {
    let prev_button = driver.find(By::Css(r"button.tw-absolute.tw-top-1\/2.tw-left-2")).await?;
    prev_button.click().await
}

#[given("the carousel is displaying an image")]
async fn carousel_is_displaying_an_image(world: &mut CarouselWorld) {
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:9515", caps)
        .await
        .expect("Failed to connect to WebDriver. Is chromedriver/geckodriver running?");

    driver
        .goto(BASE_URL)
        .await
        .expect("Failed to navigate to Dioxus app");
    world.driver = Some(driver);
}

#[when("the user clicks the right arrow")]
async fn user_clicks_the_right_arrow(world: &mut CarouselWorld) {
    let driver = world.driver.as_ref().expect("WebDriver not initialized");
    click_right_arrow(driver).await.expect("Failed to click next button");
}

#[then("the carousel should display the next image")]
async fn carousel_should_display_the_next_image(world: &mut CarouselWorld) {
    let driver = world.driver.as_ref().expect("WebDriver not initialized");
    let initial_src = get_current_image_src(driver).await.unwrap();
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await; // Wait for transition
    let next_src = get_current_image_src(driver).await.unwrap();
    assert_ne!(initial_src, next_src, "Image should have changed");
}

#[when("the user clicks the left arrow")]
async fn user_clicks_the_left_arrow(world: &mut CarouselWorld) {
    let driver = world.driver.as_ref().expect("WebDriver not initialized");
    click_left_arrow(driver).await.expect("Failed to click previous button");
}

#[then("the carousel should display the previous image")]
async fn carousel_should_display_the_previous_image(world: &mut CarouselWorld) {
    let driver = world.driver.as_ref().expect("WebDriver not initialized");
    let initial_src = get_current_image_src(driver).await.unwrap();
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await; // Wait for transition
    let prev_src = get_current_image_src(driver).await.unwrap();
    assert_ne!(initial_src, prev_src, "Image should have changed");
}

#[given("the carousel is displaying the last image")]
async fn carousel_is_displaying_the_last_image(world: &mut CarouselWorld) {
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:9515", caps)
        .await
        .expect("Failed to connect to WebDriver. Is chromedriver/geckodriver running?");

    driver
        .goto(BASE_URL)
        .await
        .expect("Failed to navigate to Dioxus app");

    for _ in 0..3 {
        click_right_arrow(&driver).await.expect("Failed to click next button");
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await; // Wait for transition
    }
    world.driver = Some(driver);
}

#[then("the carousel should display the first image")]
async fn carousel_should_display_the_first_image(world: &mut CarouselWorld) {
    let driver = world.driver.as_ref().expect("WebDriver not initialized");
    let last_image_src = get_current_image_src(driver).await.unwrap();
    click_right_arrow(driver).await.expect("Failed to click next button");
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await; // Wait for transition
    let first_image_src = get_current_image_src(driver).await.unwrap();
    assert_ne!(last_image_src, first_image_src, "Should have looped back to the first image");
}

#[given("the carousel is displaying the first image")]
async fn carousel_is_displaying_the_first_image(world: &mut CarouselWorld) {
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:9515", caps)
        .await
        .expect("Failed to connect to WebDriver. Is chromedriver/geckodriver running?");

    driver
        .goto(BASE_URL)
        .await
        .expect("Failed to navigate to Dioxus app");
    world.driver = Some(driver);
}

#[then("the carousel should display the last image")]
async fn carousel_should_display_the_last_image(world: &mut CarouselWorld) {
    let driver = world.driver.as_ref().expect("WebDriver not initialized");
    let first_image_src = get_current_image_src(driver).await.unwrap();
    click_left_arrow(driver).await.expect("Failed to click previous button");
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await; // Wait for transition
    let last_image_src = get_current_image_src(driver).await.unwrap();
    assert_ne!(first_image_src, last_image_src, "Should have looped back to the last image");
}