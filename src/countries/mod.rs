use crate::Place;

pub mod albania;
pub mod andorra;
pub mod austria;
pub mod azores;
pub mod poland;
pub mod usa;

pub fn get_data() -> Vec<Place> {
    let mut data = Vec::new();
    data.extend(albania::places());
    data.extend(andorra::places());
    data.extend(austria::places());
    data.extend(azores::places());
    data.extend(poland::places());
    data.extend(usa::places());
    data
}