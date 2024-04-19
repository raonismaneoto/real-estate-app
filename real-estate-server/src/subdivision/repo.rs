use postgres::{Row};
use postgres_types::ToSql;

use crate::{database::storage::Storage, error::{app_error::DynAppError}};

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

    // create a batch of lots assuming that the locations already exists
    pub async fn create_lots(&self, lots: Box<[Lot]>) -> Result<u64, DynAppError> {
        let mut amount = lots.len();

        let mut lot_values = String::from("VALUES\n   ");
        for i in 0..amount {
            let base = i*2;
            lot_values += format!("(${}, ${}),\n", base+1, base+2).as_str();
        }
        
        let previous_amount = amount.clone();
        for lot in lots.iter() {
            amount += lot.area.len();
        }
        
        let mut lot_locations_values = String::from("VALUES\n   ");
        for i in previous_amount*2+1..amount {
            let base = if i == previous_amount { i } else { i*3 };
            lot_locations_values += format!("(${}, ${}, ${}),\n", base+1, base+2, base+3).as_str();
        }

        let cmd = format!(
            "INSERT INTO
                lot 
                    (l_name, subdivision_id)
                {};
            
            INSERT INTO
                lot_location
                    (l_name, subdivision_id, location_id)
                {};", lot_values, lot_locations_values
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

        self.storage
            .exec(cmd, &params)
            .await
    }

    pub async fn create_lot(&self, lot: Lot) -> Result<u64, DynAppError>{
        let amount = lot.area.len();
        let start = 3;
        let mut lot_locations_values = String::from("VALUES\n   ");
        for i in start..amount {
            let base = if i == start { i } else { i*3 };
            lot_locations_values += format!("(${}, ${}, ${}),\n", base+1, base+2, base+3).as_str();
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
            {};", lot_locations_values
        );

        let mut params: Vec<&(dyn ToSql + Sync)> = vec![];
        params.push(&lot.name);
        params.push(&lot.subdivision_id);

        for location_id in lot.area.iter() {
            params.push(&lot.name);
            params.push(&lot.subdivision_id);
            params.push(location_id);
        }

        self.storage
            .exec(cmd, &params)
            .await
    }

    pub async fn get_lots_by_subdivision(&self, subdivision_id: String) -> Result<Vec<Lot>, DynAppError> {
        let lot_query_cmd: String = String::from("
            SELECT * 
            FROM lot
            WHERE subdivision_id = $1;
        ");

        let query_result = self.storage.query(lot_query_cmd, &[&subdivision_id]).await;
        
        let lot_location_query_cmd = String::from("
            SELECT *
            FROM lot_location
            WHERE l_name = $1 and subdivision_id = $2;
        ");

        match query_result {
            Ok(result) => {
                let mut lots: Vec<Lot> = vec![];
                for row in result {
                    let lot_name: String = row.get("l_name");
                    let lot_subdivision_id: String = row.get("lot_subdivision_id");
                    let lot_location_query_result = self.storage.query(lot_location_query_cmd.clone(), &[&lot_name, &lot_subdivision_id]).await;
                    match lot_location_query_result {
                        Ok(lot_location_result) => {
                            let mut ids: Vec<String> = vec![];
                            for lot_location in lot_location_result {
                                ids.push(lot_location.get("location_id"));
                            }
                            lots.push(Lot {
                                area: Box::new(ids),
                                name: row.get("l_name"),
                                subdivision_id: row.get("subdivision_id")
                            });
                        },
                        Err(err) => return Err(err)
                    }

                }
                Ok(lots)
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
