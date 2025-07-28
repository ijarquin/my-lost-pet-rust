use dioxus::prelude::*;

use crate::components::PetCard;
use crate::components::Pet;

#[component]
pub fn MissingPets() -> Element {
    let pets = vec![
        Pet {
            id: 1,
            name: "Xira",
            image: asset!("/assets/images/xira.jpg"),
            sex: "Female",
            age: "2 years",
            size: "Medium",
            breed: "Beagle",
        },
        Pet {
            id: 2,
            name: "Luna",
            image: asset!("/assets/images/luna.jpg"),
            sex: "Female",
            age: "3 years",
            size: "Small",
            breed: "Poodle",
        },
        Pet {
            id: 3,
            name: "Max",
            image: asset!("/assets/images/alma.jpg"),
            sex: "Male",
            age: "4 years",
            size: "Large",
            breed: "German Shepherd",
        },
        Pet {
            id: 4,
            name: "Rudolf",
            image: asset!("/assets/images/rudolf.jpg"),
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
