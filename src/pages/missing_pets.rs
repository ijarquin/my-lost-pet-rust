use dioxus::prelude::*;

use crate::components::pet_card::PetCard;

#[component]
pub fn MissingPets() -> Element {
    rsx! {
        div {
            id: "missing-pets",
            class: "text-center flex flex-row items-center justify-between w-full",
            PetCard {}
            PetCard {}
            PetCard {}
        }
    }
}
