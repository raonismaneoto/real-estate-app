use crate::{database::storage::Storage, subdivision::service::SubdivisionService};

#[derive(Clone)]
pub struct AppState {
    pub subdivision_service: SubdivisionService,
    pub storage: Storage,
}

impl AppState {
    pub fn new() -> Self {
        let storage = Storage::new(
            String::from(""),
            String::from(""),
            String::from(""),
            String::from(""),
        );
        let subdivision_service = SubdivisionService::new(storage.clone());

        Self {
            storage: storage,
            subdivision_service: subdivision_service,
        }
    }
}
