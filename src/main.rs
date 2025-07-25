use dioxus::prelude::*;

mod components;
mod pages;

use crate::components::{Footer, Route};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
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
