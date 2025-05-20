use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response};

pub async fn auth_middleware(req: Request, next: Next) -> Result<Response, StatusCode> {
    // Extract the authorization header from the request
    let auth_header = req.headers().get("Authorization");

    // Check if the authorization header is present
    if let Some(header) = auth_header {
        // Check if the header value starts with "Bearer "
        if header.to_str().unwrap_or("").starts_with("Bearer ") {
            // If the header is valid, call the next middleware or handler
            return Ok(next.run(req).await);
        }
    }

    // If the authorization header is missing or invalid, return an error
    Err(StatusCode::UNAUTHORIZED)
}
