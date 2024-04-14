use crate::{
    database::storage::{self, Storage},
    error::app_error::{AppError, DynAppError},
};

use super::{
    lot::Lot, repo::{self, SubdivisonRepo}, subdivision::Subdivision
};

#[derive(Clone)]
pub struct SubdivisionService {
    repo: SubdivisonRepo,
}

impl SubdivisionService {
    pub fn new(storage: Storage) -> Self {
        Self {
            repo: SubdivisonRepo::new(storage),
        }
    }

    pub async fn create(&self, subdivision: Subdivision) -> Result<String, DynAppError> {

    }

    pub async fn add_lot(&self, lot: Lot) -> Result<String, DynAppError> {
        
    }

    pub async fn delete(&self, id: String) -> Result<String, DynAppError> {}

    pub async fn update(&self, id: String) -> Result<String, DynAppError> {}

    pub async fn search_by_name(&self, name: String) -> Result<Vec<Subdivision>, DynAppError> {}

    pub async fn search_by_location(
        &self,
        coords: (f64, f64),
    ) -> Result<Vec<Subdivision>, DynAppError> {
    }
}
