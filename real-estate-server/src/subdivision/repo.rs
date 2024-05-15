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

    pub async fn create(&self, subdivision: Subdivision) -> Result<(), DynAppError> {
        let mut subdivision_locations_values = String::from("VALUES\n   ");
        for location_id in subdivision.area.into_iter() {
            subdivision_locations_values +=
                format!("('{}', '{}'),\n", subdivision.id, location_id).as_str();
        }

        subdivision_locations_values = subdivision_locations_values.trim_end().to_string();
        subdivision_locations_values.pop();

        let cmd = format!(
            "INSERT INTO
                subdivision 
                    (id, s_name)
            VALUES
                ('{}', '{}');
                
            INSERT INTO
                subdivision_location
                    (subdivision_id, location_id)
            {};",
            subdivision.id, subdivision.name, subdivision_locations_values
        );

        self.storage.batch_exec(cmd).await
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

    pub async fn search_by_name(&self, name: String) -> Result<Vec<Row>, DynAppError> {
        let cmd = String::from(
            "
            SELECT 
                s.s_name, s.id, array_agg(lat) as lats, array_agg(long) as longs 
            FROM
                subdivision s 
                join subdivision_location sl on s.id = sl.subdivision_id
                join app_location al on sl.location_id = al.id 
            WHERE
                POSITION(LOWER($1) in LOWER(s.s_name)) > 0;",
        );

         self.storage.query(cmd, &[&name]).await
    }

    pub async fn search_by_location(
        &self,
        coords: (f64, f64),
        radius: f64,
    ) -> Result<Vec<Row>, DynAppError> {
        let cmd = String::from(
            "
            SELECT 
                s.s_name, s.id, array_agg(lat) lats, array_agg(long) longs 
            FROM
                subdivision s 
                join subdivision_location sl on s.id = sl.subdivision_id
                join app_location al on sl.location_id = al.id
            GROUP BY
                s.s_name, s.id
            HAVING 
                (ST_DistanceSphere(
                    ST_MakePoint((array_agg(al.long))[1], (array_agg(al.lat))[1]),
                    ST_MakePoint($1, $2)
                )) <= $3;
            "
        );

        self
            .storage
            .query(cmd, &[&coords.1, &coords.0, &radius])
            .await
    }

    pub async fn get_all_preview(&self) -> Result<Vec<Row>, DynAppError> {
        let cmd = String::from(
            "
            SELECT 
                s.s_name, s.id, count(l.l_name) as lots
            FROM 
                subdivision s 
                left join lot l on l.subdivision_id = s.id 
            GROUP BY 
                s.s_name, s.id;"
        );

        self.storage.query(cmd, &[]).await
    }

    // create a batch of lots assuming that the locations already exists
    pub async fn create_lots(&self, lots: Box<[Lot]>) -> Result<(), DynAppError> {
        let mut lot_values = String::from("VALUES\n   ");
        let mut lot_locations_values = String::from("VALUES\n   ");
        for lot in lots.into_iter() {
            lot_values += format!("('{}', '{}'),\n", lot.name, lot.subdivision_id).as_str();
            for location_id in lot.clone().area.into_iter() {
                lot_locations_values += format!(
                    "('{}', '{}', '{}'),\n",
                    lot.name, lot.subdivision_id, location_id
                )
                .as_str();
            }
        }

        lot_values = lot_values.trim_end().to_string();
        lot_values.pop();
        lot_locations_values = lot_locations_values.trim_end().to_string();
        lot_locations_values.pop();

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

        self.storage.batch_exec(cmd).await
    }

    pub async fn create_lot(&self, lot: Lot) -> Result<(), DynAppError> {
        let mut lot_locations_values = String::from("VALUES\n   ");
        for location_id in lot.area.into_iter() {
            lot_locations_values +=
                format!("('{}', '{}', '{}'),\n", lot.name, lot.subdivision_id, location_id).as_str();
        }

        lot_locations_values = lot_locations_values.trim_end().to_string();
        lot_locations_values.pop();

        let cmd = format!(
            "INSERT INTO
                lot 
                    (l_name, subdivision_id)
            VALUES
                ('{}', '{}');
                
            INSERT INTO
                lot_location
                    (l_name, subdivision_id, location_id)
            {};",
            lot.name, lot.subdivision_id, lot_locations_values
        );

        self.storage.batch_exec(cmd).await
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
    
    pub async fn get_subdivision_lots(&self, subdivision_id : String) -> Result<Vec<Row>, DynAppError> {
        let cmd = String::from(
            "
            SELECT 
                l.l_name, array_agg(lat) as lats, array_agg(long) as longs
            FROM 
                lot l join lot_location ll on l.l_name = ll.l_name and l.subdivision_id = ll.subdivision_id 
                join app_location al on al.id = ll.location_id
            WHERE 
                l.subdivision_id = $1
            GROUP by 
                l.l_name;
            "
        );

        self.storage.query(cmd, &[&subdivision_id]).await
    }
}
