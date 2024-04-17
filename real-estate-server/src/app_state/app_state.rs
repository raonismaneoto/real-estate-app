use crate::{database::storage::Storage, location::service::LocationService, subdivision::service::SubdivisionService};

#[derive(Clone)]
pub struct AppState {
    pub subdivision_service: SubdivisionService,
    pub storage: Storage,
    pub location_service: LocationService
}

impl AppState {
    pub fn new() -> Self {
        let storage = Storage::new(
            String::from(""),
            String::from(""),
            String::from(""),
            String::from(""),
        );
        let location_service = LocationService::new(storage.clone());
        let subdivision_service = SubdivisionService::new(storage.clone(), location_service.clone());

        Self {
            storage: storage,
            location_service: location_service,
            subdivision_service: subdivision_service,
        }
    }
}
