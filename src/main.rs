use dioxus::prelude::*;

mod pages;

use crate::pages::HomePage;
use crate::pages::MissingPetsPage;
use crate::pages::AdoptionPetsPage;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    HomePage {},
    #[route("/missing-pets")]
    MissingPetsPage {},
    #[route("/adoption-pets")]
    AdoptionPetsPage {},
}

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
        document::Link { rel: "stylesheet", href: MAIN_CSS } document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}

/// Shared navbar component.
#[component]
fn Navbar() -> Element {
    rsx! {
        div {
            id: "navbar",
            Link {
                to: Route::HomePage {},
                "Home"
            }
            Link {
                to: Route::MissingPetsPage {},
                "Missing Pets"
            }
            Link {
                to: Route::AdoptionPetsPage {},
                "Adoption Pets"
            }
        }

        Outlet::<Route> {}
    }
}
