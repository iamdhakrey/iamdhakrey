use utoipa_axum::{router::OpenApiRouter, routes};

use crate::api::v1::auth::handlers as auth;

pub fn auth_router() -> OpenApiRouter {
    let router = OpenApiRouter::new()
        .routes(routes!(auth::sign_in,))
        .routes(routes!(auth::test));
    // .route("/signup", post(auth::sign_up));
    return router;
}

// pub fn auth_router() -> Router {
//     let router = Router::new().route("/signin", post(auth::sign_in));
//     // .route("/signup", post(auth::sign_up));
//     return router;
// }
