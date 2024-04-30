use std::sync::Arc;

use axum::{
    extract::{Path, Query, Request, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use axum_macros::debug_handler;

use crate::{
    api_contracts::{
        lot_dto::LotDto, search_subdivision_params::SearchSubdivisionParams,
        subdivision_dto::SubdivisionDto,
    },
    app_state::app_state::AppState,
    error::{
        app_error::{AppError, DynAppError},
        auth::{AuthError, AuthErrorStatusCode},
        default::DefaultAppError,
    },
    subdivision::{self, lot::Lot, subdivision::Subdivision},
};

// #[debug_handler]
pub async fn subdivision_creation_handler(
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<SubdivisionDto>
) -> Response {
    match app_state.subdivision_service.create(payload).await {
        Ok(id) => Json(id).into_response(),
        Err(err) => get_error_response(err)
    }
}

// #[debug_handler]
pub async fn lot_creation_handler(
    State(app_state): State<Arc<AppState>>,
    Path(subdivision_id): Path<String>,
    Json(payload): Json<LotDto>,
) -> Response {
    match app_state.subdivision_service.create_lot(payload).await {
        Ok(id) => Json(id).into_response(),
        Err(err) => get_error_response(err)
    }
}

// #[debug_handler]
pub async fn lots_creation_handler(
    State(app_state): State<Arc<AppState>>,
    Path(subdivision_id): Path<String>,
    Json(payload): Json<Box<[LotDto]>>,
) -> Response {
    match app_state.subdivision_service.create_lots(payload).await {
        Ok(lots) => Json(
            lots.into_iter()
                .map(|elem: Lot| -> String { format!("{}-{}", elem.name, elem.subdivision_id) })
                .collect::<Vec<String>>(),
        )
        .into_response(),
        Err(err) => get_error_response(err),
    }
}

// #[debug_handler]
pub async fn subdivision_listing_handler(
    State(app_state): State<Arc<AppState>>
) -> Response {
    match app_state.subdivision_service.get_all().await {
        Ok(subdivisions) => {
            let mut dtos: Vec<SubdivisionDto> = vec![];
            
            for subdivision in subdivisions.iter() {
                if let Ok(dto) = app_state.subdivision_service.to_dto(subdivision.clone()).await {
                    dtos.push(dto);
                }
            }

            Json(dtos).into_response()
        },
        Err(err) => get_error_response(err),
    }
}

pub async fn subdivision_searching_handler(
    State(app_state): State<Arc<AppState>>,
    params: Query<SearchSubdivisionParams>,
) -> Response {
    match params.name.clone() {
        Some(name) => {
            let maybe_subdivisions = app_state.subdivision_service.search_by_name(name).await;

            match maybe_subdivisions {
                Ok(subdivisions) => {
                    let mut dtos: Vec<SubdivisionDto> = vec![];
                    
                    for subdivision in subdivisions.iter() {
                        if let Ok(dto) = app_state.subdivision_service.to_dto(subdivision.clone()).await {
                            dtos.push(dto);
                        }
                    }
        
                    return Json(dtos).into_response()
                },
                Err(err) => return get_error_response(err),
            };
        }
        None => match params.coords {
            None => {
                return get_error_response(Box::new(DefaultAppError {
                    message: Some(String::from("Invalid searching params")),
                    status_code: 500,
                }))
            }
            Some(coords) => {
                let maybe_subdivisions = app_state
                    .subdivision_service
                    .search_by_location(coords)
                    .await;

                match maybe_subdivisions {
                    Ok(subdivisions) => {
                        let mut dtos: Vec<SubdivisionDto> = vec![];
                        
                        for subdivision in subdivisions.iter() {
                            if let Ok(dto) = app_state.subdivision_service.to_dto(subdivision.clone()).await {
                                dtos.push(dto);
                            }
                        }
            
                        return Json(dtos).into_response()
                    },
                    Err(err) => return get_error_response(err),
                };
            }
        },
    }
}

// pub async fn auth_handler(State(app_state): State<Arc<AppState>>, mut req: Request) -> Request {
//     if req.uri().path().contains("/api/park-here/login")
//         || req.uri().path().contains("/api/park-here/subscribe")
//     {
//         return req;
//     }

//     let maybe_auth_header = req
//         .headers()
//         .get(header::AUTHORIZATION)
//         .and_then(|header| header.to_str().ok());

//     let auth_header = match maybe_auth_header {
//         Some(v) => v,
//         None => {
//             req.extensions_mut().insert(Err::<User, AuthError>(AuthError {
//                 message: Some(String::from("Missing authentication header")),
//                 status_code: AuthErrorStatusCode::UNAUTHORIZED
//             }));
//             return req
//         }
//     };

//     match app_state
//         .auth_service
//         .authorize(auth_header.to_string())
//         .await
//     {
//         Ok(user) => {
//             req.extensions_mut().insert(Ok::<User, AuthError>(user));
//             req
//         }
//         Err(err) => {
//             req.extensions_mut().insert(Err::<User, AuthError>(AuthError {
//                 message: Some(String::from("Invalid authentication token")),
//                 status_code: AuthErrorStatusCode::UNAUTHORIZED
//             }));
//             req
//         },
//     }
// }

// pub async fn login_handler(
//     State(app_state): State<Arc<AppState>>,
//     Json(payload): Json<LoginPayload>,
// ) -> Response {
//     // let login_result = app_state.auth_service.login(payload).await;

//     // match login_result {
//     //     Ok(jwt) => (StatusCode::OK,  Json(jwt)).into_response(),
//     //     Err(err) =>  get_error_response(err)
//     // }
// }

// pub async fn subscribe_handler(
//     State(app_state): State<Arc<AppState>>,
//     Json(payload): Json<SubscriptionPayload>,
// ) -> Response {
//     // let sub_result = app_state.auth_service.subscribe(payload).await;

//     // match sub_result {
//     //     Ok(user) => (StatusCode::CREATED, Json(user)).into_response(),
//     //     Err(err) => get_error_response(err),
//     // }
// }

fn get_error_response(error: DynAppError) -> Response {
    print!("{}", error.status_code());
    match StatusCode::from_u16(error.status_code() as u16) {
        Ok(status_code) => (status_code, Json(error.in_short())).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(error.in_short())).into_response(),
    }
}

// fn get_auth_extension(mut req: Request) -> Result<User, AuthError> {
//     match req.extensions_mut().get::<User>() {
//         Some(user) => Ok(user.clone()),
//         None => {
//             match req.extensions_mut().get::<AuthError>() {
//                 Some(err) => Err(err.clone()),
//                 None => Err(AuthError {
//                     message: Some(String::from("Authentication error")),
//                     status_code: AuthErrorStatusCode::UNAUTHORIZED
//                 })
//             }
//         }
//     }
// }
