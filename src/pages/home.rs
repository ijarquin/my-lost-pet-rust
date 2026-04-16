use dioxus::prelude::*;

const HERO_IMAGE: Asset = asset!("/assets/images/tirma.jpg");

#[component]
pub fn Home() -> Element {
    rsx! {
        Hero {}
    }
}

#[component]
pub fn Hero() -> Element {
    rsx! {
        div {
            class: "home hero-section tw-flex tw-flex-row tw-p-6",
            div {
                class: "hero-content tw-w-1/2",
                id: "hero",
                h1 { class: "hero-title", id: "subtitle", "A place to find your loving pets" }
                p {
                    class: "hero-description",
                    id: "description",
                    "Find your loving pets, you can filter by regions, time they were lost. We hope you can find your pets as soon as possible." }
                div {
                    id: "links",
                    Link {
                        id: "missing-pets-link",
                        to: "/missing-pets",
                        "😿 Missing Pets"
                    }
                    Link {
                        id: "adoption-pets-link",
                        to: "/adoption-pets",
                        "🐶 Adoption Pets"
                    }
                }
            }
            img {
                id: "image",
                class: "hero-image hero-img tw-w-1/2",
                src: HERO_IMAGE
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
    fn test_hero_component() {
        let mut vdom = VirtualDom::new(Hero);
        let mut mutations = NoOpMutations;
        vdom.rebuild(&mut mutations);
        let html = render(&mut vdom);

        assert!(html.contains("subtitle"));
        assert!(html.contains("description"));
        assert!(html.contains("image"));
    }

    #[test]
    #[ignore]
    fn test_links() {
        let mut vdom = VirtualDom::new(Hero);
        let mut mutations = NoOpMutations;
        vdom.rebuild(&mut mutations);
        let html = render(&mut vdom);

        // Check that the links exist and have the correct href values
        assert!(html.contains(r#"href="/missing-pets""#));
        assert!(html.contains(r#"href="/adoption-pets""#));
        // Optionally, check the link text as well
        assert!(html.contains("😿 Missing Pets"));
        assert!(html.contains("🐶 Adoption Pets"));
    }
}
