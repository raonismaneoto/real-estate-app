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

    pub async fn create(&self, subdivision: Subdivision) -> Result<u64, DynAppError> {
        let cmd = String::from(
            "INSERT INTO
                subdivision 
                    (id, s_name, location_id)
            VALUES
                ($1, $2, $3);"
        );

        self.storage
            .exec(cmd, &[&subdivision.id, &subdivision.name, &subdivision.location_id])
            .await
    }

    pub async fn delete(&self, id: String) -> Result<u64, DynAppError> {
        let cmd = String::from(
            "DELETE FROM
                subdivision
            WHERE id = $1"
        );

        self.storage
            .exec(cmd, &[&id])
            .await
    }

    pub async fn update(&self, id: String, new_name: String) -> Result<u64, DynAppError> {
        let cmd = String::from(
            "UPDATE subdivision
            SET s_name = $1
            WHERE id = $2"
        );

        self.storage
            .exec(cmd, &[&new_name, &id])
            .await
    }

    pub async fn search_by_name(&self, name: String) -> Result<Vec<Subdivision>, DynAppError> {

    }

    pub async fn search_by_location(
        &self,
        coords: (f64, f64),
        radius: f64
    ) -> Result<Vec<Subdivision>, DynAppError> {
        
    }
}
