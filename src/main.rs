use dioxus::prelude::*;

mod pages;

use crate::pages::AdoptionPets;
use crate::pages::Home;
use crate::pages::MissingPets;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/missing-pets")]
    MissingPets {},
    #[route("/adoption-pets")]
    AdoptionPets {},
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
                    to: Route::Home {},
                    "MyLostPet"
                }
            }
            div {
                class: "items-center space-x-10 uppercase text-white md:flex",
                Link {
                    to: "#",
                    "Rescue Centers"
                }
                Link {
                    to: "#",
                    "Adoption Policy"
                }
                button {
                    class: "login-button",
                    onclick: |_| {
                        // Handle login logic here
                        println!("Login button clicked");
                    },
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

#[cfg(test)]
mod tests {
    use super::*;
    use dioxus_core::NoOpMutations;
    use dioxus_ssr::render;

    #[test]
    fn test_nav_bar_component() {
        let mut vdom = VirtualDom::new(|| rsx! { Router::<Route> {} });
        let mut mutations = NoOpMutations;
        vdom.rebuild(&mut mutations);
        let html = render(&mut vdom);

        assert!(html.contains("MyLostPet"));
        assert!(html.contains("Rescue Centers"));
        assert!(html.contains("Adoption Policy"));
        assert!(html.contains("login-button"));
    }

    #[test]
    fn test_footer_component() {
        let mut vdom = VirtualDom::new(|| rsx! { Footer {} });
        let mut mutations = NoOpMutations;
        vdom.rebuild(&mut mutations);
        let html = render(&mut vdom);

        assert!(html.contains("© 2025 MyLostPet. All rights reserved."));
        assert!(html.contains("Follow us on social media!"));
    }
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
