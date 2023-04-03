use serde::{Deserialize, Serialize};

pub mod files;
pub mod processor;

#[derive(Serialize, Deserialize)]
pub struct Node {
    pub name: String,
    pub place: String,
    pub province: String,
    pub lat: f64,
    pub lon: f64,
}

#[derive(Serialize)]
pub struct Place {
    pub name: &'static str,
    pub lat: f64,
    pub lon: f64,
}
