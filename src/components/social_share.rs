use dioxus::prelude::*;

#[component]
pub fn SocialShare() -> Element {
    rsx! {
        button {
            title: "Share",
            class: "tw-absolute tw-bottom-2 tw-right-2 tw-z-10 tw-p-2 tw-rounded-full hover:tw-bg-black/20 text-white",
            onclick: move |_| {
                // TODO: Implement social sharing logic, e.g., using the Web Share API
            },
            // Three vertical dots icon (ellipsis-vertical)
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                width: "24",
                height: "24",
                view_box: "0 0 24 24",
                fill: "currentColor",
                // Refactored from a single path to three circles for clarity and easier maintenance.
                circle { cx: "12", cy: "4", r: "2" }
                circle { cx: "12", cy: "12", r: "2" }
                circle { cx: "12", cy: "20", r: "2" }
            }
        }
    }
}