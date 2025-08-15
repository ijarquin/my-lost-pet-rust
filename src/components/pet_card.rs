use dioxus::prelude::*;

use crate::components::carousel::Carousel;
use crate::components::Pet;
use crate::components::Toggle;

#[component]
pub fn PetCard(pet: Pet) -> Element {
    let images = pet
        .images
        .iter()
        .map(|image| image.to_string())
        .collect::<Vec<String>>();
    rsx! {
        div {
            key: pet.id,
            class: "pet-card",
            Carousel { images: images }
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
        let mut vdom = VirtualDom::new(|| {
            let pet = Pet {
                id: 1,
                name: "Xira",
                images: vec![asset!("/assets/images/xira.jpg")],
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

        assert!(html.contains("Xira"));
        assert!(html.contains("Show more"));
        assert!(html.contains("pet-card"));
    }
}
