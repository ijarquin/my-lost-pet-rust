// use cucumber::Cucumber;
mod steps;

use colored::*;
use steps::toggle::*; // Import the toggle steps
use steps::carousel::*;

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
}
