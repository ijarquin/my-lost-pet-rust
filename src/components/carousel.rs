use dioxus::prelude::*;

#[component]
pub fn Carousel(images: Vec<String>) -> Element {
    let carrusel_images = images.clone();
    let more_images = images.clone();
    let images_len = images.len();
    let mut current_image = use_signal(|| 0);

    let next_image = move |_| {
        current_image.with_mut(|i| {
            *i = (*i + 1) % images.len();
        });
    };

    let prev_image = move |_| {
        current_image.with_mut(|i| {
            if *i == 0 {
                *i = carrusel_images.len() - 1;
            } else {
                *i -= 1;
            }
        });
    };

    rsx! {
        div {
            class: "tw-relative",
            div {
                class: "tw-overflow-hidden tw-relative tw-rounded-lg tw-h-64",
                // Images
                for (idx, image) in more_images.iter().enumerate() {
                    div {
                        key: "{idx}",
                        class: if idx == current_image() {
                            "tw-opacity-100 tw-transition-opacity tw-duration-500 tw-ease-in-out"
                        } else {
                            "tw-opacity-0 tw-absolute tw-top-0 tw-left-0 tw-w-full tw-h-full tw-transition-opacity tw-duration-500 tw-ease-in-out"
                        },
                        img {
                            src: "{image}",
                            alt: "Pet Image",
                            class: "tw-w-full tw-h-full tw-object-cover"
                        }
                    }
                }
            }
            if images_len > 1 {
                // Prev Button
                button {
                    class: "tw-absolute tw-top-1/2 tw-left-2 tw-transform -tw-translate-y-1/2 tw-rounded-full tw-p-2 tw-text-white hover:tw-bg-black/20",
                    onclick: prev_image,
                    "◀"
                }

                // Next Button
                button {
                    class: "tw-absolute tw-top-1/2 tw-right-2 tw-transform -tw-translate-y-1/2 tw-rounded-full tw-p-2 tw-text-white hover:tw-bg-black/20",
                    onclick: next_image,
                    "▶"
                }

            // Dots
                div {
                    class: "tw-absolute tw-bottom-2 tw-left-1/2 tw-transform -tw-translate-x-1/2 tw-flex tw-space-x-2",
                    for i in 0..images_len {
                        button {
                            class: if i == current_image() {
                                "tw-w-3 tw-h-3 tw-bg-softBlue tw-rounded-full"
                            } else {
                                "tw-w-3 tw-h-3 tw-bg-white tw-rounded-full"
                            },
                            onclick: move |_| current_image.set(i)
                        }
                    }
                }
            }
        }
    }
}
