use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct SearchSubdivisionParams {
    pub name: Option<String>,
    pub lat: Option<f64>,
    pub long: Option<f64>,
}
