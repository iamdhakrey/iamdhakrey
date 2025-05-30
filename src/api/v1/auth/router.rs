use utoipa_axum::{router::OpenApiRouter, routes};

use crate::api::v1::auth::handlers::{self as auth};

pub async fn auth_router() -> OpenApiRouter {
    let router = OpenApiRouter::new()
        .routes(routes!(auth::sign_in))
        .routes(routes!(auth::sign_up));

    return router;
}
