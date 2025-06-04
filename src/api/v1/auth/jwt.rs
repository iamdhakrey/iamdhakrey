use crate::api::v1::auth::user::info::get_user_by_uuid;
use crate::state::AppState;
use axum::Extension;
use axum::extract::FromRequestParts;
use axum::http::StatusCode;
use axum::http::request::Parts;
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use uuid::Uuid;
// use axum::http::StatusCode;
use crate::entities::user::ActiveModel as UserActiveModel;
use chrono::{Duration, Utc};

use crate::api::v1::auth::handlers::Claims;

pub fn encode_jwt(claims: Claims) -> Result<String, StatusCode> {
    let secret: String = "TestSecret".to_string();
    let now = Utc::now();
    let expire = Duration::hours(24);
    let exp = now + expire;
    let exp = exp.timestamp() as usize;
    let claims =
        Claims { sub: claims.sub, exp, provider: claims.provider };

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

// #[async_trait]
impl<S> FromRequestParts<S> for UserActiveModel
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        // Step 1: Extract Bearer token
        let TypedHeader(Authorization(bearer)) = TypedHeader::<
            Authorization<Bearer>,
        >::from_request_parts(
            parts, state
        )
        .await
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Missing token".into()))?;

        let Extension(AppState { db }) =
            parts.extensions.get::<Extension<AppState>>().cloned().ok_or(
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Missing AppState".into(),
                ),
            )?;

        let db = db.clone();

        // Step 2: Decode the JWT
        let decoded = decode_jwt(bearer.token()).map_err(|_| {
            (StatusCode::UNAUTHORIZED, "Invalid token".into())
        })?;

        let user_id = decoded.sub.parse::<Uuid>().map_err(|_| {
            (StatusCode::UNAUTHORIZED, "Invalid user ID in token".into())
        })?;

        // Step 3: Fetch user from DB
        let user = get_user_by_uuid(user_id, &db).await.map_err(|_| {
            (StatusCode::UNAUTHORIZED, "User not found".into())
        })?;
        Ok(user.into())
    }
}
