use serde::{Deserialize, Serialize};

use super::lot_dto::LotDto;

#[derive(Clone, Serialize, Deserialize)]
pub struct SubdivisionDto {
    pub id: String,
    pub name: String,
    pub location: (f64, f64),
    pub lots: Option<Box<Vec<LotDto>>>,
}
