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
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
        Footer {}
    }
}

/// Shared navbar component.
#[component]
fn Navbar() -> Element {
    rsx! {
        div {
            class: "header-navbar flex space-between items-center justify-between p-4 bg-softBlue text-white",
            div {
                class: "z-50 flex flex-row items-center space-x-2",
                Link {
                    to: Route::HomePage {},
                    "MyLostPet"
                }
            }
            div {
                class: "items-center space-x-10 upercase text-white md:flex",
                Link {
                    to: "#",
                    "Rescue Centers"
                }
                Link {
                    to: "#",
                    "Adoption Policy"
                }
                Link {
                    class: "login-button",
                    to: "#",
                    "Login"
                }
            }
        }

        Outlet::<Route> {}
    }
}

#[component]
fn Footer() -> Element {
    rsx! {
        footer {
            class: "footer text-center p-4 bg-softBlue text-white",
            p { "© 2025 MyLostPet. All rights reserved." }
            p { "Follow us on social media!" }
            // Add social media links here
        }
    }
}