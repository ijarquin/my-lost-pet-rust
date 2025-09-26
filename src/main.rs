use dioxus::prelude::*;
use dotenvy::dotenv;
use std::env;

mod components;
mod pages;

use crate::components::{Footer, Route};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const MAP_SCRIPT: Asset = asset!("/assets/map_script.js");

fn main() {
    dotenv().ok();
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let maps_api_key = env::var("MAPS_API_KEY").unwrap_or_else(|_| "your_fallback_api_key_here".to_string());
    let maps_api_url = format!("https://maps.googleapis.com/maps/api/js?key={}&callback=initMap", maps_api_key);

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Script { src: "{maps_api_url}" }
        document::Script { src: MAP_SCRIPT }
        Router::<Route> {}
        Footer {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dioxus_core::NoOpMutations;
    use dioxus_ssr::render;

    #[test]
    fn test_app_component() {
        let mut vdom = VirtualDom::new(|| rsx! { App {} });
        let mut mutations = NoOpMutations;
        vdom.rebuild(&mut mutations);
        let html = render(&mut vdom);

        assert!(html.contains("MyLostPet"));
        assert!(html.contains("Rescue Centers"));
        assert!(html.contains("Adoption Policy"));
        assert!(html.contains("login-button"));
        assert!(html.contains("© 2025 MyLostPet. All rights reserved."));
        assert!(html.contains("Follow us on social media!"));
    }
}
