use crate::components::pet::Pet;
use dioxus::prelude::*;
use serde::Serialize;

#[derive(Serialize, Clone, PartialEq)]
struct Location {
    lat: f64,
    lng: f64,
}

#[derive(Props, Clone, PartialEq)]
pub struct MapProps {
    pub pets: Vec<Pet>,
}

#[component]
pub fn Map(props: MapProps) -> Element {
    let locations: Vec<Location> = props
        .pets
        .iter()
        .map(|pet| Location {
            lat: pet.latitude,
            lng: pet.longitude,
        })
        .collect();

    let locations_json = serde_json::to_string(&locations).unwrap_or_else(|_| "[]".to_string());

    rsx! {
        div {
            id: "gmap_canvas",
            class: "map-container tw-w-full tw-my-4 tw-rounded-lg tw-shadow-md",
            "data-locations": "{locations_json}",
        }
    }
}
