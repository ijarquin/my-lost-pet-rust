use dioxus::prelude::*;

use crate::components::Pet;
use crate::components::PetCard;

#[component]
pub fn AdoptionPets() -> Element {
    let pets = [
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
        },
        Pet {
            id: 2,
            name: "Luna",
            images: vec![asset!("/assets/images/luna.jpg")],
            sex: "Female",
            age: "3 years",
            size: "Small",
            breed: "Poodle",
        },
        Pet {
            id: 3,
            name: "Max",
            images: vec![asset!("/assets/images/alma.jpg")],
            sex: "Male",
            age: "4 years",
            size: "Large",
            breed: "German Shepherd",
        },
        Pet {
            id: 4,
            name: "Rudolf",
            images: vec![asset!("/assets/images/rudolf.jpg")],
            sex: "Male",
            age: "5 years",
            size: "Medium",
            breed: "Beagle",
        },
    ];

    rsx! {
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
