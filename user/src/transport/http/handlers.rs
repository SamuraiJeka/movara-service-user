use axum::{Json, extract::State, response::IntoResponse, http::StatusCode, };
use serde::Deserialize;

use crate::application::user_service::UserService;


#[derive(Deserialize)]
pub struct RegisterSchema {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct GetUserByIdShema {
    pub user_id: i32,
}

#[derive(Deserialize)]
pub struct GetUserByEmailSchema {
    pub email: String,
}


pub async fn register(
    State(service): State<UserService>,
    Json(request): Json<RegisterSchema>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    service
        .register(request.email, request.password)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()));

    Ok(Json("ok"))
}


pub async fn get_user_by_id(
    State(service): State<UserService>,
    Json(request): Json<GetUserByIdShema>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    service
        .get_by_id(request.user_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()));

    Ok(Json("ok"))
}


pub async fn get_user_by_email(
    State(service): State<UserService>,
    Json(request): Json<GetUserByEmailSchema>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    service
        .get_by_email(request.email)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()));

    Ok(Json("ok"))
}
