use dioxus::prelude::*;

use crate::pages::AdoptionPets;
use crate::pages::Home;
use crate::pages::MissingPets;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/missing-pets")]
    MissingPets {},
    #[route("/adoption-pets")]
    AdoptionPets {},
}

#[component]
pub fn Navbar() -> Element {
    let mut menu_open = use_signal(|| false);

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
            // Desktop nav links
            div {
                class: "nav-links items-center space-x-10 text-white md:flex",
                Link { to: "#", "Rescue Centers" }
                Link { to: "#", "Adoption Policy" }
                button {
                    class: "login-button",
                    onclick: |_| { println!("Login button clicked"); },
                    "Login"
                }
            }
            // Hamburger button — mobile only
            button {
                class: "hamburger-btn",
                onclick: move |_| menu_open.set(true),
                svg {
                    xmlns: "http://www.w3.org/2000/svg",
                    width: "28",
                    height: "28",
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    line { x1: "3", y1: "6", x2: "21", y2: "6" }
                    line { x1: "3", y1: "12", x2: "21", y2: "12" }
                    line { x1: "3", y1: "18", x2: "21", y2: "18" }
                }
            }
        }

        // Mobile full-screen menu overlay
        div {
            class: if *menu_open.read() { "mobile-menu mobile-menu-open" } else { "mobile-menu" },
            // Header row inside menu with logo + X button
            div {
                class: "mobile-menu-header",
                span { class: "mobile-menu-logo", "MyLostPet" }
                button {
                    class: "mobile-menu-close",
                    onclick: move |_| menu_open.set(false),
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        width: "28",
                        height: "28",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        line { x1: "18", y1: "6", x2: "6", y2: "18" }
                        line { x1: "6", y1: "6", x2: "18", y2: "18" }
                    }
                }
            }
            // Nav links
            nav {
                class: "mobile-menu-nav",
                div {
                    onclick: move |_| menu_open.set(false),
                    Link { to: Route::Home {}, "Home" }
                }
                div {
                    onclick: move |_| menu_open.set(false),
                    Link { to: "#", "Rescue Centers" }
                }
                div {
                    onclick: move |_| menu_open.set(false),
                    Link { to: "#", "Adoption Policy" }
                }
                button {
                    class: "mobile-login-btn",
                    onclick: move |_| {
                        menu_open.set(false);
                        println!("Login button clicked");
                    },
                    "Login"
                }
            }
        }

        Outlet::<Route> {}
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
}
