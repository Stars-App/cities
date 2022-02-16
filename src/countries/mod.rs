use crate::processor::Place;

pub mod albania;
pub mod andorra;
pub mod poland;

pub fn get_data() -> Vec<Place> {
    let mut data = Vec::new();
    data.extend(albania::PLACES);
    data.extend(andorra::PLACES);
    data.extend(poland::PLACES);
    data
}