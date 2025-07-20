use dioxus::prelude::*;

mod pages;

use crate::pages::HomePage;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    HomePage {},
    #[route("/missing-pets")]
    MissingPets {},
    #[route("/adoption-pets")]
    AdoptionPets {},
    #[route("/blog/:id")]
    Blog { id: i32 },
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

#[component]
fn MissingPets() -> Element {
    rsx! {
        div {
            id: "missing-pets",

            // Content
            h1 { "Missing Pets" }
            p { "This page is a placeholder for missing pets. You can navigate to the blog or home page." }

            // Navigation links
            Link {
                to: Route::HomePage {},
                "Go to Home"
            }
            span { " | " }
            Link {
                to: Route::Blog { id: 1 },
                "Go to Blog"
            }
        }
    }
}

#[component]
fn AdoptionPets() -> Element {
    rsx! {
        div {
            id: "adoption-pets",

            // Content
            h1 { "Adoption Pets" }
            p { "This page is a placeholder for adoption pets. You can navigate to the blog or home page." }

            // Navigation links
            Link {
                to: Route::HomePage {},
                "Go to Home"
            }
            span { " | " }
            Link {
                to: Route::Blog { id: 1 },
                "Go to Blog"
            }
        }
    }
}

/// Blog page
#[component]
pub fn Blog(id: i32) -> Element {
    rsx! {
        div {
            id: "blog",

            // Content
            h1 { "This is blog #{id}!" }
            p { "In blog #{id}, we show how the Dioxus router works and how URL parameters can be passed as props to our route components." }

            // Navigation links
            Link {
                to: Route::Blog { id: id - 1 },
                "Previous"
            }
            span { " <---> " }
            Link {
                to: Route::Blog { id: id + 1 },
                "Next"
            }
        }
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
                to: Route::MissingPets {},
                "Missing Pets"
            }
            Link {
                to: Route::AdoptionPets {},
                "Adoption Pets"
            }
        }

        Outlet::<Route> {}
    }
}
