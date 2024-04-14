use super::lot::Lot;

pub struct Subdivision {
    id: String,
    name: String,
    location: (f64, f64),
    lots: [Lot]
}