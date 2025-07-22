use dioxus::prelude::*;

const HERO_IMAGE: Asset = asset!("/assets/images/tirma.jpg");
use crate::pages::MissingPetsPage;
use crate::pages::AdoptionPetsPage;


#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/missing-pets")]
    MissingPetsPage {},
    #[route("/adoption-pets")]
    AdoptionPetsPage {},
}

#[component]
pub fn HomePage() -> Element {
    rsx! {
        Hero {}
    }
}

#[component]
pub fn Hero() -> Element {
    rsx! {
        div {
            class: "home flex flex-row p-6 lg:flex-row lg:mb-0",
            div {
                class: "w-1/2",
                id: "hero",
                h1 { class: "hero-title", id: "subtitle", "A place to find your loving pets" }
                p { 
                    class: "hero-description w-1/2",
                    id: "description",
                    "Find your loving pets, you can filter by regions, time they were lost. We hope you can find your pets as soon as possible." }
                div {
                    id: "links",
                    a {
                        href: "/missing-pets",
                        "😿 Missing Pets"
                    }
                    a {
                        href: "/adoption-pets", 
                        "🐶 Adoption Pets"
                    }
                }
            }
            img {
                class: "max-w-1/2",
                src: HERO_IMAGE
            }
        }
    }
}
