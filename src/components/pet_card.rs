use dioxus::prelude::*;

#[component]
pub fn PetCard(id: String, name: String, image: String) -> Element {
    rsx! {
        div {
            key: id,
            class: "pet-card",
            img {
                src: image,
                alt: "Pet Image",
                class: "w-full h-full object-cover"
            }
            div {
                class: "pet-accordion flex flex-row items-center justify-between w-full",
                p { class: "mx-4 text-sm text-gray-600", {name} }
                div {
                    class: "flex items-center space-x-2",
                    button {
                        class: "pet-accordion-button text-blue-500 hover:text-blue-700",
                        onclick: |_| {
                            // Handle accordion toggle logic here
                            println!("Accordion button clicked");
                        },
                        "Show more"
                    }
                    svg {
                        class: "pet-accordion-icon text-gray-500",
                        view_box: "0 0 20 20",
                        fill: "currentColor",
                        width: "20",
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
        // let image = asset!("/assets/images/xira.jpg").to_string();
        let mut vdom = VirtualDom::new(
            || rsx! { PetCard { id: "1".to_string(), name: "Xira".to_string(), image: asset!("/assets/images/xira.jpg").to_string() } },
        );
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
