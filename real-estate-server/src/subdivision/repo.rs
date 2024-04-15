use postgres::{Row};

use crate::{database::storage::Storage, error::{app_error::DynAppError}};

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
        let cmd = String::from(
            "
            SELECT *
            FROM 
                subdivision 
            WHERE
                POSITION(LOWER($1) in LOWER(s_name));",
        );

        match self.storage.query(cmd, &[&name]).await {
            Ok(rows) => {
                Ok(parse_subdvisions(rows))
            }
            Err(err) => Err(err),
        }
    }

    pub async fn search_by_location(
        &self,
        coords: (f64, f64),
        radius: f64
    ) -> Result<Vec<Subdivision>, DynAppError> {
        let cmd = String::from(
            "
            SELECT 
                *
            FROM 
                subdivision sd
            WHERE 
                (ST_DistanceSphere(
                    ST_MakePoint((SELECT long FROM app_location where id = sd.location_id), (SELECT lat FROM app_location where id = sd.location_id)),
                    ST_MakePoint($1, $2)
                )) <= $3;",
        );

        match self.storage
            .query(
                cmd,
                &[
                    &coords.1,
                    &coords.0,
                    &radius
                ],
            )
            .await {
                Ok(rows) => {
                    Ok(parse_subdvisions(rows))
                },
                Err(err) => Err(err)
            }
    }
}

fn parse_subdvisions(rows: Vec<Row>) -> Vec<Subdivision> {
    let mut result: Vec<Subdivision> = vec![];

    for row in rows.iter() {
        result.push(Subdivision {
            id: row.get("id"),
            location_id: row.get("location_id"),
            name: row.get("name")
        })
    }

    result
}
