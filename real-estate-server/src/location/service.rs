use crate::{database::storage::Storage, error::{app_error::DynAppError, default::DefaultAppError}};

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

    pub async fn create_location(&self, location: Location) -> Result<Location, DynAppError> {
        match self.repo.save_location(location.clone()).await {
            Ok(rows_amount) => {
                if rows_amount == 1 {
                    Ok(location)
                } else {
                    Err(Box::new(DefaultAppError {
                        message: Some(format!("Unexpected number of rows created: {}", rows_amount)),
                        status_code: 500
                    }))
                }
            },
            Err(err) => Err(err)
        }
    }
}
