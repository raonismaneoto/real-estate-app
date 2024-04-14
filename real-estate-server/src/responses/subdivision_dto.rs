use crate::subdivision::lot::Lot;

pub struct SubdivisionDto {
    id: String,
    name: String,
    location: (f64, f64),
    lots: Box<[Lot]>,
}
