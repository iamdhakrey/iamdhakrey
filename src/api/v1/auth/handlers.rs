use axum::{Extension, Json, http::StatusCode};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use tracing::{error, info};
use utoipa::ToSchema;

use crate::{
    api::v1::{auth::jwt::encode_jwt, response::UserCreateResponse},
    entities::user,
    response::{GenericErrorResponse, GenericResponse},
    state::AppState,
};

use super::{
    schema::{SignInData, SignUpData},
    user::create::create_user,
    validators::ValidateJson,
};

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

    let email = data.username.clone();
    let _user = user::Entity::find()
        .filter(user::Column::Email.eq(data.username))
        .one(&state.db)
        .await
        .map_err(|_| {
            GenericResponse::<String>::error(
                "Database Error".to_string(),
                StatusCode::INTERNAL_SERVER_ERROR,
            )
        })?
        .ok_or_else(|| {
            GenericResponse::<String>::error(
                format!("User {} not found", email),
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

#[utoipa::path(
    post,
    path = "/auth/signup",
    request_body = SignUpData,
    responses(
        (status = 201, description = "Sign up successful", body = String),
        (status = 400, description = "Bad request"),
    ),
)]
pub async fn sign_up(
    Extension(state): Extension<AppState>, // Database connection state
    Json(data): Json<SignUpData>, // JSON payload containing sign-up data
) -> Result<GenericResponse<UserCreateResponse>, GenericErrorResponse> {
    let db = &state.db;
    let first_name = data.first_name.clone().unwrap_or_default();
    let last_name = data.last_name.clone().unwrap_or_default();
    match create_user(data, db).await {
        Ok(user) => {
            info!("User created successfully: {:?}", user.username);
            let response = UserCreateResponse {
                id: user.id.clone().unwrap().to_string(),
                username: user.username.clone().unwrap(),
                email: user.email.clone().unwrap(),
                first_name: Some(first_name.clone()),
                last_name: Some(last_name.clone()),
            };
            Ok(GenericResponse::<UserCreateResponse>::success(
                StatusCode::CREATED,
                Some("User Created".to_string()),
                response,
                "success".to_string(),
            ))
        }
        Err(e) => {
            error!("Error creating user:");
            Err(GenericErrorResponse {
                status: "error".to_string(),
                message: e.message,
                resolution: e.resolution,
                status_code: e.status_code,
            })
        }
    }
}
