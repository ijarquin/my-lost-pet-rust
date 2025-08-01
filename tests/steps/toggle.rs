// tests/steps/toggle_steps.rs
use cucumber::{given, then, when, World};
use thirtyfour::prelude::*; // Import thirtyfour
// use tokio; // For async operations
use pretty_assertions::assert_eq;

// The base URL of your running Dioxus application
const BASE_URL: &str = "http://localhost:8080";

// let caps = DesiredCapabilities::chrome();
// let driver = WebDriver::new(BASE_URL, caps) // <--- This port must be correct!
//     .await
//     .expect("Failed to connect to WebDriver. Is chromedriver/geckodriver running?");

// Define your World struct to hold the WebDriver instance
#[derive(Default, World)]
pub struct Toggle {
    driver: Option<WebDriver>, // Use Option to manage the driver lifecycle
}

impl std::fmt::Debug for Toggle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // You can choose to print the address, a placeholder, or just skip it
        // A placeholder is often best to avoid printing sensitive information.
        f.debug_struct("MyWorld")
            .field("driver", &"WebDriver connection (not printable)")
            .finish()
    }
}

// impl World for Toggle {
//     // This hook runs before each scenario
//     async fn before_all() {
//         println!("Starting WebDriver...");
//         // You might want to ensure chromedriver is running here,
//         // or have a separate script/process manage it.
//         // For simplicity, we assume it's running on localhost:9515 (default chromedriver port)
//     }

//     // This hook runs after each scenario, ensuring the browser is closed
//     async fn after_each(scenario: &cucumber::Scenario, world: &mut Self) {
//         if let Some(driver) = world.driver.take() { // Use take() to consume the driver
//             println!("Closing browser for scenario: {:?}", scenario.name);
//             driver.quit().await.expect("Failed to quit WebDriver");
//         }
//     }
// }

// --- GIVEN Steps ---

#[given(regex = r"the toggle component is rendered")]
async fn toggle_component_rendered(world: &mut Toggle) {
    let caps = DesiredCapabilities::chrome(); // Or firefox(), edge(), etc.
    let driver = WebDriver::new("http://localhost:9515", caps) // Default chromedriver port
        .await
        .expect("Failed to connect to WebDriver. Is chromedriver/geckodriver running?");

    driver.goto(BASE_URL).await.expect("Failed to navigate to Dioxus app");
    world.driver = Some(driver);
    println!("Given: Toggle component rendered (in browser).");
}

// #[given("the toggle is \"(On|Off)\"")]
// async fn toggle_is_state(world: &mut Toggle, expected_state: String) {
//     let driver = world.driver.as_ref().expect("WebDriver not initialized");

//     let button_element = driver.find(By::Id("toggle-button"))
//         .await
//         .expect("Toggle button with id 'toggle-button' not found on page.");

//     let current_text = button_element.text().await.expect("Failed to get button text");

//     if current_text != expected_state {
//         // If the state is not what's expected, click to change it
//         button_element.click().await.expect("Failed to click toggle button to set state");
//         // thirtyfour often auto-waits, but we might want an explicit wait for stability
//         driver.wait(
//             WebDriverWait::new(std::time::Duration::from_secs(2))
//                 .with_message("Timeout waiting for toggle state change.")
//         )
//         .until(|d| async move {
//             let btn = d.find(By::Id("toggle-button")).await?;
//             Ok(btn.text().await? == expected_state)
//         })
//         .await
//         .expect(&format!("Failed to set toggle to {}: Timeout waiting for expected state", expected_state));
//     }

//     // Double-check the state after ensuring it
//     let final_button_text = button_element.text().await.expect("Failed to get final button text");
//     assert_eq!(final_button_text, expected_state, "Failed to ensure toggle is in the correct initial state.");

//     println!("Given: Toggle is {}.", expected_state);
// }

// --- WHEN Steps ---

#[when(regex = r"I click the toggle button")]
async fn click_toggle_button(world: &mut Toggle) {
    let driver = world.driver.as_ref().expect("WebDriver not initialized");
    let button_element = driver.find(By::Id("toggle-button"))
        .await
        .expect("Toggle button with id 'toggle-button' not found on page.");

    button_element.click().await.expect("Failed to click toggle button");
    println!("When: Clicked the toggle button.");
}

// --- THEN Steps ---

// #[then(regex = r"the toggle should be \"(On|Off)\"")]
// async fn toggle_should_be_state(world: &mut Toggle, expected_state: String) {
//     let driver = world.driver.as_ref().expect("WebDriver not initialized");
//     let button_element = driver.find(By::Id("toggle-button"))
//         .await
//         .expect("Toggle button with id 'toggle-button' not found on page.");

//     let actual_text = button_element.text().await.expect("Failed to get button text for assertion");
//     assert_eq!(actual_text, expected_state, "Toggle state mismatch.");
//     println!("Then: Toggle is {}.", expected_state);
// }

// #[then(regex = r"the toggle button should have class \"(on|off)\"")]
// async fn toggle_button_has_class(world: &mut Toggle, expected_class: String) {
//     let driver = world.driver.as_ref().expect("WebDriver not initialized");
//     let button_element = driver.find(By::Id("toggle-button"))
//         .await
//         .expect("Toggle button with id 'toggle-button' not found on page.");

//     let classes = button_element.class_name().await.expect("Failed to get button class name");
//     assert!(classes.contains(&expected_class), "Expected class '{}' not found in '{}'", expected_class, classes);
//     println!("Then: Toggle button has class {}.", expected_class);
// }