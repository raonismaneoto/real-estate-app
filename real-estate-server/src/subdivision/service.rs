use std::vec;

use crate::{
    database::storage::{self, Storage},
    error::app_error::{AppError, DynAppError},
    location::{location::Location, service::LocationService},
    responses::{lot_dto::LotDto, subdivision_dto::SubdivisionDto},
};

use super::{
    lot::Lot,
    repo::{self, SubdivisonRepo},
    subdivision::Subdivision,
};

#[derive(Clone)]
pub struct SubdivisionService {
    repo: SubdivisonRepo,
    location_service: LocationService,
}

impl SubdivisionService {
    pub fn new(storage: Storage, location_service: LocationService) -> Self {
        Self {
            repo: SubdivisonRepo::new(storage),
            location_service: location_service,
        }
    }

    pub async fn create(&self, subdivision_dto: SubdivisionDto) -> Result<String, DynAppError> {
        let location = self
            .location_service
            .get_or_create_location(subdivision_dto.location)
            .await?;

        let subdivision: Subdivision = Subdivision {
            id: subdivision_dto.id,
            location_id: location.id,
            name: subdivision_dto.name,
        };

        self.repo.create(subdivision.clone()).await?;
        Ok(subdivision.id)
    }

    pub async fn create_lot(&self, lot: LotDto) -> Result<String, DynAppError> {
        let mut location_ids: Vec<String> = vec![];

        for coordinates in lot.area.iter() {
            let location = Location {
                id: format!(
                    "{}-{}",
                    coordinates.0.to_string(),
                    coordinates.1.to_string()
                ),
                lat: coordinates.0,
                long: coordinates.1,
            };

            self.location_service
                .create_location(location.clone())
                .await?;
            location_ids.push(location.id)
        }

        let lot_entity = Lot {
            area: Box::new(location_ids),
            name: lot.name,
            subdivision_id: lot.subdivision_id,
        };

        self.repo.create_lot(lot_entity).await?;
        Ok(lot.id)
    }

    pub async fn create_lots(
        &self,
        lots_dtos: Box<[LotDto]>,
    ) -> Result<Box<Vec<Lot>>, DynAppError> {
        let mut lots: Box<Vec<Lot>> = Box::new(vec![]);

        for lot in lots_dtos.iter() {
            let mut location_ids: Vec<String> = vec![];
            for coordinates in lot.area.iter() {
                let location = Location {
                    id: format!(
                        "{}-{}",
                        coordinates.0.to_string(),
                        coordinates.1.to_string()
                    ),
                    lat: coordinates.0,
                    long: coordinates.1,
                };

                self.location_service
                    .create_location(location.clone())
                    .await?;
                location_ids.push(location.id);
            }

            let cloned_lot = lot.clone();
            lots.push(Lot {
                area: Box::new(location_ids),
                name: cloned_lot.name,
                subdivision_id: cloned_lot.subdivision_id,
            });
        }

        self.repo.create_lots(lots.as_slice().into()).await?;
        Ok(lots)
    }

    // pub async fn delete(&self, id: String) -> Result<String, DynAppError> {}

    // pub async fn update(&self, id: String) -> Result<String, DynAppError> {}

    pub async fn search_by_name(&self, name: String) -> Result<Vec<Subdivision>, DynAppError> {
        self.repo.search_by_name(name).await
    }

    pub async fn search_by_location(
        &self,
        coords: (f64, f64),
    ) -> Result<Vec<Subdivision>, DynAppError> {
        self.repo.search_by_location(coords, 5000.0).await
    }
}
