use utoipa_axum::{router::OpenApiRouter, routes};

use crate::api::v1::auth::handlers as auth;

pub async fn auth_router() -> OpenApiRouter {
    println!("auth_router called");
    let router = OpenApiRouter::new().routes(routes!(
        auth::sign_in,
        auth::sign_up,
        auth::test,
    ));

    return router;
}
