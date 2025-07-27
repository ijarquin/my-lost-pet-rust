use dioxus::prelude::*;

use crate::components::PetCard;

#[component]
pub fn MissingPets() -> Element {
    let pets = vec![
        (1, "Xira", asset!("/assets/images/xira.jpg")),
        (2, "Luna", asset!("/assets/images/luna.jpg")),
        (3, "Max", asset!("/assets/images/alma.jpg")),
        (4, "Tirma", asset!("/assets/images/tirma-1.jpg")),
        (5, "Rufold", asset!("/assets/images/rudolf.jpg")),
    ];

    rsx! {
        div {
            id: "missing-pets",
            class: "missing-pets",
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
