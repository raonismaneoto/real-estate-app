use crate::{
    database::storage::Storage,
    error::{app_error::DynAppError, default::DefaultAppError},
};

use super::{location::Location, repo::LocationRepo};

#[derive(Clone)]
pub struct LocationService {
    repo: LocationRepo,
}

impl LocationService {
    pub fn new(storage: Storage) -> Self {
        Self {
            repo: LocationRepo::new(storage),
        }
    }

    pub async fn get_location(&self, id: String) -> Result<Location, DynAppError> {
        self.repo.get_location(id).await
    }

    pub async fn get_location_by_coords(&self, coords: (f64, f64)) -> Result<Location, DynAppError> {
        self.repo.get_location_by_coords(coords).await
    }

    pub async fn create_location(&self, location: Location) -> Result<Location, DynAppError> {
        let rows_amount = self.repo.save_location(location.clone()).await?;

        if rows_amount == 1 {
            Ok(location)
        } else {
            Err(Box::new(DefaultAppError {
                message: Some(format!(
                    "Unexpected number of rows created: {}",
                    rows_amount
                )),
                status_code: 500,
            }))
        }
    }

    pub async fn get_or_create_location(
        &self,
        coords: (f64, f64),
    ) -> Result<Location, DynAppError> {
        match self.get_location_by_coords(coords).await {
            Ok(location) => Ok(location),
            Err(err) => {
                // need to check the error message
                let created_location = Location {
                    id: format!(
                        "{}-{}",
                        coords.0.to_string(),
                        coords.1.to_string()
                    ),
                    lat: coords.0,
                    long: coords.1,
                };

                self.create_location(created_location.clone()).await?;

                Ok(created_location)
            } 
        }
    }
}
