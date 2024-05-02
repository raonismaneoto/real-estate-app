use axum::{
    routing::{get, patch, post},
    Error, Router,
};
use std::{error::Error as StdError, net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;

pub mod api_contracts;
pub mod app_state;
pub mod auth;
pub mod database;
pub mod error;
pub mod handlers;
pub mod location;
pub mod requests;
pub mod subdivision;

use handlers::subdivision::{
    lot_creation_handler, lots_creation_handler, subdivision_creation_handler,
    subdivision_listing_handler, subdivision_searching_handler,
};

use crate::app_state::app_state::AppState;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    start_web_server().await;
}

async fn start_web_server() -> Result<(), Error> {
    let app_state = Arc::new(AppState::new());

    let app = Router::new()
        .route(
            "/api/real-estate/health-check",
            get(|| async { "Real Estate server is online" }),
        )
        .route(
            "/api/real-estate/subdivisions",
            post(subdivision_creation_handler),
        )
        .route(
            "/api/real-estate/subdivisions/:subdivision_id/lots",
            post(lot_creation_handler),
        )
        .route(
            "/api/real-estate/subdivisions/:subdivision_id/lots/batch-creation",
            post(lots_creation_handler),
        )
        .route(
            "/api/real-estate/subdivisions/search",
            get(subdivision_searching_handler),
        )
        .route(
            "/api/real-estate/subdivisions",
            get(subdivision_listing_handler),
        )
        // .route_layer(map_request_with_state(app_state.clone(), auth_handler))
        .with_state(app_state);

    let addr = SocketAddr::from(([192, 168, 0, 7], 5000));
    let listener = TcpListener::bind(&addr).await.unwrap();

    println!("listening on {}", addr);

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
