use crate::{
    database::storage::Storage, location::service::LocationService,
    subdivision::service::SubdivisionService,
};

#[derive(Clone)]
pub struct AppState {
    pub subdivision_service: SubdivisionService,
    pub storage: Storage,
    pub location_service: LocationService,
}

impl AppState {
    pub fn new() -> Self {
        let storage = Storage::new(
            String::from("localhost"),
            String::from("postgres"),
            String::from("postgres"),
            String::from("postgres"),
        );
        let location_service = LocationService::new(storage.clone());
        let subdivision_service =
            SubdivisionService::new(storage.clone(), location_service.clone());

        Self {
            storage: storage.clone(),
            location_service: location_service.clone(),
            subdivision_service: subdivision_service.clone(),
        }
    }
}
