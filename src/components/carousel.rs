use dioxus::prelude::*;

#[component]
pub fn Carousel(images: Vec<String>) -> Element {
    let mut current_image = use_signal(|| 0);

    let next_image = move |_| {
        current_image.with_mut(|i| {
            *i = (*i + 1) % images.len();
        });
    };

    let prev_image = move |_| {
        current_image.with_mut(|i| {
            if *i == 0 {
                *i = images.len() - 1;
            } else {
                *i -= 1;
            }
        });
    };

    rsx! {
        div {
            class: "relative",
            div {
                class: "overflow-hidden relative rounded-lg",
                // Images
                for (idx, image) in images.iter().enumerate() {
                    div {
                        key: "{idx}",
                        class: if idx == current_image() {
                            "opacity-100 transition-opacity duration-500 ease-in-out"
                        } else {
                            "opacity-0 absolute top-0 left-0 w-full h-full transition-opacity duration-500 ease-in-out"
                        },
                        img {
                            src: "{image}",
                            alt: "Pet Image",
                            class: "w-full h-full object-cover"
                        }
                    }
                }
            }

            // Prev Button
            button {
                class: "absolute top-1/2 left-2 transform -translate-y-1/2 bg-white bg-opacity-50 rounded-full p-2",
                onclick: prev_image,
                "◀"
            }

            // Next Button
            button {
                class: "absolute top-1/2 right-2 transform -translate-y-1/2 bg-white bg-opacity-50 rounded-full p-2",
                onclick: next_image,
                "▶"
            }

            // Dots
            div {
                class: "absolute bottom-2 left-1/2 transform -translate-x-1/2 flex space-x-2",
                for i in 0..images.len() {
                    button {
                        class: if i == current_image() {
                            "w-3 h-3 bg-blue-500 rounded-full"
                        } else {
                            "w-3 h-3 bg-white rounded-full"
                        },
                        onclick: move |_| current_image.set(i)
                    }
                }
            }
        }
    }
}
