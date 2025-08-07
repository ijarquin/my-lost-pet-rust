// tests/steps/toggle_steps.rs
use cucumber::{given, then, when, World};
use thirtyfour::prelude::*; // Import thirtyfour
                            // use tokio; // For async operations
use pretty_assertions::assert_eq;

// The base URL of your running Dioxus application
const BASE_URL: &str = "http://localhost:8080/missing-pets";

#[derive(Default, World)]
pub struct Toggle {
    driver: Option<WebDriver>, // Use Option to manage the driver lifecycle
}

impl std::fmt::Debug for Toggle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MyWorld")
            .field("driver", &"WebDriver connection (not printable)")
            .finish()
    }
}

#[given(regex = r"the toggle component is rendered")]
async fn toggle_component_rendered(world: &mut Toggle) {
    let caps = DesiredCapabilities::chrome(); // Or firefox(), edge(), etc.
    let driver = WebDriver::new("http://localhost:9515", caps) // Default chromedriver port
        .await
        .expect("Failed to connect to WebDriver. Is chromedriver/geckodriver running?");

    driver
        .goto(BASE_URL)
        .await
        .expect("Failed to navigate to Dioxus app");
    world.driver = Some(driver);
    println!("Given: Toggle component rendered (in browser).");
}

#[given(regex = r#"^the toggle is "(Open|Closed)"$"#)]
async fn toggle_is_state(world: &mut Toggle, state: String) {
    let expected_state = if state == "Open" {
        "Show less"
    } else {
        "Show more"
    };
    let driver = world.driver.as_ref().expect("WebDriver not initialized");
    let button_element = driver
        .find(By::Id("accordion-toggle-button"))
        .await
        .expect("Toggle button with id 'toggle-button' not found on page.");

    let actual_text = button_element
        .text()
        .await
        .expect("Failed to get button text for assertion");
    assert_eq!(actual_text, expected_state, "Toggle state mismatch.");
    println!("Then: Toggle is {}.", expected_state);
}

#[when(regex = r"I click the toggle button")]
async fn click_toggle_button(world: &mut Toggle) {
    let driver = world.driver.as_ref().expect("WebDriver not initialized");
    let button_element = driver
        .find(By::Id("accordion-toggle-button"))
        .await
        .expect("Toggle button with id 'toggle-button' not found on page.");

    button_element
        .click()
        .await
        .expect("Failed to click toggle button");
    println!("When: Clicked the toggle button.");
}

#[then(regex = r#"^the toggle should be "(Open|Closed)"$"#)]
async fn toggle_should_be_state(world: &mut Toggle, state: String) {
    let expected_state = if state == "Open" {
        "Show less"
    } else {
        "Show more"
    };
    let driver = world.driver.as_ref().expect("WebDriver not initialized");
    let button_element = driver
        .find(By::Id("accordion-toggle-button"))
        .await
        .expect("Toggle button with id 'toggle-button' not found on page.");

    let actual_text = button_element
        .text()
        .await
        .expect("Failed to get button text for assertion");
    assert_eq!(actual_text, expected_state, "Toggle state mismatch.");
    println!("Then: Toggle is {}.", expected_state);
}
