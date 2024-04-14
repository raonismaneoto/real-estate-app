use crate::{
    database::storage::Storage,
    error::{app_error::DynAppError, default::DefaultAppError},
};

use super::location::Location;

pub struct LocationRepo {
    storage: Storage,
}

impl LocationRepo {
    pub fn new(storage: Storage) -> Self {
        Self { storage: storage }
    }

    pub async fn get_location(&self, id: String) -> Result<Location, DynAppError> {
        let cmd = String::from(
            "
            SELECT *
            FROM 
                app_location 
            WHERE
                id = $1;",
        );

        match self.storage.query(cmd, &[&id]).await {
            Ok(rows) => {
                if rows.len() != 1 {
                    return Err(Box::new(DefaultAppError {
                        message: Some(format!("Unexpected number of results: {}", rows.len())),
                        status_code: 500,
                    }));
                }

                let location = Location {
                    id: rows[0].get("id"),
                    lat: rows[0].get("lat"),
                    long: rows[0].get("long"),
                };

                Ok(location)
            }
            Err(err) => Err(err),
        }
    }

    pub async fn save_location(&self, location: Location) -> Result<u64, DynAppError> {
        let cmd = String::from(
            "INSERT INTO
                app_location 
                    (id, lat, long)
                VALUES
                    ($1, $2, $3);",
        );

        self.storage
            .exec(cmd, &[&location.id, &location.lat, &location.long])
            .await
    }
}
