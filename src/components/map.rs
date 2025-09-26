use dioxus::prelude::*;
use crate::components::pet::Pet;
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
    let locations: Vec<Location> = props.pets.iter().map(|pet| Location {
        lat: pet.latitude,
        lng: pet.longitude,
    }).collect();

    let locations_json = serde_json::to_string(&locations).unwrap_or_else(|_| "[]".to_string());

    rsx! {
        div {
            id: "gmap_canvas",
            class: "w-full h-[500px] my-4 rounded-lg shadow-md",
            "data-locations": "{locations_json}",
        }
    }
}
