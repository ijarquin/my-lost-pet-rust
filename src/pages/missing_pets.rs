use dioxus::prelude::*;

use crate::components::pet_card::PetCard;

#[component]
pub fn MissingPets() -> Element {
    let pets = vec![
        (1, "Xira", asset!("/assets/images/xira.jpg")),
        (2, "Luna", asset!("/assets/images/luna.jpg")),
        (3, "Max", asset!("/assets/images/alma.jpg")),
    ];

    rsx! {
        div {
            id: "missing-pets",
            class: "text-center flex flex-row items-center justify-between w-full",
            {
                pets.iter().map(|(id, name, image)| {
                    rsx! {
                        PetCard {
                            id: "{id}",
                            name: name,
                            image: image,
                        }
                    }
                })
            }
        }
    }
}
