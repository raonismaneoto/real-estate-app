use super::lot_dto::LotDto;

#[derive(Clone)]
pub struct SubdivisionDto {
    pub id: String,
    pub name: String,
    pub location: (f64, f64),
    pub lots: Option<Box<[LotDto]>>,
}
