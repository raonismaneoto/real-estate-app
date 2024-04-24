use serde::Serialize;

use crate::api_contracts::subdivision_dto::SubdivisionDto;

#[derive(Clone, Serialize)]
pub struct Subdivision {
    pub id: String,
    pub name: String,
    pub location_id: String,
}
