use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Pet {
    pub id: u32,
    pub name: &'static str,
    pub image: Asset,
    pub sex: &'static str,
    pub age: &'static str,
    pub size: &'static str,
    pub breed: &'static str,
}
