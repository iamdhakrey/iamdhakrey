use axum::{Extension, Json, http::StatusCode};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

use crate::{
    api::v1::{auth::jwt::encode_jwt, response::GenericResponse},
    entities::user,
    state::AppState,
};

use super::{schema::SignUpData, validators::ValidateJson};

#[derive(Serialize, Deserialize, Debug)]
// Define the Claims struct to represent the JWT claims
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub provider: String,
}

#[derive(Serialize, ToSchema)]
pub struct SignInResponse {
    pub token: String,
}

// function for handle signing in
#[utoipa::path(
    post,
    summary = "Sign in",
    description = "Sign in to the application",
    tag = "Auth",
    path = "/signin",
    request_body = SignInData,
    responses(
        (status = 200, description = "Sign in successful", body = SignInResponse),
        (status = 401, description = "Unauthorized")
    ),
)]
pub async fn sign_in(
    Extension(state): Extension<AppState>, // Database connection state
    ValidateJson(data): ValidateJson<SignInData>,
) -> Result<Json<String>, GenericResponse<String>> {
    // Check if the email and password are valid
    // let db_state = Extension(AppState);
    // Create a Claims object with the user's information

    let _user = user::Entity::find()
        .filter(user::Column::Email.eq(data.email))
        .one(&*state.db)
        .await
        .map_err(|_| {
            GenericResponse::<String>::error(
                "Database Error".to_string(),
                StatusCode::INTERNAL_SERVER_ERROR,
            )
        })?
        .ok_or_else(|| {
            GenericResponse::<String>::error(
                "{}".to_string(),
                StatusCode::NOT_FOUND,
            )
        })?;

    let claims = Claims {
        sub: "hrithik.d".to_string(),
        exp: 10000000000,
        provider: "hrithik".to_string(),
    };
    // Return the claims as a JSON response
    let token = encode_jwt(claims).map_err(|_| {
        GenericResponse::<String>::error(
            "Failed to Generate Token".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        )
    })?;
    return Ok(Json(token));
}

// Define the Auth struct to represent the authentication information
#[derive(Serialize, Deserialize, ToSchema, Validate)]
pub struct SignInData {
    pub email: String,
    pub password: String,
}

#[utoipa::path(
    get,
    path = "/test",
    request_body = SignInData,
    responses(
        (status = 200, description = "Sign in successful", body = String),
        (status = 401, description = "Unauthorized"),
    ),
)]
pub async fn test() -> Result<Json<String>, StatusCode> {
    // Check if the email and password are valid
    return Ok(Json("hrithik.d".to_string()));
}

#[utoipa::path(
    get,
    path = "/auth/signup",
    request_body = SignUpData,
    responses(
        (status = 201, description = "Sign up successful", body = String),
        (status = 400, description = "Bad request"),
    ),
)]
pub async fn sign_up(
    Json(data): Json<SignUpData>, // JSON payload containing sign-up data
) -> Result<Json<String>, StatusCode> {
    // Here you would typically save the user data to the database
    // For this example, we will just return a success message
    if data.email.is_empty() || data.password.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Simulate saving to the database
    let response_message =
        format!("User {} signed up successfully", data.username);

    Ok(Json(response_message))
}
