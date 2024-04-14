use crate::{database::storage::Storage, error::app_error::DynAppError};

use super::subdivision::Subdivision;

#[derive(Clone)]
pub struct SubdivisonRepo {
    storage: Storage,
}

impl SubdivisonRepo {
    pub fn new(storage: Storage) -> Self {
        Self { storage: storage }
    }

    pub async fn create(&self, subdivision: Subdivision) -> Result<String, DynAppError> {}

    pub async fn delete(&self, id: String) -> Result<String, DynAppError> {}

    pub async fn update(&self, id: String) -> Result<String, DynAppError> {}

    pub async fn search_by_name(&self, name: String) -> Result<Vec<Subdivision>, DynAppError> {}

    pub async fn search_by_location(
        &self,
        coords: (f64, f64),
    ) -> Result<Vec<Subdivision>, DynAppError> {
    }
}
