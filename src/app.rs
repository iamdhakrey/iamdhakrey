use axum::{Router, routing::post};

use crate::auth;
pub async fn app() -> Router {
    // Create a new Axum application
    let router = Router::new().route("/signin", post(auth::sign_in));
    return router;
}
