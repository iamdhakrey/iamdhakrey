use axum::Router;
use utoipa_axum::router::OpenApiRouter;

use crate::api::v1::auth;

pub fn v1_route(
    api: utoipa::openapi::OpenApi,
) -> (Router, utoipa::openapi::OpenApi) {
    let (router, api) = OpenApiRouter::with_openapi(api)
        .nest("/v1", auth::router::auth_router())
        .split_for_parts();
    return (router, api);
}
