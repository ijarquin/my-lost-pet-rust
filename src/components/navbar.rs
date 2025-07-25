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
                class: "items-center space-x-10 text-white md:flex",
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
