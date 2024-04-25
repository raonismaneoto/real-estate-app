use std::vec;

use postgres::Row;
use postgres_types::ToSql;

use crate::{database::storage::Storage, error::app_error::DynAppError};

use super::{lot::Lot, subdivision::Subdivision};

#[derive(Clone)]
pub struct SubdivisonRepo {
    storage: Storage,
}

impl SubdivisonRepo {
    pub fn new(storage: Storage) -> Self {
        Self { storage: storage }
    }

    pub async fn create(&self, subdivision: Subdivision) -> Result<u64, DynAppError> {
        let amount = subdivision.area.len();
        let start = 2;
        let mut subdivision_locations_values = String::from("VALUES\n   ");
        for i in start..amount {
            let base = if i == start { i } else { i * 2 };
            subdivision_locations_values +=
                format!("(${}, ${}),\n", base + 1, base + 2).as_str();
        }

        let cmd = format!(
            "INSERT INTO
                subdivision 
                    (id, s_name)
            VALUES
                ($1, $2);
                
            INSERT INTO
                subdivision_location
                    (subdivision_id, location_id)
            {};",
            subdivision_locations_values
        );

        let mut params: Vec<&(dyn ToSql + Sync)> = vec![];
        params.push(&subdivision.name);
        params.push(&subdivision.id);

        for location_id in subdivision.area.iter() {
            params.push(&subdivision.id);
            params.push(location_id);
        }

        self.storage
            .exec(
                cmd,
                &params,
            )
            .await
    }

    pub async fn delete(&self, id: String) -> Result<u64, DynAppError> {
        let cmd = String::from(
            "DELETE FROM
                subdivision
            WHERE id = $1",
        );

        self.storage.exec(cmd, &[&id]).await
    }

    pub async fn update(&self, id: String, new_name: String) -> Result<u64, DynAppError> {
        let cmd = String::from(
            "UPDATE subdivision
            SET s_name = $1
            WHERE id = $2",
        );

        self.storage.exec(cmd, &[&new_name, &id]).await
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

        let rows = self.storage.query(cmd, &[&name]).await?;

        let select_subdivision_locations_cmd = String::from(
            "SELECT 
                location_id
            FROM
                subdivision_location
            WHERE 
                subdivision_id = $1;"
        );

        let mut subdivisions: Vec<Subdivision> = vec![];

        for row in rows.into_iter() {
            let id: String = row.get("id");
            let locations = self
                .storage
                .query(select_subdivision_locations_cmd.clone(), &[&id])
                .await?
                .into_iter()
                .map(|row| { row.get("location_id")})
                .collect::<Vec<String>>();

            subdivisions.push(
                Subdivision {
                    id: row.get("id"),
                    area: Box::new(locations),
                    name: row.get("name")
                }
            );
        }

        Ok(subdivisions)
    }

    pub async fn search_by_location(
        &self,
        coords: (f64, f64),
        radius: f64,
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

        let subdivision_rows = self
            .storage
            .query(cmd, &[&coords.1, &coords.0, &radius])
            .await?;
        
            let select_subdivision_locations_cmd = String::from(
                "SELECT 
                    location_id
                FROM
                    subdivision_location
                WHERE 
                    subdivision_id = $1;"
            );
    
            let mut subdivisions: Vec<Subdivision> = vec![];
    
            for row in subdivision_rows.into_iter() {
                let id: String = row.get("id");
                let locations = self
                    .storage
                    .query(select_subdivision_locations_cmd.clone(), &[&id])
                    .await?
                    .into_iter()
                    .map(|row| { row.get("location_id")})
                    .collect::<Vec<String>>();
    
                subdivisions.push(
                    Subdivision {
                        id: row.get("id"),
                        area: Box::new(locations),
                        name: row.get("name")
                    }
                );
            }
    
            Ok(subdivisions)
    }

