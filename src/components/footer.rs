use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer {
            class: "footer text-center p-4 bg-softBlue text-white",
            p { "© 2025 MyLostPet. All rights reserved." }
            p { "Follow us on social media!" }
            // Add social media links here
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dioxus_core::NoOpMutations;
    use dioxus_ssr::render;

    #[test]
    fn test_footer_component() {
        let mut vdom = VirtualDom::new(|| rsx! { Footer {} });
        let mut mutations = NoOpMutations;
        vdom.rebuild(&mut mutations);
        let html = render(&mut vdom);

        assert!(html.contains("© 2025 MyLostPet. All rights reserved."));
        assert!(html.contains("Follow us on social media!"));
    }
}
