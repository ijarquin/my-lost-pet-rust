use dioxus::prelude::*;

use crate::components::Pet;
use crate::components::Toggle;

#[component]
pub fn PetCard(pet: Pet) -> Element {
    rsx! {
        div {
            key: pet.id,
            class: "pet-card",
            img {
                src: pet.image,
                alt: "Pet Image",
                class: "w-full h-full object-cover"
            }
            Toggle { pet: pet.clone() }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dioxus_core::NoOpMutations;
    use dioxus_ssr::render;

    #[test]
    fn test_pet_card_component() {
        let pet = Pet {
            id: 1,
            name: "Xira",
            image: asset!("/assets/images/xira.jpg"),
            sex: "Female",
            age: "2 years",
            size: "Medium",
            breed: "Beagle",
        };
        // let image = asset!("/assets/images/xira.jpg").to_string();
        let mut vdom = VirtualDom::new(|| {
            let pet = Pet {
                id: 1,
                name: "Xira",
                image: asset!("/assets/images/xira.jpg"),
                sex: "Female",
                age: "2 years",
                size: "Medium",
                breed: "Beagle",
            };
            rsx! { PetCard { pet: pet } }
        });
        let mut mutations = NoOpMutations;
        vdom.rebuild(&mut mutations);
        let html = render(&mut vdom);

        assert!(html.contains("<img"));
        assert!(html.contains("alt=\"Pet Image\""));
        assert!(html.contains(&format!(
            "src=\"{}\"",
            asset!("/assets/images/xira.jpg").to_string()
        )));
        assert!(html.contains("Xira"));
        assert!(html.contains("Show more"));
        assert!(html.contains("pet-card"));
    }
}
