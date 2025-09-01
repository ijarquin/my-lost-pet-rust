// use cucumber::Cucumber;
mod steps;

use colored::*;
use steps::carousel::*;
use steps::social_share::*;
use steps::toggle::*; // Import the toggle steps

use cucumber::World; // Import the World trait

#[tokio::main]
async fn main() {
    println!("{}", "\n\nRunning all features...\n".yellow().bold());

    Toggle::cucumber()
        .run("tests/features/toggle.feature")
        .await;

    println!("-------------------------------------------------------------------------");

    CarouselWorld::cucumber()
        .run("tests/features/carousel.feature")
        .await;

    println!("-------------------------------------------------------------------------");

    SocialShareWorld::cucumber()
        .run("tests/features/social_share.feature")
        .await;

    println!("-------------------------------------------------------------------------");
}
