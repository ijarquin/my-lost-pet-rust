use dioxus::prelude::*;

#[component]
pub fn MissingPetsPage() -> Element {
    rsx! {
        div {
            id: "missing-pets",
            class: "text-center",
            // Content
            h1 { "Missing Pets" }
            p { "This page is a placeholder for missing pets. You can navigate to the blog or home page." }
        }
    }
}
