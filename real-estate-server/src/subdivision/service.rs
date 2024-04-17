use std::vec;

use crate::{
    database::storage::{self, Storage}, error::app_error::{AppError, DynAppError}, location::{location::Location, service::LocationService}, responses::{lot_dto::LotDto, subdivision_dto::SubdivisionDto}
};

use super::{
    lot::Lot, repo::{self, SubdivisonRepo}, subdivision::Subdivision
};

#[derive(Clone)]
pub struct SubdivisionService {
    repo: SubdivisonRepo,
    location_service: LocationService
}

impl SubdivisionService {
    pub fn new(storage: Storage, location_service: LocationService) -> Self {
        Self {
            repo: SubdivisonRepo::new(storage),
            location_service: location_service
        }
    }

    pub async fn create(&self, subdivision: SubdivisionDto) -> Result<String, DynAppError> {
        match subdivision.lots {
            Some(lots) => {

            },
            None => {}
        }
        match self.repo.create(subdivision.clone()).await {
            Ok(rows) => Ok(subdivision.id),
            Err(err) => Err(err)
        }
    }

    pub async fn create_lot(&self, lot: LotDto) -> Result<String, DynAppError> {
        
    }

    pub async fn create_lots(&self, lots_dtos: Box<[LotDto]>) -> Result<Box<Vec<Lot>>, DynAppError> {
        let mut lots: Box<Vec<Lot>> =  Box::new(vec![]);

        for lot in lots_dtos.iter() {
            let mut location_ids: Vec<String> = vec![];
            for coordinates in lot.area.iter() {
                let location = Location {
                    id: format!("{}-{}", coordinates.0.to_string(), coordinates.1.to_string()),
                    lat: coordinates.0,
                    long: coordinates.1
                };

                match self.location_service.create_location(location.clone()).await {
                    Ok(_) => location_ids.push(location.id),
                    Err(err) => return Err(err)
                }
            }

            let cloned_lot = lot.clone();
            lots.push(Lot {
                area: location_ids.as_slice().into(),
                name: cloned_lot.name,
                subdivision_id: cloned_lot.subdivision_id
            });
        }

        match self.repo.create_lots(lots.as_slice().into()).await {
            Ok(_) => Ok(lots),
            Err(err) => Err(err)
        }
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
