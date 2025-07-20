use dioxus::prelude::*;

#[component]
pub fn AdoptionPetsPage() -> Element {
    rsx! {
        div {
            id: "adoption-pets",

            // Content
            h1 { "Adoption Pets" }
            p { "This page is a placeholder for adoption pets. You can navigate to the blog or home page." }
        }
    }
}
