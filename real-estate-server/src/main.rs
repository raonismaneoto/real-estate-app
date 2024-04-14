use axum::{routing::{get, patch, post}, Error, Router};
use std::{error::Error as StdError, net::SocketAddr};
use tokio::net::TcpListener;

pub mod database;
pub mod subdivision;
pub mod app_state;
pub mod handlers;
pub mod requests;
pub mod auth;
#[tokio::main(flavor = "multi_thread")]
async fn main() {
    start_web_server().await;
}

async fn start_web_server() -> Result<(), Error> {
    let app = Router::new()
        .route(
            "/api/real-estate/health-check",
            get(|| async { "Real Estate server is online" }),
        ); 
        // .route_layer(map_request_with_state(app_state.clone(), auth_handler))
        // .with_state(app_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    let listener = TcpListener::bind(&addr).await.unwrap();

    println!("listening on {}", addr);

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
