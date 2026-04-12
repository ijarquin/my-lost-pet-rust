use dioxus::prelude::*;

use crate::components::Map;
use crate::components::Pet;
use crate::components::PetCard;

#[component]
pub fn MissingPets() -> Element {
    let pets = vec![
        Pet {
            id: 1,
            name: "Xira",
            images: vec![
                asset!("/assets/images/xira.jpg"),
                asset!("/assets/images/tirma-1.jpg"),
            ],
            sex: "Female",
            age: "2 years",
            size: "Medium",
            breed: "Beagle",
            latitude: 51.5074,
            longitude: -0.1278,
        },
        Pet {
            id: 2,
            name: "Luna",
            images: vec![asset!("/assets/images/luna.jpg")],
            sex: "Female",
            age: "3 years",
            size: "Small",
            breed: "Poodle",
            latitude: 51.5114,
            longitude: -0.1321,
        },
        Pet {
            id: 3,
            name: "Max",
            images: vec![asset!("/assets/images/alma.jpg")],
            sex: "Male",
            age: "4 years",
            size: "Large",
            breed: "German Shepherd",
            latitude: 51.5050,
            longitude: -0.1250,
        },
        Pet {
            id: 4,
            name: "Rudolf",
            images: vec![asset!("/assets/images/rudolf.jpg")],
            sex: "Male",
            age: "5 years",
            size: "Medium",
            breed: "Beagle",
            latitude: 51.5085,
            longitude: -0.1299,
        },
    ];

    rsx! {
        div {
            class: "container mx-auto px-4",
            h1 {
                class: "tw-text-4xl tw-font-bold tw-text-center tw-my-8 text-softBlue",
                "Missing Pets"
            }
            Map {
                pets: pets.clone()
            }
            div {
                id: "missing-pets",
                class: "missing-pets",
                {
                    pets.iter().map(|pet| {
                        rsx! {
                            PetCard {pet: pet.clone()}
                        }
                    })
                }
            }
        }
    }
}
