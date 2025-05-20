use axum::{Json, http::StatusCode};
use serde::{Deserialize, Serialize};

use crate::jwt::encode_jwt;

#[derive(Serialize, Deserialize, Debug)]
// Define the Claims struct to represent the JWT claims
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub email: String,
    pub username: String,
}

// function for handle signing in
pub async fn sign_in(
    Json(data): Json<SignInData>, // JSON payload containing sign-in data
) -> Result<Json<String>, StatusCode> {
    // Check if the email and password are valid
    if data.email == "hrithik.d" && data.password == "hrithik@123" {
        // Create a Claims object with the user's information
        let claims = Claims {
            sub: "hrithik.d".to_string(),
            exp: 10000000000,
            email: "hrithik.d".to_string(),
            username: "hrithik".to_string(),
        };
        // Return the claims as a JSON response
        let token = encode_jwt(claims).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        return Ok(Json(token));
    } else {
        // If the email and password are invalid, return a 401 Unauthorized status
        Err(StatusCode::UNAUTHORIZED)
    }
}

// Define the Auth struct to represent the authentication information
#[derive(Serialize, Deserialize, Debug)]
pub struct SignInData {
    pub email: String,
    pub password: String,
}
