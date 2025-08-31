use cucumber::{given, then, when, World};
use thirtyfour::prelude::*;

const BASE_URL: &str = "http://localhost:8080/missing-pets";

#[derive(Default, World)]
pub struct SocialShareWorld {
    driver: Option<WebDriver>,
}

impl std::fmt::Debug for SocialShareWorld {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SocialShareWorld")
            .field("driver", &"WebDriver session")
            .finish()
    }
}

impl SocialShareWorld {
    async fn new() -> Self {
        Self { driver: None }
    }
}

#[given(regex = r"the social icons are hidden by default")]
async fn icons_hidden_by_default(world: &mut SocialShareWorld) {
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:9515", caps)
        .await
        .expect("Failed to create WebDriver");
    driver.goto(BASE_URL).await.unwrap();
    let icons = driver.find(By::Id("social-icons")).await.unwrap();
    assert!(!icons.is_displayed().await.unwrap());
    world.driver = Some(driver);
}

#[when(regex = r"the user clicks the social share button")]
async fn click_social_share_button(world: &mut SocialShareWorld) {
    let button = world.driver.as_ref().unwrap().find(By::Id("social-share-button")).await.unwrap();
    button.click().await.unwrap();
}

#[then(regex = r"the social share icons should be visible")]
async fn icons_should_be_visible(world: &mut SocialShareWorld) {
    let icons = world.driver.as_ref().unwrap().find(By::Id("social-icons")).await.unwrap();
    assert!(icons.is_displayed().await.unwrap());
}
