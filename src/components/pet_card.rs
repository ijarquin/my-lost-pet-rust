use dioxus::prelude::*;

const PET_IMAGE: Asset = asset!("/assets/images/xira.jpg");

#[component]
pub fn PetCard() -> Element {
    rsx! {
        div {
            class: "pet-card",
            img {
                src: PET_IMAGE,
                alt: "Pet Image",
                class: "w-full h-full object-cover"
            }
            div {
                class: "pet-accordion flex flex-row items-center justify-between w-full",
                p { class: "pet-name mx-4 text-sm text-gray-600", "Xira" }
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


// <div class={"transition duration-500 ease transform #{if @open_accordion == @pet.id |> Integer.to_string(), do: "rotate-180", else: "rotate-0"}"}>
//           <svg xmlns="http://www.w3.org/2000/svg" width="18" height="12">
//             <path d="M1 1l8 8 8-8" stroke="#fff" stroke-width="3" fill="none" />
//           </svg>
//         </div>