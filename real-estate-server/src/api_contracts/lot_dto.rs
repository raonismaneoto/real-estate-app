use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct LotDto {
    pub area: Box<Vec<(f64, f64)>>,
    pub id: String,
    pub name: String,
    pub subdivision_id: String,
}
