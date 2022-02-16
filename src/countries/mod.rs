use crate::processor::Place;

pub mod albania;
pub mod andorra;
pub mod austria;
pub mod azores;
pub mod poland;
pub mod usa;

pub fn get_data() -> Vec<Place> {
    let mut data = Vec::new();
    data.extend(albania::PLACES);
    data.extend(andorra::PLACES);
    data.extend(austria::PLACES);
    data.extend(azores::PLACES);
    data.extend(poland::PLACES);
    data.extend(usa::PLACES);
    data
}