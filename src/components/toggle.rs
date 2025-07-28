use crate::components::Pet;
use dioxus::{html::div, prelude::*};

#[component]
pub fn Toggle(pet: Pet) -> Element {
    let mut is_on = use_signal(|| false); // `mut` is crucial here

    let rotation = if *is_on.read() {
        "rotate-180"
    } else {
        "rotate-0"
    };
    let text_content = if *is_on.read() {
        "Show less"
    } else {
        "Show more"
    };
    let accordion_content_class = if *is_on.read() {
        "pet-accordion-content-open"
    } else {
        "pet-accordion-content-closed"
    };

    rsx! {
        div {
            class: "pet-accordion flex flex-row items-center justify-between w-full",
            onclick: move |_| is_on.toggle(), // The simplest and most idiomatic for bool

            div {
                class: "w-7 h-7 bg-white rounded-full shadow-md transform transition-transform duration-300",
            },

            div {
                class: "absolute w-16 text-center text-white font-bold text-sm pointer-events-none",
                "{pet.name}"
            },

            div {
                class: "pet-accordion-button flex items-center space-x-2",
                div {
                    class: "pet-accordion-button-text text-blue-500 hover:text-blue-700",
                    "{text_content}"
                }
                svg {
                    class: "pet-accordion-icon {rotation}",
                    view_box: "0 0 20 10
                    ",
                    fill: "currentColor",
                    width: "18",
                    height: "18",
                    path {
                        d: "M1 1l8 8 8-8",
                        stroke: "#fff",
                        stroke_width: "3",
                        fill: "none"
                    }
                }
            }
        }
        div {
            class: "{accordion_content_class}",
            div {
                class: "flex flex-row justify-between border-b-1",
                div {class: "accordion-item", "Sex"}
                div {class: "accordion-item", {pet.sex}}
            }
            div {
                class: "flex flex-row justify-between border-b-1",
                div {class: "accordion-item", "Age"}
                div {class: "accordion-item", {pet.age}}
            }
            div {
                class: "flex flex-row justify-between border-b-1",
                div {class: "accordion-item", "Size"}
                div {class: "accordion-item", {pet.size}}
            }
            div {
                class: "flex flex-row justify-between",
                div {class: "accordion-item", "Breed"}
                div {class: "accordion-item", {pet.breed}}
            }
            div {
                class: "flex items-center justify-center w-full",
                button {class: "pet-accordion-content-button-left w-1/2", "Adopt me"}
                button {class: "pet-accordion-content-button-right w-1/2", "Book a view"}
            }
        }
    }
}
