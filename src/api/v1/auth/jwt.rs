use axum::http::StatusCode;
use chrono::{Duration, Utc};

use crate::api::v1::auth::handlers::Claims;

pub fn encode_jwt(claims: Claims) -> Result<String, StatusCode> {
    let secret: String = "TestSecret".to_string();
    let now = Utc::now();
    let expire = Duration::hours(24);
    let exp = now + expire;
    let exp = exp.timestamp() as usize;
    let claims = Claims {
        sub: claims.sub,
        exp,
        email: claims.email,
        username: claims.username,
    };

    // Encode the claims into a JWT token
    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    // Return the token as a string
    Ok(token)
}

pub fn decode_jwt(token: &str) -> Result<Claims, StatusCode> {
    let secret: String = "TestSecret".to_string();
    // Decode the JWT token
    let token_data = jsonwebtoken::decode::<Claims>(
        token,
        &jsonwebtoken::DecodingKey::from_secret(secret.as_ref()),
        &jsonwebtoken::Validation::default(),
    )
    .map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Extract the claims from the token data
    let claims = token_data.claims;
    // Check if the token is expired
    if claims.exp < Utc::now().timestamp() as usize {
        return Err(StatusCode::UNAUTHORIZED);
    }
    // Return the claims
    Ok(claims)
}
