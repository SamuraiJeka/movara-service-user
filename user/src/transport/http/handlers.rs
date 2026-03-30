use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::application::error::ServiceError;
use crate::application::user_service::UserService;
use crate::domain::error::DomainError;
use crate::domain::user::User;

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(email(message = "invalid email"))]
    pub email: String,
    #[validate(length(min = 8, message = "password must be at least 8 characters"))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct GetUserByEmailQuery {
    #[validate(email(message = "invalid email"))]
    pub email: String,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub id: i32,
    pub email: String,
}

fn user_to_response(user: User) -> UserResponse {
    UserResponse {
        id: user.id,
        email: user.email,
    }
}

fn map_service_error(e: ServiceError) -> (StatusCode, String) {
    match &e {
        ServiceError::Domain(DomainError::DuplicateEmail) => (StatusCode::CONFLICT, e.to_string()),
        ServiceError::Domain(DomainError::Internal) | ServiceError::PasswordHash => {
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        }
    }
}

pub async fn register(
    State(service): State<UserService>,
    Json(request): Json<RegisterRequest>,
) -> Result<Json<UserResponse>, Response> {
    request
        .validate()
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()).into_response())?;

    let user = service
        .register(request.email, request.password)
        .await
        .map_err(|e| {
            let (code, msg) = map_service_error(e);
            (code, msg).into_response()
        })?;

    Ok(Json(user_to_response(user)))
}

pub async fn get_user_by_id(
    State(service): State<UserService>,
    Path(user_id): Path<i32>,
) -> Result<Json<UserResponse>, Response> {
    let user = service
    .get_by_id(user_id)
    .await
    .map_err(|e| {
        let (code, msg) = map_service_error(e);
        (code, msg).into_response()
    })?;

    let user = user.ok_or_else(|| StatusCode::NOT_FOUND.into_response())?;

    Ok(Json(user_to_response(user)))
}

pub async fn get_user_by_email(
    State(service): State<UserService>,
    Query(query): Query<GetUserByEmailQuery>,
) -> Result<Json<UserResponse>, Response> {
    if query.email.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "email query parameter is required").into_response());
    }

    let user = service.get_by_email(query.email).await.map_err(|e| {
        let (code, msg) = map_service_error(e);
        (code, msg).into_response()
    })?;

    let user = user.ok_or_else(|| StatusCode::NOT_FOUND.into_response())?;

    Ok(Json(user_to_response(user)))
}
