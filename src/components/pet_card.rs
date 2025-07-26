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
                p { class: "pet-name mx-4 text-sm text-gray-600", {name} }
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
