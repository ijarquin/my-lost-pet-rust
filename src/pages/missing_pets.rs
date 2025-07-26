use dioxus::prelude::*;

use crate::components::pet_card::PetCard;

#[component]
pub fn MissingPets() -> Element {
    let pets = vec![
        ("Xira", asset!("/assets/images/xira.jpg")),
        ("Luna", asset!("/assets/images/luna.jpg")),
        ("Max", asset!("/assets/images/alma.jpg")),
    ];

    rsx! {
        div {
            id: "missing-pets",
            class: "text-center flex flex-row items-center justify-between w-full",
            {
                pets.iter().map(|(name, image)| {
                    rsx! {
                        PetCard {
                            name: name,
                            image: image,
                        }
                    }
                })
            }
        }
    }
}
