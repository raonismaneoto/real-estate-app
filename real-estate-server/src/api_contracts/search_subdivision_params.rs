use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct SearchSubdivisionParams {
    pub name: Option<String>,
    pub coords: Option<(f64, f64)>,
}