    pub async fn get_all(&self) -> Result<Vec<Subdivision>, DynAppError> {
        let cmd = String::from(
            "
            SELECT *
            FROM 
                subdivision;",
        );

        let rows = self.storage.query(cmd, &[]).await?;

        let select_subdivision_locations_cmd = String::from(
            "SELECT 
                location_id
            FROM
                subdivision_location
            WHERE 
                subdivision_id = $1;"
        );

        let mut subdivisions: Vec<Subdivision> = vec![];

        for row in rows.into_iter() {
            let id: String = row.get("id");
            let locations = self
                .storage
                .query(select_subdivision_locations_cmd.clone(), &[&id])
                .await?
                .into_iter()
                .map(|row| { row.get("location_id")})
                .collect::<Vec<String>>();

            subdivisions.push(
                Subdivision {
                    id: row.get("id"),
                    area: Box::new(locations),
                    name: row.get("name")
                }
            );
        }

        Ok(subdivisions)
    }

    // create a batch of lots assuming that the locations already exists
    pub async fn create_lots(&self, lots: Box<[Lot]>) -> Result<u64, DynAppError> {
        let mut amount = lots.len();

        let mut lot_values = String::from("VALUES\n   ");
        for i in 0..amount {
            let base = i * 2;
            lot_values += format!("(${}, ${}),\n", base + 1, base + 2).as_str();
        }

        let previous_amount = amount.clone();
        for lot in lots.iter() {
            amount += lot.area.len();
        }

        let mut lot_locations_values = String::from("VALUES\n   ");
        for i in previous_amount * 2 + 1..amount {
            let base = if i == previous_amount { i } else { i * 3 };
            lot_locations_values +=
                format!("(${}, ${}, ${}),\n", base + 1, base + 2, base + 3).as_str();
        }

        let cmd = format!(
            "INSERT INTO
                lot 
                    (l_name, subdivision_id)
                {};
            
            INSERT INTO
                lot_location
                    (l_name, subdivision_id, location_id)
                {};",
            lot_values, lot_locations_values
        );

        let mut params: Vec<&(dyn ToSql + Sync)> = vec![];

        for lot in lots.iter() {
            params.push(&lot.name);
            params.push(&lot.subdivision_id);
        }

        for lot in lots.iter() {
            for location_id in lot.area.iter() {
                params.push(&lot.name);
                params.push(&lot.subdivision_id);
                params.push(location_id);
            }
        }

        self.storage.exec(cmd, &params).await
    }

    pub async fn create_lot(&self, lot: Lot) -> Result<u64, DynAppError> {
        let amount = lot.area.len();
        let start = 3;
        let mut lot_locations_values = String::from("VALUES\n   ");
        for i in start..amount {
            let base = if i == start { i } else { i * 3 };
            lot_locations_values +=
                format!("(${}, ${}, ${}),\n", base + 1, base + 2, base + 3).as_str();
        }

        let cmd = format!(
            "INSERT INTO
                lot 
                    (l_name, subdivision_id)
            VALUES
                ($1, $2);
                
            INSERT INTO
                lot_location
                    (l_name, subdivison_id, location_id)
            {};",
            lot_locations_values
        );

        let mut params: Vec<&(dyn ToSql + Sync)> = vec![];
        params.push(&lot.name);
        params.push(&lot.subdivision_id);

        for location_id in lot.area.iter() {
            params.push(&lot.name);
            params.push(&lot.subdivision_id);
            params.push(location_id);
        }

        self.storage.exec(cmd, &params).await
    }

    pub async fn get_lots_by_subdivision(
        &self,
        subdivision_id: String,
    ) -> Result<Vec<Lot>, DynAppError> {
        let lot_query_cmd: String = String::from(
            "
            SELECT * 
            FROM lot
            WHERE subdivision_id = $1;
        ",
        );

        let result = self
            .storage
            .query(lot_query_cmd, &[&subdivision_id])
            .await?;

        let lot_location_query_cmd = String::from(
            "
            SELECT *
            FROM lot_location
            WHERE l_name = $1 and subdivision_id = $2;
        ",
        );

        let mut lots: Vec<Lot> = vec![];
        for row in result {
            let lot_name: String = row.get("l_name");
            let lot_subdivision_id: String = row.get("lot_subdivision_id");
            let lot_location_result = self
                .storage
                .query(
                    lot_location_query_cmd.clone(),
                    &[&lot_name, &lot_subdivision_id],
                )
                .await?;

            let mut ids: Vec<String> = vec![];
            for lot_location in lot_location_result {
                ids.push(lot_location.get("location_id"));
            }
            lots.push(Lot {
                area: Box::new(ids),
                name: row.get("l_name"),
                subdivision_id: row.get("subdivision_id"),
            });
        }

        Ok(lots)
    }
}
