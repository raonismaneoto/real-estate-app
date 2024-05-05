use std::{
    borrow::Borrow,
    future::{self, IntoFuture},
    vec,
};

use tokio::try_join;

use crate::{
    api_contracts::{lot_dto::LotDto, subdivision_dto::SubdivisionDto, subdivision_preview::SubdivisionPreview},
    database::storage::{self, Storage},
    error::app_error::{AppError, DynAppError},
    location::{location::Location, service::LocationService},
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
        let mut location_ids: Vec<String> = vec![];

        for coords in subdivision_dto.area.into_iter() {
            location_ids.push(
                self.location_service
                    .get_or_create_location(coords)
                    .await?
                    .id,
            );
        }

        let subdivision: Subdivision = Subdivision {
            id: subdivision_dto.id,
            area: Box::new(location_ids),
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

    pub async fn search_by_name(&self, name: String) -> Result<Vec<SubdivisionDto>, DynAppError> {
        let rows = self.repo.search_by_name(name).await?;

        let mut dtos: Vec<SubdivisionDto> = vec![];
        for row in rows.iter() {
            dtos.push(
                SubdivisionDto {
                    area: Box::new(assemble_area(row.get("lats"), row.get("longs"))),
                    id: row.get("id"),
                    name: row.get("s_name"),
                    lots: None
                }
            )
        }

        Ok(dtos)
    }

    pub async fn search_by_location(
        &self,
        coords: (f64, f64),
    ) -> Result<Vec<SubdivisionDto>, DynAppError> {
        let rows = self.repo.search_by_location(coords, 5000.0).await?;

        let mut dtos: Vec<SubdivisionDto> = vec![];
        for row in rows.iter() {
            dtos.push(
                SubdivisionDto {
                    area: Box::new(assemble_area(row.get("lats"), row.get("longs"))),
                    id: row.get("id"),
                    name: row.get("s_name"),
                    lots: None
                }
            )
        }

        Ok(dtos)
    }

    // TODO: the repo and the service are calling the location service to retrieve the location
    // only one needs to do this.
    pub async fn to_dto(&self, subdivision: Subdivision) -> Result<SubdivisionDto, DynAppError> {
        let saved_lots = self
            .repo
            .get_lots_by_subdivision(subdivision.clone().id)
            .await?;

        let mut lot_dtos: Vec<LotDto> = vec![];
        for lot in saved_lots {
            let mut locations: Vec<(f64, f64)> = vec![];
            for loc_id in lot.area.iter() {
                let maybe_curr_location = self.location_service.get_location(loc_id.clone()).await;
                if let Ok(curr_location) = maybe_curr_location {
                    locations.push((curr_location.lat, curr_location.long))
                }
            }

            lot_dtos.push(LotDto {
                id: format!("{}-{}", lot.name, lot.subdivision_id),
                name: lot.name,
                subdivision_id: lot.subdivision_id,
                area: Box::new(locations),
            })
        }

        let mut locations: Vec<Location> = vec![];
        for location_id in subdivision.clone().area.into_iter() {
            locations.push(self.location_service.get_location(location_id).await?);
        }

        Ok(SubdivisionDto {
            id: subdivision.clone().id,
            area: Box::new(
                locations
                    .into_iter()
                    .map(|location| (location.lat, location.long))
                    .collect::<Vec<(f64, f64)>>(),
            ),
            lots: Some(Box::new(lot_dtos)),
            name: subdivision.clone().name,
        })
    }

    // TODO: implement pagination
    pub async fn get_all(&self) -> Result<Vec<SubdivisionPreview>, DynAppError> {
        let rows = self.repo.get_all_preview().await?;

        let mut previews: Vec<SubdivisionPreview> = vec![];
        for row in rows.iter() {
            previews.push(
                SubdivisionPreview {
                    id: row.get("id"),
                    name: row.get("s_name"),
                    lots_amount: row.get("lots")
                }
            )
        }

        Ok(previews)
    }

    pub async fn get_subdivision_lots(&self, subdivision_id : String) -> Result<Vec<LotDto>, DynAppError> {
        let lot_rows = self.repo.get_subdivision_lots(subdivision_id.clone()).await?;
        
        let mut lots: Vec<LotDto> = vec![];
        for row in lot_rows.into_iter() {
            let lot_name: String = row.get("l_name");
            lots.push(
                LotDto {
                    id: format!("{}-{}", lot_name, subdivision_id.clone()),
                    area: Box::new(assemble_area(row.get("lats"), row.get("longs"))),
                    name: lot_name,
                    subdivision_id: subdivision_id.clone()
                }
            )
        }
    
        Ok(lots)
    }
}

fn assemble_area(lats: Vec<f64>, longs: Vec<f64>) -> Vec<(f64, f64)> {
    let mut area: Vec<(f64, f64)> = vec![];
    for (pos, value) in lats.into_iter().enumerate() {
        area.push((value, longs[pos]));
    }
    area
}