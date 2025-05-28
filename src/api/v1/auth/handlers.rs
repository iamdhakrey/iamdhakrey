use axum::{Json, http::StatusCode};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

use crate::api::v1::auth::jwt::encode_jwt;

#[derive(Serialize, Deserialize, Debug)]
// Define the Claims struct to represent the JWT claims
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub provider: String,
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
        (status = 200, description = "Sign in successful", body = String),
        (status = 401, description = "Unauthorized"),
    ),
)]
pub async fn sign_in(
    Json(data): Json<SignInData>, // JSON payload containing sign-in data
) -> Result<Json<String>, StatusCode> {
    // Check if the email and password are valid
    if data.email == "hrithik.d" && data.password == "hrithik@123" {
        // Create a Claims object with the user's information
        let claims = Claims {
            sub: "hrithik.d".to_string(),
            exp: 10000000000,
            provider: "hrithik".to_string(),
        };
        // Return the claims as a JSON response
        let token = encode_jwt(claims)
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        return Ok(Json(token));
    } else {
        // If the email and password are invalid, return a 401 Unauthorized status
        Err(StatusCode::UNAUTHORIZED)
    }
}

// Define the Auth struct to represent the authentication information
#[derive(Serialize, Deserialize, ToSchema)]
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
