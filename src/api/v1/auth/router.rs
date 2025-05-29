use std::sync::Arc;

use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    api::v1::auth::handlers as auth, config, db, state::AppState,
};

use axum::{Router, routing::post};

pub async fn auth_router() -> OpenApiRouter {
    let router = OpenApiRouter::new()
        // .route("/signin", post(auth::sign_in))
        .route("/test", post(auth::test));

    return router;
}

// pub fn auth_router() -> Router {
//     let router = Router::new().route("/signin", post(auth::sign_in));
//     // .route("/signup", post(auth::sign_up));
//     return router;
// }
