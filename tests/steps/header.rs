use cucumber::{given, World};

// use crate::pages::AdoptionPets;
// use crate::pages::Home;
// use crate::pages::MissingPets;

#[derive(Debug, Default, World)]
pub struct Header {}

#[given(regex = r"I am on the (.+) page")]
async fn i_am_on_the_page(_: &mut Header, page_name: String) {
    println!("Navigating to the {} page...", page_name);
}
