// use cucumber::Cucumber;
mod steps;

use colored::*;
use steps::header::*;

use cucumber::World; // Import the World trait

#[async_std::main]
async fn main() {
    println!("{}", "\n\nRunning all features...\n".yellow().bold());

    Header::cucumber()
        .run("tests/features/header.feature")
        .await;

    println!("-------------------------------------------------------------------------");
} 
