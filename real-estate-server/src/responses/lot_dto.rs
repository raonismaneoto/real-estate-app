#[derive(Clone)]
pub struct LotDto {
    pub area: Box<[(f64, f64)]>,
    pub id: String,
    pub name: String,
    pub subdivision_id: String,
}