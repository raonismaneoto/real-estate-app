use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct SubdivisionPreview {
    pub id: String,
    pub name: String,
    pub lots_amount: i32,
}
